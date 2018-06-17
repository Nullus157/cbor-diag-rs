use hex;

use {IntegerWidth, Result, Simple, Value};

fn integer_to_diag(value: u64, bitwidth: IntegerWidth, s: &mut String) -> Result<()> {
    if bitwidth == IntegerWidth::Unknown || bitwidth == IntegerWidth::Zero {
        s.push_str(&value.to_string());
    } else {
        let encoding = match bitwidth {
            IntegerWidth::Eight => 0,
            IntegerWidth::Sixteen => 1,
            IntegerWidth::ThirtyTwo => 2,
            IntegerWidth::SixtyFour => 3,
            _ => unreachable!(),
        };
        s.push_str(&format!("{}_{}", value, encoding));
    }
    Ok(())
}

fn negative_to_diag(value: u64, bitwidth: IntegerWidth, s: &mut String) -> Result<()> {
    let value = -1i128 - i128::from(value);
    if bitwidth == IntegerWidth::Unknown || bitwidth == IntegerWidth::Zero {
        s.push_str(&value.to_string());
    } else {
        let encoding = match bitwidth {
            IntegerWidth::Eight => 0,
            IntegerWidth::Sixteen => 1,
            IntegerWidth::ThirtyTwo => 2,
            IntegerWidth::SixtyFour => 3,
            _ => unreachable!(),
        };
        s.push_str(&format!("{}_{}", value, encoding));
    }
    Ok(())
}

fn bytestring_to_diag(data: &[u8], bitwidth: Option<IntegerWidth>, s: &mut String) -> Result<()> {
    let _bitwidth = bitwidth.expect("indefinite length is unimplemented");

    s.push_str(&format!("h'{}'", hex::encode(data)));
    Ok(())
}

fn string_to_diag(data: &str, bitwidth: Option<IntegerWidth>, s: &mut String) -> Result<()> {
    let _bitwidth = bitwidth.expect("indefinite length is unimplemented");

    s.push('"');
    for c in data.chars() {
        if c == '\"' || c == '\\' {
            for c in c.escape_default() {
                s.push(c);
            }
        } else {
            s.push(c);
        }
    }
    s.push('"');

    Ok(())
}

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
        Value::Integer { value, bitwidth } => integer_to_diag(value, bitwidth, s)?,
        Value::Negative { value, bitwidth } => negative_to_diag(value, bitwidth, s)?,
        Value::ByteString { ref data, bitwidth } => bytestring_to_diag(data, bitwidth, s)?,
        Value::String { ref data, bitwidth } => string_to_diag(data, bitwidth, s)?,
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
