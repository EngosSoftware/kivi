use super::*;
use kivi::load_from_string;

#[test]
fn getting_empty_value_should_work() {
  let kvp = load_from_string("");
  assert!(kvp.is_empty());
  assert_eq!(0, kvp.len());
  assert_eq!(None, kvp.get("a"));
}

#[test]
fn getting_non_empty_value_should_work() {
  let kvp = load_from_string("a\nb\n");
  assert!(!kvp.is_empty());
  assert_eq!(1, kvp.len());
  assert_eq!("b", kvp.get("a").unwrap());
}

#[test]
fn getting_keys_should_work() {
  let kvp = load_from_string("a\nb\nc\nd\n");
  assert!(!kvp.is_empty());
  assert_eq!(2, kvp.len());
  assert_eq!("b", kvp.get("a").unwrap());
  assert_eq!("d", kvp.get("c").unwrap());
  let mut keys = kvp.keys().map(|s| s.to_owned()).collect::<Vec<String>>();
  keys.sort();
  assert_eq!("a,c", keys.join(","));
}

#[test]
fn getting_values_should_work() {
  let kvp = load_from_string("a\nb\nc\nd\n");
  assert!(!kvp.is_empty());
  assert_eq!(2, kvp.len());
  assert_eq!("b", kvp.get("a").unwrap());
  assert_eq!("d", kvp.get("c").unwrap());
  let mut values = kvp.values().map(|s| s.to_owned()).collect::<Vec<String>>();
  values.sort();
  assert_eq!("b,d", values.join(","));
}

#[test]
fn values_from_properties_file_should_work() {
  let kvp = load_from_string(DATA_PROPERTIES);
  assert!(!kvp.is_empty());
  assert_eq!(1, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  let mut values = kvp.values().map(|s| s.to_owned()).collect::<Vec<String>>();
  values.sort();
  assert_eq!("127.0.0.1", values.join(","));
}

#[test]
fn values_from_file_001_should_work() {
  let kvp = load_from_string(DATA_001);
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
  let mut values = kvp.values().map(|s| s.to_owned()).collect::<Vec<String>>();
  values.sort();
  assert_eq!("127.0.0.1,12ms,54321", values.join(","));
}

#[test]
fn values_from_file_002_should_work() {
  let kvp = load_from_string(DATA_002);
  assert!(!kvp.is_empty());
  assert_eq!(4, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
  assert_eq!(
    "This configuration file\n should be placed in the same\n directory where the server's\n binary is placed.",
    kvp.get("General\n description").unwrap()
  );
  let mut values = kvp.values().map(|s| s.to_owned()).collect::<Vec<String>>();
  values.sort();
  assert_eq!(
    "127.0.0.1,12ms,54321,This configuration file\n should be placed in the same\n directory where the server's\n binary is placed.",
    values.join(",")
  );
}

#[test]
fn values_from_file_003_should_work() {
  let kvp = load_from_string(DATA_003);
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
  let mut values = kvp.values().map(|s| s.to_owned()).collect::<Vec<String>>();
  values.sort();
  assert_eq!("127.0.0.1,12ms,54321", values.join(","));
}
