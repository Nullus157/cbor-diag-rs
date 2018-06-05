use {Error, Result, Simple, Value};

pub fn parse_diag(text: impl AsRef<str>) -> Result<Value> {
    Ok(match text.as_ref() {
        "false" => Value::Simple(Simple::FALSE),
        "true" => Value::Simple(Simple::TRUE),
        "null" => Value::Simple(Simple::NULL),
        "undefined" => Value::Simple(Simple::UNDEFINED),
        _ => unimplemented!(),
    })
}
