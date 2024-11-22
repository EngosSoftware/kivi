# kivi

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]

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
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/EngosSoftware/kivi/blob/main/CODE_OF_CONDUCT.md

[kivi]: https://github.com/EngosSoftware/kivi
[headless_chrome]: https://crates.io/crates/headless_chrome
[html2pdf]: https://crates.io/crates/html2pdf
[report a bug]: https://github.com/EngosSoftware/kivi/issues

## Overview

KIVI format deserializer.

KIVI is a simple text format for storing keys with associated values on separate lines.
While it is not as widely known as formats like JSON or INI, it is straightforward
and particularly useful in specific contexts where keys or values consist
of multiple lines of text.

An example of a configuration file in KIVI format:

```text
host
127.0.0.1

port
54321

timeout
12ms
```

`host`, `port` and `timeout` are keys; `127.0.0.1`, `54321`, `12ms` are values.

It is quite similar to properties or INI file that store key-value pair in a single line.

In KIVI format, the key and/or value may span over multiple lines.
Multiple-line keys or values must be enclosed in quotation marks.

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
 directory where the servers
 binary is placed"
```

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to **[kivi]** are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
