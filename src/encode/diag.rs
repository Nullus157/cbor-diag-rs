use half::f16;
use hex;

use {ByteString, FloatWidth, IntegerWidth, Simple, Tag, TextString, Value};

fn integer_to_diag(value: u64, bitwidth: IntegerWidth, s: &mut String) {
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
}

fn negative_to_diag(value: u64, bitwidth: IntegerWidth, s: &mut String) {
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
}

fn definite_bytestring_to_diag(bytestring: &ByteString, s: &mut String) {
    s.push_str(&format!("h'{}'", hex::encode(&bytestring.data)));
}

fn definite_textstring_to_diag(textstring: &TextString, s: &mut String) {
    s.push('"');
    for c in textstring.data.chars() {
        if c == '\"' || c == '\\' {
            for c in c.escape_default() {
                s.push(c);
            }
        } else {
            s.push(c);
        }
    }
    s.push('"');
}

fn indefinite_string_to_diag<T>(
    strings: &[T],
    definite_string_to_diag: fn(&T, &mut String),
    s: &mut String,
) {
    s.push_str("(_");
    if strings.is_empty() {
        s.push(' ');
        s.push(' ');
    }
    for string in strings {
        s.push(' ');
        definite_string_to_diag(string, s);
        s.push(',');
    }
    s.pop();
    s.push(')');
}

fn array_to_diag(array: &[Value], s: &mut String, definite: bool) {
    s.push('[');
    if !definite {
        s.push('_');
        s.push(' ');
    }
    if array.is_empty() {
        s.push(' ');
        s.push(' ');
    }
    for value in array {
        value_to_diag(value, s);
        s.push(',');
        s.push(' ');
    }
    s.pop();
    s.pop();
    s.push(']');
}

fn map_to_diag(values: &[(Value, Value)], s: &mut String, definite: bool) {
    s.push('{');
    if !definite {
        s.push('_');
        if values.is_empty() {
            s.push(' ');
        }
    }
    for (key, value) in values {
        s.push(' ');
        value_to_diag(key, s);
        s.push(':');
        s.push(' ');
        value_to_diag(value, s);
        s.push(',');
    }
    if !values.is_empty() {
        s.pop();
        s.push(' ');
    }
    s.push('}');
}

pub fn tagged_to_diag(
    tag: Tag,
    bitwidth: IntegerWidth,
    value: &Value,
    s: &mut String,
) {
    if bitwidth == IntegerWidth::Unknown || bitwidth == IntegerWidth::Zero {
        s.push_str(&tag.0.to_string());
    } else {
        let encoding = match bitwidth {
            IntegerWidth::Eight => 0,
            IntegerWidth::Sixteen => 1,
            IntegerWidth::ThirtyTwo => 2,
            IntegerWidth::SixtyFour => 3,
            _ => unreachable!(),
        };
        s.push_str(&format!("{}_{}", tag.0, encoding));
    }
    s.push('(');
    value_to_diag(value, s);
    s.push(')');
}

pub fn float_to_diag(value: f64, bitwidth: FloatWidth, s: &mut String) {
    if value.is_nan() {
        s.push_str("NaN");
    } else if value.is_infinite() {
        if value.is_sign_negative() {
            s.push('-');
        }
        s.push_str("Infinity");
    } else {
        let value = match bitwidth {
            FloatWidth::Unknown | FloatWidth::SixtyFour => value.to_string(),
            FloatWidth::Sixteen => f16::from_f64(value).to_string(),
            FloatWidth::ThirtyTwo => (value as f32).to_string(),
        };
        s.push_str(&value);
        if !value.contains('.') && !value.contains('e') {
            s.push_str(".0");
        }
    }
    s.push_str(match bitwidth {
        FloatWidth::Unknown => "",
        FloatWidth::Sixteen => "_1",
        FloatWidth::ThirtyTwo => "_2",
        FloatWidth::SixtyFour => "_3",
    });
}

fn simple_to_diag(simple: Simple, s: &mut String) {
    match simple {
        Simple::FALSE => s.push_str("false"),
        Simple::TRUE => s.push_str("true"),
        Simple::NULL => s.push_str("null"),
        Simple::UNDEFINED => s.push_str("undefined"),
        Simple(value) => s.push_str(&format!("simple({})", value)),
    }
}

fn value_to_diag(value: &Value, s: &mut String) {
    match *value {
        Value::Integer { value, bitwidth } => {
            integer_to_diag(value, bitwidth, s);
        }
        Value::Negative { value, bitwidth } => {
            negative_to_diag(value, bitwidth, s);
        }
        Value::ByteString(ref bytestring) => {
            definite_bytestring_to_diag(bytestring, s);
        }
        Value::IndefiniteByteString(ref bytestrings) => {
            indefinite_string_to_diag(
                bytestrings,
                definite_bytestring_to_diag,
                s,
            );
        }
        Value::TextString(ref textstring) => {
            definite_textstring_to_diag(textstring, s);
        }
        Value::IndefiniteTextString(ref textstrings) => {
            indefinite_string_to_diag(
                textstrings,
                definite_textstring_to_diag,
                s,
            );
        }
        Value::Array {
            ref data,
            ref bitwidth,
        } => {
            array_to_diag(data, s, bitwidth.is_some());
        }
        Value::Map {
            ref data,
            ref bitwidth,
        } => {
            map_to_diag(data, s, bitwidth.is_some());
        }
        Value::Tag {
            tag,
            bitwidth,
            ref value,
        } => {
            tagged_to_diag(tag, bitwidth, &*value, s);
        }
        Value::Float { value, bitwidth } => {
            float_to_diag(value, bitwidth, s);
        }
        Value::Simple(simple) => {
            simple_to_diag(simple, s);
        }
    }
}

impl Value {
    pub fn to_diag(&self) -> String {
        let mut s = String::with_capacity(128);
        value_to_diag(self, &mut s);
        s
    }
}
