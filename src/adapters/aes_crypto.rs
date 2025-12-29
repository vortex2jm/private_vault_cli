use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
    aead::{Aead, OsRng, rand_core::RngCore},
};
use argon2::Argon2;
use zeroize::ZeroizeOnDrop;

use crate::domain::{errors::CryptoError, ports::CryptoPort};

#[derive(ZeroizeOnDrop)]
pub struct AesGcmCrypto {
    key: Option<[u8; 32]>,
}

impl AesGcmCrypto {
    pub fn new() -> Self {
        Self { key: None }
    }

    fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
        let mut output_key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut output_key)
            .expect("Failure on key derivation!\n");
        output_key
    }
}

impl CryptoPort for AesGcmCrypto {
    fn init(&mut self, password: &str, salt: &[u8]) {
        self.key = Some(Self::derive_key(password, salt));
    }

    fn encrypt(&self, plaintext: &[u8]) -> Result<(Vec<u8>, [u8; 12]), CryptoError> {
        let key = Key::<Aes256Gcm>::from(self.key.clone().ok_or(CryptoError::NotInitialized)?);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| CryptoError::Aead(e.to_string()))?;
        Ok((ciphertext, nonce.into()))
    }

    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let key = Key::<Aes256Gcm>::from(self.key.clone().ok_or(CryptoError::NotInitialized)?);
        let cipher = Aes256Gcm::new(&key);
        let nonce_array: [u8; 12] = nonce.try_into().map_err(|_| CryptoError::InvalidNonce)?;
        let plaintext = cipher
            .decrypt(&Nonce::from(nonce_array), ciphertext)
            .map_err(|e| CryptoError::Aead(e.to_string()))?;
        Ok(plaintext)
    }

    fn salt_gen(&self) -> [u8; 16] {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        salt
    }
}
