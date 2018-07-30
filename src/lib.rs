#[macro_use]
extern crate nom;
extern crate half;
extern crate hex;

mod encode;
mod error;
mod parse;
pub mod syntax;

pub use syntax::{
    ByteString, FloatWidth, IntegerWidth, Simple, Tag, TextString, Value,
};

pub use error::{Error, Result};

pub use self::parse::parse_bytes;
pub use self::parse::parse_diag;
pub use self::parse::parse_hex;
