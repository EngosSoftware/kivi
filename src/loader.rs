//! # Implementation of KIVI deserialization functions

use crate::model::KeyValuePairs;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;
use std::{fs, io};

/// Loads key-value pairs from string in KIVI format.
///
/// # Examples
///
/// ```
/// use kivi::load_from_string;
///
/// let kvp = load_from_string("a\nb\nc\nd\n");
/// for key in kvp.values() {
///   assert!(key == "b" || key == "d");
/// }
/// ```
pub fn load_from_string(input: &str) -> KeyValuePairs {
  Loader::new().load(input, &['"'])
}

/// Loads key-value pairs from KIVI file.
///
/// # Examples
///
/// ```
/// use std::io;
/// use kivi::load_from_file;
///
/// fn main() -> io::Result<()> {
///     let kvp = load_from_file("./tests/data/properties.kivi")?;
///     let default_host = "0.0.0.0".to_string();
///     let host = kvp.get("host").unwrap_or(&default_host);
///     println!("host: {}", host);
///     Ok(())
/// }
/// ```
pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<KeyValuePairs> {
  Ok(load_from_string(&fs::read_to_string(path)?))
}

/// Loader states.
#[derive(Copy, Clone)]
enum State {
  Key,
  KeyExt,
  Value,
  ValueExt,
}

struct Loader {
  state: State,
  buffer: String,
  key: String,
  marker: char,
  output: KeyValuePairs,
}

impl Loader {
  /// Created a loader with default settings.
  fn new() -> Self {
    Loader {
      state: State::Key,
      buffer: String::new(),
      key: String::new(),
      marker: 0 as char,
      output: KeyValuePairs::new(),
    }
  }

  /// Loads key-value pairs from string.
  fn load(mut self, input: &str, markers: &[char]) -> KeyValuePairs {
    let chars = &mut input.chars().peekable();
    while let Some(ch) = chars.next() {
      match self.state {
        State::Key => match ch {
          ch if self.is_allowed_marker(ch, markers) => {
            self.marker = ch;
            self.clear_buffer(State::KeyExt);
          }
          '\n' => self.consume_non_empty_key(),
          other => self.consume_char(other),
        },
        State::KeyExt => match ch {
          ch if self.is_marker(ch) => self.consume_key(),
          '\\' => self.consume_escaped_quotation_mark(chars),
          other => self.consume_char(other),
        },
        State::Value => match ch {
          ch if self.is_allowed_marker(ch, markers) => {
            self.marker = ch;
            self.clear_buffer(State::ValueExt);
          }
          '\n' => self.consume_non_empty_value(),
          other => {
            self.consume_char(other);
            if chars.peek().is_none() {
              self.consume_non_empty_value();
            }
          }
        },
        State::ValueExt => match ch {
          ch if self.is_marker(ch) => self.consume_value(),
          '\\' => self.consume_escaped_quotation_mark(chars),
          other => self.consume_char(other),
        },
      }
    }
    self.output
  }

  /// Consumes the specified character.
  fn consume_char(&mut self, ch: char) {
    self.buffer.push(ch);
  }

  fn consume_key(&mut self) {
    self.key.clear();
    self.key.push_str(&self.buffer);
    self.buffer.clear();
    self.state = State::Value;
  }

  fn consume_non_empty_key(&mut self) {
    self.buffer = self.buffer.trim().to_string();
    if !self.buffer.is_empty() {
      self.consume_key();
    }
  }

  fn consume_escaped_quotation_mark(&mut self, chars: &mut Peekable<Chars>) {
    if let Some(ch) = chars.peek() {
      if *ch == self.marker {
        self.buffer.push(self.marker);
        chars.next();
      } else {
        self.buffer.push('\\');
      }
    } else {
      self.buffer.push('\\');
    }
  }

  fn consume_non_empty_value(&mut self) {
    self.buffer = self.buffer.trim().to_string();
    if !self.buffer.is_empty() {
      self.consume_value();
    }
  }

  fn consume_value(&mut self) {
    self.output.kv.insert(self.key.clone(), self.buffer.clone());
    self.key.clear();
    self.buffer.clear();
    self.state = State::Key;
  }

  /// Clears the input buffer and switches to the next state.
  fn clear_buffer(&mut self, next_state: State) {
    self.buffer.clear();
    self.state = next_state;
  }

  /// Returns `true` when specified character is an allowed marker.
  fn is_allowed_marker(&self, ch: char, markers: &[char]) -> bool {
    markers.contains(&ch)
  }

  /// Returns `true` when specified character is current marker.
  fn is_marker(&self, ch: char) -> bool {
    ch == self.marker
  }
}
