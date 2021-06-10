# subset-struct

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/subset-struct)
[![Crates.io](https://img.shields.io/crates/v/subset-struct)](https://crates.io/crates/subset-struct)
[![Docs.rs](https://docs.rs/subset-struct/badge.svg)](https://docs.rs/subset-struct)

![Rust 1.51](https://img.shields.io/static/v1?logo=Rust&label=&message=1.51&color=grey)
[![CI](https://github.com/Tamschi/subset-struct/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/subset-struct/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/subset-struct/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/subset-struct)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/subset-struct)](https://github.com/Tamschi/subset-struct/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/subset-struct)](https://github.com/Tamschi/subset-struct/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/subset-struct.svg)](https://web.crev.dev/rust-reviews/crate/subset-struct/)

Convenient and robust subset struct generation, for example for redacting a public API.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add subset-struct
```

## Example

```rust
// TODO_EXAMPLE
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`subset-struct` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
