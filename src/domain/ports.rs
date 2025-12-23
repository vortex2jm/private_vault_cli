pub trait CryptoPort {
    fn salt_gen() -> [u8; 16];
    fn encrypt(&self, plaintext: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Vec<u8>;
}

pub trait StoragePort {
    fn save(&self, data: &[u8]) -> Result<(), String>;
    fn load(&self) -> Result<Vec<u8>, String>;
    fn exists(&self) -> bool;
}
