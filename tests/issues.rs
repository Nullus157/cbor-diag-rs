use cbor_diag::{ByteString, DataItem, IntegerWidth, Tag};
use data_encoding_macro::hexlower as hex;

#[macro_use]
mod utils;

testcases! {
    issue_113_length_estimate_underflow(diag2value, value2diag) {
        DataItem::Map {
            data: vec![
                (
                    DataItem::Integer {
                        value: 0,
                        bitwidth: IntegerWidth::Zero,
                    },
                    DataItem::ByteString(
                        ByteString {
                            data: hex!("0128bf0000002c").into(),
                            bitwidth: IntegerWidth::Unknown,
                        },
                    ),
                ),
                (
                    DataItem::Integer {
                        value: 1,
                        bitwidth: IntegerWidth::Zero,
                    },
                    DataItem::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::Integer {
                            value: 1654099789,
                            bitwidth: IntegerWidth::ThirtyTwo,
                        }),
                    },
                ),
                (
                    DataItem::Integer {
                        value: 2,
                        bitwidth: IntegerWidth::Zero,
                    },
                    DataItem::Map {
                        data: vec![
                            (
                                DataItem::Integer {
                                    value: 0,
                                    bitwidth: IntegerWidth::Zero,
                                },
                                DataItem::Array {
                                    data: vec![
                                        DataItem::Integer {
                                            value: 4,
                                            bitwidth: IntegerWidth::Zero,
                                        },
                                        DataItem::Integer {
                                            value: 0,
                                            bitwidth: IntegerWidth::Zero,
                                        },
                                    ],
                                    bitwidth: Some(IntegerWidth::Unknown),
                                },
                            ),
                        ],
                        bitwidth: Some(IntegerWidth::Unknown),
                    },
                ),
            ],
            bitwidth: Some(IntegerWidth::Unknown),
        },
        {
            "{0:h'0128bf0000002c',1:1(1654099789_2),2:{0:[4,0]}}",
            "
                {
                    0: h'0128bf0000002c',
                    1: 1(1654099789_2),
                    2: {0: [4, 0]},
                }
            ",
        }
    }
}
