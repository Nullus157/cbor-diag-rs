use std::{ascii, cmp, i64, iter};

use super::diag;
use chrono::{DateTime, NaiveDateTime};
use half::f16;
use hex;
use num::{
    bigint::Sign, pow::pow, rational::Ratio, BigInt, BigRational, BigUint,
};

use {ByteString, FloatWidth, IntegerWidth, Simple, Tag, TextString, Value};

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
                definite_bytestring_to_hex(bytestring)
            }
            Value::IndefiniteByteString(ref bytestrings) => {
                indefinite_string_to_hex(
                    0x02,
                    "bytes",
                    bytestrings,
                    definite_bytestring_to_hex,
                )
            }
            Value::TextString(ref textstring) => {
                definite_textstring_to_hex(textstring)
            }
            Value::IndefiniteTextString(ref textstrings) => {
                indefinite_string_to_hex(
                    0x03,
                    "text",
                    textstrings,
                    definite_textstring_to_hex,
                )
            }
            Value::Array { ref data, bitwidth } => array_to_hex(data, bitwidth),
            Value::Map { ref data, bitwidth } => map_to_hex(data, bitwidth),
            Value::Tag {
                tag,
                bitwidth,
                ref value,
            } => tagged_to_hex(tag, bitwidth, &*value),
            Value::Float { value, bitwidth } => float_to_hex(value, bitwidth),
            Value::Simple(simple) => simple_to_hex(simple),
        }
    }

    fn merge(self) -> String {
        let hex_width = self.hex_width();
        let mut output = String::with_capacity(128);
        self.do_merge(hex_width as isize, 0, &mut output);
        output
    }

    fn do_merge(
        self,
        hex_width: isize,
        indent_level: usize,
        output: &mut String,
    ) {
        let (hex_indent, width) = if hex_width < 0 {
            (indent_level * 3 - hex_width.abs() as usize, 0)
        } else {
            (indent_level * 3, hex_width as usize)
        };

        output.push_str(&format!(
            "{blank:hex_indent$}{hex:width$} # {blank:comment_indent$}{comment}\n",
            blank = "",
            hex_indent = hex_indent,
            comment_indent = indent_level * 2,
            hex = self.hex,
            width = width,
            comment = self.comment
        ));

        for line in self.sublines {
            line.do_merge(hex_width - 3, indent_level + 1, output);
        }
    }

    fn hex_width(&self) -> usize {
        cmp::max(
            self.hex.len(),
            self.sublines
                .iter()
                .map(|line| {
                    let subwidth = line.hex_width();
                    if subwidth == 0 {
                        0
                    } else {
                        subwidth + 3
                    }
                }).max()
                .unwrap_or(0),
        )
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

fn length_to_hex(
    length: Option<usize>,
    mut bitwidth: Option<IntegerWidth>,
    major: u8,
    kind: &str,
) -> Line {
    // TODO: Rearrange the data to remove the unwraps.

    if bitwidth == Some(IntegerWidth::Unknown) {
        bitwidth = if length.unwrap() < 24 {
            Some(IntegerWidth::Zero)
        } else if length.unwrap() < usize::from(u8::max_value()) {
            Some(IntegerWidth::Eight)
        } else if length.unwrap() < usize::from(u16::max_value()) {
            Some(IntegerWidth::Sixteen)
        } else if length.unwrap() < u32::max_value() as usize {
            Some(IntegerWidth::ThirtyTwo)
        } else {
            Some(IntegerWidth::SixtyFour)
        };
    }

    let hex = match bitwidth {
        Some(IntegerWidth::Unknown) => unreachable!(),
        Some(IntegerWidth::Zero) => {
            format!("{:02x}", (length.unwrap() as u8) + (major << 5))
        }
        Some(IntegerWidth::Eight) => {
            format!("{:02x} {:02x}", (major << 5) | 0x18, length.unwrap())
        }
        Some(IntegerWidth::Sixteen) => {
            format!("{:02x} {:04x}", (major << 5) | 0x19, length.unwrap())
        }
        Some(IntegerWidth::ThirtyTwo) => {
            format!("{:02x} {:08x}", (major << 5) | 0x1a, length.unwrap())
        }
        Some(IntegerWidth::SixtyFour) => {
            format!("{:02x} {:016x}", (major << 5) | 0x1b, length.unwrap())
        }
        None => format!("{:02x}", (major << 5) | 0x1F),
    };

    let comment = format!(
        "{kind}({length})",
        kind = kind,
        length = if bitwidth.is_some() {
            length.unwrap().to_string()
        } else {
            "*".to_owned()
        },
    );

    Line::new(hex, comment)
}

fn definite_bytestring_to_hex(bytestring: &ByteString) -> Line {
    let ByteString { ref data, bitwidth } = *bytestring;

    let mut line = length_to_hex(Some(data.len()), Some(bitwidth), 2, "bytes");

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

fn definite_textstring_to_hex(textstring: &TextString) -> Line {
    let TextString { ref data, bitwidth } = *textstring;

    let mut line = length_to_hex(Some(data.len()), Some(bitwidth), 3, "text");

    if data.is_empty() {
        line.sublines.push(Line::new("", "\"\""));
    } else {
        let mut push_line = |datum: &str| {
            let hex = hex::encode(datum);
            let mut comment = String::with_capacity(datum.len());
            comment.push('"');
            for c in datum.chars() {
                if c == '\"' || c == '\\' {
                    for c in c.escape_default() {
                        comment.push(c);
                    }
                } else {
                    comment.push(c);
                }
            }
            comment.push('"');
            line.sublines.push(Line::new(hex, comment));
        };

        if data.len() <= 24 {
            push_line(&data);
        } else {
            let mut data = data.as_str();
            while !data.is_empty() {
                let mut split = 16;
                while !data.is_char_boundary(split) {
                    split -= 1;
                }
                let (datum, new_data) = data.split_at(split);
                data = new_data;
                push_line(datum);
            }
        }
    }

    line
}

fn indefinite_string_to_hex<T>(
    major: u8,
    name: &str,
    strings: &[T],
    definite_string_to_hex: fn(&T) -> Line,
) -> Line {
    let mut line = length_to_hex(None, None, major, name);

    line.sublines
        .extend(strings.iter().map(definite_string_to_hex));
    line.sublines.push(Line::new("ff", "break"));

    line
}

fn array_to_hex(array: &[Value], bitwidth: Option<IntegerWidth>) -> Line {
    let mut line = length_to_hex(Some(array.len()), bitwidth, 4, "array");

    line.sublines.extend(array.iter().map(Line::from_value));

    if bitwidth.is_none() {
        line.sublines.push(Line::new("ff", "break"));
    }

    line
}

fn map_to_hex(
    values: &[(Value, Value)],
    bitwidth: Option<IntegerWidth>,
) -> Line {
    let mut line = length_to_hex(Some(values.len()), bitwidth, 5, "map");

    line.sublines.extend(
        values
            .iter()
            .flat_map(|(v1, v2)| iter::once(v1).chain(iter::once(v2)))
            .map(Line::from_value),
    );

    if bitwidth.is_none() {
        line.sublines.push(Line::new("ff", "break"));
    }

    line
}

fn tagged_to_hex(tag: Tag, mut bitwidth: IntegerWidth, value: &Value) -> Line {
    if bitwidth == IntegerWidth::Unknown {
        bitwidth = if tag.0 < 24 {
            IntegerWidth::Zero
        } else if tag.0 < u64::from(u8::max_value()) {
            IntegerWidth::Eight
        } else if tag.0 < u64::from(u16::max_value()) {
            IntegerWidth::Sixteen
        } else if tag.0 < u64::from(u32::max_value()) {
            IntegerWidth::ThirtyTwo
        } else {
            IntegerWidth::SixtyFour
        };
    }

    let hex = match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => format!("{:02x}", 0xc0 | tag.0),
        IntegerWidth::Eight => format!("d8 {:02x}", tag.0),
        IntegerWidth::Sixteen => format!("d9 {:04x}", tag.0),
        IntegerWidth::ThirtyTwo => format!("da {:08x}", tag.0),
        IntegerWidth::SixtyFour => format!("db {:016x}", tag.0),
    };

    let (extra, extra_line) = match tag {
        Tag::DATETIME => {
            ("standard datetime string, ", Some(datetime_epoch(value)))
        }
        Tag::EPOCH_DATETIME => {
            ("epoch datetime value, ", Some(epoch_datetime(value)))
        }
        Tag::POSITIVE_BIGNUM => {
            ("positive bignum, ", Some(positive_bignum(value)))
        }
        Tag::NEGATIVE_BIGNUM => {
            ("negative bignum, ", Some(negative_bignum(value)))
        }
        Tag::DECIMAL_FRACTION => {
            ("decimal fraction, ", Some(decimal_fraction(value)))
        }
        Tag::BIGFLOAT => ("bigfloat, ", Some(bigfloat(value))),
        Tag::ENCODED_BASE64URL => ("suggested base64url encoding, ", None),
        Tag::ENCODED_BASE64 => ("suggested base64 encoding, ", None),
        Tag::ENCODED_BASE16 => ("suggested base16 encoding, ", None),
        Tag::ENCODED_CBOR => ("encoded cbor data item, ", None),
        Tag::URI => ("uri, ", None),
        Tag::BASE64URL => ("base64url encoded text, ", None),
        Tag::BASE64 => ("base64 encoded text, ", None),
        Tag::REGEX => ("regex, ", None),
        Tag::MIME => ("mime message, ", None),
        Tag::SELF_DESCRIBE_CBOR => ("positive bignum, ", None),
        _ => unimplemented!(),
    };

    let comment = format!("{}tag({})", extra, tag.0);

    Line {
        hex,
        comment,
        sublines: iter::once(Line::from_value(value))
            .chain(extra_line)
            .collect(),
    }
}

fn datetime_epoch(value: &Value) -> Line {
    let date = if let Value::TextString(TextString { data, .. }) = value {
        match DateTime::parse_from_rfc3339(data) {
            Ok(value) => value,
            Err(err) => {
                return Line::new("", format!("error parsing datetime: {}", err))
            }
        }
    } else {
        return Line::new("", "invalid type for datetime");
    };

    Line::new("", format!("epoch({})", date.format("%s%.f")))
}

fn epoch_datetime(value: &Value) -> Line {
    let date = match *value {
        Value::Integer { value, .. } => {
            if value >= (i64::max_value() as u64) {
                return Line::new("", "offset is too large");
            }
            NaiveDateTime::from_timestamp(value as i64, 0)
        }
        Value::Negative { value, .. } => {
            if value >= (i64::max_value() as u64) {
                return Line::new("", "offset is too large");
            }
            if let Some(value) = (-1i64).checked_sub(value as i64) {
                NaiveDateTime::from_timestamp(value, 0)
            } else {
                return Line::new("", "offset is too large");
            }
        }
        Value::Float { value, .. } => NaiveDateTime::from_timestamp(
            value.abs() as i64,
            (value.fract() * 1_000_000_000.0) as u32,
        ),

        Value::ByteString(..)
        | Value::IndefiniteByteString(..)
        | Value::TextString(..)
        | Value::IndefiniteTextString(..)
        | Value::Array { .. }
        | Value::Map { .. }
        | Value::Tag { .. }
        | Value::Simple(..) => {
            return Line::new("", "invalid type for epoch datetime");
        }
    };

    Line::new("", format!("datetime({})", date.format("%FT%T%.fZ")))
}

fn extract_positive_bignum(value: &Value) -> Option<BigUint> {
    if let Value::ByteString(ByteString { data, .. }) = value {
        Some(BigUint::from_bytes_be(data))
    } else {
        None
    }
}

fn positive_bignum(value: &Value) -> Line {
    extract_positive_bignum(value)
        .map(|num| Line::new("", format!("bignum({})", num)))
        .unwrap_or_else(|| Line::new("", "invalid type for bignum"))
}

fn extract_negative_bignum(value: &Value) -> Option<BigInt> {
    if let Value::ByteString(ByteString { data, .. }) = value {
        Some(BigInt::from(-1) - BigInt::from_bytes_be(Sign::Plus, data))
    } else {
        None
    }
}

fn negative_bignum(value: &Value) -> Line {
    extract_negative_bignum(value)
        .map(|num| Line::new("", format!("bignum({})", num)))
        .unwrap_or_else(|| Line::new("", "invalid type for bignum"))
}

fn extract_fraction(
    value: &Value,
    base: usize,
) -> Result<BigRational, &'static str> {
    Ok(match value {
        Value::Array { data, .. } => {
            if data.len() != 2 {
                return Err("invalid type");
            }
            let (exponent, positive_exponent) = match data[0] {
                Value::Integer { value, .. } => {
                    if value <= usize::max_value() as u64 {
                        (value as usize, true)
                    } else {
                        return Err("exponent is too large");
                    }
                }
                Value::Negative { value, .. } => {
                    if value < usize::max_value() as u64 {
                        (value as usize + 1, false)
                    } else {
                        return Err("exponent is too large");
                    }
                }
                _ => return Err("invalid type"),
            };
            let mantissa = match data[1] {
                Value::Integer { value, .. } => BigInt::from(value),
                Value::Negative { value, .. } => {
                    BigInt::from(-1) - BigInt::from(value)
                }
                Value::Tag {
                    tag: Tag::POSITIVE_BIGNUM,
                    ref value,
                    ..
                } => match extract_positive_bignum(&*value) {
                    Some(value) => BigInt::from_biguint(Sign::Plus, value),
                    _ => return Err("invalid type"),
                },
                Value::Tag {
                    tag: Tag::NEGATIVE_BIGNUM,
                    ref value,
                    ..
                } => match extract_negative_bignum(&*value) {
                    Some(value) => value,
                    _ => return Err("invalid type"),
                },
                _ => return Err("invalid type"),
            };
            let multiplier = if positive_exponent {
                Ratio::from_integer(pow(BigInt::from(base), exponent))
            } else {
                Ratio::new(BigInt::from(1), pow(BigInt::from(base), exponent))
            };
            Ratio::from_integer(mantissa) * multiplier
        }
        _ => return Err("invalid type"),
    })
}

fn decimal_fraction(value: &Value) -> Line {
    // TODO: https://github.com/rust-num/num-rational/issues/10
    extract_fraction(value, 10)
        .map(|fraction| {
            Line::new("", format!("decimal fraction({})", fraction))
        }).unwrap_or_else(|err| {
            Line::new("", format!("{} for decimal fraction", err))
        })
}

fn bigfloat(value: &Value) -> Line {
    // TODO: https://github.com/rust-num/num-rational/issues/10
    extract_fraction(value, 2)
        .map(|fraction| Line::new("", format!("bigfloat({})", fraction)))
        .unwrap_or_else(|err| Line::new("", format!("{} for bigfloat", err)))
}

fn float_to_hex(value: f64, mut bitwidth: FloatWidth) -> Line {
    if bitwidth == FloatWidth::Unknown {
        bitwidth = FloatWidth::SixtyFour;
    }

    let hex = match bitwidth {
        FloatWidth::Unknown => unreachable!(),
        FloatWidth::Sixteen => {
            format!("f9 {:04x}", f16::from_f64(value).as_bits())
        }
        FloatWidth::ThirtyTwo => format!("fa {:08x}", (value as f32).to_bits()),
        FloatWidth::SixtyFour => format!("fb {:016x}", value.to_bits()),
    };

    let mut comment = "float(".to_owned();
    diag::Context::new(&mut comment).float_to_diag(value, FloatWidth::Unknown);
    comment.push(')');

    Line::new(hex, comment)
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
