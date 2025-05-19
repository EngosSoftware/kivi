[1mdiff --git a/src/loader.rs b/src/loader.rs[m
[1mindex 8fb914d..0f2d4e5 100644[m
[1m--- a/src/loader.rs[m
[1m+++ b/src/loader.rs[m
[36m@@ -54,10 +54,10 @@[m [mmacro_rules! consume_non_empty_value {[m
 }[m
 [m
 macro_rules! consume_escaped_quotation_mark {[m
[31m-  ($chars:ident, $buffer:ident) => {{[m
[32m+[m[32m  ($chars:ident, $buffer:ident, $marker:ident) => {{[m
     if let Some(ch) = $chars.peek() {[m
[31m-      if *ch == '"' {[m
[31m-        $buffer.push('"');[m
[32m+[m[32m      if *ch == $marker {[m
[32m+[m[32m        $buffer.push($marker);[m
         $chars.next();[m
       } else {[m
         $buffer.push('\\');[m
[36m@@ -88,26 +88,55 @@[m [mmacro_rules! clear_buffer {[m
 /// }[m
 /// ```[m
 pub fn load_from_string(input: &str) -> KeyValuePairs {[m
[32m+[m[32m  load(input, &['"'])[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32m/// Loads key-value pairs from KIVI file.[m
[32m+[m[32m///[m
[32m+[m[32m/// # Examples[m
[32m+[m[32m///[m
[32m+[m[32m/// ```[m
[32m+[m[32m/// use std::io;[m
[32m+[m[32m/// use kivi::load_from_file;[m
[32m+[m[32m///[m
[32m+[m[32m/// fn main() -> io::Result<()> {[m
[32m+[m[32m///     let kvp = load_from_file("./tests/data/properties.kivi")?;[m
[32m+[m[32m///     let default_host = "0.0.0.0".to_string();[m
[32m+[m[32m///     let host = kvp.get("host").unwrap_or(&default_host);[m
[32m+[m[32m///     println!("host: {}", host);[m
[32m+[m[32m///     Ok(())[m
[32m+[m[32m/// }[m
[32m+[m[32m/// ```[m
[32m+[m[32mpub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<KeyValuePairs> {[m
[32m+[m[32m  Ok(load_from_string(&fs::read_to_string(path)?))[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32m/// Loads key-value pairs from string.[m
[32m+[m[32mfn load(input: &str, markers: &[char]) -> KeyValuePairs {[m
   let mut output = KeyValuePairs::new();[m
   let mut state = State::Key;[m
   let mut buffer = String::new();[m
   let mut key = String::new();[m
   let mut chars = input.chars().peekable();[m
[32m+[m[32m  let mut marker = 0 as char;[m
   while let Some(ch) = chars.next() {[m
     match state {[m
       State::Key => match ch {[m
[31m-        '"' => clear_buffer!(buffer, state, State::KeyExt),[m
         '\n' => consume_non_empty_key!(key, buffer, state),[m
[32m+[m[32m        ch if is_quotation_marker(ch, markers) => {[m
[32m+[m[32m          marker = ch;[m
[32m+[m[32m          clear_buffer!(buffer, state, State::KeyExt);[m
[32m+[m[32m        }[m
         other => consume_char!(buffer, other),[m
       },[m
       State::KeyExt => match ch {[m
[32m+[m[32m        '\\' => consume_escaped_quotation_mark!(chars, buffer, marker),[m
         '"' => consume_key!(key, buffer, state),[m
[31m-        '\\' => consume_escaped_quotation_mark!(chars, buffer),[m
         other => consume_char!(buffer, other),[m
       },[m
       State::Value => match ch {[m
[31m-        '"' => clear_buffer!(buffer, state, State::ValueExt),[m
         '\n' => consume_non_empty_value!(output, key, buffer, state),[m
[32m+[m[32m        '"' => clear_buffer!(buffer, state, State::ValueExt),[m
         other => {[m
           consume_char!(buffer, other);[m
           if chars.peek().is_none() {[m
[36m@@ -116,8 +145,8 @@[m [mpub fn load_from_string(input: &str) -> KeyValuePairs {[m
         }[m
       },[m
       State::ValueExt => match ch {[m
[32m+[m[32m        '\\' => consume_escaped_quotation_mark!(chars, buffer, marker),[m
         '"' => consume_value!(output, key, buffer, state),[m
[31m-        '\\' => consume_escaped_quotation_mark!(chars, buffer),[m
         other => consume_char!(buffer, other),[m
       },[m
     }[m
[36m@@ -125,22 +154,7 @@[m [mpub fn load_from_string(input: &str) -> KeyValuePairs {[m
   output[m
 }[m
 [m
[31m-/// Loads key-value pairs from KIVI file.[m
[31m-///[m
[31m-/// # Examples[m
[31m-///[m
[31m-/// ```[m
[31m-/// use std::io;[m
[31m-/// use kivi::load_from_file;[m
[31m-///[m
[31m-/// fn main() -> io::Result<()> {[m
[31m-///     let kvp = load_from_file("./tests/data/properties.kivi")?;[m
[31m-///     let default_host = "0.0.0.0".to_string();[m
[31m-///     let host = kvp.get("host").unwrap_or(&default_host);[m
[31m-///     println!("host: {}", host);[m
[31m-///     Ok(())[m
[31m-/// }[m
[31m-/// ```[m
[31m-pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<KeyValuePairs> {[m
[31m-  Ok(load_from_string(&fs::read_to_string(path)?))[m
[32m+[m[32m/// Returns `true` when specified character is a quotation marker.[m
[32m+[m[32mfn is_quotation_marker(ch: char, markers: &[char]) -> bool {[m
[32m+[m[32m  markers.contains(&ch)[m
 }[m
