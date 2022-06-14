use cbor_diag::{DataItem, IntegerWidth};
use indoc::indoc;

#[macro_use]
mod utils;

testcases! {
    mod utiny {
        zero {
            DataItem::Integer {
                value: 0,
                bitwidth: IntegerWidth::Zero,
            },
            {
                "0",
                "0",
            },
            indoc!("
                00 # unsigned(0)
            ")
        }

        one {
            DataItem::Integer {
                value: 1,
                bitwidth: IntegerWidth::Zero,
            },
            {
                "1",
                "1",
            },
            indoc!("
                01 # unsigned(1)
            ")
        }

        twenty_three {
            DataItem::Integer {
                value: 23,
                bitwidth: IntegerWidth::Zero,
            },
            {
                "23",
                "23",
            },
            indoc!("
                17 # unsigned(23)
            ")
        }
    }

    mod u8 {
        zero {
            DataItem::Integer {
                value: 0,
                bitwidth: IntegerWidth::Eight,
            },
            {
                "0_0",
                "0_0",
            },
            indoc!("
                18 00 # unsigned(0)
            ")
        }

        one {
            DataItem::Integer {
                value: 1,
                bitwidth: IntegerWidth::Eight,
            },
            {
                "1_0",
                "1_0",
            },
            indoc!("
                18 01 # unsigned(1)
            ")
        }

        twenty_four {
            DataItem::Integer {
                value: 24,
                bitwidth: IntegerWidth::Eight,
            },
            {
                "24_0",
                "24_0",
            },
            indoc!("
                18 18 # unsigned(24)
            ")
        }
    }

    mod u16 {
        zero {
            DataItem::Integer {
                value: 0,
                bitwidth: IntegerWidth::Sixteen,
            },
            {
                "0_1",
                "0_1",
            },
            indoc!("
                19 0000 # unsigned(0)
            ")
        }

        one {
            DataItem::Integer {
                value: 1,
                bitwidth: IntegerWidth::Sixteen,
            },
            {
                "1_1",
                "1_1",
            },
            indoc!("
                19 0001 # unsigned(1)
            ")
        }

        twenty_four {
            DataItem::Integer {
                value: 24,
                bitwidth: IntegerWidth::Sixteen,
            },
            {
                "24_1",
                "24_1",
            },
            indoc!("
                19 0018 # unsigned(24)
            ")
        }
    }

    mod u32 {
        zero {
            DataItem::Integer {
                value: 0,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            {
                "0_2",
                "0_2",
            },
            indoc!("
                1a 00000000 # unsigned(0)
            ")
        }

        one {
            DataItem::Integer {
                value: 1,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            {
                "1_2",
                "1_2",
            },
            indoc!("
                1a 00000001 # unsigned(1)
            ")
        }

        twenty_four {
            DataItem::Integer {
                value: 24,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            {
                "24_2",
                "24_2",
            },
            indoc!("
                1a 00000018 # unsigned(24)
            ")
        }
    }

    mod u64 {
        zero {
            DataItem::Integer {
                value: 0,
                bitwidth: IntegerWidth::SixtyFour,
            },
            {
                "0_3",
                "0_3",
            },
            indoc!("
                1b 0000000000000000 # unsigned(0)
            ")
        }

        one {
            DataItem::Integer {
                value: 1,
                bitwidth: IntegerWidth::SixtyFour,
            },
            {
                "1_3",
                "1_3",
            },
            indoc!("
                1b 0000000000000001 # unsigned(1)
            ")
        }

        twenty_four {
            DataItem::Integer {
                value: 24,
                bitwidth: IntegerWidth::SixtyFour,
            },
            {
                "24_3",
                "24_3",
            },
            indoc!("
                1b 0000000000000018 # unsigned(24)
            ")
        }
    }

    mod negative_utiny {
        one {
            DataItem::Negative {
                value: 0,
                bitwidth: IntegerWidth::Zero,
            },
            {
                "-1",
                "-1",
            },
            indoc!("
                20 # negative(-1)
            ")
        }

        twenty_four {
            DataItem::Negative {
                value: 23,
                bitwidth: IntegerWidth::Zero,
            },
            {
                "-24",
                "-24",
            },
            indoc!("
                37 # negative(-24)
            ")
        }
    }

    mod negative_u8 {
        one {
            DataItem::Negative {
                value: 0,
                bitwidth: IntegerWidth::Eight,
            },
            {
                "-1_0",
                "-1_0",
            },
            indoc!("
                38 00 # negative(-1)
            ")
        }

        twenty_five {
            DataItem::Negative {
                value: 24,
                bitwidth: IntegerWidth::Eight,
            },
            {
                "-25_0",
                "-25_0",
            },
            indoc!("
                38 18 # negative(-25)
            ")
        }
    }

    mod negative_u16 {
        one {
            DataItem::Negative {
                value: 0,
                bitwidth: IntegerWidth::Sixteen,
            },
            {
                "-1_1",
                "-1_1",
            },
            indoc!("
                39 0000 # negative(-1)
            ")
        }

        twenty_five {
            DataItem::Negative {
                value: 24,
                bitwidth: IntegerWidth::Sixteen,
            },
            {
                "-25_1",
                "-25_1",
            },
            indoc!("
                39 0018 # negative(-25)
            ")
        }
    }

    mod negative_u32 {
        one {
            DataItem::Negative {
                value: 0,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            {
                "-1_2",
                "-1_2",
            },
            indoc!("
                3a 00000000 # negative(-1)
            ")
        }

        twenty_five {
            DataItem::Negative {
                value: 24,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            {
                "-25_2",
                "-25_2",
            },
            indoc!("
                3a 00000018 # negative(-25)
            ")
        }
    }

    mod negative_u64 {
        one {
            DataItem::Negative {
                value: 0,
                bitwidth: IntegerWidth::SixtyFour,
            },
            {
                "-1_3",
                "-1_3",
            },
            indoc!("
                3b 0000000000000000 # negative(-1)
            ")
        }

        twenty_five {
            DataItem::Negative {
                value: 24,
                bitwidth: IntegerWidth::SixtyFour,
            },
            {
                "-25_3",
                "-25_3",
            },
            indoc!("
                3b 0000000000000018 # negative(-25)
            ")
        }
    }

    // RFC 8610 Appendix G.5
    mod formats {
        decimal(diag2value) {
            DataItem::Integer {
                value: 4711,
                bitwidth: IntegerWidth::Unknown,
            },
            { "4711" }
        }

        hexadecimal(diag2value) {
            DataItem::Integer {
                value: 4711,
                bitwidth: IntegerWidth::Unknown,
            },
            { "0x1267" }
        }

        octal(diag2value) {
            DataItem::Integer {
                value: 4711,
                bitwidth: IntegerWidth::Unknown,
            },
            { "0o11147" }
        }

        binary(diag2value) {
            DataItem::Integer {
                value: 4711,
                bitwidth: IntegerWidth::Unknown,
            },
            { "0b1001001100111" }
        }
    }
}
