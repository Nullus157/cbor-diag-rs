use {Error, Result, Simple, Value};

pub fn parse_bytes(bytes: impl AsRef<[u8]>) -> Result<Value> {
    Ok(Value::Simple(Simple::NULL))
}
