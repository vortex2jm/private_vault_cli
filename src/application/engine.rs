use std::collections::BTreeMap;

use zeroize::Zeroize;

use crate::domain::{
    entry::Entry,
    ports::{CryptoPort, StoragePort},
    vault_state::VaultState,
};

pub struct VaultEngine<S: StoragePort, C: CryptoPort> {
    storage: S,
    crypto: C,
    vault_state: Option<VaultState>,
    entries: BTreeMap<String, Entry>,
}

impl<S: StoragePort, C: CryptoPort> VaultEngine<S, C> {
    pub fn new(storage: S, crypto: C) -> Self {
        Self {
            storage: storage,
            crypto: crypto,
            vault_state: None,
            entries: BTreeMap::new(),
        }
    }

    pub fn unlock(&mut self, password: &str) -> Result<(), String> {
        if !self.storage.exists() {
            return Err("Create a new vault before trying unlock it!".into());
        }

        if self.vault_state.is_some() {
            return Err("Vault already unlocked".to_string());
        }

        // Loads file bytes
        let buffer = self.storage.load().expect("Deu merda no buffer");

        // Deserialize into vault state
        let v_state: VaultState =
            wincode::deserialize_from(&mut buffer.as_slice()).expect("Deserialize error");
        self.vault_state = Some(v_state.clone());

        // Derive key
        self.crypto.init(password, &v_state.salt);

        // Decrypt entries
        let v_state = self
            .vault_state
            .as_ref()
            .expect("Cofre nao deveria estar bloqueado aqui");
        let stream = self.crypto.decrypt(&v_state.cipher, &v_state.nonce);

        // Deserialize entries into BTreeMap
        self.entries = wincode::deserialize_from(&mut stream?.as_slice())
            .expect("erro ao desserializar entradas");

        Ok(())
    }

    pub fn lock(&mut self) -> Result<(), String> {
        if self.vault_state.is_none() {
            return Err("Vault already locked!".into());
        }

        for entry in self.entries.values_mut() {
            entry.zeroize();
        }
        self.entries.clear();
        self.vault_state = None;

        Ok(())
    }

    pub fn add(&self, entry: &str) -> Result<(), String> {
        todo!()
    }

    pub fn delete(&self, id: usize) -> Result<(), String> {
        todo!()
    }

    pub fn get(&self, id: usize) -> Result<String, String> {
        todo!()
    }

    pub fn get_all() -> Result<Vec<String>, String> {
        todo!()
    }

    pub fn update(&self, id: usize, entry: &str) -> Result<(), String> {
        todo!()
    }
}
