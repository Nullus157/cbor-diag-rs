#[macro_use]
extern crate nom;
extern crate hex;

mod encode;
mod error;
mod parse;
pub mod syntax;

pub use syntax::{FloatWidth, IntegerWidth, Simple, Tag, Value, ByteString, TextString};

pub use error::{Error, Result};

pub use self::parse::parse_bytes;
pub use self::parse::parse_diag;
pub use self::parse::parse_hex;
