extern crate cbor_diag;

pub use cbor_diag::{parse_diag, parse_hex};

#[macro_export]
macro_rules! testcases {
    (
        @testcase $name:ident(diag2value $(, $rest:ident)+) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn diag2value() {
            let value = $crate::utils::parse_diag($diag).unwrap();
            assert_eq!(value, $value);
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident(hex2value $(, $rest:ident)+) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn hex2value() {
            let value = $crate::utils::parse_hex($hex).unwrap();
            assert_eq!(value, $value);
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident(value2diag $(, $rest:ident)*) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn value2diag() {
            let diag = $value.to_diag().unwrap();
            assert_eq!(diag, $diag);
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident(value2hex $(, $rest:ident)*) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn value2hex() {
            let hex = $value.to_hex().unwrap();
            assert_eq!(hex, $hex);
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident() {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
    };

    (
        @testcases $name:ident($($test:ident),+) {
            $value:expr, $diag:expr, $hex:expr $(,)*
        }
    ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! {
                @testcase $name($($test),+) { $value, $diag, $hex }
            }
        }
    };

    (
        @testcases $name:ident($($test:ident),+) {
            $value:expr, $hexordiag:expr $(,)*
        }
    ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! {
                @testcase $name($($test),+) { $value, $hexordiag, $hexordiag }
            }
        }
    };

    ($($name:ident($($test:ident),+) { $($tt:tt)+ })*) => {
        $(testcases! {
            @testcases $name($($test),+) { $($tt)+ }
        })*
    };

    ($($name:ident { $($tt:tt)+ })*) => {
        $(testcases! {
            @testcases $name(diag2value, hex2value, value2diag, value2hex) {
                $($tt)+
            }
        })*
    };

    ($(mod $name:ident { $($tt:tt)* })*) => {
        $(
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! { $($tt)* }
        }
        )*
    };
}
