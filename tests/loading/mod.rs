mod test_load_from_file;
mod test_load_from_string;

use kivi::load_from_string;
use std::collections::BTreeMap;

fn eq(input: &str, expected: &str) {
  let mut items = vec![];
  for (k, v) in &load_from_string(input).into_iter().collect::<BTreeMap<String, String>>() {
    let mut buffer = String::new();
    buffer.push('"');
    buffer.push_str(k);
    buffer.push('"');
    buffer.push(':');
    buffer.push(' ');
    buffer.push('"');
    buffer.push_str(v);
    buffer.push('"');
    items.push(buffer);
  }
  let mut actual = "{".to_string();
  actual.push_str(&items.join(", "));
  actual.push('}');
  assert_eq!(actual, expected);
}

fn eqn(input: &str, expected: &str) {
  assert_eq!(format!("{:?}", load_from_string(input).into_iter().collect::<BTreeMap<String, String>>()), expected);
}
