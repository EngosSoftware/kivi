use super::*;
use kivi::{load_from_file, load_from_file_markers};

#[test]
fn loading_from_properties_file_should_work() {
  let kvp = load_from_file(FILE_PROPERTIES).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(2, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("Multiline\n description", kvp.get("description").unwrap());
  let mut keys = kvp.keys().map(|s| s.to_owned()).collect::<Vec<String>>();
  keys.sort();
  assert_eq!("description,host", keys.join(","));
  let mut values = kvp.values().map(|s| s.to_owned()).collect::<Vec<String>>();
  values.sort();
  assert_eq!("127.0.0.1,Multiline\n description", values.join(","));
}

#[test]
fn loading_from_issues_file_should_work() {
  let kvp = load_from_file_markers(FILE_ISSUES, &['@', '~', '^']).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(2, kvp.len());
  assert_eq!("Build a separate\n server", kvp.get("Issue1").unwrap());
  assert_eq!("Develop a new\n compiler", kvp.get("Issue2").unwrap());
  let mut keys = kvp.keys().map(|s| s.to_owned()).collect::<Vec<String>>();
  keys.sort();
  assert_eq!("Issue1,Issue2", keys.join(","));
  let mut values = kvp.values().map(|s| s.to_owned()).collect::<Vec<String>>();
  values.sort();
  assert_eq!("Build a separate\n server,Develop a new\n compiler", values.join(","));
}

#[test]
fn loading_from_file_should_work() {
  let kvp = load_from_file(FILE_001).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
}

#[test]
fn loading_multi_line_from_file_should_work() {
  let kvp = load_from_file(FILE_002).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(4, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
  assert_eq!(
    "This configuration file\n should be placed in the same\n directory where the server's\n binary is placed.",
    kvp.get("General\n description").unwrap()
  );
}

#[test]
fn loading_from_file_003_should_work() {
  let kvp = load_from_file(FILE_003).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
}

#[test]
fn loading_from_file_004_should_work() {
  let kvp = load_from_file(FILE_004).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
}

#[test]
fn loading_from_file_005_should_work() {
  let kvp = load_from_file(FILE_005).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("127.0.0.1", kvp.get("host").unwrap());
  assert_eq!("54321", kvp.get("port").unwrap());
  assert_eq!("12ms", kvp.get("timeout").unwrap());
}

#[test]
fn loading_from_file_006_should_work() {
  let kvp = load_from_file(FILE_006).unwrap();
  assert!(!kvp.is_empty());
  assert_eq!(3, kvp.len());
  assert_eq!("the name of the \"host\"", kvp.get("ho\\st").unwrap());
  assert_eq!("\"port\" number", kvp.get("port").unwrap());
  assert_eq!("this \"is\" timeout", kvp.get("time\"out").unwrap());
}

#[cfg(not(target_os = "windows"))]
#[test]
fn loading_from_non_existing_file_should_fail() {
  assert!(load_from_file("non-existing.kivi").is_err());
  assert_eq!(
    "No such file or directory (os error 2)",
    load_from_file("../data/non-existing.kivi").unwrap_err().to_string()
  )
}

#[cfg(target_os = "windows")]
#[test]
fn loading_from_non_existing_file_should_fail() {
  assert!(load_from_file("non-existing.kivi").is_err());
  assert_eq!(
    "The system cannot find the path specified. (os error 3)",
    load_from_file("../data/non-existing.kivi").unwrap_err().to_string()
  )
}

#[cfg(not(target_os = "windows"))]
#[test]
fn loading_from_non_existing_file_with_markers_should_fail() {
  assert!(load_from_file_markers("non-existing.kivi", &['@']).is_err());
  assert_eq!(
    "No such file or directory (os error 2)",
    load_from_file_markers("../data/non-existing.kivi", &['@']).unwrap_err().to_string()
  )
}

#[cfg(target_os = "windows")]
#[test]
fn loading_from_non_existing_file_with_markers_should_fail() {
  assert!(load_from_file_markers("non-existing.kivi", &['@']).is_err());
  assert_eq!(
    "The system cannot find the path specified. (os error 3)",
    load_from_file_markers("../data/non-existing.kivi", &['@']).unwrap_err().to_string()
  )
}
