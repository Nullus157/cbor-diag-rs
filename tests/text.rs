#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{DataItem, IntegerWidth, TextString};

#[macro_use]
mod utils;

// CBOR diagnostic notation provides for no way to encode the width of the
// length value of a string, so unfortunately roundtripping cannot be
// supported.
//
// Maybe I can just extend diagnostic notation for this?

testcases! {
    mod diag {
        empty(diag2value, value2diag) {
            DataItem::TextString(TextString {
                data: "".into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                r#""""#,
                r#""""#,
            }
        }

        hello(diag2value, value2diag) {
            DataItem::TextString(TextString {
                data: "hello".into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                r#""hello""#,
                r#""hello""#,
            }
        }

        alpha(diag2value, value2diag) {
            DataItem::TextString(TextString {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                r#""abcdefghijklmnopqrstuvwxyz""#,
                r#""abcdefghijklmnopqrstuvwxyz""#,
            }
        }

        non_alpha(diag2value, value2diag) {
            DataItem::TextString(TextString {
                data: "\u{1f1f3}\u{1f1ff}".into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                "\"\u{1f1f3}\u{1f1ff}\"",
                "\"\u{1f1f3}\u{1f1ff}\"",
            }
        }

        escaped(diag2value, value2diag) {
            DataItem::TextString(TextString {
                data: "\\\"".into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                r#""\\\"""#,
                r#""\\\"""#,
            }
        }
    }

    mod tiny {
        empty(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "".into(),
                bitwidth: IntegerWidth::Zero,
            }),
            indoc!(r#"
                60 # text(0)
                   #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "hello".into(),
                bitwidth: IntegerWidth::Zero,
            }),
            indoc!(r#"
                65            # text(5)
                   68656c6c6f #   "hello"
            "#)
        }

        escaped(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "\\\"".into(),
                bitwidth: IntegerWidth::Zero,
            }),
            indoc!(r#"
                62      # text(2)
                   5c22 #   "\\\""
            "#)
        }
    }

    mod u8 {
        empty(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "".into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!(r#"
                78 00 # text(0)
                      #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "hello".into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!(r#"
                78 05         # text(5)
                   68656c6c6f #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!(r#"
                78 1a                               # text(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }

        non_alpha(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "\u{1f1f3}\u{1f1ff}".into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!("
                78 08               # text(8)
                   f09f87b3f09f87bf #   \"\u{1f1f3}\u{1f1ff}\"
            ")
        }

        non_alpha_across_break(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "0123456789ab\u{1f1f3}\u{1f1ff}12345".into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!("
                78 19                               # text(25)
                   303132333435363738396162f09f87b3 #   \"0123456789ab\u{1f1f3}\"
                   f09f87bf3132333435               #   \"\u{1f1ff}12345\"
            ")
        }

        non_alpha_not_quite_at_break(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "0123456789abc\u{1f1f3}\u{1f1ff}1234".into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!("
                78 19                         # text(25)
                   30313233343536373839616263 #   \"0123456789abc\"
                   f09f87b3f09f87bf31323334   #   \"\u{1f1f3}\u{1f1ff}1234\"
            ")
        }
    }

    mod u16 {
        empty(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "".into(),
                bitwidth: IntegerWidth::Sixteen,
            }),
            indoc!(r#"
                79 0000 # text(0)
                        #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "hello".into(),
                bitwidth: IntegerWidth::Sixteen,
            }),
            indoc!(r#"
                79 0005       # text(5)
                   68656c6c6f #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: IntegerWidth::Sixteen,
            }),
            indoc!(r#"
                79 001a                             # text(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }
    }

    mod u32 {
        empty(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "".into(),
                bitwidth: IntegerWidth::ThirtyTwo,
            }),
            indoc!(r#"
                7a 00000000 # text(0)
                            #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "hello".into(),
                bitwidth: IntegerWidth::ThirtyTwo,
            }),
            indoc!(r#"
                7a 00000005   # text(5)
                   68656c6c6f #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: IntegerWidth::ThirtyTwo,
            }),
            indoc!(r#"
                7a 0000001a                         # text(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }
    }

    mod u64 {
        empty(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "".into(),
                bitwidth: IntegerWidth::SixtyFour,
            }),
            indoc!(r#"
                7b 0000000000000000 # text(0)
                                    #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "hello".into(),
                bitwidth: IntegerWidth::SixtyFour,
            }),
            indoc!(r#"
                7b 0000000000000005 # text(5)
                   68656c6c6f       #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::TextString(TextString {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: IntegerWidth::SixtyFour,
            }),
            indoc!(r#"
                7b 000000000000001a                 # text(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }
    }

    mod indefinite {
        mod diag {
            empty(value2diag) {
                DataItem::IndefiniteTextString(vec![]),
                {
                    "(_)",
                    "(_ )",
                }
            }

            one_empty(diag2value, value2diag) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    r#"(_"")"#,
                    r#"(_ "" )"#,
                }
            }

            some_empty(diag2value, value2diag) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    r#"(_"","")"#,
                    r#"(_ "", "" )"#,
                }
            }

            hello(diag2value, value2diag) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    r#"(_"hello")"#,
                    r#"(_ "hello" )"#,
                }
            }

            hello_world(diag2value, value2diag) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    TextString {
                        data: "world".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    r#"(_"hello","world")"#,
                    r#"(_ "hello", "world" )"#,
                }
            }

            alpha(diag2value, value2diag) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "abc".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    TextString {
                        data: "defghijklmnopqrstuv".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    TextString {
                        data: "wxyz".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    r#"(_"abc","","defghijklmnopqrstuv","wxyz")"#,
                    r#"(_ "abc", "", "defghijklmnopqrstuv", "wxyz" )"#,
                }
            }

            non_alpha(diag2value, value2diag) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    TextString {
                        data: "\u{1f1ff}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    "(_\"\u{1f1f3}\",\"\u{1f1ff}\")",
                    "(_ \"\u{1f1f3}\", \"\u{1f1ff}\" )",
                }
            }

            escaped(diag2value, value2diag) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "\\".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    TextString {
                        data: "\"".into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    r#"(_"\\","\"")"#,
                    r#"(_ "\\", "\"" )"#,
                }
            }
        }

        mod hex {
            empty(hex2value, value2hex) {
                DataItem::IndefiniteTextString(vec![]),
                indoc!(r#"
                    7f    # text(*)
                       ff #   break
                "#)
            }

            one_empty(hex2value, value2hex) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                ]),
                indoc!(r#"
                    7f    # text(*)
                       60 #   text(0)
                          #     ""
                       ff #   break
                "#)
            }

            some_empty(hex2value, value2hex) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                ]),
                indoc!(r#"
                    7f    # text(*)
                       60 #   text(0)
                          #     ""
                       60 #   text(0)
                          #     ""
                       ff #   break
                "#)
            }

            hello_world(hex2value, value2hex) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    TextString {
                        data: "world".into(),
                        bitwidth: IntegerWidth::Sixteen,
                    },
                ]),
                indoc!(r#"
                    7f               # text(*)
                       65            #   text(5)
                          68656c6c6f #     "hello"
                       79 0005       #   text(5)
                          776f726c64 #     "world"
                       ff            #   break
                "#)
            }

            alpha(hex2value, value2hex) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "abc".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    TextString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Sixteen,
                    },
                    TextString {
                        data: "defghijklmnopqrstuv".into(),
                        bitwidth: IntegerWidth::ThirtyTwo,
                    },
                    TextString {
                        data: "wxyz".into(),
                        bitwidth: IntegerWidth::SixtyFour,
                    },
                ]),
                indoc!(r#"
                    7f                                           # text(*)
                       63                                        #   text(3)
                          616263                                 #     "abc"
                       79 0000                                   #   text(0)
                                                                 #     ""
                       7a 00000013                               #   text(19)
                          6465666768696a6b6c6d6e6f70717273747576 #     "defghijklmnopqrstuv"
                       7b 0000000000000004                       #   text(4)
                          7778797a                               #     "wxyz"
                       ff                                        #   break
                "#)
            }

            non_alpha(hex2value, value2hex) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    TextString {
                        data: "\u{1f1ff}".into(),
                        bitwidth: IntegerWidth::Eight,
                    },
                ]),
                indoc!("
                    7f             # text(*)
                       64          #   text(4)
                          f09f87b3 #     \"\u{1f1f3}\"
                       78 04       #   text(4)
                          f09f87bf #     \"\u{1f1ff}\"
                       ff          #   break
                ")
            }

            escaped(hex2value, value2hex) {
                DataItem::IndefiniteTextString(vec![
                    TextString {
                        data: "\\".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    TextString {
                        data: "\"".into(),
                        bitwidth: IntegerWidth::Eight,
                    },
                ]),
                indoc!(r#"
                    7f       # text(*)
                       61    #   text(1)
                          5c #     "\\"
                       78 01 #   text(1)
                          22 #     "\""
                       ff    #   break
                "#)
            }
        }
    }
}
