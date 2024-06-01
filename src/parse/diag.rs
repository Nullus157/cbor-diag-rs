#![allow(clippy::needless_pass_by_value, clippy::useless_let_if_seq)]

use std::f64;
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::complete::{char, digit1, hex_digit1, none_of, oct_digit1},
    combinator::{map, map_res, opt, recognize, value, verify},
    error::context,
    multi::{many0, many0_count, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

use crate::{ByteString, DataItem, FloatWidth, IntegerWidth, Result, Simple, Tag, TextString};

fn ws<O: Default>(input: &str) -> IResult<&str, O> {
    map(nom::character::complete::multispace1, |_| O::default())(input)
}

fn comment<O: Default>(input: &str) -> IResult<&str, O> {
    map(delimited(tag("/"), many0(none_of("/")), tag("/")), |_| {
        O::default()
    })(input)
}

fn ws_or_comment<O: Default>(input: &str) -> IResult<&str, O> {
    map(many0_count(alt((comment::<O>, ws::<O>))), |_| O::default())(input)
}

fn wrapws<'a, T>(
    parser: impl FnMut(&'a str) -> IResult<&'a str, T>,
) -> impl FnMut(&'a str) -> IResult<&'a str, T> {
    delimited(ws_or_comment::<()>, parser, ws_or_comment::<()>)
}

fn wrapws_strings<'a>(
    parser: impl FnMut(&'a str) -> IResult<&'a str, &'a str>,
) -> impl FnMut(&'a str) -> IResult<&'a str, String> {
    map(
        many0(delimited(ws_or_comment::<()>, parser, ws_or_comment::<()>)),
        |strings| strings.into_iter().flat_map(|s| s.chars()).collect(),
    )
}

fn opt_comma_tag<'a>(t: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    alt((tag(t), map(tuple((tag(","), many0_count(ws::<()>), tag(t))), |(_, _, f)| f)))
}

/// Recognizes one or more binary numerical characters: 0, 1
fn bin_digit1<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            c != '0' && c != '1'
        },
        nom::error::ErrorKind::Digit,
    )
}

/// Recognizes one or more base16 characters: 0-9, A-F, a-f
fn base16_digit0<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position1_complete(
        |item| !item.as_char().is_ascii_hexdigit(),
        nom::error::ErrorKind::Digit,
    )
}

/// Recognizes one or more base32 characters: A-Z, 2-7, =
fn base32_digit0<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !c.is_ascii_uppercase() && !matches!(c, '2'..='7') && c != '='
        },
        nom::error::ErrorKind::Digit,
    )
}

/// Recognizes one or more base32hex characters: 0-9, A-V, =
fn base32hex_digit0<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !c.is_ascii_digit() && !matches!(c, 'A'..='V') && c != '='
        },
        nom::error::ErrorKind::Digit,
    )
}

/// Recognizes one or more base64url characters: 0-9, A-Z, a-z, -, _
fn base64url_digit0<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !c.is_ascii_alphanumeric() && c != '-' && c != '_'
        },
        nom::error::ErrorKind::Digit,
    )
}

/// Recognizes one or more base64 characters: 0-9, A-Z, a-z, +, /, =
fn base64_digit0<T>(input: T) -> IResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar + Copy,
{
    use nom::AsChar;
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !c.is_ascii_alphanumeric() && c != '+' && c != '/' && c != '='
        },
        nom::error::ErrorKind::Digit,
    )
}

fn encoding(input: &str) -> IResult<&str, u64> {
    preceded(tag("_"), verify(map_res(digit1, u64::from_str), |&e| e < 4))(input)
}

fn hexadecimal(input: &str) -> IResult<&str, u128> {
    preceded(
        tag("0x"),
        map_res(hex_digit1, |s| u128::from_str_radix(s, 16)),
    )(input)
}

fn octal(input: &str) -> IResult<&str, u128> {
    preceded(
        tag("0o"),
        map_res(oct_digit1, |s| u128::from_str_radix(s, 8)),
    )(input)
}

fn binary(input: &str) -> IResult<&str, u128> {
    preceded(
        tag("0b"),
        map_res(bin_digit1, |s| u128::from_str_radix(s, 2)),
    )(input)
}

fn decimal(input: &str) -> IResult<&str, u128> {
    map_res(digit1, u128::from_str)(input)
}

fn number<T: TryFrom<u128>>(input: &str) -> IResult<&str, (T, IntegerWidth)> {
    let (input, value) = map_res(alt((hexadecimal, octal, binary, decimal)), T::try_from)(input)?;
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

fn integer(input: &str) -> IResult<&str, DataItem> {
    map_res(number::<u64>, |(value, bitwidth)| {
        Ok::<_, std::num::TryFromIntError>(DataItem::Integer {
            value,
            bitwidth: if bitwidth == IntegerWidth::Unknown && value <= 23 {
                IntegerWidth::Zero
            } else {
                bitwidth
            },
        })
    })(input)
}

fn negative(input: &str) -> IResult<&str, DataItem> {
    preceded(
        tag("-"),
        map_res(
            verify(number::<u128>, |&(value, _)| value > 0),
            |(value, bitwidth)| {
                Ok::<_, std::num::TryFromIntError>(DataItem::Negative {
                    value: u64::try_from(value - 1)?,
                    bitwidth: if bitwidth == IntegerWidth::Unknown && value <= 24 {
                        IntegerWidth::Zero
                    } else {
                        bitwidth
                    },
                })
            },
        ),
    )(input)
}

fn definite_bytestring(input: &str) -> IResult<&str, Vec<u8>> {
    wrapws(alt((
        map_res(
            preceded(
                tag("h"),
                delimited(tag("'"), wrapws_strings(base16_digit0), tag("'")),
            ),
            |s| data_encoding::HEXLOWER_PERMISSIVE.decode(s.as_bytes()),
        ),
        map_res(
            preceded(
                tag("b32"),
                delimited(tag("'"), wrapws_strings(base32_digit0), tag("'")),
            ),
            |s| data_encoding::BASE32.decode(s.as_bytes()),
        ),
        map_res(
            preceded(
                tag("h32"),
                delimited(tag("'"), wrapws_strings(base32hex_digit0), tag("'")),
            ),
            |s| data_encoding::BASE32HEX.decode(s.as_bytes()),
        ),
        map_res(
            preceded(
                tag("b64"),
                delimited(tag("'"), wrapws_strings(base64url_digit0), tag("'")),
            ),
            |s| data_encoding::BASE64URL_NOPAD.decode(s.as_bytes()),
        ),
        map_res(
            preceded(
                tag("b64"),
                delimited(tag("'"), wrapws_strings(base64_digit0), tag("'")),
            ),
            |s| data_encoding::BASE64.decode(s.as_bytes()),
        ),
        map(
            delimited(tag("<<"), separated_list0(tag(","), data_item), tag(">>")),
            |items| items.into_iter().flat_map(|item| item.to_bytes()).collect(),
        ),
        map(
            delimited(
                tag("'"),
                opt(escaped_transform(
                    none_of("\\'"),
                    '\\',
                    alt((tag("\\"), tag("'"))),
                )),
                tag("'"),
            ),
            |s| s.unwrap_or_default().into_bytes(),
        ),
    )))(input)
}

fn concatenated_definite_bytestring(input: &str) -> IResult<&str, ByteString> {
    map(many1(definite_bytestring), |data| ByteString {
        data: data.into_iter().flatten().collect(),
        bitwidth: IntegerWidth::Unknown,
    })(input)
}

fn indefinite_bytestring(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            tag("(_"),
            separated_list0(tag(","), concatenated_definite_bytestring),
            opt_comma_tag(")"),
        ),
        DataItem::IndefiniteByteString,
    )(input)
}

fn bytestring(input: &str) -> IResult<&str, DataItem> {
    alt((
        map(concatenated_definite_bytestring, DataItem::ByteString),
        indefinite_bytestring,
    ))(input)
}

fn definite_textstring(input: &str) -> IResult<&str, String> {
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
        |data| data.unwrap_or_default(),
    ))(input)
}

fn concatenated_definite_textstring(input: &str) -> IResult<&str, TextString> {
    map(
        pair(
            definite_textstring,
            map_res(
                many0(alt((
                    definite_bytestring,
                    map(definite_textstring, |s| s.into_bytes()),
                ))),
                |rest| String::from_utf8(rest.into_iter().flatten().collect()),
            ),
        ),
        |(first, rest)| TextString {
            data: first + &rest,
            bitwidth: IntegerWidth::Unknown,
        },
    )(input)
}

fn indefinite_textstring(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            tag("(_"),
            separated_list0(tag(","), concatenated_definite_textstring),
            opt_comma_tag(")"),
        ),
        DataItem::IndefiniteTextString,
    )(input)
}

fn textstring(input: &str) -> IResult<&str, DataItem> {
    alt((
        map(concatenated_definite_textstring, DataItem::TextString),
        indefinite_textstring,
    ))(input)
}

fn definite_array(input: &str) -> IResult<&str, DataItem> {
    map(
        delimited(
            wrapws(tag("[")),
            separated_list0(tag(","), data_item),
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
            separated_list0(tag(","), data_item),
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
            separated_list0(tag(","), separated_pair(data_item, tag(":"), data_item)),
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
            separated_list0(tag(","), separated_pair(data_item, tag(":"), data_item)),
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
    let (input, (tag_, bitwidth)) = number::<u64>(input)?;
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

fn recognize_decimal_float(input: &str) -> IResult<&str, &str> {
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

fn hexadecimal_float(input: &str) -> IResult<&str, f64> {
    let (input, sign) = opt(alt((char('+'), char('-'))))(input)?;
    let (input, value) = hexadecimal(input)?;
    let mut value = value as f64;
    let (input, radix) = opt(preceded(
        tag("."),
        map_res(hex_digit1, |s| {
            u64::from_str_radix(s, 16).map(|v| (s.len(), v))
        }),
    ))(input)?;
    if let Some((radix_len, radix)) = radix {
        value += radix as f64 / (16.0f64).powi(radix_len as i32);
    }
    let (input, (exp_sign, exponent)) = preceded(tag("p"), pair(opt(char('-')), decimal))(input)?;
    let mut exponent = exponent as f64;
    if exp_sign == Some('-') {
        exponent *= -1.0;
    }
    value *= exponent.exp2();
    if sign == Some('-') {
        value *= -1.0;
    }
    Ok((input, value))
}

fn float_value(input: &str) -> IResult<&str, f64> {
    alt((
        hexadecimal_float,
        map_res(recognize_decimal_float, f64::from_str),
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
            context("integer", integer),
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
        data_item(text.as_ref()).map_err(|e| format!("Parsing error ({e:?})"))?;
    if !remaining.is_empty() {
        return Err(format!("Remaining text ({remaining:?})").into());
    }
    Ok(parsed)
}
