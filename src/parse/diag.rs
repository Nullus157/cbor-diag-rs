use std::f64;
use std::str::FromStr;

use base64;
use hex;
use nom::{self, digit, hex_digit0, AsChar};

use nom::Needed;

use {
    ByteString, DataItem, Error, FloatWidth, IntegerWidth, Result, Simple, Tag,
    TextString,
};

type NStr<'a> = nom::types::CompleteStr<'a>;

fn parse<T: FromStr>(s: NStr) -> ::std::result::Result<T, T::Err> {
    T::from_str(s.0)
}

/// Recognizes zero or more base64url characters: 0-9, A-Z, a-z, -, _
fn base64url_digit0<T>(input: T) -> nom::IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    input.split_at_position(|item| {
        !(item.is_alphanum() || item.as_char() == '-' || item.as_char() == '_')
    })
}

/// Recognizes zero or more base64url characters: 0-9, A-Z, a-z, +, /
fn base64_digit0<T>(input: T) -> nom::IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    input.split_at_position(|item| {
        !(item.is_alphanum() || item.as_char() == '+' || item.as_char() == '/')
    })
}

named! {
    encoding<NStr, u64>,
    preceded!(tag!("_"), verify!(map_res!(digit, parse::<u64>), |e| e < 4))
}

named! {
    positive<NStr, DataItem>,
    do_parse!(
        value: map_res!(digit, parse::<u64>) >>
        encoding: opt!(encoding) >>
        (DataItem::Integer {
            value,
            bitwidth: match (encoding, value) {
                (None, 0...23) => IntegerWidth::Zero,
                (Some(0), _) => IntegerWidth::Eight,
                (Some(1), _) => IntegerWidth::Sixteen,
                (Some(2), _) => IntegerWidth::ThirtyTwo,
                (Some(3), _) => IntegerWidth::SixtyFour,
                (None, _) => IntegerWidth::Unknown,
                (Some(_), _) => unreachable!(),
            },
        })
    )
}

named! {
    negative<NStr, DataItem>,
    preceded!(
        tag!("-"),
        do_parse!(
            value: map_res!(digit, parse::<u64>) >>
            encoding: opt!(encoding) >>
            (DataItem::Negative {
                value: value - 1,
                bitwidth: match (encoding, value) {
                    (None, 0...24) => IntegerWidth::Zero,
                    (Some(0), _) => IntegerWidth::Eight,
                    (Some(1), _) => IntegerWidth::Sixteen,
                    (Some(2), _) => IntegerWidth::ThirtyTwo,
                    (Some(3), _) => IntegerWidth::SixtyFour,
                    (None, _) => IntegerWidth::Unknown,
                    (Some(_), _) => unreachable!(),
                },
            })
        )
    )
}

named! {
    definite_bytestring<NStr, ByteString>,
    map!(
        alt_complete!(
            map_res!(
                preceded!(tag!("h"), delimited!(tag!("'"), hex_digit0, tag!("'"))),
                |s: NStr| hex::decode(s.as_ref()))
          | map_res!(
                preceded!(tag!("b64"), delimited!(tag!("'"), base64url_digit0, tag!("'"))),
                |s: NStr| base64::decode_config(s.as_ref(), base64::URL_SAFE_NO_PAD))
          | map_res!(
                preceded!(tag!("b64"), delimited!(tag!("'"), base64_digit0, tag!("'"))),
                |s: NStr| base64::decode_config(s.as_ref(), base64::STANDARD_NO_PAD))
        ),
        |data| ByteString { data, bitwidth: IntegerWidth::Unknown })
}

named! {
    indefinite_bytestring<NStr, DataItem>,
    map!(
        delimited!(
            tag!("(_ "),
            separated_list_complete!(tag!(", "), definite_bytestring),
            tag!(")")),
        DataItem::IndefiniteByteString)
}

named! {
    bytestring<NStr, DataItem>,
    alt_complete!(
        definite_bytestring => { DataItem::ByteString }
      | indefinite_bytestring
    )
}

named! {
    definite_textstring<NStr, TextString>,
    map!(
        delimited!(
            tag!("\""),
            escaped_transform!(
                none_of!("\\\""),
                '\\',
                alt!(
                    tag!("\\") => { |_| "\\" }
                  | tag!("\"") => { |_| "\"" }
                )),
            tag!("\"")),
        |data| TextString { data, bitwidth: IntegerWidth::Unknown })
}

named! {
    indefinite_textstring<NStr, DataItem>,
    map!(
        delimited!(
            tag!("(_ "),
            separated_list_complete!(tag!(", "), definite_textstring),
            tag!(")")),
        DataItem::IndefiniteTextString)
}

named! {
    textstring<NStr, DataItem>,
    alt_complete!(
        definite_textstring => { DataItem::TextString }
      | indefinite_textstring
    )
}

named! {
    definite_array<NStr, DataItem>,
    map!(
        delimited!(
            tag!("["),
            separated_list_complete!(tag!(", "), data_item),
            tag!("]")),
        |data| DataItem::Array { data, bitwidth: Some(IntegerWidth::Unknown) })
}

named! {
    indefinite_array<NStr, DataItem>,
    map!(
        delimited!(
            tag!("[_ "),
            separated_list_complete!(tag!(", "), data_item),
            tag!("]")),
        |data| DataItem::Array { data, bitwidth: None })
}

named! {
    array<NStr, DataItem>,
    alt_complete!(definite_array | indefinite_array)
}

named! {
    definite_map<NStr, DataItem>,
    map!(
        delimited!(
            tag!("{"),
            separated_list_complete!(tag!(","), separated_pair!(data_item, tag!(":"), data_item)),
            tag!("}")),
        |data| DataItem::Map { data, bitwidth: Some(IntegerWidth::Unknown) })
}

named! {
    indefinite_map<NStr, DataItem>,
    map!(
        ws!(delimited!(
            tag!("{_"),
            separated_list_complete!(tag!(","), separated_pair!(data_item, tag!(":"), data_item)),
            tag!("}"))),
        |data| DataItem::Map { data, bitwidth: None })
}

named! {
    map<NStr, DataItem>,
    alt_complete!(definite_map | indefinite_map)
}

named! {
    tagged<NStr, DataItem>,
    do_parse!(
        tag: map_res!(digit, parse::<u64>) >>
        encoding: opt!(encoding) >>
        value: delimited!(tag!("("), data_item, tag!(")")) >>
        (DataItem::Tag {
            tag: Tag(tag),
            bitwidth: match (encoding, tag) {
                (None, 0...23) => IntegerWidth::Zero,
                (Some(0), _) => IntegerWidth::Eight,
                (Some(1), _) => IntegerWidth::Sixteen,
                (Some(2), _) => IntegerWidth::ThirtyTwo,
                (Some(3), _) => IntegerWidth::SixtyFour,
                (None, _) => IntegerWidth::Unknown,
                (Some(_), _) => unreachable!(),
            },
            value: Box::new(value),
        })
    )
}

#[allow(unused_imports)]
fn recognize_float<T>(input: T) -> nom::IResult<T, T, u32>
where
    T: nom::Slice<::std::ops::Range<usize>>
        + nom::Slice<::std::ops::RangeFrom<usize>>
        + nom::Slice<::std::ops::RangeTo<usize>>
        + Clone
        + nom::Offset
        + nom::InputIter
        + nom::AtEof
        + nom::InputTakeAtPosition,
    <T as nom::InputIter>::Item: nom::AsChar,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar,
{
    recognize!(
        input,
        tuple!(
            opt!(alt!(char!('+') | char!('-'))),
            tuple!(digit, pair!(char!('.'), digit)),
            opt!(tuple!(
                alt!(char!('e') | char!('E')),
                opt!(alt!(char!('+') | char!('-'))),
                digit
            ))
        )
    )
}

named! {
    float_value<NStr, f64>,
    alt_complete!(
        map_res!(recognize_float, parse::<f64>)
      | value!(f64::INFINITY, tag!("Infinity"))
      | value!(f64::NEG_INFINITY, tag!("-Infinity"))
      | value!(f64::NAN, tag!("NaN"))
    )
}

named! {
    float<NStr, DataItem>,
    do_parse!(
        value: float_value >>
        encoding: opt!(verify!(encoding, |e| e > 0)) >>
        (DataItem::Float {
            value,
            bitwidth: match encoding {
                Some(1) => FloatWidth::Sixteen,
                Some(2) => FloatWidth::ThirtyTwo,
                Some(3) => FloatWidth::SixtyFour,
                Some(_) => unreachable!(),
                None => FloatWidth::Unknown,
            },
        })
    )
}

named! {
    simple<NStr, DataItem>,
    map!(
        alt_complete!(
            value!(Simple::FALSE, tag!("false"))
          | value!(Simple::TRUE, tag!("true"))
          | value!(Simple::NULL, tag!("null"))
          | value!(Simple::UNDEFINED, tag!("undefined"))
          | map!(preceded!(tag!("simple"),
                map_res!(delimited!(tag!("("), digit, tag!(")")), parse::<u8>)),
                Simple)
        ),
        DataItem::Simple)
}

named! {
    data_item<NStr, DataItem>,
    ws!(alt_complete!(
        float
      | tagged
      | positive
      | negative
      | bytestring
      | textstring
      | array
      | map
      | simple
    ))
}

/// Parse a string containing a diagnostic notation encoded CBOR data item.
///
/// ⚠️ Take special note of the warning from [RFC 7049 § 6][RFC 6]:
///
/// > Note that this truly is a diagnostic format; it is not meant to be parsed.
///
/// That means that this parser does not guarantee anything. Where possible it
/// attempts to preserve round-tripping support, but will certainly fail to
/// parse output from other tools that generate diagnostic notation. You should
/// always prefer to pass around binary or hex-encoded CBOR data items and
/// generate the diagnostic notation from them.
///
/// This lack of guarantee also extends to semantic versioning of this crate. On
/// any update this parser can completely change what it parses with no prior
/// warning.
///
/// [RFC 6]: https://tools.ietf.org/html/rfc7049#section-6
///
/// # Examples
///
/// ```rust
/// use cbor_diag::{DataItem, IntegerWidth, Tag, TextString};
///
/// assert_eq!(
///     cbor_diag::parse_diag(r#"32("https://example.com")"#).unwrap(),
///     DataItem::Tag {
///         tag: Tag::URI,
///         bitwidth: IntegerWidth::Unknown,
///         value: Box::new(DataItem::TextString(TextString {
///             data: "https://example.com".into(),
///             bitwidth: IntegerWidth::Unknown,
///         })),
///     });
/// ```
pub fn parse_diag(text: impl AsRef<str>) -> Result<DataItem> {
    let text = nom::types::CompleteStr(text.as_ref());
    let (remaining, parsed) = data_item(text).map_err(|e| {
        println!("{}: {:?}", e, e);
        Error::Todos("Parsing error")
    })?;
    if !remaining.is_empty() {
        println!("parsed: {:?} remaining: {:?}", parsed, remaining);
        return Err(Error::Todos("Remaining text"));
    }
    Ok(parsed)
}
