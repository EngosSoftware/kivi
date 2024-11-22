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

macro_rules! consume_char {
  ($buffer:ident, $ch:expr) => {{
    $buffer.push($ch);
  }};
}

macro_rules! consume_key {
  ($key:ident, $buffer:ident, $state:ident) => {{
    $key.clear();
    $key.push_str(&$buffer);
    $buffer.clear();
    $state = State::Value;
  }};
}

macro_rules! consume_non_empty_key {
  ($key:ident, $buffer:ident, $state:ident) => {{
    $buffer = $buffer.trim().to_string();
    if !$buffer.is_empty() {
      consume_key!($key, $buffer, $state);
    }
  }};
}

macro_rules! consume_value {
  ($output:ident, $key:ident, $buffer:ident, $state:ident) => {{
    $output.kv.insert($key.clone(), $buffer.clone());
    $key.clear();
    $buffer.clear();
    $state = State::Key;
  }};
}

macro_rules! consume_non_empty_value {
  ($output:ident, $key:ident, $buffer:ident, $state:ident) => {{
    $buffer = $buffer.trim().to_string();
    if !$buffer.is_empty() {
      consume_value!($output, $key, $buffer, $state);
    }
  }};
}

macro_rules! consume_escaped_quotation_mark {
  ($chars:ident, $buffer:ident) => {{
    if let Some(ch) = $chars.peek() {
      if *ch == '"' {
        $buffer.push('"');
        $chars.next();
      } else {
        $buffer.push('\\');
      }
    } else {
      $buffer.push('\\');
    }
  }};
}

macro_rules! clear_buffer {
  ($buffer:ident, $state:ident, $next_state:expr) => {{
    $buffer.clear();
    $state = $next_state;
  }};
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
        '"' => clear_buffer!(buffer, state, State::KeyExt),
        '\n' => consume_non_empty_key!(key, buffer, state),
        other => consume_char!(buffer, other),
      },
      State::KeyExt => match ch {
        '"' => consume_key!(key, buffer, state),
        '\\' => consume_escaped_quotation_mark!(chars, buffer),
        other => consume_char!(buffer, other),
      },
      State::Value => match ch {
        '"' => clear_buffer!(buffer, state, State::ValueExt),
        '\n' => consume_non_empty_value!(output, key, buffer, state),
        other => {
          consume_char!(buffer, other);
          if chars.peek().is_none() {
            consume_non_empty_value!(output, key, buffer, state);
          }
        }
      },
      State::ValueExt => match ch {
        '"' => consume_value!(output, key, buffer, state),
        '\\' => consume_escaped_quotation_mark!(chars, buffer),
        other => consume_char!(buffer, other),
      },
    }
  }
  output
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<KeyValuePairs> {
  Ok(load_from_string(&fs::read_to_string(path)?))
}
