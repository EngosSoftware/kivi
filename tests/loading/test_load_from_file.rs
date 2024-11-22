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
