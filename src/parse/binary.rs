#![allow(clippy::useless_let_if_seq)]
use std::{convert::TryFrom, str};

use half::f16;
use nom::{
    bits::{bits, bytes},
    branch::alt,
    bytes::streaming::take as take_bytes,
    combinator::{map, map_res, verify},
    error::{make_error, ErrorKind},
    multi::{count, many_till},
    number::streaming::{be_f32, be_f64, be_u16},
    sequence::{pair, preceded},
    Err, IResult,
};

use crate::{ByteString, DataItem, FloatWidth, IntegerWidth, Result, Simple, Tag, TextString};

pub fn take_bits<I, O>(count: usize) -> impl FnMut((I, usize)) -> IResult<(I, usize), O>
where
    I: nom::Slice<std::ops::RangeFrom<usize>> + nom::InputIter<Item = u8> + nom::InputLength,
    O: From<u8>
        + core::ops::AddAssign
        + core::ops::Shl<usize, Output = O>
        + core::ops::Shr<usize, Output = O>,
{
    nom::bits::streaming::take(count)
}

pub fn tag_bits<I, O>(pattern: O, count: usize) -> impl FnMut((I, usize)) -> IResult<(I, usize), O>
where
    I: nom::Slice<std::ops::RangeFrom<usize>>
        + nom::InputIter<Item = u8>
        + nom::InputLength
        + Clone,
    O: From<u8>
        + core::ops::AddAssign
        + core::ops::Shl<usize, Output = O>
        + core::ops::Shr<usize, Output = O>
        + PartialEq,
{
    nom::bits::streaming::tag(pattern, count)
}

fn integer(input: (&[u8], usize)) -> IResult<(&[u8], usize), (u64, IntegerWidth)> {
    alt((
        pair(verify(take_bits(5), |&v| v < 24), |i| {
            Ok((i, IntegerWidth::Zero))
        }),
        pair(preceded(tag_bits(24, 5), take_bits(8)), |i| {
            Ok((i, IntegerWidth::Eight))
        }),
        pair(preceded(tag_bits(25, 5), take_bits(16)), |i| {
            Ok((i, IntegerWidth::Sixteen))
        }),
        pair(preceded(tag_bits(26, 5), take_bits(32)), |i| {
            Ok((i, IntegerWidth::ThirtyTwo))
        }),
        pair(preceded(tag_bits(27, 5), take_bits(64)), |i| {
            Ok((i, IntegerWidth::SixtyFour))
        }),
    ))(input)
}

fn positive(input: &[u8]) -> IResult<&[u8], DataItem> {
    bits(preceded(
        tag_bits(0, 3),
        map(integer, |(value, bitwidth)| DataItem::Integer {
            value,
            bitwidth,
        }),
    ))(input)
}

fn negative(input: &[u8]) -> IResult<&[u8], DataItem> {
    bits(preceded(
        tag_bits(1, 3),
        map(integer, |(value, bitwidth)| DataItem::Negative {
            value,
            bitwidth,
        }),
    ))(input)
}

fn definite_bytestring(input: &[u8]) -> IResult<&[u8], ByteString> {
    let (input, (length, bitwidth)) = bits(preceded(tag_bits(2, 3), integer))(input)?;
    let length = usize::try_from(length)
        .map_err(|_| Err::Error(make_error(input, ErrorKind::LengthValue)))?;
    let (input, data) = take_bytes(length)(input)?;
    let data = data.to_owned();
    Ok((input, ByteString { data, bitwidth }))
}

fn indefinite_bytestring(input: &[u8]) -> IResult<&[u8], DataItem> {
    preceded(
        bits(pair(tag_bits(2, 3), tag_bits(31, 5))),
        map(many_till(definite_bytestring, stop_code), |(strings, _)| {
            DataItem::IndefiniteByteString(strings)
        }),
    )(input)
}

fn bytestring(input: &[u8]) -> IResult<&[u8], DataItem> {
    alt((
        map(definite_bytestring, DataItem::ByteString),
        indefinite_bytestring,
    ))(input)
}

fn definite_textstring(input: &[u8]) -> IResult<&[u8], TextString> {
    let (input, (length, bitwidth)) = bits(preceded(tag_bits(3, 3), integer))(input)?;
    let length = usize::try_from(length)
        .map_err(|_| Err::Error(make_error(input, ErrorKind::LengthValue)))?;
    let (input, data) = map_res(take_bytes(length), str::from_utf8)(input)?;
    let data = data.to_owned();
    Ok((input, TextString { data, bitwidth }))
}

fn indefinite_textstring(input: &[u8]) -> IResult<&[u8], DataItem> {
    preceded(
        bits(pair(tag_bits(3, 3), tag_bits(31, 5))),
        map(many_till(definite_textstring, stop_code), |(strings, _)| {
            DataItem::IndefiniteTextString(strings)
        }),
    )(input)
}

fn textstring(input: &[u8]) -> IResult<&[u8], DataItem> {
    alt((
        map(definite_textstring, DataItem::TextString),
        indefinite_textstring,
    ))(input)
}

fn definite_array(input: &[u8]) -> IResult<&[u8], DataItem> {
    let (input, (length, bitwidth)) = bits(preceded(tag_bits(4, 3), integer))(input)?;
    let (input, data) = count(data_item, length as usize)(input)?;
    Ok((
        input,
        DataItem::Array {
            data,
            bitwidth: Some(bitwidth),
        },
    ))
}

fn indefinite_array(input: &[u8]) -> IResult<&[u8], DataItem> {
    preceded(
        bits(pair(tag_bits(4, 3), tag_bits(31, 5))),
        map(many_till(data_item, stop_code), |(data, _)| {
            DataItem::Array {
                data,
                bitwidth: None,
            }
        }),
    )(input)
}

fn array(input: &[u8]) -> IResult<&[u8], DataItem> {
    alt((definite_array, indefinite_array))(input)
}

fn definite_map(input: &[u8]) -> IResult<&[u8], DataItem> {
    let (input, (length, bitwidth)) = bits(preceded(tag_bits(5, 3), integer))(input)?;
    let (input, data) = count(pair(data_item, data_item), length as usize)(input)?;
    Ok((
        input,
        DataItem::Map {
            data,
            bitwidth: Some(bitwidth),
        },
    ))
}

fn indefinite_map(input: &[u8]) -> IResult<&[u8], DataItem> {
    preceded(
        bits(pair(tag_bits(5, 3), tag_bits(31, 5))),
        map(
            many_till(pair(data_item, data_item), stop_code),
            |(data, _)| DataItem::Map {
                data,
                bitwidth: None,
            },
        ),
    )(input)
}

fn data_map(input: &[u8]) -> IResult<&[u8], DataItem> {
    alt((definite_map, indefinite_map))(input)
}

fn tag_bitsged(input: &[u8]) -> IResult<&[u8], DataItem> {
    let (input, (tag, bitwidth)) = bits(preceded(tag_bits(6, 3), integer))(input)?;
    let (input, value) = data_item(input)?;
    let value = Box::new(value);
    Ok((
        input,
        DataItem::Tag {
            tag: Tag(tag),
            bitwidth,
            value,
        },
    ))
}

fn float(input: &[u8]) -> IResult<&[u8], DataItem> {
    bits(preceded(
        tag_bits(7, 3),
        map(
            alt((
                preceded(
                    tag_bits(25, 5),
                    bytes::<_, _, nom::error::Error<&[u8]>, _, _>(map(be_u16, |u| {
                        (f16::from_bits(u).to_f64(), FloatWidth::Sixteen)
                    })),
                ),
                preceded(
                    tag_bits(26, 5),
                    bytes::<_, _, nom::error::Error<&[u8]>, _, _>(map(be_f32, |f| {
                        (f64::from(f), FloatWidth::ThirtyTwo)
                    })),
                ),
                preceded(
                    tag_bits(27, 5),
                    bytes::<_, _, nom::error::Error<&[u8]>, _, _>(map(be_f64, |f| {
                        (f, FloatWidth::SixtyFour)
                    })),
                ),
            )),
            |(value, bitwidth)| DataItem::Float { value, bitwidth },
        ),
    ))(input)
}

fn simple(input: &[u8]) -> IResult<&[u8], DataItem> {
    bits(preceded(
        tag_bits(7, 3),
        map(
            alt((
                verify(take_bits(5), |&v| v < 24),
                preceded(tag_bits(24, 5), take_bits(8)),
            )),
            |value| DataItem::Simple(Simple(value)),
        ),
    ))(input)
}

fn stop_code(input: &[u8]) -> IResult<&[u8], DataItem> {
    bits(preceded(
        tag_bits(7, 3),
        map(tag_bits(31, 5), |value| DataItem::Simple(Simple(value))),
    ))(input)
}

fn data_item(input: &[u8]) -> IResult<&[u8], DataItem> {
    alt((
        positive,
        negative,
        bytestring,
        textstring,
        array,
        data_map,
        tag_bitsged,
        float,
        simple,
    ))(input)
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
    let (remaining, parsed) =
        data_item(bytes.as_ref()).map_err(|e| format!("Parsing error ({e:?})"))?;
    if !remaining.is_empty() {
        return Err(format!(
            "Remaining bytes ({})",
            data_encoding::HEXLOWER.encode(remaining)
        )
        .into());
    }
    Ok(parsed)
}

/// Parse a string containing a binary encoded CBOR data item, optionally followed by more data.
///
/// Returns one of:
///
///  * `Err(_)` => a parsing error if there was an issue encountered
///  * `Ok(None)` => the end of a data item was not reached
///  * `Ok(Some(_))` => the parsed item along with how many bytes were used to parse this item
///
/// # Examples
///
/// ```rust
/// use cbor_diag::{DataItem, IntegerWidth, Tag, TextString};
///
/// assert_eq!(
///     cbor_diag::parse_bytes_partial(&b"\
///         \xd8\x20\x73\x68\x74\x74\x70\x73\x3a\x2f\x2f\x65\x78\x61\x6d\x70\
///         \x6c\x65\x2e\x63\x6f\
///     "[..]).unwrap(),
///     None);
///
/// assert_eq!(
///     cbor_diag::parse_bytes_partial(&b"\
///         \xd8\x20\x73\x68\x74\x74\x70\x73\x3a\x2f\x2f\x65\x78\x61\x6d\x70\
///         \x6c\x65\x2e\x63\x6f\x6d\xff\
///     "[..]).unwrap(),
///     Some((
///         DataItem::Tag {
///             tag: Tag::URI,
///             bitwidth: IntegerWidth::Eight,
///             value: Box::new(DataItem::TextString(TextString {
///                 data: "https://example.com".into(),
///                 bitwidth: IntegerWidth::Zero,
///             })),
///         },
///         22
///     )));
/// ```
pub fn parse_bytes_partial(bytes: impl AsRef<[u8]>) -> Result<Option<(DataItem, usize)>> {
    match data_item(bytes.as_ref()) {
        Ok((remaining, item)) => Ok(Some((item, bytes.as_ref().len() - remaining.len()))),
        Err(nom::Err::Incomplete(_)) => Ok(None),
        Err(nom::Err::Failure(_)) | Err(nom::Err::Error(_)) => Err("Parser error".into()),
    }
}
