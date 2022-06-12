use crate::{DataItem, Result};

mod binary;
mod diag;

pub use self::binary::{parse_bytes, parse_bytes_partial};
pub use self::diag::parse_diag;

fn remove_comments(hex: impl AsRef<str>) -> String {
    hex.as_ref()
        .lines()
        .map(|line| line.split('#').next().unwrap().replace(' ', ""))
        .collect()
}

/// Parse a string containing a hex encoded CBOR data item.
///
/// The provided string may contain comments, where a comment is started with a
/// `#` character and proceeds until the end of the line. Any spaces in the
/// string will also be ignored, but any other non-hex characters will cause an
/// error.
///
/// # Examples
///
/// ```rust
/// use cbor_diag::{DataItem, IntegerWidth, Tag, TextString};
///
/// assert_eq!(
///     cbor_diag::parse_hex(r#"
///         d8 20                                        # uri, tag(32)
///            73                                        #   text(19)
///               68747470733a2f2f6578616d706c652e636f6d #     "https://example.com"
///     "#).unwrap(),
///     DataItem::Tag {
///         tag: Tag::URI,
///         bitwidth: IntegerWidth::Eight,
///         value: Box::new(DataItem::TextString(TextString {
///             data: "https://example.com".into(),
///             bitwidth: IntegerWidth::Zero,
///         })),
///     });
/// ```
pub fn parse_hex(hex: impl AsRef<str>) -> Result<DataItem> {
    let hex = remove_comments(hex);
    let bytes = hex::decode(hex)?;
    parse_bytes(bytes)
}
