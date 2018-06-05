use {Error, Result, Simple, Value};

pub fn parse_bytes(bytes: impl AsRef<[u8]>) -> Result<Value> {
    Ok(match bytes.as_ref() {
        [0xf4] => Value::Simple(Simple::FALSE),
        [0xf5] => Value::Simple(Simple::TRUE),
        [0xf6] => Value::Simple(Simple::NULL),
        [0xf7] => Value::Simple(Simple::UNDEFINED),
        _ => unimplemented!(),
    })
}
