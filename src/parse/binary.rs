// TODO(https://github.com/Geal/nom/pull/791)
use nom::Context;

use {Error, IntegerWidth, Result, Value};

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
    bytestring<(&[u8], usize), Value>,
    do_parse!(
        tag_bits!(u8, 3, 2) >>
        length: integer >>
        data: bytes!(take!(length.0)) >>
        (Value::ByteString { data: data.into(), bitwidth: Some(length.1) }))
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
    bits!(alt_complete!(positive | negative | bytestring | simple))
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
