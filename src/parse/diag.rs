#![allow(clippy::needless_pass_by_value, clippy::useless_let_if_seq)]

use std::f64;
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::complete::{char, digit1, hex_digit0, none_of},
    combinator::{map, map_res, opt, recognize, value, verify},
    error::context,
    multi::separated_list,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

use crate::{ByteString, DataItem, FloatWidth, IntegerWidth, Result, Simple, Tag, TextString};

fn ws<O: Default>(input: &str) -> IResult<&str, O> {
    map(nom::character::complete::multispace0, |_| O::default())(input)
}

fn wrapws<'a, T>(
    parser: impl Fn(&'a str) -> IResult<&'a str, T>,
) -> impl Fn(&'a str) -> IResult<&'a str, T> {
    delimited(ws::<()>, parser, ws::<()>)
}

#[allow(clippy::needless_lifetimes)]
fn opt_comma_tag<'a>(t: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    alt((tag(t), map(tuple((tag(","), ws, tag(t))), |(_, (), f)| f)))
}

/// Recognizes zero or more base64url characters: 0-9, A-Z, a-z, -, _
fn base64url_digit0<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position(|item| {
        !(item.is_alphanum() || item.as_char() == '-' || item.as_char() == '_')
    })
}

/// Recognizes zero or more base64 characters: 0-9, A-Z, a-z, +, /, =
fn base64_digit0<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position(|item| {
        !(item.is_alphanum()
            || item.as_char() == '+'
            || item.as_char() == '/'
            || item.as_char() == '=')
    })
}

fn encoding(input: &str) -> IResult<&str, u64> {
    preceded(tag("_"), verify(map_res(digit1, u64::from_str), |&e| e < 4))(input)
}

fn integer(input: &str) -> IResult<&str, (u64, IntegerWidth)> {
    let (input, value) = map_res(digit1, u64::from_str)(input)?;
    let (input, encoding) = opt(encoding)(input)?;
    Ok((
        input,
        (
            value,
            match encoding {
                Some(0) => IntegerWidth::Eight,
                Some(1) => IntegerWidth::Sixteen,
                Some(2) => IntegerWidth::ThirtyTwo,
                Some(3) => IntegerWidth::SixtyFour,
                None => IntegerWidth::Unknown,
                Some(_) => unreachable!(),
            },
        ),
    ))
}

fn positive(input: &str) -> IResult<&str, DataItem> {
    map(integer, |(value, bitwidth)| DataItem::Integer {
        value,
        bitwidth: if bitwidth == IntegerWidth::Unknown && value <= 23 {
            IntegerWidth::Zero
        } else {
            bitwidth
        },
    })(input)
}

fn negative(input: &str) -> IResult<&str, DataItem> {
    preceded(
        tag("-"),
        map(
            verify(integer, |&(value, _)| value > 0),
            |(value, bitwidth)| DataItem::Negative {
                value: value - 1,
                bitwidth: if bitwidth == IntegerWidth::Unknown && value <= 24 {
                    IntegerWidth::Zero
                } else {
                    bitwidth
                },
            },
        ),
    )(input)
}

fn definite_bytestring(input: &str) -> IResult<&str, ByteString> {
    wrapws(map(
        alt((
            map_res(
                preceded(tag("h"), delimited(tag("'"), hex_digit0, tag("'"))),
                hex::decode,
            ),
            map_res(
                preceded(tag("b64"), delimited(tag("'"), base64url_digit0, tag("'"))),
                |s: &str| base64::decode_config(s, base64::URL_SAFE_NO_PAD),
            ),
            map_res(
                preceded(tag("b64"), delimited(tag("'"), base64_digit0, tag("'"))),
                |s: &str| base64::decode_config(s, base64::STANDARD),
            ),
        )),
        |data| ByteString {
            data,
            bitwidth: IntegerWidth::Unknown,
        },
    ))(input)
}

fn indefinite_bytestring(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            tag("(_"),
            separated_list(tag(","), definite_bytestring),
            opt_comma_tag(")"),
        ),
        DataItem::IndefiniteByteString,
    )(input)
}

fn bytestring(input: &str) -> IResult<&str, DataItem> {
    alt((
        map(definite_bytestring, DataItem::ByteString),
        indefinite_bytestring,
    ))(input)
}

fn definite_textstring(input: &str) -> IResult<&str, TextString> {
    wrapws(map(
        delimited(
            tag("\""),
            opt(escaped_transform(
                none_of("\\\""),
                '\\',
                alt((tag("\\"), tag("\""))),
            )),
            tag("\""),
        ),
        |data| TextString {
            data: data.unwrap_or_default(),
            bitwidth: IntegerWidth::Unknown,
        },
    ))(input)
}

fn indefinite_textstring(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            tag("(_"),
            separated_list(tag(","), definite_textstring),
            opt_comma_tag(")"),
        ),
        DataItem::IndefiniteTextString,
    )(input)
}

fn textstring(input: &str) -> IResult<&str, DataItem> {
    alt((
        map(definite_textstring, DataItem::TextString),
        indefinite_textstring,
    ))(input)
}

fn definite_array(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            wrapws(tag("[")),
            separated_list(tag(","), data_item),
            opt_comma_tag("]"),
        ),
        |data| DataItem::Array {
            data,
            bitwidth: Some(IntegerWidth::Unknown),
        },
    )(input)
}

fn indefinite_array(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            wrapws(tag("[_")),
            separated_list(tag(","), data_item),
            opt_comma_tag("]"),
        ),
        |data| DataItem::Array {
            data,
            bitwidth: None,
        },
    )(input)
}

fn array(input: &str) -> IResult<&str, DataItem> {
    alt((definite_array, indefinite_array))(input)
}

fn definite_map(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            wrapws(tag("{")),
            separated_list(tag(","), separated_pair(data_item, tag(":"), data_item)),
            opt_comma_tag("}"),
        ),
        |data| DataItem::Map {
            data,
            bitwidth: Some(IntegerWidth::Unknown),
        },
    )(input)
}

fn indefinite_map(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            wrapws(tag("{_")),
            separated_list(tag(","), separated_pair(data_item, tag(":"), data_item)),
            opt_comma_tag("}"),
        ),
        |data| DataItem::Map {
            data,
            bitwidth: None,
        },
    )(input)
}

fn data_map(input: &str) -> IResult<&str, DataItem> {
    alt((definite_map, indefinite_map))(input)
}

fn tagged(input: &str) -> IResult<&str, DataItem> {
    let (input, (tag_, bitwidth)) = integer(input)?;
    let (input, value) = delimited(tag("("), data_item, tag(")"))(input)?;
    Ok((
        input,
        DataItem::Tag {
            tag: Tag(tag_),
            bitwidth: if bitwidth == IntegerWidth::Unknown && tag_ <= 23 {
                IntegerWidth::Zero
            } else {
                bitwidth
            },
            value: Box::new(value),
        },
    ))
}

fn recognize_float(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        opt(alt((char('+'), char('-')))),
        tuple((digit1, pair(char('.'), digit1))),
        opt(tuple((
            alt((char('e'), char('E'))),
            opt(alt((char('+'), char('-')))),
            digit1,
        ))),
    )))(input)
}

fn float_value(input: &str) -> IResult<&str, f64> {
    alt((
        map_res(recognize_float, f64::from_str),
        value(f64::INFINITY, tag("Infinity")),
        value(f64::NEG_INFINITY, tag("-Infinity")),
        value(f64::NAN, tag("NaN")),
    ))(input)
}

fn float(input: &str) -> IResult<&str, DataItem> {
    let (input, value) = float_value(input)?;
    let (input, encoding) = opt(verify(encoding, |&e| e > 0))(input)?;
    Ok((
        input,
        DataItem::Float {
            value,
            bitwidth: match encoding {
                Some(1) => FloatWidth::Sixteen,
                Some(2) => FloatWidth::ThirtyTwo,
                Some(3) => FloatWidth::SixtyFour,
                Some(_) => unreachable!(),
                None => FloatWidth::Unknown,
            },
        },
    ))
}

fn simple(input: &str) -> IResult<&str, DataItem> {
    map(
        alt((
            value(Simple::FALSE, tag("false")),
            value(Simple::TRUE, tag("true")),
            value(Simple::NULL, tag("null")),
            value(Simple::UNDEFINED, tag("undefined")),
            map(
                preceded(
                    tag("simple"),
                    map_res(delimited(tag("("), digit1, tag(")")), u8::from_str),
                ),
                Simple,
            ),
        )),
        DataItem::Simple,
    )(input)
}

fn data_item(input: &str) -> IResult<&str, DataItem> {
    context(
        "data item",
        wrapws(alt((
            context("float", float),
            context("tagged", tagged),
            context("positive", positive),
            context("negative", negative),
            context("bytestring", bytestring),
            context("textstring", textstring),
            context("array", array),
            context("map", data_map),
            context("simple", simple),
        ))),
    )(input)
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
    let (remaining, parsed) =
        data_item(text.as_ref()).map_err(|e| format!("Parsing error ({:?})", e))?;
    if !remaining.is_empty() {
        return Err(format!("Remaining text ({:?})", remaining).into());
    }
    Ok(parsed)
}
