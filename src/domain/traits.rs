pub trait CryptoPort {
    fn encrypt(&self, plaintext: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Vec<u8>;
    fn salt_gen() -> [u8; 16];
}

pub trait StoragePort {
}

pub trait EntryManager {
    fn add_entry(&self, entry: &str) -> Result<(), String>;
    fn get_entry(&self, id: usize) -> Result<String, String>;
    fn delete_entry(&self, id: usize) -> Result<(), String>;
    fn update_entry(&self, id: usize, entry: &str) -> Result<(), String>;
}
