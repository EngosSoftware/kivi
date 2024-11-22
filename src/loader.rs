//! # Loader

use crate::model::KeyValuePairs;
use std::path::Path;
use std::{fs, io};

enum State {
  Key,
  KeyExt,
  Value,
  ValueExt,
}

pub fn load_from_string(input: &str) -> KeyValuePairs {
  let mut output = KeyValuePairs::new();
  let mut state = State::Key;
  let mut buffer = String::new();
  let mut key = String::new();
  let mut chars = input.chars().peekable();

  while let Some(ch) = chars.next() {
    match state {
      State::Key => match ch {
        '"' => {
          buffer.clear();
          state = State::KeyExt;
        }
        '\n' => {
          buffer = buffer.trim().to_string();
          if !buffer.is_empty() {
            key.clear();
            key.push_str(&buffer);
            buffer.clear();
            state = State::Value;
          }
        }
        other => buffer.push(other),
      },
      State::KeyExt => match ch {
        '"' => {
          key.clear();
          key.push_str(&buffer);
          buffer.clear();
          state = State::Value;
        }
        '\\' => {
          if let Some(nch) = chars.peek() {
            if *nch == '"' {
              buffer.push('"');
              chars.next();
              continue; // WATCH OUT!
            }
          }
          buffer.push('\\');
        }
        other => buffer.push(other),
      },
      State::Value => match ch {
        '"' => {
          buffer.clear();
          state = State::ValueExt;
        }
        '\n' => {
          buffer = buffer.trim().to_string();
          if !buffer.is_empty() {
            // Consume the key and value.
            output.kv.insert(key.clone(), buffer.clone());
            key.clear();
            buffer.clear();
            state = State::Key;
          }
          buffer.clear();
        }
        other => {
          buffer.push(other);
          if chars.peek().is_none() {
            // If there are no more characters on input,
            // then consume the key and value.
            buffer = buffer.trim().to_string();
            if !buffer.is_empty() {
              output.kv.insert(key.clone(), buffer.clone());
              key.clear();
              buffer.clear();
              state = State::Key;
            }
            buffer.clear();
          }
        }
      },
      State::ValueExt => match ch {
        '"' => {
          output.kv.insert(key.clone(), buffer.clone());
          key.clear();
          buffer.clear();
          state = State::Key;
        }
        '\\' => {
          if let Some(nch) = chars.peek() {
            if *nch == '"' {
              buffer.push('"');
              chars.next();
              continue; // WATCH OUT!
            }
          }
          buffer.push('\\');
        }
        other => buffer.push(other),
      },
    }
  }
  output
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<KeyValuePairs> {
  Ok(load_from_string(&fs::read_to_string(path)?))
}
