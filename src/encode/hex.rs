use {IntegerWidth, Result, Simple, Value};

fn integer_to_hex(value: u64, mut bitwidth: IntegerWidth, s: &mut String) -> Result<()> {
    if bitwidth == IntegerWidth::Unknown {
        bitwidth = if value < 24 {
            IntegerWidth::Zero
        } else if value < u64::from(u8::max_value()) {
            IntegerWidth::Eight
        } else if value < u64::from(u16::max_value()) {
            IntegerWidth::Sixteen
        } else if value < u64::from(u32::max_value()) {
            IntegerWidth::ThirtyTwo
        } else {
            IntegerWidth::SixtyFour
        };
    }

    match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => s.push_str(&format!("{:02x}", value)),
        IntegerWidth::Eight => s.push_str(&format!("18 {:02x}", value)),
        IntegerWidth::Sixteen => s.push_str(&format!("19 {:02x} {:02x}", value >> 8, value)),
        IntegerWidth::ThirtyTwo => s.push_str(&format!(
            "1a {:02x} {:02x} {:02x} {:02x}",
            value >> 24,
            value >> 16,
            value >> 8,
            value
        )),
        IntegerWidth::SixtyFour => s.push_str(&format!(
            "1b {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
            value >> 56,
            value >> 48,
            value >> 40,
            value >> 32,
            value >> 24,
            value >> 16,
            value >> 8,
            value
        )),
    }

    s.push_str(&format!(" # unsigned({})", value));
    Ok(())
}

fn negative_to_hex(value: u64, mut bitwidth: IntegerWidth, s: &mut String) -> Result<()> {
    if bitwidth == IntegerWidth::Unknown {
        bitwidth = if value < 24 {
            IntegerWidth::Zero
        } else if value < u64::from(u8::max_value()) {
            IntegerWidth::Eight
        } else if value < u64::from(u16::max_value()) {
            IntegerWidth::Sixteen
        } else if value < u64::from(u32::max_value()) {
            IntegerWidth::ThirtyTwo
        } else {
            IntegerWidth::SixtyFour
        };
    }

    match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => s.push_str(&format!("{:02x}", value + 0x20)),
        IntegerWidth::Eight => s.push_str(&format!("38 {:02x}", value)),
        IntegerWidth::Sixteen => s.push_str(&format!("39 {:02x} {:02x}", value >> 8, value)),
        IntegerWidth::ThirtyTwo => s.push_str(&format!(
            "3a {:02x} {:02x} {:02x} {:02x}",
            value >> 24,
            value >> 16,
            value >> 8,
            value
        )),
        IntegerWidth::SixtyFour => s.push_str(&format!(
            "3b {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
            value >> 56,
            value >> 48,
            value >> 40,
            value >> 32,
            value >> 24,
            value >> 16,
            value >> 8,
            value
        )),
    }

    s.push_str(&format!(" # negative({})", value));
    Ok(())
}

fn simple_to_hex(simple: Simple, s: &mut String) -> Result<()> {
    let Simple(value) = simple;

    if value < 24 {
        s.push_str(&format!("{:02x} # ", 0b1110_0000 | value));
    } else {
        s.push_str(&format!("f8 {:02x} # ", value));
    }

    match simple {
        Simple::FALSE => s.push_str("false, "),
        Simple::TRUE => s.push_str("true, "),
        Simple::NULL => s.push_str("null, "),
        Simple::UNDEFINED => s.push_str("undefined, "),
        Simple(24...32) => s.push_str("reserved, "),
        _ => s.push_str("unassigned, "),
    }

    s.push_str(&format!("simple({})", value));
    Ok(())
}

fn to_hex(value: &Value, s: &mut String) -> Result<()> {
    match *value {
        Value::Integer { value, bitwidth } => integer_to_hex(value, bitwidth, s)?,
        Value::NegativeInteger { value, bitwidth } => negative_to_hex(value, bitwidth, s)?,
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
