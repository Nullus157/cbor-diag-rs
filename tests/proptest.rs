#[macro_use]
extern crate proptest;
extern crate cbor_diag;
extern crate hex;

proptest! {
    #[test]
    fn diag_doesnt_crash_with_anything(ref s in ".*") {
        let _ = cbor_diag::parse_diag(s);
    }

    #[test]
    fn hex_doesnt_crash_with_anything(ref s in ".*") {
        let _ = cbor_diag::parse_hex(s);
    }

    #[test]
    fn hex_doesnt_crash_with_hex(ref s in "(:?[0-9a-f]{2})*") {
        let _ = cbor_diag::parse_hex(s);
    }

    #[test]
    fn bytes_doesnt_crash_with_anything(ref s in ".*") {
        let _ = cbor_diag::parse_bytes(s);
    }
}

#[test]
fn multiply_overflow() {
    let _ = cbor_diag::parse_bytes(hex::decode("7b2000000000000000").unwrap());
    let _ = cbor_diag::parse_bytes(hex::decode("5b2000000000000000").unwrap());
}
