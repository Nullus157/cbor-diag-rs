use std::ascii;

use hex;

use {IntegerWidth, Result, Simple, Value, ByteString, TextString};

fn integer_to_hex(value: u64, mut bitwidth: IntegerWidth, hex: &mut Vec<String>, comment: &mut Vec<String>) -> Result<()> {
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
        IntegerWidth::Zero => hex.push(format!("{:02x}", value)),
        IntegerWidth::Eight => hex.push(format!("18 {:02x}", value)),
        IntegerWidth::Sixteen => hex.push(format!("19 {:04x}", value)),
        IntegerWidth::ThirtyTwo => hex.push(format!("1a {:08x}", value)),
        IntegerWidth::SixtyFour => hex.push(format!("1b {:016x}", value)),
    }

    comment.push(format!("unsigned({})", value));
    Ok(())
}

fn negative_to_hex(value: u64, mut bitwidth: IntegerWidth, hex: &mut Vec<String>, comment: &mut Vec<String>) -> Result<()> {
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
        IntegerWidth::Zero => hex.push(format!("{:02x}", value + 0x20)),
        IntegerWidth::Eight => hex.push(format!("38 {:02x}", value)),
        IntegerWidth::Sixteen => hex.push(format!("39 {:04x}", value)),
        IntegerWidth::ThirtyTwo => hex.push(format!("3a {:08x}", value)),
        IntegerWidth::SixtyFour => hex.push(format!("3b {:016x}", value)),
    }

    comment.push(format!("negative({})", value));
    Ok(())
}

fn string_length_to_hex(length: usize, mut bitwidth: IntegerWidth, major: u8, kind: &str, hex: &mut Vec<String>, comment: &mut Vec<String>) -> Result<()> {
    if bitwidth == IntegerWidth::Unknown {
        bitwidth = if length < 24 {
            IntegerWidth::Zero
        } else if length < usize::from(u8::max_value()) {
            IntegerWidth::Eight
        } else if length < usize::from(u16::max_value()) {
            IntegerWidth::Sixteen
        } else if length < u32::max_value() as usize {
            IntegerWidth::ThirtyTwo
        } else {
            IntegerWidth::SixtyFour
        };
    }

    match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => hex.push(format!("{:02x}", (length as u8) + (major << 5))),
        IntegerWidth::Eight => hex.push(format!("{:02x} {:02x}", (major << 5) | 0x18, length)),
        IntegerWidth::Sixteen => hex.push(format!("{:02x} {:04x}", (major << 5) | 0x19, length)),
        IntegerWidth::ThirtyTwo => hex.push(format!("{:02x} {:08x}", (major << 5) | 0x1a, length)),
        IntegerWidth::SixtyFour => hex.push(format!("{:02x} {:016x}", (major << 5) | 0x1b, length)),
    }

    comment.push(format!("{kind}({length})", kind=kind, length=length));

    Ok(())
}

fn bytestring_to_hex(bytestring: &ByteString, hex: &mut Vec<String>, comment: &mut Vec<String>) -> Result<()> {
    let ByteString { ref data, bitwidth } = *bytestring;

    string_length_to_hex(bytestring.data.len(), bitwidth, 2, "bytes", hex, comment)?;

    if data.is_empty() {
        hex.push("".into());
        comment.push("\"\"".into());
    } else {
        for line in data.chunks(16) {
            let text: String = line
                .iter()
                .cloned()
                .flat_map(ascii::escape_default)
                .map(char::from)
                .collect();
            hex.push(format!("   {}", hex::encode(line)));
            comment.push(format!("\"{}\"", text));
        }
    }

    Ok(())
}

fn textstring_to_hex(textstring: &TextString, hex: &mut Vec<String>, comment: &mut Vec<String>) -> Result<()> {
    let TextString { ref data, bitwidth } = *textstring;

    string_length_to_hex(data.len(), bitwidth, 3, "text", hex, comment)?;

    if data.is_empty() {
        hex.push("".into());
        comment.push("\"\"".into());
        return Ok(());
    } else {
        let mut data = data.as_str();
        while !data.is_empty() {
            let mut split = 16;
            while !data.is_char_boundary(split) {
                split -= 1;
            }
            let (line, new_data) = data.split_at(split);
            data = new_data;
            hex.push(format!("   {}", hex::encode(line)));
            comment.push(format!("\"{}\"", line));
        }
    }

    Ok(())
}

fn simple_to_hex(simple: Simple, hex: &mut Vec<String>, comment: &mut Vec<String>) -> Result<()> {
    let Simple(value) = simple;

    if value < 24 {
        hex.push(format!("{:02x}", 0b1110_0000 | value));
    } else {
        hex.push(format!("f8 {:02x}", value));
    }

    let extra = match simple {
        Simple::FALSE => "false, ",
        Simple::TRUE => "true, ",
        Simple::NULL => "null, ",
        Simple::UNDEFINED => "undefined, ",
        Simple(24...32) => "reserved, ",
        _ => "unassigned, ",
    };

    comment.push(format!("{}simple({})", extra, value));
    Ok(())
}

fn to_hex(value: &Value, hex: &mut Vec<String>, comment: &mut Vec<String>) -> Result<()> {
    match *value {
        Value::Integer { value, bitwidth } => integer_to_hex(value, bitwidth, hex, comment)?,
        Value::Negative { value, bitwidth } => negative_to_hex(value, bitwidth, hex, comment)?,
        Value::ByteString(ref bytestring) => bytestring_to_hex(bytestring, hex, comment)?,
        Value::TextString(ref textstring) => textstring_to_hex(textstring, hex, comment)?,
        Value::Simple(simple) => simple_to_hex(simple, hex, comment)?,
        _ => unimplemented!(),
    }
    Ok(())
}

fn merge(hex: &[String], comment: &[String]) -> String {
    let width = hex.iter().map(|line| line.len()).max().unwrap_or(0);

    hex.iter()
        .zip(comment)
        .map(|(hex, comment)| format!("{hex:width$} # {comment}\n", hex=hex, width=width, comment=comment))
        .collect()
}

impl Value {
    pub fn to_hex(&self) -> Result<String> {
        let mut hex = Vec::with_capacity(16);
        let mut comment = Vec::with_capacity(16);
        to_hex(self, &mut hex, &mut comment)?;
        Ok(merge(&hex, &comment))
    }
}
