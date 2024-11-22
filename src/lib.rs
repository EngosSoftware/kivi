//! # KIVI format deserializer
//!
//! KIVI is a simple text format for storing keys with associated values on separate lines.
//! While it is not as widely known as formats like JSON or INI, it is straightforward
//! and particularly useful in specific contexts where keys or values consist
//! of multiple lines of text.
//!
//! An example of a configuration file in KIVI format:
//!
//! ```text
//! host
//! 127.0.0.1
//!
//! port
//! 54321
//!
//! timeout
//! 12ms
//! ```
//!
//! `host`, `port` and `timeout` are keys; `127.0.0.1`, `54321`, `12ms` are values.
//! It is quite similar to properties or INI file that store key-value pair in a single line.
//!
//! In KIVI format, the key and/or value may span over multiple lines:
//!
//! ```text
//! host
//! 127.0.0.1
//!
//! port
//! 54321
//!
//! timeout
//! 12ms
//!
//! "General
//!  description"
//! "This configuration file
//!  should be placed in the same
//!  directory where the servers
//!  binary is placed"
//! ```
//!

mod loader;
mod model;

pub use loader::{load_from_file, load_from_string};
pub use model::KeyValuePairs;
