use super::SECRET_KEY_BYTES_LEN;
use zeroize::Zeroize;

/// Provides a wrapper around a `[u8; SECRET_KEY_BYTES_LEN]` that implements `Zeroize` on `Drop`.
#[derive(Zeroize)]
#[zeroize(drop)]
pub struct SecretHash([u8; SECRET_KEY_BYTES_LEN]);

impl SecretHash {
    /// Instantiates `Self` with all zeros.
    pub fn zero() -> Self {
        Self([0; SECRET_KEY_BYTES_LEN])
    }

    /// Returns a reference to the underlying bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns a mutable reference to the underlying bytes.
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; SECRET_KEY_BYTES_LEN]> for SecretHash {
    fn from(array: [u8; SECRET_KEY_BYTES_LEN]) -> Self {
        Self(array)
    }
}

impl AsRef<[u8]> for SecretHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}