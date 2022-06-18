use cbor_diag::{DataItem, IntegerWidth, TextString};

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
}
