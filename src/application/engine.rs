use crate::{application::engine_trait::EngineTrait, domain::ports::{CryptoPort, StoragePort}};

pub struct VaultEngine<S: StoragePort, C:CryptoPort> {
    locked: bool,
    storage: S,
    crypto: C,
}

impl<S: StoragePort, C: CryptoPort> VaultEngine<S, C> {
    pub fn new(storage: S, crypto: C) -> Self {
        Self {
            locked: false,
            storage: storage,
            crypto: crypto,
        }
    }
} 

impl<S: StoragePort, C: CryptoPort> EngineTrait for VaultEngine<S, C> {
    fn add_entry(&self, entry: &str) -> Result<(), String> {
        todo!()
    }
    
    fn delete_entry(&self, id: usize) -> Result<(), String> {
        todo!()
    }
    
    fn get_entry(&self, id: usize) -> Result<String, String> {
        todo!()
    }
    
    fn update_entry(&self, id: usize, entry: &str) -> Result<(), String> {
        todo!()
    }    
}
