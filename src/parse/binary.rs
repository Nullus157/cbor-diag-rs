use {Error, Result, Simple, Value};

named! {
    simple_inline<(&[u8], usize), u8>,
    verify!(take_bits!(u8, 5), |v| v < 24)
}

named! {
    simple_byte<(&[u8], usize), u8>,
    do_parse!(
        tag_bits!(u8, 5, 24) >>
        value: take_bits!(u8, 8) >>
        (value))
}

named! {
    simple<(&[u8], usize), Value>,
    do_parse!(
        tag_bits!(u8, 3, 7) >>
        value: alt!(simple_inline | simple_byte) >>
        (Value::Simple(Simple(value))))
}

pub fn parse_bytes(bytes: impl AsRef<[u8]>) -> Result<Value> {
    let ((remaining, _), parsed) = simple((bytes.as_ref(), 0)).map_err(|e| {
        println!("{}: {:?}", e, e);
        Error::Todos("Parsing error")
    })?;
    if !remaining.is_empty() {
        return Err(Error::Todos("Remaining text"));
    }
    Ok(parsed)
}
