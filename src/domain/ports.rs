pub trait CryptoPort {
    fn salt_gen(&self) -> [u8; 16];
    fn init(&mut self, password: &str, salt: &[u8]);
    fn encrypt(&self, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String>;
    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String>;
}

pub trait StoragePort {
    fn save(&self, data: &[u8]) -> Result<(), String>;
    fn load(&self) -> Result<Vec<u8>, String>;
    fn exists(&self) -> bool;
}
