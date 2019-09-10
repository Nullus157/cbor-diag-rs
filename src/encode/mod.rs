mod bytes;
mod diag;
mod hex;

#[derive(Copy, Clone)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Encoding {
    Base16,
    Base64,
    Base64Url,
}
