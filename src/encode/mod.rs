mod bytes;
mod diag;
mod hex;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Encoding {
    Base16,
    Base64,
    Base64Url,
}

impl Encoding {
    /// Return overrider if given, otherwise return self
    pub(crate) fn override_with(self, overrider: Option<Self>) -> Self {
        overrider.unwrap_or(self)
    }
}
