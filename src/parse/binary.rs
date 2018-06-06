use {Error, IntegerWidth, Result, Value};

named! {
    integer<(&[u8], usize), Value>,
    preceded!(
        tag_bits!(u8, 3, 0),
        alt_complete!(
            map!(
                verify!(take_bits!(u64, 5), |v| v < 24),
                |value| Value::Integer { value, bitwidth: IntegerWidth::Zero })
            | map!(
                preceded!(tag_bits!(u8, 5, 24), take_bits!(u64, 8)),
                |value| Value::Integer { value, bitwidth: IntegerWidth::Eight })
            | map!(
                preceded!(tag_bits!(u8, 5, 25), take_bits!(u64, 16)),
                |value| Value::Integer { value, bitwidth: IntegerWidth::Sixteen })
            | map!(
                preceded!(tag_bits!(u8, 5, 26), take_bits!(u64, 32)),
                |value| Value::Integer { value, bitwidth: IntegerWidth::ThirtyTwo })
            | map!(
                preceded!(tag_bits!(u8, 5, 27), take_bits!(u64, 64)),
                |value| Value::Integer { value, bitwidth: IntegerWidth::SixtyFour })
        ))
}

named! {
    negative<(&[u8], usize), Value>,
    preceded!(
        tag_bits!(u8, 3, 1),
        alt_complete!(
            map!(
                verify!(take_bits!(u64, 5), |v| v < 24),
                |value| Value::Negative { value, bitwidth: IntegerWidth::Zero })
            | map!(
                preceded!(tag_bits!(u8, 5, 24), take_bits!(u64, 8)),
                |value| Value::Negative { value, bitwidth: IntegerWidth::Eight })
            | map!(
                preceded!(tag_bits!(u8, 5, 25), take_bits!(u64, 16)),
                |value| Value::Negative { value, bitwidth: IntegerWidth::Sixteen })
            | map!(
                preceded!(tag_bits!(u8, 5, 26), take_bits!(u64, 32)),
                |value| Value::Negative { value, bitwidth: IntegerWidth::ThirtyTwo })
            | map!(
                preceded!(tag_bits!(u8, 5, 27), take_bits!(u64, 64)),
                |value| Value::Negative { value, bitwidth: IntegerWidth::SixtyFour })
        ))
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
    bits!(alt_complete!(integer | negative | simple))
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
