#![allow(clippy::useless_let_if_seq)]
use std::str;

use half::f16;
use nom::{be_f32, be_f64, be_u16};

use {
    ByteString, DataItem, FloatWidth, IntegerWidth, Result, Simple, Tag,
    TextString,
};

named! {
    integer<(&[u8], usize), (u64, IntegerWidth)>,
    alt_complete!(
        pair!(
            verify!(take_bits!(u64, 5), |v| v < 24),
            value!(IntegerWidth::Zero))
      | pair!(
            preceded!(tag_bits!(u8, 5, 24), take_bits!(u64, 8)),
            value!(IntegerWidth::Eight))
      | pair!(
            preceded!(tag_bits!(u8, 5, 25), take_bits!(u64, 16)),
            value!(IntegerWidth::Sixteen))
      | pair!(
            preceded!(tag_bits!(u8, 5, 26), take_bits!(u64, 32)),
            value!(IntegerWidth::ThirtyTwo))
      | pair!(
            preceded!(tag_bits!(u8, 5, 27), take_bits!(u64, 64)),
            value!(IntegerWidth::SixtyFour))
    )
}

named! {
    positive<(&[u8], usize), DataItem>,
    preceded!(
        tag_bits!(u8, 3, 0),
        map!(integer, |(value, bitwidth)| DataItem::Integer {
            value,
            bitwidth,
        }))
}

named! {
    negative<(&[u8], usize), DataItem>,
    preceded!(
        tag_bits!(u8, 3, 1),
        map!(integer, |(value, bitwidth)| DataItem::Negative {
            value,
            bitwidth,
        }))
}

named! {
    definite_bytestring<(&[u8], usize), ByteString>,
    do_parse!(
        tag_bits!(u8, 3, 2) >>
        // TODO: verify is workaround for https://github.com/Geal/nom/issues/848
        length: verify!(integer, |(l, _)| l < 0x2000_0000_0000_0000) >>
        data: bytes!(take!(length.0)) >>
        (ByteString { data: data.into(), bitwidth: length.1 }))
}

named! {
    indefinite_bytestring<(&[u8], usize), DataItem>,
    preceded!(
        pair!(tag_bits!(u8, 3, 2), tag_bits!(u8, 5, 31)),
        map!(
            many_till!(definite_bytestring, stop_code),
            |(strings, _)| DataItem::IndefiniteByteString(strings)))
}

named! {
    bytestring<(&[u8], usize), DataItem>,
    alt_complete!(
        definite_bytestring => { DataItem::ByteString }
      | indefinite_bytestring
    )
}

named! {
    definite_textstring<(&[u8], usize), TextString>,
    do_parse!(
        tag_bits!(u8, 3, 3) >>
        // TODO: verify is workaround for https://github.com/Geal/nom/issues/848
        length: verify!(integer, |(l, _)| l < 0x2000_0000_0000_0000) >>
        data: map_res!(bytes!(take!(length.0)), |b| str::from_utf8(b)) >>
        (TextString { data: data.to_owned(), bitwidth: length.1 }))
}

named! {
    indefinite_textstring<(&[u8], usize), DataItem>,
    preceded!(
        pair!(tag_bits!(u8, 3, 3), tag_bits!(u8, 5, 31)),
        map!(
            many_till!(definite_textstring, stop_code),
            |(strings, _)| DataItem::IndefiniteTextString(strings)))
}

named! {
    textstring<(&[u8], usize), DataItem>,
    alt_complete!(
        definite_textstring => { DataItem::TextString }
      | indefinite_textstring
    )
}

named! {
    definite_array<(&[u8], usize), DataItem>,
    do_parse!(
        tag_bits!(u8, 3, 4) >>
        length: integer >>
        data: bytes!(count!(data_item, length.0 as usize)) >>
        (DataItem::Array { data, bitwidth: Some(length.1) }))
}

named! {
    indefinite_array<(&[u8], usize), DataItem>,
    preceded!(
        pair!(tag_bits!(u8, 3, 4), tag_bits!(u8, 5, 31)),
        map!(
            many_till!(bytes!(data_item), stop_code),
            |(data, _)| DataItem::Array { data, bitwidth: None }))
}

named! {
    array<(&[u8], usize), DataItem>,
    alt_complete!(definite_array | indefinite_array)
}

named! {
    definite_map<(&[u8], usize), DataItem>,
    do_parse!(
        tag_bits!(u8, 3, 5) >>
        length: integer >>
        data: bytes!(count!(pair!(data_item, data_item), length.0 as usize)) >>
        (DataItem::Map { data, bitwidth: Some(length.1) }))
}

named! {
    indefinite_map<(&[u8], usize), DataItem>,
    preceded!(
        pair!(tag_bits!(u8, 3, 5), tag_bits!(u8, 5, 31)),
        map!(
            many_till!(bytes!(pair!(data_item, data_item)), stop_code),
            |(data, _)| DataItem::Map { data, bitwidth: None }))
}

named! {
    map<(&[u8], usize), DataItem>,
    alt_complete!(definite_map | indefinite_map)
}

named! {
    tagged<(&[u8], usize), DataItem>,
    do_parse!(
        tag_bits!(u8, 3, 6) >>
        tag: integer >>
        value: bytes!(data_item) >>
        (DataItem::Tag {
            tag: Tag(tag.0),
            bitwidth: tag.1,
            value: Box::new(value),
        })
    )
}

named! {
    float<(&[u8], usize), DataItem>,
    preceded!(
        tag_bits!(u8, 3, 7),
        map!(
            alt_complete!(
                pair!(
                    preceded!(
                        tag_bits!(u8, 5, 25),
                        map!(bytes!(be_u16), |u| f16::from_bits(u).to_f64())),
                    value!(FloatWidth::Sixteen))
              | pair!(
                    preceded!(
                        tag_bits!(u8, 5, 26),
                        map!(bytes!(be_f32), f64::from)),
                    value!(FloatWidth::ThirtyTwo))
              | pair!(
                    preceded!(tag_bits!(u8, 5, 27), bytes!(be_f64)),
                    value!(FloatWidth::SixtyFour))
            ),
            |(value, bitwidth)| DataItem::Float { value, bitwidth }))
}

named! {
    simple<(&[u8], usize), DataItem>,
    preceded!(
        tag_bits!(u8, 3, 7),
        map!(
            alt_complete!(
                verify!(take_bits!(u8, 5), |v| v < 24)
              | preceded!(tag_bits!(u8, 5, 24), take_bits!(u8, 8))
            ),
            |value| DataItem::Simple(Simple(value))
        )
    )
}

named! {
    stop_code<(&[u8], usize), DataItem>,
    preceded!(
        tag_bits!(u8, 3, 7),
        map!(tag_bits!(u8, 5, 31), |value| DataItem::Simple(Simple(value))))
}

named! {
    data_item<&[u8], DataItem>,
    bits!(alt_complete!(
        positive
      | negative
      | bytestring
      | textstring
      | array
      | map
      | tagged
      | float
      | simple
    ))
}

/// Parse a string containing a binary encoded CBOR data item.
///
/// # Examples
///
/// ```rust
/// use cbor_diag::{DataItem, IntegerWidth, Tag, TextString};
///
/// assert_eq!(
///     cbor_diag::parse_bytes(&b"\
///         \xd8\x20\x73\x68\x74\x74\x70\x73\x3a\x2f\x2f\x65\x78\x61\x6d\x70\
///         \x6c\x65\x2e\x63\x6f\x6d\
///     "[..]).unwrap(),
///     DataItem::Tag {
///         tag: Tag::URI,
///         bitwidth: IntegerWidth::Eight,
///         value: Box::new(DataItem::TextString(TextString {
///             data: "https://example.com".into(),
///             bitwidth: IntegerWidth::Zero,
///         })),
///     });
/// ```
pub fn parse_bytes(bytes: impl AsRef<[u8]>) -> Result<DataItem> {
    let (remaining, parsed) = data_item(bytes.as_ref()).map_err(|e| {
        println!("{}: {:?}", e, e);
        "Parsing error"
    })?;
    if !remaining.is_empty() {
        println!("parsed: {:?} remaining: {:?}", parsed, remaining);
        return Err("Remaining text".into());
    }
    Ok(parsed)
}
