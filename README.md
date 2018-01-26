# IoVec

A specialized byte slice type for performing vectored I/O operations.

[![Crates.io](https://img.shields.io/crates/v/iovec.svg?maxAge=2592000)](https://crates.io/crates/iovec)
[![Build Status](https://travis-ci.org/carllerche/iovec.svg?branch=master)](https://travis-ci.org/carllerche/iovec)

[Documentation](https://docs.rs/iovec)

## Usage

To use `iovec`, first add this to your `Cargo.toml`:

```toml
[dependencies]
iovec = "0.1"
```

Next, add this to your crate:

```rust
extern crate iovec;

use iovec::IoVec;
```

For more detail, see [documentation](https://docs.rs/iovec).

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in iovec by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
