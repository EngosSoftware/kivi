//! # Data model for key-value pairs

use std::collections::hash_map::{IntoIter, Keys, Values};
use std::collections::HashMap;
use std::slice::Iter;

/// A struct representing key-value pairs deserialized from KIVI format.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValuePairs {
  pub(crate) key_value_pairs: HashMap<String, String>,
  pub(crate) ordered_keys: Vec<String>,
  pub(crate) ordered_values: Vec<String>,
}

impl KeyValuePairs {
  /// Creates an empty set of key-value pairs.
  pub(crate) fn new() -> Self {
    Self {
      key_value_pairs: HashMap::new(),
      ordered_keys: vec![],
      ordered_values: vec![],
    }
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
    self.key_value_pairs.get(key)
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
    self.key_value_pairs.is_empty()
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
    self.key_value_pairs.len()
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
    self.key_value_pairs.keys()
  }

  /// Returns the iterator over ordered keys.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("d\nc\nb\na\n");
  /// assert_eq!("d,b", kvp.ordered_keys().map(|s| s.to_owned()).collect::<Vec<String>>().join(","));
  /// ```
  pub fn ordered_keys(&self) -> Iter<'_, String> {
    self.ordered_keys.iter()
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
    self.key_value_pairs.values()
  }

  /// Returns the iterator over ordered values.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("d\nc\nb\na\n");
  /// assert_eq!("c,a", kvp.ordered_values().map(|s| s.to_owned()).collect::<Vec<String>>().join(","));
  /// ```
  pub fn ordered_values(&self) -> Iter<'_, String> {
    self.ordered_values.iter()
  }

  /// Returns the iterator over ordered key-value pairs.
  ///
  /// # Examples
  ///
  /// ```
  /// use kivi::load_from_string;
  ///
  /// let kvp = load_from_string("d\nc\nb\na\n");
  /// assert_eq!("d:c,b:a", kvp.ordered_key_value_pairs().map(|(k,v)| format!("{}:{}", k,v)).collect::<Vec<String>>().join(","));
  /// ```
  pub fn ordered_key_value_pairs(&self) -> impl Iterator<Item = (&String, &String)> {
    self.ordered_keys.iter().zip(self.ordered_values.iter())
  }
}

impl IntoIterator for KeyValuePairs {
  type Item = (String, String);
  type IntoIter = IntoIter<String, String>;

  fn into_iter(self) -> Self::IntoIter {
    self.key_value_pairs.into_iter()
  }
}
