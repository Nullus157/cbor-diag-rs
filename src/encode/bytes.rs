use half::f16;

use {ByteString, DataItem, FloatWidth, IntegerWidth, Simple, Tag, TextString};

fn item_to_bytes(bytes: &mut Vec<u8>, value: &DataItem) {
    match *value {
        DataItem::Integer { value, bitwidth } => positive_to_bytes(bytes, value, bitwidth),
        DataItem::Negative { value, bitwidth } => negative_to_bytes(bytes, value, bitwidth),
        DataItem::ByteString(ref bytestring) => definite_bytestring_to_bytes(bytes, bytestring),
        DataItem::IndefiniteByteString(ref bytestrings) => {
            indefinite_string_to_bytes(bytes, 0x02, bytestrings, definite_bytestring_to_bytes)
        }
        DataItem::TextString(ref textstring) => definite_textstring_to_bytes(bytes, textstring),
        DataItem::IndefiniteTextString(ref textstrings) => {
            indefinite_string_to_bytes(bytes, 0x03, textstrings, definite_textstring_to_bytes)
        }
        DataItem::Array { ref data, bitwidth } => array_to_bytes(bytes, data, bitwidth),
        DataItem::Map { ref data, bitwidth } => map_to_bytes(bytes, data, bitwidth),
        DataItem::Tag {
            tag,
            bitwidth,
            ref value,
        } => tagged_to_bytes(bytes, tag, bitwidth, &*value),
        DataItem::Float { value, bitwidth } => float_to_bytes(bytes, value, bitwidth),
        DataItem::Simple(simple) => simple_to_bytes(bytes, simple),
    }
}

fn integer_to_bytes(bytes: &mut Vec<u8>, value: u64, bitwidth: IntegerWidth, major: u8) {
    const U8_MAX: u64 = u8::max_value() as u64;
    const U16_MAX: u64 = u16::max_value() as u64;
    const U32_MAX: u64 = u32::max_value() as u64;
    const U64_MAX: u64 = u64::max_value();

    match bitwidth {
        IntegerWidth::Unknown => {
            integer_to_bytes(
                bytes,
                value,
                #[allow(clippy::match_overlapping_arm)]
                match value {
                    0..=23 => IntegerWidth::Zero,
                    0..=U8_MAX => IntegerWidth::Eight,
                    0..=U16_MAX => IntegerWidth::Sixteen,
                    0..=U32_MAX => IntegerWidth::ThirtyTwo,
                    0..=U64_MAX => IntegerWidth::SixtyFour,
                },
                major,
            );
        }
        IntegerWidth::Zero => {
            bytes.push(major << 5 | (value as u8));
        }
        IntegerWidth::Eight => {
            bytes.push(major << 5 | 0x18);
            bytes.extend_from_slice(&(value as u8).to_be_bytes());
        }
        IntegerWidth::Sixteen => {
            bytes.push(major << 5 | 0x19);
            bytes.extend_from_slice(&(value as u16).to_be_bytes());
        }
        IntegerWidth::ThirtyTwo => {
            bytes.push(major << 5 | 0x1a);
            bytes.extend_from_slice(&(value as u32).to_be_bytes());
        }
        IntegerWidth::SixtyFour => {
            bytes.push(major << 5 | 0x1b);
            bytes.extend_from_slice(&value.to_be_bytes());
        }
    }
}

fn positive_to_bytes(bytes: &mut Vec<u8>, value: u64, bitwidth: IntegerWidth) {
    integer_to_bytes(bytes, value, bitwidth, 0);
}

fn negative_to_bytes(bytes: &mut Vec<u8>, value: u64, bitwidth: IntegerWidth) {
    integer_to_bytes(bytes, value, bitwidth, 1);
}

fn definite_bytestring_to_bytes(bytes: &mut Vec<u8>, ByteString { data, bitwidth }: &ByteString) {
    integer_to_bytes(bytes, data.len() as u64, *bitwidth, 2);
    bytes.extend_from_slice(data);
}

fn definite_textstring_to_bytes(bytes: &mut Vec<u8>, TextString { data, bitwidth }: &TextString) {
    integer_to_bytes(bytes, data.len() as u64, *bitwidth, 3);
    bytes.extend_from_slice(data.as_bytes());
}

fn indefinite_string_to_bytes<T>(
    bytes: &mut Vec<u8>,
    major: u8,
    strings: &[T],
    definite_string_to_bytes: impl Fn(&mut Vec<u8>, &T),
) {
    bytes.push(major << 5 | 0x1f);
    strings
        .iter()
        .for_each(|string| definite_string_to_bytes(bytes, string));
    bytes.push(0xff);
}

fn array_to_bytes(bytes: &mut Vec<u8>, array: &[DataItem], bitwidth: Option<IntegerWidth>) {
    if let Some(bitwidth) = bitwidth {
        integer_to_bytes(bytes, array.len() as u64, bitwidth, 4);
    } else {
        bytes.push(4 << 5 | 0x1f);
    }

    array.iter().for_each(|item| item_to_bytes(bytes, item));

    if bitwidth.is_none() {
        bytes.push(0xff);
    }
}

fn map_to_bytes(
    bytes: &mut Vec<u8>,
    values: &[(DataItem, DataItem)],
    bitwidth: Option<IntegerWidth>,
) {
    if let Some(bitwidth) = bitwidth {
        integer_to_bytes(bytes, values.len() as u64, bitwidth, 5);
    } else {
        bytes.push(5 << 5 | 0x1f);
    }

    values.iter().for_each(|(item1, item2)| {
        item_to_bytes(bytes, item1);
        item_to_bytes(bytes, item2);
    });

    if bitwidth.is_none() {
        bytes.push(0xff);
    }
}

fn tagged_to_bytes(bytes: &mut Vec<u8>, tag: Tag, bitwidth: IntegerWidth, value: &DataItem) {
    integer_to_bytes(bytes, tag.0, bitwidth, 6);
    item_to_bytes(bytes, value);
}

fn float_to_bytes(bytes: &mut Vec<u8>, value: f64, mut bitwidth: FloatWidth) {
    if bitwidth == FloatWidth::Unknown {
        bitwidth = FloatWidth::SixtyFour;
    }

    match bitwidth {
        FloatWidth::Unknown => unreachable!(),
        FloatWidth::Sixteen => {
            bytes.push(0xf9);
            bytes.extend_from_slice(&f16::from_f64(value).to_bits().to_be_bytes());
        }
        FloatWidth::ThirtyTwo => {
            bytes.push(0xfa);
            bytes.extend_from_slice(&(value as f32).to_bits().to_be_bytes());
        }
        FloatWidth::SixtyFour => {
            bytes.push(0xfb);
            bytes.extend_from_slice(&value.to_bits().to_be_bytes());
        }
    }
}

fn simple_to_bytes(bytes: &mut Vec<u8>, Simple(value): Simple) {
    integer_to_bytes(bytes, value.into(), IntegerWidth::Unknown, 7);
}

impl DataItem {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(128);
        item_to_bytes(&mut bytes, self);
        bytes
    }
}
