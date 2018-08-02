use std::f64;
use std::str::FromStr;

use hex;
use nom::{self, digit, hex_digit0};

use nom::Needed;

use {
    ByteString, Error, FloatWidth, IntegerWidth, Result, Simple, Tag,
    TextString, Value,
};

type NStr<'a> = nom::types::CompleteStr<'a>;

fn parse<T: FromStr>(s: NStr) -> ::std::result::Result<T, T::Err> {
    T::from_str(s.0)
}

named! {
    encoding<NStr, u64>,
    preceded!(tag!("_"), verify!(map_res!(digit, parse::<u64>), |e| e < 4))
}

named! {
    positive<NStr, Value>,
    do_parse!(
        value: map_res!(digit, parse::<u64>) >>
        encoding: opt!(encoding) >>
        (Value::Integer {
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
    negative<NStr, Value>,
    preceded!(
        tag!("-"),
        do_parse!(
            value: map_res!(digit, parse::<u64>) >>
            encoding: opt!(encoding) >>
            (Value::Negative {
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
        map_res!(
            preceded!(tag!("h"), delimited!(tag!("'"), hex_digit0, tag!("'"))),
            |s: NStr| hex::decode(s.as_ref())),
        |data| ByteString { data, bitwidth: IntegerWidth::Unknown })
}

named! {
    indefinite_bytestring<NStr, Value>,
    map!(
        delimited!(
            tag!("(_ "),
            separated_list_complete!(tag!(", "), definite_bytestring),
            tag!(")")),
        Value::IndefiniteByteString)
}

named! {
    bytestring<NStr, Value>,
    alt_complete!(
        definite_bytestring => { Value::ByteString }
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
    indefinite_textstring<NStr, Value>,
    map!(
        delimited!(
            tag!("(_ "),
            separated_list_complete!(tag!(", "), definite_textstring),
            tag!(")")),
        Value::IndefiniteTextString)
}

named! {
    textstring<NStr, Value>,
    alt_complete!(
        definite_textstring => { Value::TextString }
      | indefinite_textstring
    )
}

named! {
    definite_array<NStr, Value>,
    map!(
        delimited!(
            tag!("["),
            separated_list_complete!(tag!(", "), value),
            tag!("]")),
        |data| Value::Array { data, bitwidth: Some(IntegerWidth::Unknown) })
}

named! {
    indefinite_array<NStr, Value>,
    map!(
        delimited!(
            tag!("[_ "),
            separated_list_complete!(tag!(", "), value),
            tag!("]")),
        |data| Value::Array { data, bitwidth: None })
}

named! {
    array<NStr, Value>,
    alt_complete!(definite_array | indefinite_array)
}

named! {
    definite_map<NStr, Value>,
    map!(
        delimited!(
            tag!("{"),
            separated_list_complete!(tag!(","), separated_pair!(value, tag!(":"), value)),
            tag!("}")),
        |data| Value::Map { data, bitwidth: Some(IntegerWidth::Unknown) })
}

named! {
    indefinite_map<NStr, Value>,
    map!(
        ws!(delimited!(
            tag!("{_"),
            separated_list_complete!(tag!(","), separated_pair!(value, tag!(":"), value)),
            tag!("}"))),
        |data| Value::Map { data, bitwidth: None })
}

named! {
    map<NStr, Value>,
    alt_complete!(definite_map | indefinite_map)
}

named! {
    tagged<NStr, Value>,
    do_parse!(
        tag: map_res!(digit, parse::<u64>) >>
        encoding: opt!(encoding) >>
        value: delimited!(tag!("("), value, tag!(")")) >>
        (Value::Tag {
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
    float<NStr, Value>,
    do_parse!(
        value: float_value >>
        encoding: opt!(verify!(encoding, |e| e > 0)) >>
        (Value::Float {
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
    simple<NStr, Value>,
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
        Value::Simple)
}

named! {
    value<NStr, Value>,
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

pub fn parse_diag(text: impl AsRef<str>) -> Result<Value> {
    let text = nom::types::CompleteStr(text.as_ref());
    let (remaining, parsed) = value(text).map_err(|e| {
        println!("{}: {:?}", e, e);
        Error::Todos("Parsing error")
    })?;
    if !remaining.is_empty() {
        println!("parsed: {:?} remaining: {:?}", parsed, remaining);
        return Err(Error::Todos("Remaining text"));
    }
    Ok(parsed)
}
