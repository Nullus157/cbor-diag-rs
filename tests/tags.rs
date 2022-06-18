use cbor_diag::{ByteString, DataItem, FloatWidth, IntegerWidth, Tag, TextString};
use data_encoding_macro::hexlower as hex;
use indoc::indoc;

#[macro_use]
mod utils;

testcases! {
    mod both {
        self_describe_cbor {
            DataItem::Tag {
                tag: Tag::SELF_DESCRIBE_CBOR,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::Integer {
                    value: 0,
                    bitwidth: IntegerWidth::Zero,
                }),
            },
            {
                "55799_1(0)",
                "55799_1(0)",
            },
            indoc!("
                d9 d9f7 # self describe cbor, tag(55799)
                   00   #   unsigned(0)
            "),
        }

        epoch_date_birth {
            DataItem::Tag {
                tag: Tag::EPOCH_DATE,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Negative {
                    value: 10675,
                    bitwidth: IntegerWidth::Sixteen,
                }),
            },
            {
                "100_0(-10676_1)",
                "100_0(-10676_1)",
            },
            indoc!("
                d8 64      # epoch date value, tag(100)
                   39 29b3 #   negative(-10,676)
                           #   date(1940-10-09)
            "),
        }

        epoch_date_death {
            DataItem::Tag {
                tag: Tag::EPOCH_DATE,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Integer {
                    value: 3994,
                    bitwidth: IntegerWidth::Sixteen,
                }),
            },
            {
                "100_0(3994_1)",
                "100_0(3994_1)",
            },
            indoc!("
                d8 64      # epoch date value, tag(100)
                   19 0f9a #   unsigned(3,994)
                           #   date(1980-12-08)
            "),
        }
    }

    mod diag {
        date_time(diag2value, value2diag) {
            DataItem::Array {
                data: vec![
                    DataItem::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::TextString(TextString {
                            data: "2018-08-02T18:19:38Z".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }))
                    },
                    DataItem::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::TextString(TextString {
                            data: "1921-06-01T05:40:21Z".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }))
                    },
                    DataItem::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::TextString(TextString {
                            data: "2018-08-02T18:19:38.125Z".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }))
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                r#"[0("2018-08-02T18:19:38Z"),0("1921-06-01T05:40:21Z"),0("2018-08-02T18:19:38.125Z")]"#,
                r#"
                [
                    0("2018-08-02T18:19:38Z"),
                    0("1921-06-01T05:40:21Z"),
                    0("2018-08-02T18:19:38.125Z"),
                ]
                "#,
            }
        }

        epoch_date_time(diag2value, value2diag) {
            DataItem::Array {
                data: vec![
                    DataItem::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::Integer {
                            value: 1_533_233_978,
                            bitwidth: IntegerWidth::Unknown,
                        })
                    },
                    DataItem::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::Negative {
                            value: 1_533_233_978,
                            bitwidth: IntegerWidth::Unknown,
                        })
                    },
                    DataItem::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::Float {
                            value: 1_533_233_978.125,
                            bitwidth: FloatWidth::Unknown,
                        })
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                r#"[1(1533233978),1(-1533233979),1(1533233978.125)]"#,
                r#"
                [
                    1(1533233978),
                    1(-1533233979),
                    1(1533233978.125),
                ]
                "#,
            }
        }

        positive_bignum(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::POSITIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("000001ffffffffffffffffffffff0000000000000000000000").into(),
                    bitwidth: IntegerWidth::Unknown,
                }))
            },
            {
                "2(h'000001ffffffffffffffffffffff0000000000000000000000')",
                "2(h'000001ffffffffffffffffffffff0000000000000000000000')",
            }
        }

        negative_bignum(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::NEGATIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba987654321").into(),
                    bitwidth: IntegerWidth::Unknown,
                }))
            },
            {
                "3(h'123456789abcdeffedcba987654321')",
                "3(h'123456789abcdeffedcba987654321')",
            }
        }

        decimal_fraction(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::DECIMAL_FRACTION,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 1,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Integer {
                            value: 27315,
                            bitwidth: IntegerWidth::Unknown,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Unknown),
                })
            },
            {
                "4([-2,27315])",
                "4([-2, 27315])",
            }
        }

        bigfloat(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::BIGFLOAT,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 0,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Integer {
                            value: 3,
                            bitwidth: IntegerWidth::Zero,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Unknown),
                })
            },
            {
                "5([-1,3])",
                "5([-1, 3])",
            }
        }

        decimal_fraction_bignum(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::DECIMAL_FRACTION,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 1,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Tag {
                            tag: Tag::POSITIVE_BIGNUM,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::ByteString(ByteString {
                                data: hex!("000001ffffffffffffffffffffff0000000000000000000000").into(),
                                bitwidth: IntegerWidth::Unknown,
                            })),
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Unknown),
                })
            },
            {
                "4([-2,2(h'000001ffffffffffffffffffffff0000000000000000000000')])",
                "
                4([
                    -2,
                    2(h'000001ffffffffffffffffffffff0000000000000000000000'),
                ])
                ",
            }
        }

        bigfloat_bignum(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::BIGFLOAT,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 0,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Tag {
                            tag: Tag::POSITIVE_BIGNUM,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::ByteString(ByteString {
                                data: hex!("000001ffffffffffffffffffffff0000000000000000000000").into(),
                                bitwidth: IntegerWidth::Unknown,
                            })),
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Unknown),
                })
            },
            {
                "5([-1,2(h'000001ffffffffffffffffffffff0000000000000000000000')])",
                "
                5([
                    -1,
                    2(h'000001ffffffffffffffffffffff0000000000000000000000'),
                ])
                ",
            }
        }

        base64url_encoding(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64URL,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba9876543").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "21(b64'EjRWeJq83v_ty6mHZUM')",
                "21(b64'EjRWeJq83v_ty6mHZUM')",
            }
        }

        base64url_encoding_padded(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64URL,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("12").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "21(b64'Eg')",
                "21(b64'Eg')",
            }
        }

        base64url_encoding_nested(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64URL,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ],
                    bitwidth: None,
                })
            },
            {
                "21([_b64'EjRWeJq83v_ty6mHZUM'])",
                "21([_ b64'EjRWeJq83v_ty6mHZUM'])",
            }
        }

        base64_encoding(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba9876543").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "22(b64'EjRWeJq83v/ty6mHZUM=')",
                "22(b64'EjRWeJq83v/ty6mHZUM=')",
            }
        }

        base64_encoding_padded(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("12").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "22(b64'Eg==')",
                "22(b64'Eg==')",
            }
        }

        base64_encoding_nested(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ],
                    bitwidth: None,
                })
            },
            {
                "22([_b64'EjRWeJq83v/ty6mHZUM='])",
                "22([_ b64'EjRWeJq83v/ty6mHZUM='])",
            }
        }

        base16_encoding(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE16,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba9876543").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "23(h'123456789abcdeffedcba9876543')",
                "23(h'123456789abcdeffedcba9876543')",
            }
        }

        base16_encoding_nested(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE16,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ],
                    bitwidth: None,
                })
            },
            {
                "23([_h'123456789abcdeffedcba9876543'])",
                "23([_ h'123456789abcdeffedcba9876543'])",
            }
        }

        multiple_encodings(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64URL,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::Tag {
                            tag: Tag::ENCODED_BASE64,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::Array {
                                data: vec![
                                    DataItem::ByteString(ByteString {
                                        data: hex!("123456789abcdeffedcba9876543").into(),
                                        bitwidth: IntegerWidth::Unknown,
                                    })
                                ],
                                bitwidth: None,
                            })
                        },
                        DataItem::Tag {
                            tag: Tag::ENCODED_BASE16,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::ByteString(ByteString {
                                data: hex!("123456789abcdeffedcba9876543").into(),
                                bitwidth: IntegerWidth::Unknown,
                            })),
                        },
                    ],
                    bitwidth: None,
                })
            },
            {
                "21([_b64'EjRWeJq83v_ty6mHZUM',22([_b64'EjRWeJq83v/ty6mHZUM=']),23(h'123456789abcdeffedcba9876543')])",
                "
                21([_
                    b64'EjRWeJq83v_ty6mHZUM',
                    22([_ b64'EjRWeJq83v/ty6mHZUM=']),
                    23(h'123456789abcdeffedcba9876543'),
                ])
                ",
            }
        }

        encoded_cbor(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("9f64f09f87b317ff").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"24(<<[_"🇳",23]>>)"#,
                r#"24(<<[_ "🇳", 23]>>)"#,
            }
        }

        encoded_cbor_invalid(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("ff").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "24(h'ff')",
                "24(h'ff')",
            }
        }

        encoded_cbor_nested(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("d818489f64f09f87b317ff").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"24(<<24_0(<<[_"🇳",23]>>)>>)"#,
                r#"24(<<24_0(<<[_ "🇳", 23]>>)>>)"#,
            }
        }

        encoded_cbor_seq(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR_SEQ,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("9f64f09f87b317ff1615").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"63(<<[_"🇳",23],22,21>>)"#,
                r#"63(<<[_ "🇳", 23], 22, 21>>)"#,
            }
        }

        encoded_cbor_seq_invalid(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR_SEQ,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("1617ff").into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "63(<<22,23>>h'ff')",
                "63(<<22, 23>> h'ff')",
            }
        }

        uri(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::URI,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::TextString(TextString {
                    data: "https://example.com".into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"32("https://example.com")"#,
                r#"32("https://example.com")"#,
            }
        }

        uri_non_http(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::URI,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::TextString(TextString {
                    data: "urn:oasis:names:specification:docbook:dtd:xml:4.1.2".into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"32("urn:oasis:names:specification:docbook:dtd:xml:4.1.2")"#,
                r#"32("urn:oasis:names:specification:docbook:dtd:xml:4.1.2")"#,
            }
        }

        uri_invalid(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::URI,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::TextString(TextString {
                    data: "foo".into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"32("foo")"#,
                r#"32("foo")"#,
            }
        }

        base64url(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::BASE64URL,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::TextString(TextString {
                    data: "aHR0cHM6Ly9leGFtcGxlLmNvbS_wn5C2".into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"33("aHR0cHM6Ly9leGFtcGxlLmNvbS_wn5C2")"#,
                r#"33("aHR0cHM6Ly9leGFtcGxlLmNvbS_wn5C2")"#,
            }
        }

        base64(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::BASE64,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::TextString(TextString {
                    data: "aHR0cHM6Ly9leGFtcGxlLmNvbS/wn5C2".into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"34("aHR0cHM6Ly9leGFtcGxlLmNvbS/wn5C2")"#,
                r#"34("aHR0cHM6Ly9leGFtcGxlLmNvbS/wn5C2")"#,
            }
        }

        self_describe_cbor(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::SELF_DESCRIBE_CBOR,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::Integer {
                    value: 0,
                    bitwidth: IntegerWidth::Zero,
                }),
            },
            {
                "55799(0)",
                "55799(0)",
            }
        }

        date_birth(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "1940-10-09".into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"1004_1("1940-10-09")"#,
                r#"1004_1("1940-10-09")"#,
            }
        }

        date_death(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "1980-12-08".into(),
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"1004_1("1980-12-08")"#,
                r#"1004_1("1980-12-08")"#,
            }
        }
    }

    mod hex_tests {
        date_time(hex2value, value2hex) {
            DataItem::Array {
                data: vec![
                    DataItem::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::TextString(TextString {
                            data: "2018-08-02T18:19:38Z".into(),
                            bitwidth: IntegerWidth::Zero,
                        }))
                    },
                    DataItem::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::TextString(TextString {
                            data: "1921-06-01T05:40:21Z".into(),
                            bitwidth: IntegerWidth::Zero,
                        }))
                    },
                    DataItem::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::TextString(TextString {
                            data: "2018-08-02T18:19:38.125Z".into(),
                            bitwidth: IntegerWidth::Eight,
                        }))
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
               83                                                        # array(3)
                  c0                                                     #   standard datetime string, tag(0)
                     74                                                  #     text(20)
                        323031382d30382d30325431383a31393a33385a         #       "2018-08-02T18:19:38Z"
                                                                         #     epoch(1533233978)
                  c0                                                     #   standard datetime string, tag(0)
                     74                                                  #     text(20)
                        313932312d30362d30315430353a34303a32315a         #       "1921-06-01T05:40:21Z"
                                                                         #     epoch(-1533233979)
                  c0                                                     #   standard datetime string, tag(0)
                     78 18                                               #     text(24)
                        323031382d30382d30325431383a31393a33382e3132355a #       "2018-08-02T18:19:38.125Z"
                                                                         #     epoch(1533233978.125)
            "#),
        }

        epoch_date_time(hex2value, value2hex) {
            DataItem::Array {
                data: vec![
                    DataItem::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::Integer {
                            value: 1_533_233_978,
                            bitwidth: IntegerWidth::ThirtyTwo,
                        })
                    },
                    DataItem::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::Negative {
                            value: 1_533_233_978,
                            bitwidth: IntegerWidth::ThirtyTwo,
                        })
                    },
                    DataItem::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(DataItem::Float {
                            value: 1_533_233_978.125,
                            bitwidth: FloatWidth::SixtyFour,
                        })
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
                83                        # array(3)
                   c1                     #   epoch datetime value, tag(1)
                      1a 5b634b3a         #     unsigned(1,533,233,978)
                                          #     datetime(2018-08-02T18:19:38Z)
                   c1                     #   epoch datetime value, tag(1)
                      3a 5b634b3a         #     negative(-1,533,233,979)
                                          #     datetime(1921-06-01T05:40:21Z)
                   c1                     #   epoch datetime value, tag(1)
                      fb 41d6d8d2ce880000 #     float(1,533,233,978.125)
                                          #     datetime(2018-08-02T18:19:38.125Z)
            "#),
        }

        positive_bignum(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::POSITIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("000001ffffffffffffffffffffff0000000000000000000000").into(),
                    bitwidth: IntegerWidth::Eight,
                }))
            },
            indoc!(r#"
                c2                                     # positive bignum, tag(2)
                   58 19                               #   bytes(25)
                      000001ffffffffffffffffffffff0000 #     "\x00\x00\x01\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x00\x00"
                      000000000000000000               #     "\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                                                       #   bignum(191561942608236107294793378084303638130997321548169216)
            "#),
        }

        negative_bignum(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::NEGATIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba987654321").into(),
                    bitwidth: IntegerWidth::Eight,
                }))
            },
            indoc!(r#"
                c3                                   # negative bignum, tag(3)
                   58 0f                             #   bytes(15)
                      123456789abcdeffedcba987654321 #     "\x124Vx\x9a\xbc\xde\xff\xed\xcb\xa9\x87eC!"
                                                     #   bignum(-94522879700260684208272210605196066)
            "#),
        }

        decimal_fraction(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DECIMAL_FRACTION,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 1,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Integer {
                            value: 27315,
                            bitwidth: IntegerWidth::Sixteen,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                })
            },
            indoc!(r#"
                c4            # decimal fraction, tag(4)
                   82         #   array(2)
                      21      #     negative(-2)
                      19 6ab3 #     unsigned(27,315)
                              #   decimal fraction(5463/20)
            "#),
        }

        bigfloat(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::BIGFLOAT,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 0,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Integer {
                            value: 3,
                            bitwidth: IntegerWidth::Zero,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                })
            },
            indoc!(r#"
                c5       # bigfloat, tag(5)
                   82    #   array(2)
                      20 #     negative(-1)
                      03 #     unsigned(3)
                         #   bigfloat(3/2)
            "#),
        }

        decimal_fraction_bignum(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DECIMAL_FRACTION,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 52,
                            bitwidth: IntegerWidth::Eight,
                        },
                        DataItem::Tag {
                            tag: Tag::POSITIVE_BIGNUM,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::ByteString(ByteString {
                                data: hex!("000001ffffffffffffffffffffff0000000000000000000000").into(),
                                bitwidth: IntegerWidth::Eight,
                            }))
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                })
            },
            indoc!(r#"
                c4                                           # decimal fraction, tag(4)
                   82                                        #   array(2)
                      38 34                                  #     negative(-53)
                      c2                                     #     positive bignum, tag(2)
                         58 19                               #       bytes(25)
                            000001ffffffffffffffffffffff0000 #         "\x00\x00\x01\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x00\x00"
                            000000000000000000               #         "\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                                                             #       bignum(191561942608236107294793378084303638130997321548169216)
                                                             #   decimal fraction(21267647932558653966460912930125774848/11102230246251565404236316680908203125)
            "#),
        }

        bigfloat_bignum(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::BIGFLOAT,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Negative {
                            value: 175,
                            bitwidth: IntegerWidth::Eight,
                        },
                        DataItem::Tag {
                            tag: Tag::POSITIVE_BIGNUM,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::ByteString(ByteString {
                                data: hex!("000001ffffffffffffffffffffff0000000000000000000000").into(),
                                bitwidth: IntegerWidth::Eight,
                            }))
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                })
            },
            indoc!(r#"
                c5                                           # bigfloat, tag(5)
                   82                                        #   array(2)
                      38 af                                  #     negative(-176)
                      c2                                     #     positive bignum, tag(2)
                         58 19                               #       bytes(25)
                            000001ffffffffffffffffffffff0000 #         "\x00\x00\x01\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x00\x00"
                            000000000000000000               #         "\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                                                             #       bignum(191561942608236107294793378084303638130997321548169216)
                                                             #   bigfloat(618970019642690137449562111/309485009821345068724781056)
            "#),
        }

        base64url_encoding(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64URL,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba9876543").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d5                                 # suggested base64url encoding, tag(21)
                   4e                              #   bytes(14)
                      123456789abcdeffedcba9876543 #     b64'EjRWeJq83v_ty6mHZUM'
            "#),
        }

        base64url_encoding_nested(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64URL,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ],
                    bitwidth: None,
                })
            },
            indoc!(r#"
                d5                                    # suggested base64url encoding, tag(21)
                   9f                                 #   array(*)
                      4e                              #     bytes(14)
                         123456789abcdeffedcba9876543 #       b64'EjRWeJq83v_ty6mHZUM'
                      ff                              #     break
            "#),
        }

        base64_encoding(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba9876543").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d6                                 # suggested base64 encoding, tag(22)
                   4e                              #   bytes(14)
                      123456789abcdeffedcba9876543 #     b64'EjRWeJq83v/ty6mHZUM='
            "#),
        }

        base64_encoding_nested(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ],
                    bitwidth: None,
                })
            },
            indoc!(r#"
                d6                                    # suggested base64 encoding, tag(22)
                   9f                                 #   array(*)
                      4e                              #     bytes(14)
                         123456789abcdeffedcba9876543 #       b64'EjRWeJq83v/ty6mHZUM='
                      ff                              #     break
            "#),
        }

        base16_encoding(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE16,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("123456789abcdeffedcba9876543").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d7                                 # suggested base16 encoding, tag(23)
                   4e                              #   bytes(14)
                      123456789abcdeffedcba9876543 #     h'123456789abcdeffedcba9876543'
            "#),
        }

        base16_encoding_nested(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE16,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ],
                    bitwidth: None,
                })
            },
            indoc!(r#"
                d7                                    # suggested base16 encoding, tag(23)
                   9f                                 #   array(*)
                      4e                              #     bytes(14)
                         123456789abcdeffedcba9876543 #       h'123456789abcdeffedcba9876543'
                      ff                              #     break
            "#),
        }

        multiple_encodings(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_BASE64URL,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("123456789abcdeffedcba9876543").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Tag {
                            tag: Tag::ENCODED_BASE64,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::Array {
                                data: vec![
                                    DataItem::ByteString(ByteString {
                                        data: hex!("123456789abcdeffedcba9876543").into(),
                                        bitwidth: IntegerWidth::Zero,
                                    })
                                ],
                                bitwidth: None,
                            })
                        },
                        DataItem::Tag {
                            tag: Tag::ENCODED_BASE16,
                            bitwidth: IntegerWidth::Zero,
                            value: Box::new(DataItem::ByteString(ByteString {
                                data: hex!("123456789abcdeffedcba9876543").into(),
                                bitwidth: IntegerWidth::Zero,
                            })),
                        },
                    ],
                    bitwidth: None,
                })
            },
            indoc!(r#"
                d5                                          # suggested base64url encoding, tag(21)
                   9f                                       #   array(*)
                      4e                                    #     bytes(14)
                         123456789abcdeffedcba9876543       #       b64'EjRWeJq83v_ty6mHZUM'
                      d6                                    #     suggested base64 encoding, tag(22)
                         9f                                 #       array(*)
                            4e                              #         bytes(14)
                               123456789abcdeffedcba9876543 #           b64'EjRWeJq83v/ty6mHZUM='
                            ff                              #         break
                      d7                                    #     suggested base16 encoding, tag(23)
                         4e                                 #       bytes(14)
                            123456789abcdeffedcba9876543    #         h'123456789abcdeffedcba9876543'
                      ff                                    #     break
            "#),
        }

        encoded_cbor(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("9f64f09f87b317ff").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!("
                d8 18                  # encoded cbor data item, tag(24)
                   48                  #   bytes(8)
                      9f64f09f87b317ff #     \"\\x9fd\\xf0\\x9f\\x87\\xb3\\x17\\xff\"
                                       #   encoded cbor data item
                                       #     9f             # array(*)
                                       #        64          #   text(4)
                                       #           f09f87b3 #     \"\u{1f1f3}\"
                                       #        17          #   unsigned(23)
                                       #        ff          #   break
            "),
        }

        encoded_cbor_invalid(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("ff").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 18    # encoded cbor data item, tag(24)
                   41    #   bytes(1)
                      ff #     "\xff"
                         #   failed to parse encoded cbor data item
                         #     Todo("Parsing error (Error(([255], TagBits)))")
            "#),
        }

        uri(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::URI,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::TextString(TextString {
                    data: "https://example.com".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 20                                        # uri, tag(32)
                   73                                        #   text(19)
                      68747470733a2f2f6578616d706c652e636f6d #     "https://example.com"
                                                             #   valid URL (checked against URL Standard, not RFC 3986)
            "#),
        }

        uri_non_http(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::URI,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::TextString(TextString {
                    data: "urn:oasis:names:specification:docbook:dtd:xml:4.1.2".into(),
                    bitwidth: IntegerWidth::Eight,
                })),
            },
            indoc!(r#"
                d8 20                                  # uri, tag(32)
                   78 33                               #   text(51)
                      75726e3a6f617369733a6e616d65733a #     "urn:oasis:names:"
                      73706563696669636174696f6e3a646f #     "specification:do"
                      63626f6f6b3a6474643a786d6c3a342e #     "cbook:dtd:xml:4."
                      312e32                           #     "1.2"
                                                       #   valid URL (checked against URL Standard, not RFC 3986)
            "#),
        }

        uri_invalid(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::URI,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::TextString(TextString {
                    data: "foo".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 20        # uri, tag(32)
                   63        #   text(3)
                      666f6f #     "foo"
                             #   invalid URL (checked against URL Standard, not RFC 3986)
            "#),
        }

        base64url(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::BASE64URL,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::TextString(TextString {
                    data: "aHR0cHM6Ly9leGFtcGxlLmNvbS_wn5C2".into(),
                    bitwidth: IntegerWidth::Eight,
                })),
            },
            indoc!(r#"
                d8 21                                  # base64url encoded text, tag(33)
                   78 20                               #   text(32)
                      6148523063484d364c79396c65474674 #     "aHR0cHM6Ly9leGFt"
                      6347786c4c6d4e7662535f776e354332 #     "cGxlLmNvbS_wn5C2"
                                                       #   base64url decoded
                                                       #     68747470733a2f2f6578616d706c652e # "https://example."
                                                       #     636f6d2ff09f90b6                 # "com/\xf0\x9f\x90\xb6"
            "#),
        }

        base64(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::BASE64,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::TextString(TextString {
                    data: "aHR0cHM6Ly9leGFtcGxlLmNvbS/wn5C2".into(),
                    bitwidth: IntegerWidth::Eight,
                })),
            },
            indoc!(r#"
                d8 22                                  # base64 encoded text, tag(34)
                   78 20                               #   text(32)
                      6148523063484d364c79396c65474674 #     "aHR0cHM6Ly9leGFt"
                      6347786c4c6d4e7662532f776e354332 #     "cGxlLmNvbS/wn5C2"
                                                       #   base64 decoded
                                                       #     68747470733a2f2f6578616d706c652e # "https://example."
                                                       #     636f6d2ff09f90b6                 # "com/\xf0\x9f\x90\xb6"
            "#),
        }

        encoded_cbor_nested(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("d818489f64f09f87b317ff").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!("
                d8 18                        # encoded cbor data item, tag(24)
                   4b                        #   bytes(11)
                      d818489f64f09f87b317ff #     \"\\xd8\\x18H\\x9fd\\xf0\\x9f\\x87\\xb3\\x17\\xff\"
                                             #   encoded cbor data item
                                             #     d8 18                  # encoded cbor data item, tag(24)
                                             #        48                  #   bytes(8)
                                             #           9f64f09f87b317ff #     \"\\x9fd\\xf0\\x9f\\x87\\xb3\\x17\\xff\"
                                             #                            #   encoded cbor data item
                                             #                            #     9f             # array(*)
                                             #                            #        64          #   text(4)
                                             #                            #           f09f87b3 #     \"\u{1f1f3}\"
                                             #                            #        17          #   unsigned(23)
                                             #                            #        ff          #   break
            "),
        }

        uuid(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::UUID,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("8c8a8d48c00f42209cf8b75a882bf586").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 25                                  # uuid, tag(37)
                   50                                  #   bytes(16)
                      8c8a8d48c00f42209cf8b75a882bf586 #     h'8c8a8d48c00f42209cf8b75a882bf586'
                                                       #   uuid(variant(RFC4122), version(4, Random))
                                                       #     base16(8c8a8d48-c00f-4220-9cf8-b75a882bf586)
                                                       #     base58(JMZyLNqHizfirvWvE2EXBK)
                                                       #     base64(jIqNSMAPQiCc+LdaiCv1hg)
            "#),
        }

        uuid_invalid_length(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::UUID,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("0123456789").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 25            # uuid, tag(37)
                   45            #   bytes(5)
                      0123456789 #     h'0123456789'
                                 #   invalid data length for uuid
            "#),
        }

        uuid_invalid_type(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::UUID,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::TextString(TextString {
                    data: "0123456789ab".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 25                          # uuid, tag(37)
                   6c                          #   text(12)
                      303132333435363738396162 #     "0123456789ab"
                                               #   invalid type for uuid
            "#),
        }

        network_address_ipv4(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::NETWORK_ADDRESS,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("c00a0a01").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d9 0104        # network address, tag(260)
                   44          #   bytes(4)
                      c00a0a01 #     h'c00a0a01'
                               #   IPv4 address(192.10.10.1)
            "#),
        }

        network_address_mac(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::NETWORK_ADDRESS,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("0123456789ab").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d9 0104            # network address, tag(260)
                   46              #   bytes(6)
                      0123456789ab #     h'0123456789ab'
                                   #   MAC address(01:23:45:67:89:ab)
            "#),
        }

        network_address_ipv6(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::NETWORK_ADDRESS,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("20010db885a3000000008a2e03707334").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d9 0104                                # network address, tag(260)
                   50                                  #   bytes(16)
                      20010db885a3000000008a2e03707334 #     h'20010db885a3000000008a2e03707334'
                                                       #   IPv6 address(2001:db8:85a3::8a2e:370:7334)
            "#),
        }

        network_address_invalid_length(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::NETWORK_ADDRESS,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("0123456789").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d9 0104          # network address, tag(260)
                   45            #   bytes(5)
                      0123456789 #     h'0123456789'
                                 #   invalid data length for network address
            "#),
        }

        network_address_invalid_type(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::NETWORK_ADDRESS,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "0123456789ab".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d9 0104                        # network address, tag(260)
                   6c                          #   text(12)
                      303132333435363738396162 #     "0123456789ab"
                                               #   invalid type for network address
            "#),
        }

        date_birth(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "1940-10-09".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d9 03ec                    # standard date string, tag(1004)
                   6a                      #   text(10)
                      313934302d31302d3039 #     "1940-10-09"
                                           #   epoch(-10,676)
            "#),
        }

        date_death(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "1980-12-08".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d9 03ec                    # standard date string, tag(1004)
                   6a                      #   text(10)
                      313938302d31322d3038 #     "1980-12-08"
                                           #   epoch(3,994)
            "#),
        }
    }
}
