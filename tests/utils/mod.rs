extern crate cbor_diag;

pub use cbor_diag::{parse_diag, parse_hex};

#[macro_export]
macro_rules! testcases {
    ($($name:ident { $value:expr, $diag:expr, $hex:expr $(,)* })*) => {
        $(
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            fn diag2value() {
                let value = $crate::utils::parse_diag($diag).unwrap();
                assert_eq!(value, $value);
            }

            #[test]
            fn hex2value() {
                let value = $crate::utils::parse_hex($hex).unwrap();
                assert_eq!(value, $value);
            }
        }
        )*
    }
}
