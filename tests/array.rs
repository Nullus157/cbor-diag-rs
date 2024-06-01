use cbor_diag::{DataItem, IntegerWidth, TextString};
use indoc::indoc;

#[macro_use]
mod utils;

// CBOR diagnostic notation provides for no way to encode the width of the
// length value of an array, so unfortunately roundtripping cannot be
// supported.
//
// Maybe I can just extend diagnostic notation for this?

testcases! {
    mod diag {
        empty(diag2value, value2diag) {
            DataItem::Array {
                data: vec![],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                "[]",
                "[]",
            }
        }

        hello(diag2value, value2diag) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                r#"["hello"]"#,
                r#"["hello"]"#,
            }
        }

        hello_trailingcomma(diag2value) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                r#"["hello",]"#,
                r#"["hello",]"#,
            }
        }

        hello_world(diag2value, value2diag) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    DataItem::TextString(TextString {
                        data: "world".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                r#"["hello","world"]"#,
                r#"["hello", "world"]"#,
            }
        }

        non_alpha(diag2value, value2diag) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    DataItem::TextString(TextString {
                        data: "\u{1f1ff}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                "[\"\u{1f1f3}\",\"\u{1f1ff}\"]",
                "[\"\u{1f1f3}\", \"\u{1f1ff}\"]",
            }
        }

        heterogenous(diag2value, value2diag) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    DataItem::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::Zero,
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                "[\"\u{1f1f3}\",23]",
                "[\"\u{1f1f3}\", 23]",
            }
        }

        nested(diag2value, value2diag) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    DataItem::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::Zero,
                    },
                    DataItem::Array {
                        data: vec![
                            DataItem::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                            DataItem::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                        ],
                        bitwidth: Some(IntegerWidth::Unknown),
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                "[\"\u{1f1f3}\",23,[\"\u{1f1f3}\",23]]",
                "[\"\u{1f1f3}\", 23, [\"\u{1f1f3}\", 23]]",
            }
        }
    }

    mod hex {
        empty(hex2value, value2hex) {
            DataItem::Array {
                data: vec![],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                80 # array(0)
            "),
        }

        hello_world(hex2value, value2hex) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    DataItem::TextString(TextString {
                        data: "world".into(),
                        bitwidth: IntegerWidth::Sixteen,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                98 02            # array(2)
                   65            #   text(5)
                      68656c6c6f #     "hello"
                   79 0005       #   text(5)
                      776f726c64 #     "world"
            "#),
        }

        non_alpha(hex2value, value2hex) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    DataItem::TextString(TextString {
                        data: "\u{1f1ff}".into(),
                        bitwidth: IntegerWidth::Eight,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!("
                99 0002        # array(2)
                   64          #   text(4)
                      f09f87b3 #     \"\u{1f1f3}\"
                   78 04       #   text(4)
                      f09f87bf #     \"\u{1f1ff}\"
            "),
        }

        heterogenous(hex2value, value2hex) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    DataItem::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::SixtyFour,
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                82                     # array(2)
                   64                  #   text(4)
                      f09f87b3         #     \"\u{1f1f3}\"
                   1b 0000000000000017 #   unsigned(23)
            "),
        }

        nested(hex2value, value2hex) {
            DataItem::Array {
                data: vec![
                    DataItem::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    DataItem::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::SixtyFour,
                    },
                    DataItem::Array {
                        data: vec![
                            DataItem::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                            DataItem::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::SixtyFour,
                            },
                        ],
                        bitwidth: Some(IntegerWidth::ThirtyTwo),
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                83                        # array(3)
                   64                     #   text(4)
                      f09f87b3            #     \"\u{1f1f3}\"
                   1b 0000000000000017    #   unsigned(23)
                   9a 00000002            #   array(2)
                      64                  #     text(4)
                         f09f87b3         #       \"\u{1f1f3}\"
                      1b 0000000000000017 #     unsigned(23)
            "),
        }
    }

    mod indefinite {
        mod diag {
            empty(diag2value, value2diag) {
                DataItem::Array {
                    data: vec![],
                    bitwidth: None,
                },
                {
                    "[_]",
                    "[_ ]",
                }
            }

            hello(diag2value, value2diag) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ],
                    bitwidth: None,
                },
                {
                    r#"[_"hello"]"#,
                    r#"[_ "hello"]"#,
                }
            }

            hello_world(diag2value, value2diag) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::TextString(TextString {
                            data: "world".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ],
                    bitwidth: None,
                },
                {
                    r#"[_"hello","world"]"#,
                    r#"[_ "hello", "world"]"#,
                }
            }

            non_alpha(diag2value, value2diag) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::TextString(TextString {
                            data: "\u{1f1ff}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ],
                    bitwidth: None,
                },
                {
                    "[_\"\u{1f1f3}\",\"\u{1f1ff}\"]",
                    "[_ \"\u{1f1f3}\", \"\u{1f1ff}\"]",
                }
            }

            heterogenous(diag2value, value2diag) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                    ],
                    bitwidth: None,
                },
                {
                    "[_\"\u{1f1f3}\",23]",
                    "[_ \"\u{1f1f3}\", 23]",
                }
            }

            nested(diag2value, value2diag) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Array {
                            data: vec![
                                DataItem::TextString(TextString {
                                    data: "\u{1f1f3}".into(),
                                    bitwidth: IntegerWidth::Unknown,
                                }),
                                DataItem::Integer {
                                    value: 23,
                                    bitwidth: IntegerWidth::Zero,
                                },
                            ],
                            bitwidth: None,
                        },
                    ],
                    bitwidth: None,
                },
                {
                    "[_\"\u{1f1f3}\",23,[_\"\u{1f1f3}\",23]]",
                    "[_ \"\u{1f1f3}\", 23, [_ \"\u{1f1f3}\", 23]]",
                }
            }
        }

        mod hex {
            empty(hex2value, value2hex) {
                DataItem::Array {
                    data: vec![],
                    bitwidth: None,
                },
                indoc!(r#"
                    9f    # array(*)
                       ff #   break
                "#),
            }

            hello_world(hex2value, value2hex) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::TextString(TextString {
                            data: "world".into(),
                            bitwidth: IntegerWidth::Sixteen,
                        }),
                    ],
                    bitwidth: None,
                },
                indoc!(r#"
                    9f               # array(*)
                       65            #   text(5)
                          68656c6c6f #     "hello"
                       79 0005       #   text(5)
                          776f726c64 #     "world"
                       ff            #   break
                "#),
            }

            non_alpha(hex2value, value2hex) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::TextString(TextString {
                            data: "\u{1f1ff}".into(),
                            bitwidth: IntegerWidth::Eight,
                        }),
                    ],
                    bitwidth: None,
                },
                indoc!("
                    9f             # array(*)
                       64          #   text(4)
                          f09f87b3 #     \"\u{1f1f3}\"
                       78 04       #   text(4)
                          f09f87bf #     \"\u{1f1ff}\"
                       ff          #   break
                "),
            }

            heterogenous(hex2value, value2hex) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::SixtyFour,
                        },
                    ],
                    bitwidth: None,
                },
                indoc!("
                    9f                     # array(*)
                       64                  #   text(4)
                          f09f87b3         #     \"\u{1f1f3}\"
                       1b 0000000000000017 #   unsigned(23)
                       ff                  #   break
                "),
            }

            nested(hex2value, value2hex) {
                DataItem::Array {
                    data: vec![
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::SixtyFour,
                        },
                        DataItem::Array {
                            data: vec![
                                DataItem::TextString(TextString {
                                    data: "\u{1f1f3}".into(),
                                    bitwidth: IntegerWidth::Zero,
                                }),
                                DataItem::Integer {
                                    value: 23,
                                    bitwidth: IntegerWidth::SixtyFour,
                                },
                            ],
                            bitwidth: None,
                        },
                    ],
                    bitwidth: None,
                },
                indoc!("
                    9f                        # array(*)
                       64                     #   text(4)
                          f09f87b3            #     \"\u{1f1f3}\"
                       1b 0000000000000017    #   unsigned(23)
                       9f                     #   array(*)
                          64                  #     text(4)
                             f09f87b3         #       \"\u{1f1f3}\"
                          1b 0000000000000017 #     unsigned(23)
                          ff                  #     break
                       ff                     #   break
                ")
            }
        }
    }
}
