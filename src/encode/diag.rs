use base64::{self, display::Base64Display};
use half::f16;
use hex;

use super::Encoding;
use {ByteString, DataItem, FloatWidth, IntegerWidth, Simple, Tag, TextString};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum Layout {
    Pretty,
    Compact,
}

pub(crate) struct Context<'a> {
    output: &'a mut String,
    layout: Layout,
    encoding: Encoding,
    indent: usize,
}

trait IsTrivial {
    fn is_trivial(&self) -> bool;
}

fn is_trivial(value: &impl IsTrivial) -> bool {
    value.is_trivial()
}

impl IsTrivial for DataItem {
    fn is_trivial(&self) -> bool {
        match self {
            DataItem::Integer { .. }
            | DataItem::Negative { .. }
            | DataItem::Float { .. }
            | DataItem::Simple(_) => true,
            DataItem::Map { .. } => false,
            DataItem::ByteString(value) => value.is_trivial(),
            DataItem::TextString(value) => value.is_trivial(),
            DataItem::Array { data, .. } => data.len() < 2 && data.iter().all(is_trivial),
            DataItem::IndefiniteByteString(strings) => {
                strings.len() < 2 && strings.iter().all(is_trivial)
            }
            DataItem::IndefiniteTextString(strings) => {
                strings.len() < 2 && strings.iter().all(is_trivial)
            }
            DataItem::Tag { value, .. } => value.is_trivial(),
        }
    }
}

impl IsTrivial for (DataItem, DataItem) {
    fn is_trivial(&self) -> bool {
        self.0.is_trivial() && self.1.is_trivial()
    }
}

impl IsTrivial for ByteString {
    fn is_trivial(&self) -> bool {
        self.data.len() < 16
    }
}

impl IsTrivial for TextString {
    fn is_trivial(&self) -> bool {
        self.data.len() < 32
    }
}

impl<'a> Context<'a> {
    pub(crate) fn new(output: &'a mut String, layout: Layout) -> Self {
        Self {
            output,
            layout,
            encoding: Encoding::Base16,
            indent: 0,
        }
    }

    pub(crate) fn with_encoding(&mut self, encoding: Encoding) -> Context<'_> {
        Context {
            output: self.output,
            layout: self.layout,
            encoding,
            indent: self.indent,
        }
    }

    fn pretty(&self) -> bool {
        self.layout == Layout::Pretty
    }

    fn indent(&mut self) {
        for _ in 0..self.indent {
            self.output.push(' ');
        }
    }

    fn line(&mut self) {
        self.output.push('\n');
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
                    Base64Display::with_config(&bytestring.data, base64::URL_SAFE_NO_PAD)
                ));
            }
            Encoding::Base64 => {
                self.output.push_str(&format!(
                    "b64'{}'",
                    Base64Display::with_config(&bytestring.data, base64::STANDARD_NO_PAD)
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

    fn container_to_diag<T: IsTrivial>(
        &mut self,
        begin: char,
        items: &[T],
        end: char,
        definite: bool,
        item_to_diag: fn(&mut Self, &T),
    ) {
        self.output.push(begin);
        if !definite {
            self.output.push_str("_");
        }
        if items.len() < 2 && items.iter().all(IsTrivial::is_trivial) {
            if self.pretty() {
                self.output.push(' ');
            }
            if let Some(item) = items.first() {
                item_to_diag(self, item);
                if self.pretty() {
                    self.output.push(' ');
                }
            }
        } else {
            self.indent += 4;
            for item in items {
                if self.pretty() {
                    self.line();
                    self.indent();
                }
                item_to_diag(self, item);
                self.output.push(',');
            }
            self.indent -= 4;
            if self.pretty() {
                self.line();
                self.indent();
            } else {
                self.output.pop();
            }
        }
        self.output.push(end);
    }

    fn indefinite_string_to_diag<T: IsTrivial>(
        &mut self,
        strings: &[T],
        definite_string_to_diag: fn(&mut Self, &T),
    ) {
        self.container_to_diag('(', strings, ')', false, definite_string_to_diag);
    }

    fn array_to_diag(&mut self, array: &[DataItem], definite: bool) {
        self.container_to_diag('[', array, ']', definite, Self::item_to_diag);
    }

    fn map_to_diag(&mut self, values: &[(DataItem, DataItem)], definite: bool) {
        self.container_to_diag('{', values, '}', definite, |this, (key, value)| {
            this.item_to_diag(key);
            this.output.push(':');
            if this.pretty() {
                this.output.push(' ');
            }
            this.item_to_diag(value);
        });
    }

    pub fn tagged_to_diag(&mut self, tag: Tag, bitwidth: IntegerWidth, value: &DataItem) {
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
                self.with_encoding(Encoding::Base64Url).item_to_diag(value);
            }
            Tag::ENCODED_BASE64 => {
                self.with_encoding(Encoding::Base64).item_to_diag(value);
            }
            Tag::ENCODED_BASE16 => {
                self.with_encoding(Encoding::Base16).item_to_diag(value);
            }
            _ => {
                self.item_to_diag(value);
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
                FloatWidth::Unknown | FloatWidth::SixtyFour => value.to_string(),
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
            Simple(value) => self.output.push_str(&format!("simple({})", value)),
        }
    }

    fn item_to_diag(&mut self, value: &DataItem) {
        match *value {
            DataItem::Integer { value, bitwidth } => {
                self.integer_to_diag(value, bitwidth);
            }
            DataItem::Negative { value, bitwidth } => {
                self.negative_to_diag(value, bitwidth);
            }
            DataItem::ByteString(ref bytestring) => {
                self.definite_bytestring_to_diag(bytestring);
            }
            DataItem::IndefiniteByteString(ref bytestrings) => {
                self.indefinite_string_to_diag(bytestrings, Self::definite_bytestring_to_diag);
            }
            DataItem::TextString(ref textstring) => {
                self.definite_textstring_to_diag(textstring);
            }
            DataItem::IndefiniteTextString(ref textstrings) => {
                self.indefinite_string_to_diag(textstrings, Self::definite_textstring_to_diag);
            }
            DataItem::Array {
                ref data,
                ref bitwidth,
            } => {
                self.array_to_diag(data, bitwidth.is_some());
            }
            DataItem::Map {
                ref data,
                ref bitwidth,
            } => {
                self.map_to_diag(data, bitwidth.is_some());
            }
            DataItem::Tag {
                tag,
                bitwidth,
                ref value,
            } => {
                self.tagged_to_diag(tag, bitwidth, &*value);
            }
            DataItem::Float { value, bitwidth } => {
                self.float_to_diag(value, bitwidth);
            }
            DataItem::Simple(simple) => {
                self.simple_to_diag(simple);
            }
        }
    }
}

impl DataItem {
    pub fn to_diag(&self) -> String {
        let mut s = String::with_capacity(128);
        Context::new(&mut s, Layout::Compact).item_to_diag(self);
        s
    }

    pub fn to_diag_pretty(&self) -> String {
        let mut s = String::with_capacity(128);
        Context::new(&mut s, Layout::Pretty).item_to_diag(self);
        s
    }
}
