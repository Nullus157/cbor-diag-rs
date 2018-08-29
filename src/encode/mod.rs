mod diag;
mod hex;

#[derive(Copy, Clone)]
pub(crate) enum Encoding {
    Base16,
    Base64,
    Base64Url,
}
