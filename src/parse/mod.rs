use hex;
use {DataItem, Result};

mod binary;
mod diag;

pub use self::binary::parse_bytes;
pub use self::diag::parse_diag;

fn remove_comments(hex: impl AsRef<str>) -> String {
    hex.as_ref()
        .lines()
        .map(|line| line.split('#').next().unwrap().replace(" ", ""))
        .collect()
}

pub fn parse_hex(hex: impl AsRef<str>) -> Result<DataItem> {
    let hex = remove_comments(hex);
    let bytes = hex::decode(hex)?;
    parse_bytes(bytes)
}
