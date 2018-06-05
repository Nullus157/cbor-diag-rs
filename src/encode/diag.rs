use {Result, Simple, Value};

fn simple_to_diag(simple: Simple, s: &mut String) -> Result<()> {
    match simple {
        Simple::FALSE => s.push_str("false"),
        Simple::TRUE => s.push_str("true"),
        Simple::NULL => s.push_str("null"),
        Simple::UNDEFINED => s.push_str("undefined"),
        Simple(value) => s.push_str(&format!("simple({})", value)),
    }
    Ok(())
}

fn value_to_diag(value: &Value, s: &mut String) -> Result<()> {
    match *value {
        Value::Simple(simple) => simple_to_diag(simple, s)?,
        _ => unimplemented!(),
    }
    Ok(())
}

impl Value {
    pub fn to_diag(&self) -> Result<String> {
        let mut s = String::with_capacity(128);
        value_to_diag(self, &mut s)?;
        Ok(s)
    }
}
