pub trait CryptoPort {
    fn salt_gen(&self) -> [u8; 16];
    fn init(&mut self, password: &str, salt: &[u8]);
    fn encrypt(&self, plaintext: &[u8]) -> Result<(Vec<u8>, [u8; 12]), String>;
    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String>;
}

pub trait StoragePort {
    fn list_vaults(&self) -> Result<Vec<String>, String>;
    fn exists(&self) -> bool;
    fn set_path(&mut self, path: String);
    fn load(&self) -> Result<Vec<u8>, String>;
    fn save(&self, data: &[u8]) -> Result<(), String>;
}
