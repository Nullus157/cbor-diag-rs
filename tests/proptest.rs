#[macro_use]
extern crate proptest;
extern crate cbor_diag;
extern crate hex;

use cbor_diag::{parse_bytes, parse_diag, parse_hex, DataItem, IntegerWidth};

use proptest::{
    arbitrary::any,
    collection::{self, SizeRange},
    option,
    strategy::{Just, Strategy},
};

fn arb_integer_width() -> impl Strategy<Value = IntegerWidth> {
    prop_oneof![
        Just(IntegerWidth::Eight),
        Just(IntegerWidth::Sixteen),
        Just(IntegerWidth::ThirtyTwo),
        Just(IntegerWidth::SixtyFour),
    ]
}

fn arb_integer() -> impl Strategy<Value = DataItem> {
    arb_integer_width().prop_flat_map(|bitwidth| {
        match bitwidth {
            IntegerWidth::SixtyFour => (0..u64::max_value()),
            IntegerWidth::ThirtyTwo => (0..0xffffffff),
            IntegerWidth::Sixteen => (0..0xffff),
            IntegerWidth::Eight => (0..0xff),
            IntegerWidth::Zero => (0..24),
            IntegerWidth::Unknown => unreachable!(),
        }
        .prop_map(move |value| DataItem::Integer { value, bitwidth })
    })
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
    prop_oneof![arb_integer(),]
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
