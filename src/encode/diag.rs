use half::f16;

use std::fmt::Write;

use super::Encoding;
use crate::{ByteString, DataItem, FloatWidth, IntegerWidth, Simple, Tag, TextString};

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

trait LengthEstimate {
    /// Can shortcircuit and return `max` if it is more than that
    fn estimate(&self, max: usize) -> usize;
}

fn is_trivial(value: &impl LengthEstimate) -> bool {
    const MAX: usize = 60;
    value.estimate(MAX) < MAX
}

impl LengthEstimate for DataItem {
    fn estimate(&self, max: usize) -> usize {
        match self {
            DataItem::Integer { value, .. } => value.to_string().len() + 2,
            DataItem::Negative { value, .. } => value.to_string().len() + 3,
            DataItem::Float { value, .. } => value.to_string().len() + 3,
            DataItem::Simple(value) => value.estimate(max),
            DataItem::ByteString(value) => value.estimate(max),
            DataItem::TextString(value) => value.estimate(max),
            DataItem::Array { data, .. } => {
                let mut len = 4;
                for item in data {
                    len += item.estimate(max.saturating_sub(len)) + 2;
                    if len >= max {
                        return len;
                    }
                }
                len
            }
            DataItem::Map { data, .. } => {
                let mut len = 4;
                for entry in data {
                    len += entry.estimate(max.saturating_sub(len)) + 2;
                    if len >= max {
                        return len;
                    }
                }
                len
            }
            DataItem::IndefiniteByteString(strings) => {
                let mut len = 4;
                for string in strings {
                    len += string.estimate(max.saturating_sub(len)) + 2;
                    if len >= max {
                        return len;
                    }
                }
                len
            }
            DataItem::IndefiniteTextString(strings) => {
                let mut len = 4;
                for string in strings {
                    len += string.estimate(max.saturating_sub(len)) + 2;
                    if len >= max {
                        return len;
                    }
                }
                len
            }
            DataItem::Tag { tag, value, .. } => (tag, value).estimate(max),
        }
    }
}

impl<T: LengthEstimate + ?Sized> LengthEstimate for &T {
    fn estimate(&self, max: usize) -> usize {
        (&**self).estimate(max)
    }
}

impl<T: LengthEstimate + ?Sized> LengthEstimate for Box<T> {
    fn estimate(&self, max: usize) -> usize {
        (&**self).estimate(max)
    }
}

impl<T: LengthEstimate, U: LengthEstimate> LengthEstimate for (T, U) {
    fn estimate(&self, max: usize) -> usize {
        let mut len = self.0.estimate(max);
        if len < max {
            len += self.1.estimate(max - len);
        }
        len
    }
}

impl LengthEstimate for ByteString {
    fn estimate(&self, _: usize) -> usize {
        self.data.len() * 2 + 4
    }
}

impl LengthEstimate for TextString {
    fn estimate(&self, _: usize) -> usize {
        self.data.len() + 2
    }
}

impl LengthEstimate for Tag {
    fn estimate(&self, _: usize) -> usize {
        self.0.to_string().len() + 2
    }
}

impl LengthEstimate for Simple {
    fn estimate(&self, _: usize) -> usize {
        self.0.to_string().len() + 8
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
            write!(self.output, "{}_{}", value, encoding).unwrap();
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
            write!(self.output, "{}_{}", value, encoding).unwrap();
        }
    }

    fn definite_bytestring_to_diag(&mut self, bytestring: &ByteString) {
        match self.encoding {
            Encoding::Base64Url => {
                self.output.push_str("b64'");
                data_encoding::BASE64URL_NOPAD.encode_append(&bytestring.data, self.output);
                self.output.push('\'');
            }
            Encoding::Base64 => {
                self.output.push_str("b64'");
                data_encoding::BASE64.encode_append(&bytestring.data, self.output);
                self.output.push('\'');
            }
            Encoding::Base16 => {
                self.output.push_str("h'");
                data_encoding::HEXLOWER.encode_append(&bytestring.data, self.output);
                self.output.push('\'');
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

    fn container_to_diag<T>(
        &mut self,
        begin: char,
        items: impl IntoIterator<Item = T>,
        end: char,
        definite: bool,
        trivial: bool,
        item_to_diag: fn(&mut Self, T),
    ) {
        self.output.push(begin);
        if !definite {
            self.output.push('_');
            if trivial && self.pretty() {
                self.output.push(' ');
            }
        }
        if !trivial {
            self.indent += 4;
        }
        let mut items = items.into_iter();
        if let Some(item) = items.next() {
            if self.pretty() && !trivial {
                self.line();
                self.indent();
            }
            item_to_diag(self, item);
        }
        for item in items {
            self.output.push(',');
            if self.pretty() {
                if trivial {
                    self.output.push(' ');
                } else {
                    self.line();
                    self.indent();
                }
            }
            item_to_diag(self, item);
        }
        if !trivial {
            self.indent -= 4;
            if self.pretty() {
                self.output.push(',');
                self.line();
                self.indent();
            }
        }
        self.output.push(end);
    }

    fn indefinite_string_to_diag<T>(
        &mut self,
        strings: &[T],
        trivial: bool,
        definite_string_to_diag: fn(&mut Self, &T),
    ) {
        self.container_to_diag('(', strings, ')', false, trivial, definite_string_to_diag);
    }

    fn array_to_diag(&mut self, array: &[DataItem], definite: bool, trivial: bool) {
        self.container_to_diag('[', array, ']', definite, trivial, Self::item_to_diag);
    }

    fn map_to_diag(&mut self, values: &[(DataItem, DataItem)], definite: bool, trivial: bool) {
        self.container_to_diag('{', values, '}', definite, trivial, |this, (key, value)| {
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
            write!(self.output, "{}_{}", tag.0, encoding).unwrap();
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
            Simple(value) => write!(self.output, "simple({})", value).unwrap(),
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
                self.indefinite_string_to_diag(
                    bytestrings,
                    is_trivial(value),
                    Self::definite_bytestring_to_diag,
                );
            }
            DataItem::TextString(ref textstring) => {
                self.definite_textstring_to_diag(textstring);
            }
            DataItem::IndefiniteTextString(ref textstrings) => {
                self.indefinite_string_to_diag(
                    textstrings,
                    is_trivial(value),
                    Self::definite_textstring_to_diag,
                );
            }
            DataItem::Array {
                ref data,
                ref bitwidth,
            } => {
                self.array_to_diag(data, bitwidth.is_some(), is_trivial(value));
            }
            DataItem::Map {
                ref data,
                ref bitwidth,
            } => {
                self.map_to_diag(data, bitwidth.is_some(), is_trivial(value));
            }
            DataItem::Tag {
                tag,
                bitwidth,
                ref value,
            } => {
                self.tagged_to_diag(tag, bitwidth, value);
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
