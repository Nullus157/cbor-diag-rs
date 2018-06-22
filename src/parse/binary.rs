use std::str;

// TODO(https://github.com/Geal/nom/pull/791)
use nom::Context;

use {ByteString, Error, IntegerWidth, Result, TextString, Value};

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
    positive<(&[u8], usize), Value>,
    preceded!(
        tag_bits!(u8, 3, 0),
        map!(integer, |(value, bitwidth)| Value::Integer { value, bitwidth }))
}

named! {
    negative<(&[u8], usize), Value>,
    preceded!(
        tag_bits!(u8, 3, 1),
        map!(integer, |(value, bitwidth)| Value::Negative { value, bitwidth }))
}

named! {
    definite_bytestring<(&[u8], usize), ByteString>,
    do_parse!(
        tag_bits!(u8, 3, 2) >>
        length: integer >>
        data: bytes!(take!(length.0)) >>
        (ByteString { data: data.into(), bitwidth: length.1 }))
}

named! {
    indefinite_bytestring<(&[u8], usize), Value>,
    preceded!(
        pair!(tag_bits!(u8, 3, 2), tag_bits!(u8, 5, 31)),
        map!(
            many_till!(definite_bytestring, pair!(tag_bits!(u8, 3, 7), tag_bits!(u8, 5, 31))),
            |(strings, _)| Value::IndefiniteByteString(strings)))
}

named! {
    bytestring<(&[u8], usize), Value>,
    alt_complete!(
        definite_bytestring => { Value::ByteString }
      | indefinite_bytestring
    )
}

named! {
    definite_textstring<(&[u8], usize), TextString>,
    do_parse!(
        tag_bits!(u8, 3, 3) >>
        length: integer >>
        data: map_res!(bytes!(take!(length.0)), |b| str::from_utf8(b)) >>
        (TextString { data: data.to_owned(), bitwidth: length.1 }))
}

named! {
    indefinite_textstring<(&[u8], usize), Value>,
    preceded!(
        pair!(tag_bits!(u8, 3, 3), tag_bits!(u8, 5, 31)),
        map!(
            many_till!(definite_textstring, pair!(tag_bits!(u8, 3, 7), tag_bits!(u8, 5, 31))),
            |(strings, _)| Value::IndefiniteTextString(strings)))
}

named! {
    textstring<(&[u8], usize), Value>,
    alt_complete!(
        definite_textstring => { Value::TextString }
      | indefinite_textstring
    )
}

named! {
    definite_array<(&[u8], usize), Value>,
    do_parse!(
        tag_bits!(u8, 3, 4) >>
        length: integer >>
        data: bytes!(count!(value, length.0 as usize)) >>
        (Value::Array { data, bitwidth: Some(length.1) }))
}

named! {
    simple<(&[u8], usize), Value>,
    preceded!(
        tag_bits!(u8, 3, 7),
        map!(
            alt_complete!(
                verify!(take_bits!(u8, 5), |v| v < 24)
              | preceded!(tag_bits!(u8, 5, 24), take_bits!(u8, 8))
            ),
            Value::simple
        )
    )
}

named! {
    value<&[u8], Value>,
    bits!(alt_complete!(
        positive
      | negative
      | bytestring
      | textstring
      | definite_array
      | simple
    ))
}

pub fn parse_bytes(bytes: impl AsRef<[u8]>) -> Result<Value> {
    let (remaining, parsed) = value(bytes.as_ref()).map_err(|e| {
        println!("{}: {:?}", e, e);
        Error::Todos("Parsing error")
    })?;
    if !remaining.is_empty() {
        return Err(Error::Todos("Remaining text"));
    }
    Ok(parsed)
}
