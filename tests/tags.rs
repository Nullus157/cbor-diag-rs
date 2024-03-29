use cbor_diag::{ByteString, DataItem, FloatWidth, IntegerWidth, Simple, Tag, TextString};
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

        epoch_date_min_repr {
            DataItem::Tag {
                tag: Tag::EPOCH_DATE,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Negative {
                    value: 96465657,
                    bitwidth: IntegerWidth::ThirtyTwo,
                }),
            },
            {
                "100_0(-96465658_2)",
                "100_0(-96465658_2)",
            },
            indoc!("
                d8 64          # epoch date value, tag(100)
                   3a 05bff2f9 #   negative(-96,465,658)
                               #   date(-262144-01-01)
            "),
        }

        epoch_date_min_repr_minus_one {
            DataItem::Tag {
                tag: Tag::EPOCH_DATE,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Negative {
                    value: 96465658,
                    bitwidth: IntegerWidth::ThirtyTwo,
                }),
            },
            {
                "100_0(-96465659_2)",
                "100_0(-96465659_2)",
            },
            indoc!("
                d8 64          # epoch date value, tag(100)
                   3a 05bff2fa #   negative(-96,465,659)
                               #   date offset is too large for this tool
            "),
        }

        epoch_date_max_repr {
            DataItem::Tag {
                tag: Tag::EPOCH_DATE,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Integer {
                    value: 95026601,
                    bitwidth: IntegerWidth::ThirtyTwo,
                }),
            },
            {
                "100_0(95026601_2)",
                "100_0(95026601_2)",
            },
            indoc!("
               d8 64          # epoch date value, tag(100)
                  1a 05a9fda9 #   unsigned(95,026,601)
                              #   date(+262143-12-31)
            "),
        }

        epoch_date_max_repr_plus_one {
            DataItem::Tag {
                tag: Tag::EPOCH_DATE,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Integer {
                    value: 95026602,
                    bitwidth: IntegerWidth::ThirtyTwo,
                }),
            },
            {
                "100_0(95026602_2)",
                "100_0(95026602_2)",
            },
            indoc!("
               d8 64          # epoch date value, tag(100)
                  1a 05a9fdaa #   unsigned(95,026,602)
                              #   date offset is too large for this tool
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

        encoded_cbor_empty(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: vec![],
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                r#"24(<<>>)"#,
                r#"24(<<>>)"#,
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

        encoded_cbor_seq_empty(diag2value, value2diag) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR_SEQ,
                bitwidth: IntegerWidth::Unknown,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: vec![],
                    bitwidth: IntegerWidth::Unknown,
                })),
            },
            {
                "63(<<>>)",
                "63(<<>>)",
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
                         #     Todo("Parsing error (Error(Error { input: [255], code: TagBits }))")
            "#),
        }

        encoded_cbor_empty(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: vec![],
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 18 # encoded cbor data item, tag(24)
                   40 #   bytes(0)
                      #     ""
                      #   failed to parse encoded cbor data item
                      #     Todo("Parsing error (Incomplete(Size(1)))")
            "#),
        }

        encoded_cbor_seq(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR_SEQ,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("9f64f09f87b317ff1615").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 3f                      # encoded cbor sequence, tag(63)
                   4a                      #   bytes(10)
                      9f64f09f87b317ff1615 #     "\x9fd\xf0\x9f\x87\xb3\x17\xff\x16\x15"
                                           #   encoded cbor data item
                                           #     9f             # array(*)
                                           #        64          #   text(4)
                                           #           f09f87b3 #     "🇳"
                                           #        17          #   unsigned(23)
                                           #        ff          #   break
                                           #   encoded cbor data item
                                           #     16 # unsigned(22)
                                           #   encoded cbor data item
                                           #     15 # unsigned(21)
            "#),
        }

        encoded_cbor_seq_invalid(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR_SEQ,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("1617ff").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 3f        # encoded cbor sequence, tag(63)
                   43        #   bytes(3)
                      1617ff #     "\x16\x17\xff"
                             #   encoded cbor data item
                             #     16 # unsigned(22)
                             #   encoded cbor data item
                             #     17 # unsigned(23)
                             #   failed to parse remaining encoded cbor sequence
                             #     Todo("Parsing error (Error(Error { input: [255], code: TagBits }))")
            "#),
        }

        encoded_cbor_seq_empty(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::ENCODED_CBOR_SEQ,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: vec![],
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
                d8 3f # encoded cbor sequence, tag(63)
                   40 #   bytes(0)
                      #     ""
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

        date_min_repr(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "-262144-01-01".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
               d9 03ec                          # standard date string, tag(1004)
                  6d                            #   text(13)
                     2d3236323134342d30312d3031 #     "-262144-01-01"
                                                #   epoch(-96,465,658)
            "#),
        }

        date_min_repr_minus_one(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "-262145-12-31".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
               d9 03ec                          # standard date string, tag(1004)
                  6d                            #   text(13)
                     2d3236323134352d31322d3331 #     "-262145-12-31"
                                                #   error parsing date: input is out of range
            "#),
        }

        date_max_repr(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "+262143-12-31".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
               d9 03ec                          # standard date string, tag(1004)
                  6d                            #   text(13)
                     2b3236323134332d31322d3331 #     "+262143-12-31"
                                                #   epoch(95,026,601)
            "#),
        }

        date_max_repr_plus_one(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::DATE,
                bitwidth: IntegerWidth::Sixteen,
                value: Box::new(DataItem::TextString(TextString {
                    data: "+262144-01-01".into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!(r#"
               d9 03ec                          # standard date string, tag(1004)
                  6d                            #   text(13)
                     2b3236323134342d30312d3031 #     "+262144-01-01"
                                                #   error parsing date: input is out of range
            "#),
        }

        shared_ref_cyclic(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::SHAREABLE,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Tag {
                            tag: Tag::SHARED_REF,
                            bitwidth: IntegerWidth::Eight,
                            value: Box::new(DataItem::Integer {
                                value: 0,
                                bitwidth: IntegerWidth::Zero,
                            }),
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
               d8 1c       # shareable value, tag(28)
                  81       #   array(1)
                     d8 1d #     reference to shared value, tag(29)
                        00 #       unsigned(0)
                           #       reference-to(0)
                           #   reference(0)
            "),
        }

        missing_shared_ref(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::SHARED_REF,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Integer {
                    value: 0,
                    bitwidth: IntegerWidth::Zero,
                }),
            },
            indoc!("
               d8 1d # reference to shared value, tag(29)
                  00 #   unsigned(0)
                     #   reference-to(0), not previously shared
            "),
        }

        encoded_cbor_with_shared_refs(hex2value, value2hex) {
            DataItem::Array {
                data: vec![
                    DataItem::Tag {
                        tag: Tag::SHAREABLE,
                        bitwidth: IntegerWidth::Eight,
                        value: Box::new(DataItem::Integer {
                            value: 0,
                            bitwidth: IntegerWidth::Zero,
                        }),
                    },
                    DataItem::Tag {
                        tag: Tag::ENCODED_CBOR,
                        bitwidth: IntegerWidth::Eight,
                        value: Box::new(DataItem::ByteString(ByteString {
                            data: hex!("d81c00").into(),
                            bitwidth: IntegerWidth::Zero,
                        })),
                    },
                    DataItem::Tag {
                        tag: Tag::SHAREABLE,
                        bitwidth: IntegerWidth::Eight,
                        value: Box::new(DataItem::Integer {
                            value: 0,
                            bitwidth: IntegerWidth::Zero,
                        }),
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
                83              # array(3)
                   d8 1c        #   shareable value, tag(28)
                      00        #     unsigned(0)
                                #     reference(0)
                   d8 18        #   encoded cbor data item, tag(24)
                      43        #     bytes(3)
                         d81c00 #       "\xd8\x1c\x00"
                                #     encoded cbor data item
                                #       d8 1c # shareable value, tag(28)
                                #          00 #   unsigned(0)
                                #             #   reference(0)
                   d8 1c        #   shareable value, tag(28)
                      00        #     unsigned(0)
                                #     reference(1)
            "#),
        }

        ipv4_address(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV4,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("c0000201").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!("
                d8 34          # ipv4 address and/or prefix, tag(52)
                   44          #   bytes(4)
                      c0000201 #     h'c0000201'
                               #   IPv4 address(192.0.2.1)
            "),
        }

        ipv4_prefix(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV4,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Integer {
                            value: 24,
                            bitwidth: IntegerWidth::Eight,
                        },
                        DataItem::ByteString(ByteString {
                            data: hex!("c00002").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 34           # ipv4 address and/or prefix, tag(52)
                   82           #   array(2)
                      18 18     #     unsigned(24)
                      43        #     bytes(3)
                         c00002 #       h'c00002'
                                #   IPv4 prefix(192.0.2.0/24)
            "),
        }

        ipv4_address_and_prefix(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV4,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("c0000201").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Integer {
                            value: 24,
                            bitwidth: IntegerWidth::Eight,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 34             # ipv4 address and/or prefix, tag(52)
                   82             #   array(2)
                      44          #     bytes(4)
                         c0000201 #       h'c0000201'
                      18 18       #     unsigned(24)
                                  #   IPv4 address-and-prefix(192.0.2.1/24)
            "),
        }

        ipv4_address_and_zone(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV4,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("c0000201").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Simple(Simple::NULL),
                        DataItem::Integer {
                            value: 6,
                            bitwidth: IntegerWidth::Zero,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 34             # ipv4 address and/or prefix, tag(52)
                   83             #   array(3)
                      44          #     bytes(4)
                         c0000201 #       h'c0000201'
                      f6          #     null, simple(22)
                      06          #     unsigned(6)
                                  #   IPv4 address-and-zone(192.0.2.1%6)
            "),
        }

        ipv4_address_and_text_zone(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV4,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("c0000201").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Simple(Simple::NULL),
                        DataItem::TextString(TextString {
                            data: "eth0".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!(r#"
                d8 34             # ipv4 address and/or prefix, tag(52)
                   83             #   array(3)
                      44          #     bytes(4)
                         c0000201 #       h'c0000201'
                      f6          #     null, simple(22)
                      64          #     text(4)
                         65746830 #       "eth0"
                                  #   IPv4 address-and-zone(192.0.2.1%eth0)
            "#),
        }

        ipv4_address_and_zone_and_prefix(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV4,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("c0000201").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Integer {
                            value: 24,
                            bitwidth: IntegerWidth::Eight,
                        },
                        DataItem::Integer {
                            value: 6,
                            bitwidth: IntegerWidth::Zero,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 34             # ipv4 address and/or prefix, tag(52)
                   83             #   array(3)
                      44          #     bytes(4)
                         c0000201 #       h'c0000201'
                      18 18       #     unsigned(24)
                      06          #     unsigned(6)
                                  #   IPv4 address-and-zone-and-prefix(192.0.2.1%6/24)
            "),
        }

        ipv6_address(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV6,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::ByteString(ByteString {
                    data: hex!("20010db81234deedbeefcafefacefeed").into(),
                    bitwidth: IntegerWidth::Zero,
                })),
            },
            indoc!("
                d8 36                                  # ipv6 address and/or prefix, tag(54)
                   50                                  #   bytes(16)
                      20010db81234deedbeefcafefacefeed #     h'20010db81234deedbeefcafefacefeed'
                                                       #   IPv6 address(2001:db8:1234:deed:beef:cafe:face:feed)
            "),
        }

        ipv6_prefix(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV6,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::Integer {
                            value: 48,
                            bitwidth: IntegerWidth::Eight,
                        },
                        DataItem::ByteString(ByteString {
                            data: hex!("20010db81234").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 36                 # ipv6 address and/or prefix, tag(54)
                   82                 #   array(2)
                      18 30           #     unsigned(48)
                      46              #     bytes(6)
                         20010db81234 #       h'20010db81234'
                                      #   IPv6 prefix(2001:db8:1234::/48)
            "),
        }

        ipv6_address_and_prefix(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV6,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("20010db81234deedbeefcafefacefeed").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Integer {
                            value: 56,
                            bitwidth: IntegerWidth::Eight,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 36                                     # ipv6 address and/or prefix, tag(54)
                   82                                     #   array(2)
                      50                                  #     bytes(16)
                         20010db81234deedbeefcafefacefeed #       h'20010db81234deedbeefcafefacefeed'
                      18 38                               #     unsigned(56)
                                                          #   IPv6 address-and-prefix(2001:db8:1234:deed:beef:cafe:face:feed/56)
            "),
        }

        ipv6_address_and_zone(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV6,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("fe8000000000020202fffffffe030303").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Simple(Simple::NULL),
                        DataItem::Integer {
                            value: 42,
                            bitwidth: IntegerWidth::Eight,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 36                                     # ipv6 address and/or prefix, tag(54)
                   83                                     #   array(3)
                      50                                  #     bytes(16)
                         fe8000000000020202fffffffe030303 #       h'fe8000000000020202fffffffe030303'
                      f6                                  #     null, simple(22)
                      18 2a                               #     unsigned(42)
                                                          #   IPv6 address-and-zone(fe80::202:2ff:ffff:fe03:303%42)
            "),
        }

        ipv6_address_and_text_zone(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV6,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("fe8000000000020202fffffffe030303").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Simple(Simple::NULL),
                        DataItem::TextString(TextString {
                            data: "eth0".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!(r#"
                d8 36                                     # ipv6 address and/or prefix, tag(54)
                   83                                     #   array(3)
                      50                                  #     bytes(16)
                         fe8000000000020202fffffffe030303 #       h'fe8000000000020202fffffffe030303'
                      f6                                  #     null, simple(22)
                      64                                  #     text(4)
                         65746830                         #       "eth0"
                                                          #   IPv6 address-and-zone(fe80::202:2ff:ffff:fe03:303%eth0)
            "#),
        }

        ipv6_address_and_zone_and_prefix(hex2value, value2hex) {
            DataItem::Tag {
                tag: Tag::IPV6,
                bitwidth: IntegerWidth::Eight,
                value: Box::new(DataItem::Array {
                    data: vec![
                        DataItem::ByteString(ByteString {
                            data: hex!("fe8000000000020202fffffffe030303").into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::Integer {
                            value: 64,
                            bitwidth: IntegerWidth::Eight,
                        },
                        DataItem::Integer {
                            value: 42,
                            bitwidth: IntegerWidth::Eight,
                        },
                    ],
                    bitwidth: Some(IntegerWidth::Zero),
                }),
            },
            indoc!("
                d8 36                                     # ipv6 address and/or prefix, tag(54)
                   83                                     #   array(3)
                      50                                  #     bytes(16)
                         fe8000000000020202fffffffe030303 #       h'fe8000000000020202fffffffe030303'
                      18 40                               #     unsigned(64)
                      18 2a                               #     unsigned(42)
                                                          #   IPv6 address-and-zone-and-prefix(fe80::202:2ff:ffff:fe03:303%42/64)
            "),
        }

        mod typed_array {
            u16_be(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_U16_BIG_ENDIAN,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("000200040008000400100100").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!("
                    d8 41      # typed array of u16, big endian, tag(65)
                       4c      #   bytes(12)
                          0002 #     unsigned(2)
                          0004 #     unsigned(4)
                          0008 #     unsigned(8)
                          0004 #     unsigned(4)
                          0010 #     unsigned(16)
                          0100 #     unsigned(256)
                "),
            }

            u8_clamped(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_U8_CLAMPED,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("020408041000").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!("
                    d8 44    # typed array of u8, clamped, tag(68)
                       46    #   bytes(6)
                          02 #     unsigned(2)
                          04 #     unsigned(4)
                          08 #     unsigned(8)
                          04 #     unsigned(4)
                          10 #     unsigned(16)
                          00 #     unsigned(0)
                "),
            }

            u64_le(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_U64_LITTLE_ENDIAN,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("00020004000800040010010000000001").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!("
                    d8 47                  # typed array of u64, little endian, tag(71)
                       50                  #   bytes(16)
                          0002000400080004 #     unsigned(288,239,172,311,843,328)
                          0010010000000001 #     unsigned(72,057,594,037,997,568)
                "),
            }

            i64_le(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_I64_LITTLE_ENDIAN,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("00020004000800040010010000000001").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!("
                    d8 4f                  # typed array of i64, little endian, twos-complement, tag(79)
                       50                  #   bytes(16)
                          0002000400080004 #     signed(288,239,172,311,843,328)
                          0010010000000001 #     signed(72,057,594,037,997,568)
                "),
            }

            f16_be(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_F16_BIG_ENDIAN,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("0002000400080004").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!("
                    d8 50      # typed array of f16, big endian, tag(80)
                       48      #   bytes(8)
                          0002 #     float(0.00000011920928955078125)
                          0004 #     float(0.0000002384185791015625)
                          0008 #     float(0.000000476837158203125)
                          0004 #     float(0.0000002384185791015625)
                "),
            }

            f64_le(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_F64_LITTLE_ENDIAN,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("f2fff4fff8fff441").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!("
                    d8 56                  # typed array of f64, little endian, tag(86)
                       48                  #   bytes(8)
                          f2fff4fff8fff441 #     float(5,637,115,903.312487)
                "),
            }

            f128_le(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_F128_LITTLE_ENDIAN,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("3ff2fff4fff8fff43ff2fff4fff8fff4").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!("
                    d8 57                                  # typed array of f128, little endian, tag(87)
                       50                                  #   bytes(16)
                          3ff2fff4fff8fff43ff2fff4fff8fff4 #     float(TODO: f128 unsupported)
                "),
            }

            u16_be_invalid_length(hex2value, value2hex) {
                DataItem::Tag {
                    tag: Tag::TYPED_ARRAY_U16_BIG_ENDIAN,
                    bitwidth: IntegerWidth::Eight,
                    value: Box::new(DataItem::ByteString(ByteString {
                        data: hex!("000200").into(),
                        bitwidth: IntegerWidth::Zero,
                    })),
                },
                indoc!(r#"
                    d8 41        # typed array of u16, big endian, tag(65)
                       43        #   bytes(3)
                          000200 #     "\x00\x02\x00"
                                 #   invalid data length for typed array
                "#),
            }

        }
    }
}
