use cbor_diag::{DataItem, FloatWidth};
use indoc::indoc;
use std::f64::{INFINITY, NAN, NEG_INFINITY};

#[macro_use]
mod utils;

testcases! {
    mod unknown {
        zero(diag2value, value2diag) {
            DataItem::Float {
                value: 0.0,
                bitwidth: FloatWidth::Unknown,
            },
            {
                "0.0",
                "0.0",
            }
        }

        one(diag2value, value2diag) {
            DataItem::Float {
                value: 1.0,
                bitwidth: FloatWidth::Unknown,
            },
            {
                "1.0",
                "1.0",
            }
        }

        half(diag2value, value2diag) {
            DataItem::Float {
                value: 0.5,
                bitwidth: FloatWidth::Unknown,
            },
            {
                "0.5",
                "0.5",
            }
        }

        infinity(diag2value, value2diag) {
            DataItem::Float {
                value: INFINITY,
                bitwidth: FloatWidth::Unknown,
            },
            {
                "Infinity",
                "Infinity",
            }
        }

        neg_infinity(diag2value, value2diag) {
            DataItem::Float {
                value: NEG_INFINITY,
                bitwidth: FloatWidth::Unknown,
            },
            {
                "-Infinity",
                "-Infinity",
            }
        }

        nan(value2diag) {
            DataItem::Float {
                value: NAN,
                bitwidth: FloatWidth::Unknown,
            },
            {
                "NaN",
                "NaN",
            }
        }
    }

    mod f16 {
        zero {
            DataItem::Float {
                value: 0.0,
                bitwidth: FloatWidth::Sixteen,
            },
            {
                "0.0_1",
                "0.0_1",
            },
            indoc!("
                f9 0000 # float(0)
            ")
        }

        one {
            DataItem::Float {
                value: 1.0,
                bitwidth: FloatWidth::Sixteen,
            },
            {
                "1.0_1",
                "1.0_1",
            },
            indoc!("
                f9 3c00 # float(1)
            ")
        }

        half {
            DataItem::Float {
                value: 0.5,
                bitwidth: FloatWidth::Sixteen,
            },
            {
                "0.5_1",
                "0.5_1",
            },
            indoc!("
                f9 3800 # float(0.5)
            ")
        }

        infinity {
            DataItem::Float {
                value: INFINITY,
                bitwidth: FloatWidth::Sixteen,
            },
            {
                "Infinity_1",
                "Infinity_1",
            },
            indoc!("
                f9 7c00 # float(Infinity)
            ")
        }

        neg_infinity {
            DataItem::Float {
                value: NEG_INFINITY,
                bitwidth: FloatWidth::Sixteen,
            },
            {
                "-Infinity_1",
                "-Infinity_1",
            },
            indoc!("
                f9 fc00 # float(-Infinity)
            ")
        }

        nan(value2diag, value2hex) {
            DataItem::Float {
                value: NAN,
                bitwidth: FloatWidth::Sixteen,
            },
            {
                "NaN_1",
                "NaN_1",
            },
            indoc!("
                f9 7e00 # float(NaN)
            ")
        }
    }

    mod f32 {
        zero {
            DataItem::Float {
                value: 0.0,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            {
                "0.0_2",
                "0.0_2",
            },
            indoc!("
                fa 00000000 # float(0)
            ")
        }

        one {
            DataItem::Float {
                value: 1.0,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            {
                "1.0_2",
                "1.0_2",
            },
            indoc!("
                fa 3f800000 # float(1)
            ")
        }

        half {
            DataItem::Float {
                value: 0.5,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            {
                "0.5_2",
                "0.5_2",
            },
            indoc!("
                fa 3f000000 # float(0.5)
            ")
        }

        infinity {
            DataItem::Float {
                value: INFINITY,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            {
                "Infinity_2",
                "Infinity_2",
            },
            indoc!("
                fa 7f800000 # float(Infinity)
            ")
        }

        neg_infinity {
            DataItem::Float {
                value: NEG_INFINITY,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            {
                "-Infinity_2",
                "-Infinity_2",
            },
            indoc!("
                fa ff800000 # float(-Infinity)
            ")
        }

        nan(value2diag, value2hex) {
            DataItem::Float {
                value: NAN,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            {
                "NaN_2",
                "NaN_2",
            },
            indoc!("
                fa 7fc00000 # float(NaN)
            ")
        }
    }

    mod f64 {
        zero {
            DataItem::Float {
                value: 0.0,
                bitwidth: FloatWidth::SixtyFour,
            },
            {
                "0.0_3",
                "0.0_3",
            },
            indoc!("
                fb 0000000000000000 # float(0)
            ")
        }

        one {
            DataItem::Float {
                value: 1.0,
                bitwidth: FloatWidth::SixtyFour,
            },
            {
                "1.0_3",
                "1.0_3",
            },
            indoc!("
                fb 3ff0000000000000 # float(1)
            ")
        }

        half {
            DataItem::Float {
                value: 0.5,
                bitwidth: FloatWidth::SixtyFour,
            },
            {
                "0.5_3",
                "0.5_3",
            },
            indoc!("
                fb 3fe0000000000000 # float(0.5)
            ")
        }

        infinity {
            DataItem::Float {
                value: f64::INFINITY,
                bitwidth: FloatWidth::SixtyFour,
            },
            {
                "Infinity_3",
                "Infinity_3",
            },
            indoc!("
                fb 7ff0000000000000 # float(Infinity)
            ")
        }

        neg_infinity {
            DataItem::Float {
                value: f64::NEG_INFINITY,
                bitwidth: FloatWidth::SixtyFour,
            },
            {
                "-Infinity_3",
                "-Infinity_3",
            },
            indoc!("
                fb fff0000000000000 # float(-Infinity)
            ")
        }

        nan(value2diag, value2hex) {
            DataItem::Float {
                value: f64::NAN,
                bitwidth: FloatWidth::SixtyFour,
            },
            {
                "NaN_3",
                "NaN_3",
            },
            indoc!("
                fb 7ff8000000000000 # float(NaN)
            ")
        }
    }
}
