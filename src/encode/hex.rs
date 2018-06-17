use std::{ascii, cmp};

use hex;

use {IntegerWidth, Simple, Value, ByteString, TextString};

struct Line {
    hex: String,
    comment: String,
    sublines: Vec<Line>,
}

impl Line {
    fn new(hex: impl Into<String>, comment: impl Into<String>) -> Line {
        Line {
            hex: hex.into(),
            comment: comment.into(),
            sublines: Vec::new(),
        }
    }

    fn from_value(value: &Value) -> Line {
        match *value {
            Value::Integer { value, bitwidth } => {
                integer_to_hex(value, bitwidth)
            }
            Value::Negative { value, bitwidth } => {
                negative_to_hex(value, bitwidth)
            }
            Value::ByteString(ref bytestring) => {
                bytestring_to_hex(bytestring)
            }
            Value::TextString(ref textstring) => {
                textstring_to_hex(textstring)
            }
            Value::Simple(simple) => {
                simple_to_hex(simple)
            }
            _ => unimplemented!(),
        }
    }

    fn merge(self) -> String {
        let hex_width = self.hex_width();
        let mut output = String::with_capacity(128);
        self.do_merge(hex_width, 0, &mut output);
        output
    }

    fn do_merge(self, hex_width: usize, indent: usize, output: &mut String) {
        output.push_str(&format!(
            "{blank:indent$}{hex:width$} # {comment}\n",
            blank="",
            indent=indent,
            hex=self.hex,
            width=hex_width,
            comment=self.comment));

        for line in self.sublines {
            line.do_merge(hex_width - 3, indent + 3, output);
        }
    }

    fn hex_width(&self) -> usize {
        cmp::max(
            self.hex.len(),
            self.sublines.iter()
                .map(|line| line.hex_width() + 3)
                .max()
                .unwrap_or(0))
    }
}

fn integer_to_hex(value: u64, mut bitwidth: IntegerWidth) -> Line {
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

    let hex = match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => format!("{:02x}", value),
        IntegerWidth::Eight => format!("18 {:02x}", value),
        IntegerWidth::Sixteen => format!("19 {:04x}", value),
        IntegerWidth::ThirtyTwo => format!("1a {:08x}", value),
        IntegerWidth::SixtyFour => format!("1b {:016x}", value),
    };

    let comment = format!("unsigned({})", value);

    Line::new(hex, comment)
}

fn negative_to_hex(value: u64, mut bitwidth: IntegerWidth) -> Line {
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

    let hex = match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => format!("{:02x}", value + 0x20),
        IntegerWidth::Eight => format!("38 {:02x}", value),
        IntegerWidth::Sixteen => format!("39 {:04x}", value),
        IntegerWidth::ThirtyTwo => format!("3a {:08x}", value),
        IntegerWidth::SixtyFour => format!("3b {:016x}", value),
    };

    let comment = format!("negative({})", value);

    Line::new(hex, comment)
}

fn string_length_to_hex(length: usize, mut bitwidth: IntegerWidth, major: u8, kind: &str) -> Line {
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

    let hex = match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => format!("{:02x}", (length as u8) + (major << 5)),
        IntegerWidth::Eight => format!("{:02x} {:02x}", (major << 5) | 0x18, length),
        IntegerWidth::Sixteen => format!("{:02x} {:04x}", (major << 5) | 0x19, length),
        IntegerWidth::ThirtyTwo => format!("{:02x} {:08x}", (major << 5) | 0x1a, length),
        IntegerWidth::SixtyFour => format!("{:02x} {:016x}", (major << 5) | 0x1b, length),
    };

    let comment = format!("{kind}({length})", kind=kind, length=length);

    Line::new(hex, comment)
}

fn bytestring_to_hex(bytestring: &ByteString) -> Line {
    let ByteString { ref data, bitwidth } = *bytestring;

    let mut line = string_length_to_hex(data.len(), bitwidth, 2, "bytes");

    if data.is_empty() {
        line.sublines.push(Line::new("", "\"\""));
    } else {
        for datum in data.chunks(16) {
            let text: String = datum
                .iter()
                .cloned()
                .flat_map(ascii::escape_default)
                .map(char::from)
                .collect();
            let hex = hex::encode(datum);
            let comment = format!("\"{}\"", text);
            line.sublines.push(Line::new(hex, comment));
        }
    }

    line
}

fn textstring_to_hex(textstring: &TextString) -> Line {
    let TextString { ref data, bitwidth } = *textstring;

    let mut line = string_length_to_hex(data.len(), bitwidth, 3, "text");

    if data.is_empty() {
        line.sublines.push(Line::new("", "\"\""));
    } else {
        let mut data = data.as_str();
        while !data.is_empty() {
            let mut split = 16;
            while !data.is_char_boundary(split) {
                split -= 1;
            }
            let (datum, new_data) = data.split_at(split);
            data = new_data;
            let hex = hex::encode(datum);
            let comment = format!("\"{}\"", datum);
            line.sublines.push(Line::new(hex, comment));
        }
    }

    line
}

fn simple_to_hex(simple: Simple) -> Line {
    let Simple(value) = simple;

    let hex = if value < 24 {
        format!("{:02x}", 0b1110_0000 | value)
    } else {
        format!("f8 {:02x}", value)
    };

    let extra = match simple {
        Simple::FALSE => "false, ",
        Simple::TRUE => "true, ",
        Simple::NULL => "null, ",
        Simple::UNDEFINED => "undefined, ",
        Simple(24...32) => "reserved, ",
        _ => "unassigned, ",
    };

    let comment = format!("{}simple({})", extra, value);

    Line::new(hex, comment)
}

impl Value {
    pub fn to_hex(&self) -> String {
        Line::from_value(self).merge()
    }
}
