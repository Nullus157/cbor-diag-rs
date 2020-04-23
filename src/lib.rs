//! A crate for parsing data encoded in [Concise Binary Object Representation
//! (CBOR)](https://cbor.io) (in any of raw binary, hex encoded (with comments)
//! or [diagnostic notation][]) then printing it out in either annotated hex
//! form or diagnostic notation. While doing so as much of the structured data
//! as possible is retained to improve the debugging experience.  The primary
//! intention of this crate is to be used in diagnostic tools working with CBOR
//! data.
//!
//! [diagnostic notation]: https://tools.ietf.org/html/rfc7049#section-6

#![warn(rust_2018_idioms)]

mod encode;
mod error;
mod parse;
mod syntax;

pub use self::{
    error::{Error, Result},
    parse::{parse_bytes, parse_bytes_partial, parse_diag, parse_hex},
    syntax::{ByteString, DataItem, FloatWidth, IntegerWidth, Simple, Tag, TextString},
};
