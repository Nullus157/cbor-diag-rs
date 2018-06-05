use {Error, Result, Simple, Value};

fn simple_to_hex(simple: Simple, s: &mut String) -> Result<()> {
    let Simple(value) = simple;

    if value < 24 {
        s.push_str(&format!("{:02x} # ", 0b1110_0000 | value));
    } else if value < 32 {
        return Err(Error::Todos("simple out of range"));
    } else {
        s.push_str(&format!("ff {:02x} # ", value));
    }

    match simple {
        Simple::FALSE => s.push_str("false, "),
        Simple::TRUE => s.push_str("true, "),
        Simple::NULL => s.push_str("null, "),
        Simple::UNDEFINED => s.push_str("undefined, "),
        _ => (),
    }

    s.push_str(&format!("simple({})", value));
    Ok(())
}

fn to_hex(value: &Value, s: &mut String) -> Result<()> {
    match *value {
        Value::Simple(simple) => simple_to_hex(simple, s)?,
        _ => unimplemented!(),
    }
    Ok(())
}

impl Value {
    pub fn to_hex(&self) -> Result<String> {
        let mut s = String::with_capacity(128);
        to_hex(self, &mut s)?;
        Ok(s)
    }
}
