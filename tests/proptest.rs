#[macro_use]
extern crate proptest;
extern crate cbor_diag;
extern crate hex;

#[macro_use]
extern crate pretty_assertions;

use cbor_diag::{
    parse_bytes, parse_diag, parse_hex, ByteString, DataItem, IntegerWidth,
    TextString,
};
use proptest::{
    arbitrary::any,
    collection::{self, SizeRange},
    option,
    strategy::{Just, Strategy},
};
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

fn arb_unsigned() -> impl Strategy<Value = (u64, IntegerWidth)> {
    arb_integer_width().prop_flat_map(|bitwidth| {
        (0..=bitwidth_max(bitwidth)).prop_map(move |value| (value, bitwidth))
    })
}

fn arb_integer() -> impl Strategy<Value = DataItem> {
    arb_unsigned()
        .prop_map(|(value, bitwidth)| DataItem::Integer { value, bitwidth })
}

fn arb_negative() -> impl Strategy<Value = DataItem> {
    arb_unsigned()
        .prop_map(|(value, bitwidth)| DataItem::Negative { value, bitwidth })
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
    collection::vec(arb_bytestring(), 0..10)
        .prop_map(DataItem::IndefiniteByteString)
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
    collection::vec(arb_textstring(), 0..10)
        .prop_map(DataItem::IndefiniteTextString)
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

fn arb_data_item_leaf() -> impl Strategy<Value = DataItem> {
    prop_oneof![
        arb_integer(),
        arb_negative(),
        arb_definite_bytestring(),
        arb_indefinite_bytestring(),
        arb_definite_textstring(),
        arb_indefinite_textstring()
    ]
}

fn arb_data_item() -> impl Strategy<Value = DataItem> {
    arb_data_item_leaf().prop_recursive(8, 256, 10, |inner| {
        prop_oneof![arb_array(inner.clone(), 0..10),]
    })
}

proptest! {
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
}

#[test]
fn multiply_overflow() {
    let _ = parse_bytes(hex::decode("7b2000000000000000").unwrap());
    let _ = parse_bytes(hex::decode("5b2000000000000000").unwrap());
}

#[test]
fn newline_in_string() {
    let item = DataItem::TextString(TextString {
        data: "\n".into(),
        bitwidth: IntegerWidth::Zero,
    });
    assert_eq!(item, parse_hex(item.to_hex()).unwrap());
}
