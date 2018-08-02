#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{IntegerWidth, Tag, TextString, Value};

#[macro_use]
mod utils;

testcases! {
    mod diag {
        date_time(diag2value, value2diag) {
            Value::Tag {
                tag: Tag::DATETIME,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(Value::TextString(TextString {
                    data: "2018-08-02T15:54:37Z".into(),
                    bitwidth: IntegerWidth::Unknown,
                }))
            },
            r#"0("2018-08-02T15:54:37Z")"#,
        }
    }

    mod hex {
        date_time(hex2value, value2hex) {
            Value::Tag {
                tag: Tag::DATETIME,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(Value::TextString(TextString {
                    data: "2018-08-02T15:54:37Z".into(),
                    bitwidth: IntegerWidth::Zero,
                }))
            },
            indoc!(r#"
                c0                                     # standard date/time string, tag(0)
                   74                                  #   text(20)
                      323031382d30382d30325431353a3534 #     "2018-08-02T15:54"
                      3a33375a                         #     ":37Z"
            "#),
        }
    }
}
