# conditional

[![Crates.io](https://img.shields.io/crates/v/conditional)](https://crates.io/crates/conditional)
[![Crates.io](https://img.shields.io/crates/l/conditional)](https://crates.io/crates/conditional)
[![Crates.io](https://img.shields.io/crates/d/conditional)](https://crates.io/crates/conditional)
[![Docs.io](https://docs.rs/conditional/badge.svg)](https://docs.rs/conditional)

Lets you use the syntax of the conditional operator in Rust. Also known as the ternary operator.

## Examples
```rust
let x = conditional!(69 > 42 ? "hello" : "world");
assert_eq!(x, "hello");
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
