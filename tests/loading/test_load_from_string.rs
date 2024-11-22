use super::*;

#[test]
fn _0001() {
  // Input string is empty.
  eq("", "{}");
}

#[test]
fn _0002() {
  // Input string contains only spaces or tabs.
  eq("        \t   \t   \t      ", "{}");
}

#[test]
fn _0003() {
  // Input string contains one empty line and second line is not ended by a newline.
  eq("        \n      ", "{}");
}

#[test]
fn _0004() {
  // String contains two empty lines ended by a newline.
  eq("        \n          \n", "{}");
}

#[test]
fn _0005() {
  // String contains two empty lines ended by a newline.
  eq("  \"   ", "{}");
}

#[test]
fn _0006() {
  // Input string is a single character.
  eq("a", "{}");
}

#[test]
fn _0007() {
  // Input string is a single character in first line.
  eq("a\n", "{}");
}

#[test]
fn _0008() {
  // Input string is a single character in first line, second line is empty.
  eq("a\n      ", "{}");
}

#[test]
fn _0009() {
  // Input string is a single character in first line, second line is empty and ended with a newline.
  eq("a\n      \n", "{}");
}

#[test]
fn _0010() {
  // Key and value pair.
  eq("a\nb\n", r#"{"a": "b"}"#);
}

#[test]
fn _0011() {
  // Key and value pair, but the value is not closed by a newline.
  eq("a\nb", r#"{"a": "b"}"#);
}

#[test]
fn _0012() {
  // Key and value are separated by an empty line.
  eq("a\n\nb", r#"{"a": "b"}"#);
}

#[test]
fn _0013() {
  // Key and value are separated by an empty line, and the value is ended by a newline.
  eq("a\n\nb\n", r#"{"a": "b"}"#);
}

#[test]
fn _0014() {
  // Multiple empty lines around the key and value.
  eq("\n\na\n\n\n\nb\n\n\n", r#"{"a": "b"}"#);
}

#[test]
fn _0015() {
  // Key and value a surrounded by whitespaces.
  eq("   a     \n    b     \n", r#"{"a": "b"}"#);
}

#[test]
fn _0016() {
  // Key and value a surrounded by whitespaces, the value is not closed by a newline.
  eq("     a           \n                 b   ", r#"{"a": "b"}"#);
}

#[test]
fn _0017() {
  // Key is given in quotation marks.
  eq("\" a \"\nb\n", r#"{" a ": "b"}"#);
}

#[test]
fn _0018() {
  // Value is given in quotation marks.
  eq(" a \n\" b \"\n", r#"{"a": " b "}"#);
}

#[test]
fn _0019() {
  // Key and value is given in quotation marks.
  eq("\" a \"\n\" b \"\n", r#"{" a ": " b "}"#);
}

#[test]
fn _0020() {
  // Key is given in quotation marks and contains quotation escape sequence.
  eq("\" \\\"a \"\nb\n", r#"{" "a ": "b"}"#);
}

#[test]
fn _0021() {
  // Key is given in quotation marks and contains backslash that is not a quotation escape sequence.
  eq("\" \\a \"\nb\n", r#"{" \a ": "b"}"#);
}

#[test]
fn _0022() {
  // Value is given in quotation marks.
  eq("a\n\" b \"\n", r#"{"a": " b "}"#);
}

#[test]
fn _0023() {
  // Value is given in quotation marks and contains quotation escape sequence.
  eq("a\n\" \\\"b \"\n", r#"{"a": " "b "}"#);
}

#[test]
fn _0024() {
  // Value is given in quotation marks and contains backslash that is not a quotation escape sequence.
  eq("a\n\" \\b \"\n", r#"{"a": " \b "}"#);
}

#[test]
fn _0025() {
  // Two key-value pairs.
  eq("a\nb\nc\nd\n", r#"{"a": "b", "c": "d"}"#);
}

#[test]
fn _0026() {
  // Multiline key and value.
  eqn("\" a \n a \"\n\" b \n b \"\n", r#"{" a \n a ": " b \n b "}"#);
}

#[test]
fn _0027() {
  // Multiple keys and values.
  let input = r#"

    " a1
      a2
      a3 "

    " b1 b2
      b3 b4 "

    " c1 c2 c3
      c4
      c5"

    "d1
    d2 d3
    d4"
  "#;
  eqn(
    input,
    r#"{" a1\n      a2\n      a3 ": " b1 b2\n      b3 b4 ", " c1 c2 c3\n      c4\n      c5": "d1\n    d2 d3\n    d4"}"#,
  );
}
