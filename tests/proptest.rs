use ::proptest::{
    arbitrary::any,
    collection::{self, SizeRange},
    option, prop_oneof,
    sample::select,
    strategy::{Just, Strategy},
};
use cbor_diag::{
    parse_bytes, parse_diag, parse_hex, ByteString, DataItem, FloatWidth, IntegerWidth, Simple,
    Tag, TextString,
};
use data_encoding_macro::hexlower as hex;
use half::f16;
use pretty_assertions::assert_eq;
use std::cmp;

fn bitwidth_max(width: IntegerWidth) -> u64 {
    match width {
        IntegerWidth::SixtyFour => u64::max_value(),
        IntegerWidth::ThirtyTwo => u32::max_value().into(),
        IntegerWidth::Sixteen => u16::max_value().into(),
        IntegerWidth::Eight => u8::max_value().into(),
        IntegerWidth::Zero => 23,
        IntegerWidth::Unknown => unreachable!(),
    }
}

fn arb_integer_width() -> impl Strategy<Value = IntegerWidth> {
    prop_oneof![
        Just(IntegerWidth::Zero),
        Just(IntegerWidth::Eight),
        Just(IntegerWidth::Sixteen),
        Just(IntegerWidth::ThirtyTwo),
        Just(IntegerWidth::SixtyFour),
    ]
}

fn arb_float_width() -> impl Strategy<Value = FloatWidth> {
    prop_oneof![
        Just(FloatWidth::Sixteen),
        Just(FloatWidth::ThirtyTwo),
        Just(FloatWidth::SixtyFour),
    ]
}

fn arb_unsigned() -> impl Strategy<Value = (u64, IntegerWidth)> {
    arb_integer_width().prop_flat_map(|bitwidth| {
        (0..=bitwidth_max(bitwidth)).prop_map(move |value| (value, bitwidth))
    })
}

fn arb_integer() -> impl Strategy<Value = DataItem> {
    arb_unsigned().prop_map(|(value, bitwidth)| DataItem::Integer { value, bitwidth })
}

fn arb_negative() -> impl Strategy<Value = DataItem> {
    arb_unsigned().prop_map(|(value, bitwidth)| DataItem::Negative { value, bitwidth })
}

fn arb_bytestring() -> impl Strategy<Value = ByteString> {
    arb_integer_width().prop_flat_map(|bitwidth| {
        collection::vec(
            any::<u8>(),
            0..=cmp::min(bitwidth_max(bitwidth) as usize, 300),
        )
        .prop_map(move |data| ByteString { data, bitwidth })
    })
}

fn arb_definite_bytestring() -> impl Strategy<Value = DataItem> {
    arb_bytestring().prop_map(DataItem::ByteString)
}

fn arb_indefinite_bytestring() -> impl Strategy<Value = DataItem> {
    collection::vec(arb_bytestring(), 0..10).prop_map(DataItem::IndefiniteByteString)
}

fn arb_textstring() -> impl Strategy<Value = TextString> {
    arb_integer_width().prop_flat_map(|bitwidth| {
        ".{0,32}"
            .prop_filter("string too long", move |data| {
                data.len() as u64 <= bitwidth_max(bitwidth)
            })
            .prop_map(move |data| TextString { data, bitwidth })
    })
}

fn arb_definite_textstring() -> impl Strategy<Value = DataItem> {
    arb_textstring().prop_map(DataItem::TextString)
}

fn arb_indefinite_textstring() -> impl Strategy<Value = DataItem> {
    collection::vec(arb_textstring(), 0..10).prop_map(DataItem::IndefiniteTextString)
}

fn arb_array(
    data: impl Strategy<Value = DataItem>,
    count: impl Into<SizeRange>,
) -> impl Strategy<Value = DataItem> {
    (
        collection::vec(data, count),
        option::of(arb_integer_width()),
    )
        .prop_map(|(data, bitwidth)| DataItem::Array { data, bitwidth })
}

fn arb_map(
    data: impl Strategy<Value = DataItem> + Clone,
    count: impl Into<SizeRange>,
) -> impl Strategy<Value = DataItem> {
    (
        collection::vec((data.clone(), data), count),
        option::of(arb_integer_width()),
    )
        .prop_map(|(data, bitwidth)| DataItem::Map { data, bitwidth })
}

fn arb_tagged(value: impl Strategy<Value = DataItem> + Clone) -> impl Strategy<Value = DataItem> {
    arb_integer_width().prop_flat_map(move |bitwidth| {
        (
            (0..=bitwidth_max(bitwidth)).prop_map(Tag),
            value.clone().prop_map(Box::new),
        )
            .prop_map(move |(tag, value)| DataItem::Tag {
                tag,
                bitwidth,
                value,
            })
    })
}

fn arb_float() -> impl Strategy<Value = DataItem> {
    arb_float_width().prop_flat_map(|bitwidth| {
        match bitwidth {
            FloatWidth::SixtyFour => any::<f64>().boxed(),
            FloatWidth::ThirtyTwo => any::<f32>().prop_map_into().boxed(),
            FloatWidth::Sixteen => any::<f32>().prop_map(f16::from_f32).prop_map_into().boxed(),
            FloatWidth::Unknown => unreachable!(),
        }
        .prop_map(move |value| DataItem::Float { value, bitwidth })
    })
}

fn arb_simple() -> impl Strategy<Value = DataItem> {
    select((0..24).chain(32..=255).collect::<Vec<u8>>())
        .prop_map(Simple)
        .prop_map(DataItem::Simple)
}

fn arb_data_item_leaf() -> impl Strategy<Value = DataItem> {
    prop_oneof![
        arb_integer(),
        arb_negative(),
        arb_definite_bytestring(),
        arb_indefinite_bytestring(),
        arb_definite_textstring(),
        arb_indefinite_textstring(),
        arb_float(),
        arb_simple(),
    ]
}

fn arb_data_item() -> impl Strategy<Value = DataItem> {
    arb_data_item_leaf().prop_recursive(8, 256, 10, |inner| {
        prop_oneof![
            arb_array(inner.clone(), 0..10),
            arb_map(inner.clone(), 0..10),
            arb_tagged(inner),
        ]
    })
}

mod proptest {
    use super::arb_data_item;
    use cbor_diag::{parse_bytes, parse_diag, parse_hex};
    use proptest::arbitrary::any;

    proptest::proptest! {
        #[test]
        fn diag_doesnt_crash_with_anything(ref s in ".*") {
            let _ = parse_diag(s);
        }

        #[test]
        fn hex_doesnt_crash_with_anything(ref s in ".*") {
            let _ = parse_hex(s);
        }

        #[test]
        fn hex_doesnt_crash_with_hex(ref s in "(:?[0-9a-f]{2})*") {
            let _ = parse_hex(s);
        }

        #[test]
        fn bytes_doesnt_crash_with_anything(ref s in any::<Vec<u8>>()) {
            let _ = parse_bytes(s);
        }

        #[test]
        fn to_hex_and_back(item in arb_data_item()) {
            assert_eq!(item, parse_hex(item.to_hex()).unwrap());
        }

        #[test]
        fn to_diag(item in arb_data_item()) {
            let _ = item.to_diag();
        }
    }
}

#[test]
fn multiply_overflow() {
    let _ = parse_bytes(hex!("7b2000000000000000"));
    let _ = parse_bytes(hex!("5b2000000000000000"));
}

#[test]
fn newline_in_string() {
    let item = DataItem::TextString(TextString {
        data: "\n".into(),
        bitwidth: IntegerWidth::Zero,
    });
    assert_eq!(item, parse_hex(item.to_hex()).unwrap());
}

#[test]
fn datetime_invalid() {
    let item = DataItem::Tag {
        tag: Tag::EPOCH_DATETIME,
        bitwidth: IntegerWidth::Zero,
        value: Box::new(DataItem::Float {
            value: -0.000_024_616_718_292_236_328,
            bitwidth: FloatWidth::Sixteen,
        }),
    };
    assert_eq!(item, parse_hex(item.to_hex()).unwrap());
}

#[test]
fn datetime_overflow() {
    let item = DataItem::Tag {
        tag: Tag::EPOCH_DATETIME,
        bitwidth: IntegerWidth::Zero,
        value: Box::new(DataItem::Negative {
            value: 8_334_632_851_200,
            bitwidth: IntegerWidth::SixtyFour,
        }),
    };
    assert_eq!(item, parse_hex(item.to_hex()).unwrap());
}

#[test]
fn negative_underflow() {
    assert!(parse_diag("-0").is_err());
}
