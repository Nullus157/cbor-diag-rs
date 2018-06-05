use {Error, Result, Simple, Value};

pub fn parse_diag(text: impl AsRef<str>) -> Result<Value> {
    Ok(Value::Simple(Simple::NULL))
}
