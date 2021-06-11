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

(This is work in progress. Nudge me about it if you're interested in using such a library.)

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add subset-struct
```

## Example

```rust
use serde::{Deserialize, Serialize};
use subset_struct::subset;

struct Extant {}

#[subset(
    /// This documentation will appear on `Base`, as will the attribute.
    #[derive(Deserialize)]
    default,

    /// Attributes, including documentation, added here will appear on `Filtered`, each conversion method and the `From` impl block.
    {
        // A `From` implementation is also generated,
        // but a more specific method is easier to use flexibly.
        // Use `&self` to clone each field instead.

        /// Document specific conversion methods like this.
        into_forwarded(self),
    } ->
        /// This will appear only on `Filtered`.
        #[derive(Serialize)]
        Filtered,

    // Add a new lifetime (which will appear before any others)
    // to generate a proxy structure instead.
    // `mut` is optional.
    {
        // Conversions, including `From`, must happen from the matching reference.
        as_proxy(&mut self),
    } -> mut Proxy<'a>,

    // Specifying custom conversion methods or additional attributes is optional:

    /// This documentation will appear only on `Empty`, but not the `From` implementation.
    Empty,

    // You can use `ref` to skip generating the subset struct definition, which will only generate conversions.
    // Use `ref mut` if needed.
    // Additional attributes for the struct definition are not allowed here,
    // but you can still specify some for the `From` impl if you write `{} -> ref Extant` instead and put them above that.
    ref Extant,
)]
pub struct Base {
    pub secret: String,
    #[subset(
        Filtered (
            // You could specify set of subset-specific attributes here, as above.
        ),
        Proxy,
    )]
    pub public: String,
}
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
