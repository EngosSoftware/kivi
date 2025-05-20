//! # Implementation of KIVI deserialization functions

use crate::model::KeyValuePairs;
use normalized_line_endings::Normalized;
use std::path::Path;
use std::{fs, io};

/// Loads key-value pairs from string in KIVI format using
/// quotation mark (U+0022) as a default multiline marker.
///
/// # Examples
///
/// ## Single line keys and values
///
/// ```
/// use kivi::load_from_string;
///
/// let kvp = load_from_string(r#"
///    a
///    b
///    c
///    d
/// "#);
/// assert_eq!("b", kvp.get("a").unwrap());
/// assert_eq!("d", kvp.get("c").unwrap());
/// ```
///
/// ## Multiline key
///
/// ```  
/// use kivi::load_from_string;
///
/// let kvp = load_from_string(r#"
///    "a1
///     a2"
///    b
/// "#);
/// assert_eq!("b", kvp.get("a1\n    a2").unwrap());
/// ```
///
/// ## Multiline value
///
/// ```  
/// use kivi::load_from_string;
///
/// let kvp = load_from_string(r#"
///    a
///    "b1
///     b2"
/// "#);
/// assert_eq!("b1\n    b2", kvp.get("a").unwrap());
/// ```
///
/// ## Multiline key and value
///
/// ```  
/// use kivi::load_from_string;
///
/// let kvp = load_from_string(r#"
///    "a1
///     a2"
///    "b1
///     b2"
/// "#);
/// assert_eq!("b1\n    b2", kvp.get("a1\n    a2").unwrap());
/// ```
pub fn load_from_string(input: &str) -> KeyValuePairs {
  Loader::new(input, &['"']).load()
}

/// Loads key-value pairs from string in KIVI format using
/// custom multiline markers.
///
/// # Examples
///
/// ## Single line keys and values
///
/// ```
/// use kivi::load_from_string_markers;
///
/// let kvp = load_from_string_markers(r#"
///    a
///    b
///    c
///    d
/// "#, &['$']);
/// assert_eq!("b", kvp.get("a").unwrap());
/// assert_eq!("d", kvp.get("c").unwrap());
/// ```
///
/// ## Multiline key
///
/// ```  
/// use kivi::load_from_string_markers;
///
/// let kvp = load_from_string_markers(r#"
///    @a1
///     a2@
///    b
/// "#, &['@']);
/// assert_eq!("b", kvp.get("a1\n    a2").unwrap());
/// ```
///
/// ## Multiline value
///
/// ```  
/// use kivi::load_from_string_markers;
///
/// let kvp = load_from_string_markers(r#"
///    a
///    `b1
///     b2`
/// "#, &['`']);
/// assert_eq!("b1\n    b2", kvp.get("a").unwrap());
/// ```
///
/// ## Multiline key and value
///
/// **NOTE**: You can use different markers for keys and values.
///
/// ```  
/// use kivi::load_from_string_markers;
///
/// let kvp = load_from_string_markers(r#"
///    *a1
///     a2*
///    ^b1
///     b2^
/// "#, &['^', '*']);
/// assert_eq!("b1\n    b2", kvp.get("a1\n    a2").unwrap());
/// ```
pub fn load_from_string_markers(input: &str, markers: &[char]) -> KeyValuePairs {
  Loader::new(input, markers).load()
}

/// Loads key-value pairs from file in KIVI format.
///
/// The default multiline key or value marker is a quotation mark (U+0022).
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
///     assert_eq!("127.0.0.1", host);
///     let default_description = "".to_string();
///     let description = kvp.get("description").unwrap_or(&default_description);
///     assert_eq!("Multiline\n description", description);
///     Ok(())
/// }
/// ```
pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<KeyValuePairs> {
  Ok(load_from_string(&fs::read_to_string(path)?))
}

/// Loads key-value pairs from file in KIVI format using
/// custom multiline markers.
///
/// # Examples
///
/// ```
/// use std::io;
/// use kivi::load_from_file_markers;
///
/// fn main() -> io::Result<()> {
///     let kvp = load_from_file_markers("./tests/data/issues.kivi", &['@','~','^'])?;
///     assert_eq!("Build a separate\n server", kvp.get("Issue1").unwrap());
///     assert_eq!("Develop a new\n compiler", kvp.get("Issue2").unwrap());
///     Ok(())
/// }
/// ```
pub fn load_from_file_markers<P: AsRef<Path>>(path: P, markers: &[char]) -> io::Result<KeyValuePairs> {
  Ok(load_from_string_markers(&fs::read_to_string(path)?, markers))
}

/// Line feed character.
pub const LF: char = '\n';

/// Empty character (zero).
pub const NULL: char = 0 as char;

/// Loader states.
#[derive(Copy, Clone)]
enum State {
  Key,
  KeyExt,
  Value,
  ValueExt,
}

struct Loader<'a> {
  state: State,
  buffer: String,
  key: String,
  marker: char,
  markers: &'a [char],
  input: &'a str,
  output: KeyValuePairs,
}

impl<'a> Loader<'a> {
  /// Created a loader with default settings.
  fn new(input: &'a str, markers: &'a [char]) -> Self {
    Loader {
      state: State::Key,
      buffer: String::new(),
      key: String::new(),
      marker: 0 as char,
      markers,
      input,
      output: KeyValuePairs::new(),
    }
  }

  /// Loads key-value pairs from string.
  fn load(mut self) -> KeyValuePairs {
    let mut chars = self.input.chars().normalized().peekable();
    loop {
      let current_char = chars.next().unwrap_or(NULL);
      if current_char == NULL {
        return self.output;
      }
      let next_char = chars.peek().cloned().unwrap_or(NULL);
      match self.state {
        State::Key => match (current_char, next_char) {
          (ch, _) if self.is_allowed_marker(ch) => {
            self.marker = ch;
            self.clear_buffer(State::KeyExt);
          }
          (LF, _) => self.consume_non_empty_key(),
          (ch, _) => self.consume_char(ch),
        },
        State::KeyExt => match (current_char, next_char) {
          (ch, LF) if self.is_marker(ch) => self.consume_key(),
          (ch, _) => self.consume_char(ch),
        },
        State::Value => match (current_char, next_char) {
          (ch, _) if self.is_allowed_marker(ch) => {
            self.marker = ch;
            self.clear_buffer(State::ValueExt);
          }
          (LF, _) => self.consume_non_empty_value(),
          (ch1, ch2) => {
            self.consume_char(ch1);
            if ch2 == NULL {
              self.consume_non_empty_value();
            }
          }
        },
        State::ValueExt => match (current_char, next_char) {
          (ch, LF) if self.is_marker(ch) => self.consume_value(),
          (ch, _) => self.consume_char(ch),
        },
      }
    }
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

  fn consume_non_empty_value(&mut self) {
    self.buffer = self.buffer.trim().to_string();
    if !self.buffer.is_empty() {
      self.consume_value();
    }
  }

  fn consume_value(&mut self) {
    self.output.key_value_pairs.insert(self.key.clone(), self.buffer.clone());
    self.output.ordered_keys.push(self.key.clone());
    self.output.ordered_values.push(self.buffer.clone());
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
  fn is_allowed_marker(&self, ch: char) -> bool {
    self.markers.contains(&ch)
  }

  /// Returns `true` when specified character is current marker.
  fn is_marker(&self, ch: char) -> bool {
    ch == self.marker
  }
}
