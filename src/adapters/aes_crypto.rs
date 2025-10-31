use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
    aead::{Aead, OsRng, rand_core::RngCore},
};
use argon2::Argon2;
use zeroize::{ZeroizeOnDrop};

use crate::domain::traits::CryptoPort;


#[derive(ZeroizeOnDrop)]
pub struct AesGcmCrypto {
    key: [u8; 32],
}

impl AesGcmCrypto {
    pub fn new(password: &str, salt: &[u8]) -> Self {
        let key = Self::derive_key(password, salt);
        Self { key }
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
    fn encrypt(&self, plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let key = Key::<Aes256Gcm>::from(self.key.clone());
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .expect("Encryption failed");
        (ciphertext, nonce.to_vec())
    }

    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Vec<u8> {
        let key = Key::<Aes256Gcm>::from(self.key.clone());
        let cipher = Aes256Gcm::new(&key);
        let nonce_array: [u8; 12] = nonce.try_into().expect("Nonce must have 12 bytes!");
        let plaintext = cipher
            .decrypt(&Nonce::from(nonce_array), ciphertext)
            .expect("Decryption failed");
        plaintext
    }

    fn salt_gen() -> [u8; 16] {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        salt
    }
}
