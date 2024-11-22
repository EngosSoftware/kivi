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
