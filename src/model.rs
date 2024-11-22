//! # Key-value pairs

use std::collections::hash_map::{IntoIter, Keys, Values};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValuePairs {
  pub(crate) kv: HashMap<String, String>,
}

impl KeyValuePairs {
  pub(crate) fn new() -> Self {
    Self { kv: HashMap::new() }
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.kv.get(key)
  }

  pub fn is_empty(&self) -> bool {
    self.kv.is_empty()
  }

  pub fn len(&self) -> usize {
    self.kv.len()
  }

  pub fn keys(&self) -> Keys<'_, String, String> {
    self.kv.keys()
  }

  pub fn values(&self) -> Values<'_, String, String> {
    self.kv.values()
  }
}

impl IntoIterator for KeyValuePairs {
  type Item = (String, String);
  type IntoIter = IntoIter<String, String>;

  fn into_iter(self) -> Self::IntoIter {
    self.kv.into_iter()
  }
}
