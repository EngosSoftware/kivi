use kivi::load_from_string;

#[test]
fn cloning_should_work() {
  let input = "";
  let kv1 = load_from_string(input);
  let kv2 = kv1.clone();
  assert_eq!(kv1, kv2);
}
