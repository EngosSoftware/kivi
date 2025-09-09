# kivi

[![Crates.io][crates-badge]][crates-url]
[![Code coverage][cov-badge-kivi]][cov-url]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
![build MacOs arm64][build-badge-macos-arm64]
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]
[![Made by Human][made-by-human-badge]][made-by-human-url]

[crates-badge]: https://img.shields.io/crates/v/kivi.svg
[crates-url]: https://crates.io/crates/kivi
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/kivi/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/kivi/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/kivi/blob/main/NOTICE
[build-badge-linux]: https://github.com/EngosSoftware/kivi/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/kivi/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/kivi/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/kivi/actions/workflows/build-macos-arm64.yml/badge.svg
[cov-url]: https://crates.io/crates/coverio
[cov-badge-kivi]: https://img.shields.io/badge/cov-100%25-21b577.svg
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-blue.svg
[cc-url]: https://github.com/EngosSoftware/kivi/blob/main/CODE_OF_CONDUCT.md
[made-by-human-badge]: https://img.shields.io/badge/Made_by-HUMAN-red.svg
[made-by-human-url]: https://github.com/DariuszDepta
[repository-url]: https://github.com/EngosSoftware/kivi

**Key-value pair with key and value in separate lines**

## Overview

**KIVI** format deserializer.

**KIVI** is a text format where each key and its corresponding value are stored on separate lines.
While not as widely known as formats like JSON or INI, it is straightforward and especially useful
in contexts where keys or values span multiple lines.

Example of a configuration file written in KIVI format:

```text
host
127.0.0.1

port
54321

timeout
12ms
```

`host`, `port` and `timeout` are keys, `127.0.0.1`, `54321`, `12ms` are values.

In **KIVI** format, keys and values may span multiple lines.
Multi-line keys or values must be enclosed in quotation markers.
The default quotation marker is the quotation mark `"` (U+0022).

Example configuration file with multi-line keys and values is shown below:

```text
host
127.0.0.1

port
54321

timeout
12ms

"General
 description"
"This configuration file
 should be placed in the same
 directory where the server's
 binary is placed" 
```

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [**kivi**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.

---

Brought to you with 💙 by [Engos Software](https://engos.de)
