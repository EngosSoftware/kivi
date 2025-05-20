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
  // Input string contains one empty line and second line is not ended by a newline.
  eq("        \r      ", "{}");
}

#[test]
fn _0005() {
  // Input string contains one empty line and second line is not ended by a newline.
  eq("        \r\n      ", "{}");
}

#[test]
fn _0006() {
  // String contains two empty lines ended by a newline.
  eq("        \n          \n", "{}");
}

#[test]
fn _0007() {
  // String contains two empty lines ended by a newline.
  eq("  \"   ", "{}");
}

#[test]
fn _0008() {
  // Input string is a single character.
  eq("a", "{}");
}

#[test]
fn _0009() {
  // Input string is a single character in first line.
  eq("a\n", "{}");
}

#[test]
fn _0010() {
  // Input string is a single character in first line, second line is empty.
  eq("a\n      ", "{}");
}

#[test]
fn _0011() {
  // Input string is a single character in first line, second line is empty and ended with a newline.
  eq("a\n      \n", "{}");
}

#[test]
fn _0012() {
  // Key and value pair.
  eq("a\nb\n", r#"{"a": "b"}"#);
}

#[test]
fn _0013() {
  // Key and value pair, but the value is not closed by a newline.
  eq("a\nb", r#"{"a": "b"}"#);
}

#[test]
fn _0014() {
  // Key and value are separated by an empty line.
  eq("a\n\nb", r#"{"a": "b"}"#);
}

#[test]
fn _0015() {
  // Key and value are separated by an empty line, and the value is ended by a newline.
  eq("a\n\nb\n", r#"{"a": "b"}"#);
}

#[test]
fn _0016() {
  // Multiple empty lines around the key and value.
  eq("\n\na\n\n\n\nb\n\n\n", r#"{"a": "b"}"#);
}

#[test]
fn _0017() {
  // Key and value a surrounded by whitespaces.
  eq("   a     \n    b     \n", r#"{"a": "b"}"#);
}

#[test]
fn _0018() {
  // Key and value a surrounded by whitespaces, the value is not closed by a newline.
  eq("     a           \n                 b   ", r#"{"a": "b"}"#);
}

#[test]
fn _0019() {
  // Key is given in quotation marks.
  eq("\" a \"\nb\n", r#"{" a ": "b"}"#);
}

#[test]
fn _0020() {
  // Value is given in quotation marks.
  eq(" a \n\" b \"\n", r#"{"a": " b "}"#);
}

#[test]
fn _0021() {
  // Key and value is given in quotation marks.
  eq("\" a \"\n\" b \"\n", r#"{" a ": " b "}"#);
}

#[test]
fn _0022() {
  // Key is given in quotation marks and contains quotation escape sequence.
  eq("\" \"a \"\nb\n", r#"{" "a ": "b"}"#);
}

#[test]
fn _0023() {
  // Key is given in quotation marks and contains backslash that is not a quotation escape sequence.
  eq("\" \\a \"\nb\n", r#"{" \a ": "b"}"#);
}

#[test]
fn _0024() {
  // Value is given in quotation marks.
  eq("a\n\" b \"\n", r#"{"a": " b "}"#);
}

#[test]
fn _0025() {
  // Value is given in quotation marks and contains quotation escape sequence.
  eq("a\n\" \"b \"\n", r#"{"a": " "b "}"#);
}

#[test]
fn _0026() {
  // Value is given in quotation marks and contains backslash that is not a quotation escape sequence.
  eq("a\n\" \\b \"\n", r#"{"a": " \b "}"#);
}

#[test]
fn _0027() {
  // Two key-value pairs.
  eq("a\nb\nc\nd\n", r#"{"a": "b", "c": "d"}"#);
}

#[test]
fn _0028() {
  // Multiline key and value.
  eqn("\" a \n a \"\n\" b \n b \"\n", r#"{" a \n a ": " b \n b "}"#);
}

#[test]
fn _0029() {
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

#[test]
fn _0030() {
  // Multiline key is not closed.
  eqn("\"key", r#"{}"#);
}

#[test]
fn _0031() {
  // Multiline value is not closed.
  eqn("\"key\"\n\"value", r#"{}"#);
}

#[test]
fn _0032() {
  // Multiline key and non multiline value.
  eqn("\"key\"\nvalue", r#"{"key": "value"}"#);
}

#[test]
fn _0033() {
  // Multiline key and non multiline value in one line.
  eqn("\"key\"  value", r#"{}"#);
}

#[test]
fn _0034() {
  // Multiline key not ended with newline after marker.
  eqn("\"key\" \n value", r#"{}"#);
}

#[test]
fn _0035() {
  // Multiline key with multiline value not ended with newline after marker.
  eqn("\"key\"\n \"value\" \r", r#"{}"#);
}

#[test]
fn _0036() {
  // Custom multiline marker.
  eqnm("`key`\n`value`\n", r#"{"key": "value"}"#, &['`']);
}

#[test]
fn _0037() {
  // Custom multiline markers inside key and value.
  eqnm("`key`key`\n`value`value`\n", r#"{"key`key": "value`value"}"#, &['`']);
}

#[test]
fn _0038() {
  // Multiple custom multiline markers.
  eqnm("@key@\n`value`\n", r#"{"key": "value"}"#, &['`', '@']);
}

#[test]
fn _0039() {
  // Multiple custom multiline markers inside key and value, line ending: \n
  eqnm("@key@key@\n#value#value#\n", r#"{"key@key": "value#value"}"#, &['@', '#']);
}

#[test]
fn _0040() {
  // Multiple custom multiline markers inside key and value, line ending: \r
  eqnm("@key@key@\r#value#value#\r", r#"{"key@key": "value#value"}"#, &['@', '#']);
}

#[test]
fn _0041() {
  // Multiple custom multiline markers inside key and value, line ending: \r\n
  eqnm("@key@key@\r\n#value#value#\r\n", r#"{"key@key": "value#value"}"#, &['@', '#']);
}
