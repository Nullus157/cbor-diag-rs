use cbor_diag::{ByteString, DataItem, IntegerWidth, TextString};
use data_encoding_macro::hexlower as hex;

#[macro_use]
mod utils;

// RFC 8610 Appendix G.6
testcases! {
    rfc_example(diag2value) {
        DataItem::Array {
            data: vec![
                DataItem::Integer {
                    value: 1,
                    bitwidth: IntegerWidth::Zero,
                },
                DataItem::Integer {
                    value: 10584416,
                    bitwidth: IntegerWidth::Unknown,
                },
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "opsonize".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::Integer {
                            value: 7,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Integer {
                            value: 105,
                            bitwidth: IntegerWidth::Unknown,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Unknown),
                },
            ],
            bitwidth: Some(IntegerWidth::Unknown),
        },
        {
            r#"
            /grasp-message/ [
                /M_DISCOVERY/ 1,
                /session-id/ 10584416,
                /objective/ [
                    /objective-name/ "opsonize",
                    /D, N, S/ 7,
                    /loop-count/ 105
                ]
            ]
            "#
        }
    }

    rfc_bytestring(diag2value) {
        DataItem::ByteString(ByteString {
            data: hex!("68656c6c6f20776f726c64").into(),
            bitwidth: IntegerWidth::Unknown,
        }),
        {
            "
                h'68 65 6c /doubled l!/ 6c 6f /hello/
                  20 /space/
                  77 6f 72 6c 64' /world/
            "
        }
    }

    not_unprefixed_bytestrings(diag2value) {
        DataItem::ByteString(ByteString {
            data: b"hello /world/"[..].into(),
            bitwidth: IntegerWidth::Unknown,
        }),
        {
            "
                'hello /world/'
            "
        }
    }
}
