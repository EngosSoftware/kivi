use kivi::load_from_file;

#[test]
fn loading_from_file_should_work() {
  let kvp = load_from_file("./tests/loading/data/data01.kivi").unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
}

#[test]
fn loading_multi_line_from_file_should_work() {
  let kvp = load_from_file("./tests/loading/data/data02.kivi").unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(4, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
  assert_eq!(
    "This configuration file\n should be placed in the same\n directory where the servers\n binary is placed.",
    kvp.get("General\n description").unwrap()
  );
}
