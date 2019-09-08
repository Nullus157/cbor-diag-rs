//! A crate for parsing data encoded in [Concise Binary Object Representation
//! (CBOR)](https://cbor.io) (in any of raw binary, hex encoded (with comments)
//! or [diagnostic notation][]) then printing it out in either annotated hex
//! form or diagnostic notation. While doing so as much of the structured data
//! as possible is retained to improve the debugging experience.  The primary
//! intention of this crate is to be used in diagnostic tools working with CBOR
//! data.
//!
//! [diagnostic notation]: https://tools.ietf.org/html/rfc7049#section-6

extern crate base64;
extern crate bs58;
extern crate chrono;
extern crate half;
extern crate hex;
extern crate nom;
extern crate num_bigint;
extern crate num_rational;
extern crate num_traits;
extern crate separator;
extern crate url;
extern crate uuid;

mod encode;
mod error;
mod parse;
mod syntax;

pub use syntax::{ByteString, DataItem, FloatWidth, IntegerWidth, Simple, Tag, TextString};

pub use error::{Error, Result};

pub use self::parse::parse_bytes;
pub use self::parse::parse_diag;
pub use self::parse::parse_hex;
