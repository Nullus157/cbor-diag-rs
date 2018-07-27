# cbor-diag-rs

A crate for parsing data encoded in [Concise Binary Object Representation
(CBOR)](https://cbor.io) (in any of raw binary, hex encoded (with comments) or
[diagnostic notation][]) then printing it out in either annotated hex form or
diagnostic notation. While doing so as much of the structured data as possible
is retained to improve the debugging experience.  The primary intention of this
crate is to be used in diagnostic tools working with CBOR data.

[diagnostic notation]: https://tools.ietf.org/html/rfc7049#section-6

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
