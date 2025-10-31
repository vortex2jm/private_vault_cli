use crate::{adapters::aes_crypto::AesGcmCrypto, domain::traits::CryptoPort};
use zeroize::Zeroize;

mod adapters;
mod core;
mod domain;

fn main() {
    let mut password = "joao".to_string();
    let salt = AesGcmCrypto::salt_gen();
    let crypto = AesGcmCrypto::new(&password, &salt);

    // Erasing password bytes on memory
    password.zeroize();

    // Enctrypt
    let plaintext = "This is the plaintext".to_string();
    let (ciphertext, nonce) = crypto.encrypt(plaintext.as_bytes());

    // Decrypt
    let decrypted_text = String::from_utf8(crypto.decrypt(&ciphertext, &nonce))
        .expect("Could not decode byte stream!\n");

    println!("Mensagem desencriptada: {}", decrypted_text);
}
