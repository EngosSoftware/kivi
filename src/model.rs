//! # Data model for key-value pairs

use std::collections::hash_map::{IntoIter, Keys, Values};
use std::collections::HashMap;

/// A struct representing key-value pairs deserialized from KIVI format.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValuePairs {
  pub(crate) kv: HashMap<String, String>,
}

impl KeyValuePairs {
  /// Creates an empty set of key-value pairs.
  pub(crate) fn new() -> Self {
    Self { kv: HashMap::new() }
  }

  /// Returns the value associated with the specified key.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("a\nb\n");
  /// assert_eq!("b", kvp.get("a").unwrap());
  ///
  /// let kvp = load_from_string("");
  /// assert_eq!(None, kvp.get("a"));
  /// ```
  pub fn get(&self, key: &str) -> Option<&String> {
    self.kv.get(key)
  }

  /// Returns [true] when the set of key-value pairs is empty.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("a\nb\n");
  /// assert_eq!(false, kvp.is_empty());
  ///
  /// let kvp = load_from_string("");
  /// assert_eq!(true, kvp.is_empty());
  /// ```
  pub fn is_empty(&self) -> bool {
    self.kv.is_empty()
  }

  /// Returns the number of key-value pairs.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("a\nb\n");
  /// assert_eq!(1, kvp.len());
  ///
  /// let kvp = load_from_string("");
  /// assert_eq!(0, kvp.len());
  /// ```
  pub fn len(&self) -> usize {
    self.kv.len()
  }

  /// Returns the iterator over the keys.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("a\nb\nc\nd\n");
  /// for key in kvp.keys() {
  ///   assert!(key == "a" || key == "c");
  /// }
  /// ```
  pub fn keys(&self) -> Keys<'_, String, String> {
    self.kv.keys()
  }

  /// Returns the iterator over the values.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("a\nb\nc\nd");
  /// for key in kvp.values() {
  ///   assert!(key == "b" || key == "d");
  /// }
  /// ```
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
