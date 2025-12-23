use crate::{adapters::{outbound::aes_crypto::AesGcmCrypto, inbound::cli::parse_command, outbound::file_storage::FileStorage}, domain::ports::{CryptoPort, StoragePort}};
use zeroize::Zeroize;

mod adapters;
mod application;
mod domain;

fn main() {

    loop {
        print!(">vault: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if let Some(cmd) = parse_command(&input) {
            
        } else {
            print!("Unkown command. Type help to see the options")
        }
    }

    let mut password = "joao".to_string();
    let salt = AesGcmCrypto::salt_gen();
    let crypto = AesGcmCrypto::new(&password, &salt);

    // Erasing password bytes on memory
    password.zeroize();
    
    // file storage instance    
    let storage = FileStorage::new().custom_path("./passwd.vault".into());

    // Enctrypt
    let plaintext = "This is the plaintext".to_string();
    let (ciphertext, nonce) = crypto.encrypt(plaintext.as_bytes());

    
    storage.save(&ciphertext).unwrap();

    // Decrypt
    let decrypted_text = String::from_utf8(crypto.decrypt(&ciphertext, &nonce))
        .expect("Could not decode byte stream!\n");

    println!("Mensagem desencriptada: {}", decrypted_text);
}
