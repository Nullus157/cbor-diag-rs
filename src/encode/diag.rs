use base64::{self, display::Base64Display};
use half::f16;
use hex;

use super::Encoding;
use {ByteString, FloatWidth, IntegerWidth, Simple, Tag, TextString, Value};

pub(crate) struct Context<'a> {
    encoding: Encoding,
    output: &'a mut String,
}

impl<'a> Context<'a> {
    pub(crate) fn new(output: &'a mut String) -> Self {
        Self {
            encoding: Encoding::Base16,
            output,
        }
    }

    pub(crate) fn with_encoding<'b>(
        &'b mut self,
        encoding: Encoding,
    ) -> Context<'b> {
        Context {
            encoding,
            output: self.output,
        }
    }

    fn integer_to_diag(&mut self, value: u64, bitwidth: IntegerWidth) {
        if bitwidth == IntegerWidth::Unknown || bitwidth == IntegerWidth::Zero {
            self.output.push_str(&value.to_string());
        } else {
            let encoding = match bitwidth {
                IntegerWidth::Eight => 0,
                IntegerWidth::Sixteen => 1,
                IntegerWidth::ThirtyTwo => 2,
                IntegerWidth::SixtyFour => 3,
                _ => unreachable!(),
            };
            self.output.push_str(&format!("{}_{}", value, encoding));
        }
    }

    fn negative_to_diag(&mut self, value: u64, bitwidth: IntegerWidth) {
        let value = -1i128 - i128::from(value);
        if bitwidth == IntegerWidth::Unknown || bitwidth == IntegerWidth::Zero {
            self.output.push_str(&value.to_string());
        } else {
            let encoding = match bitwidth {
                IntegerWidth::Eight => 0,
                IntegerWidth::Sixteen => 1,
                IntegerWidth::ThirtyTwo => 2,
                IntegerWidth::SixtyFour => 3,
                _ => unreachable!(),
            };
            self.output.push_str(&format!("{}_{}", value, encoding));
        }
    }

    fn definite_bytestring_to_diag(&mut self, bytestring: &ByteString) {
        match self.encoding {
            Encoding::Base64Url => {
                self.output.push_str(&format!(
                    "b64'{}'",
                    Base64Display::with_config(
                        &bytestring.data,
                        base64::URL_SAFE_NO_PAD
                    ).unwrap()
                ));
            }
            Encoding::Base64 => {
                self.output.push_str(&format!(
                    "b64'{}'",
                    Base64Display::with_config(
                        &bytestring.data,
                        base64::STANDARD_NO_PAD
                    ).unwrap()
                ));
            }
            Encoding::Base16 => {
                self.output
                    .push_str(&format!("h'{}'", hex::encode(&bytestring.data)));
            }
        }
    }

    fn definite_textstring_to_diag(&mut self, textstring: &TextString) {
        self.output.push('"');
        for c in textstring.data.chars() {
            if c == '\"' || c == '\\' {
                for c in c.escape_default() {
                    self.output.push(c);
                }
            } else {
                self.output.push(c);
            }
        }
        self.output.push('"');
    }

    fn indefinite_string_to_diag<T>(
        &mut self,
        strings: &[T],
        definite_string_to_diag: fn(&mut Self, &T),
    ) {
        self.output.push_str("(_");
        if strings.is_empty() {
            self.output.push(' ');
            self.output.push(' ');
        }
        for string in strings {
            self.output.push(' ');
            definite_string_to_diag(self, string);
            self.output.push(',');
        }
        self.output.pop();
        self.output.push(')');
    }

    fn array_to_diag(&mut self, array: &[Value], definite: bool) {
        self.output.push('[');
        if !definite {
            self.output.push('_');
            self.output.push(' ');
        }
        if array.is_empty() {
            self.output.push(' ');
            self.output.push(' ');
        }
        for value in array {
            self.value_to_diag(value);
            self.output.push(',');
            self.output.push(' ');
        }
        self.output.pop();
        self.output.pop();
        self.output.push(']');
    }

    fn map_to_diag(&mut self, values: &[(Value, Value)], definite: bool) {
        self.output.push('{');
        if !definite {
            self.output.push('_');
            if values.is_empty() {
                self.output.push(' ');
            }
        }
        for (key, value) in values {
            self.output.push(' ');
            self.value_to_diag(key);
            self.output.push(':');
            self.output.push(' ');
            self.value_to_diag(value);
            self.output.push(',');
        }
        if !values.is_empty() {
            self.output.pop();
            self.output.push(' ');
        }
        self.output.push('}');
    }

    pub fn tagged_to_diag(
        &mut self,
        tag: Tag,
        bitwidth: IntegerWidth,
        value: &Value,
    ) {
        if bitwidth == IntegerWidth::Unknown || bitwidth == IntegerWidth::Zero {
            self.output.push_str(&tag.0.to_string());
        } else {
            let encoding = match bitwidth {
                IntegerWidth::Eight => 0,
                IntegerWidth::Sixteen => 1,
                IntegerWidth::ThirtyTwo => 2,
                IntegerWidth::SixtyFour => 3,
                _ => unreachable!(),
            };
            self.output.push_str(&format!("{}_{}", tag.0, encoding));
        }
        self.output.push('(');

        match tag {
            Tag::ENCODED_BASE64URL => {
                self.with_encoding(Encoding::Base64Url).value_to_diag(value);
            }
            Tag::ENCODED_BASE64 => {
                self.with_encoding(Encoding::Base64).value_to_diag(value);
            }
            Tag::ENCODED_BASE16 => {
                self.with_encoding(Encoding::Base16).value_to_diag(value);
            }
            _ => {
                self.value_to_diag(value);
            }
        }

        self.output.push(')');
    }

    fn float_to_diag(&mut self, value: f64, bitwidth: FloatWidth) {
        if value.is_nan() {
            self.output.push_str("NaN");
        } else if value.is_infinite() {
            if value.is_sign_negative() {
                self.output.push('-');
            }
            self.output.push_str("Infinity");
        } else {
            let value = match bitwidth {
                FloatWidth::Unknown | FloatWidth::SixtyFour => {
                    value.to_string()
                }
                FloatWidth::Sixteen => f16::from_f64(value).to_string(),
                FloatWidth::ThirtyTwo => (value as f32).to_string(),
            };
            self.output.push_str(&value);
            if !value.contains('.') && !value.contains('e') {
                self.output.push_str(".0");
            }
        }
        self.output.push_str(match bitwidth {
            FloatWidth::Unknown => "",
            FloatWidth::Sixteen => "_1",
            FloatWidth::ThirtyTwo => "_2",
            FloatWidth::SixtyFour => "_3",
        });
    }

    fn simple_to_diag(&mut self, simple: Simple) {
        match simple {
            Simple::FALSE => self.output.push_str("false"),
            Simple::TRUE => self.output.push_str("true"),
            Simple::NULL => self.output.push_str("null"),
            Simple::UNDEFINED => self.output.push_str("undefined"),
            Simple(value) => {
                self.output.push_str(&format!("simple({})", value))
            }
        }
    }

    fn value_to_diag(&mut self, value: &Value) {
        match *value {
            Value::Integer { value, bitwidth } => {
                self.integer_to_diag(value, bitwidth);
            }
            Value::Negative { value, bitwidth } => {
                self.negative_to_diag(value, bitwidth);
            }
            Value::ByteString(ref bytestring) => {
                self.definite_bytestring_to_diag(bytestring);
            }
            Value::IndefiniteByteString(ref bytestrings) => {
                self.indefinite_string_to_diag(
                    bytestrings,
                    Self::definite_bytestring_to_diag,
                );
            }
            Value::TextString(ref textstring) => {
                self.definite_textstring_to_diag(textstring);
            }
            Value::IndefiniteTextString(ref textstrings) => {
                self.indefinite_string_to_diag(
                    textstrings,
                    Self::definite_textstring_to_diag,
                );
            }
            Value::Array {
                ref data,
                ref bitwidth,
            } => {
                self.array_to_diag(data, bitwidth.is_some());
            }
            Value::Map {
                ref data,
                ref bitwidth,
            } => {
                self.map_to_diag(data, bitwidth.is_some());
            }
            Value::Tag {
                tag,
                bitwidth,
                ref value,
            } => {
                self.tagged_to_diag(tag, bitwidth, &*value);
            }
            Value::Float { value, bitwidth } => {
                self.float_to_diag(value, bitwidth);
            }
            Value::Simple(simple) => {
                self.simple_to_diag(simple);
            }
        }
    }
}

impl Value {
    pub fn to_diag(&self) -> String {
        let mut s = String::with_capacity(128);
        Context::new(&mut s).value_to_diag(self);
        s
    }
}
