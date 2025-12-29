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

    pub fn is_locked(&self) -> bool {
        self.vault_state.is_none()
    }

    pub fn create_vault(&mut self, name: &str, password: &str) -> Result<(), String> {

        if !self.is_locked() {
            return Err("Lock the current vault before creating a new one!".into());
        }

        let salt = self.crypto.salt_gen();
        self.vault_state = Some(VaultState::new(&salt));

        self.storage.set_path(name.into());
        self.crypto.init(password, &salt);

        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), String> {
        let vault_state = self.vault_state.as_mut().ok_or("Vault locked")?;

        let mut entries_buffer: Vec<u8> = Vec::new();
        wincode::serialize_into(&mut entries_buffer, &self.entries)
            .expect("Error on entries serialization");

        let (cipher, nonce) = self.crypto.encrypt(&entries_buffer)?;

        vault_state.cipher = cipher;
        vault_state.nonce = nonce.into();

        let mut vault_buffer: Vec<u8> = Vec::new();
        wincode::serialize_into(&mut vault_buffer, vault_state).expect("Error on vault serialization");

        self.storage
            .save(&vault_buffer)
            .expect("Error on vault commit");

        Ok(())
    }

    pub fn unlock(&mut self, vault: &str, password: &str) -> Result<(), String> {
        self.storage.set_path(vault.into());

        if !self.storage.exists() {
            return Err("This Vault does not exists!".into());
        }

        if !self.is_locked() {
            return Err("This Vault is already unlocked".to_string());
        }

        // Loads file bytes
        let buffer = self.storage.load().expect("Error on unlock buffer loading");

        // Deserialize into vault state
        let v_state: VaultState =
            wincode::deserialize_from(&mut buffer.as_slice()).expect("Error on vault deserialization");
        self.vault_state = Some(v_state.clone());

        // Derive key
        self.crypto.init(password, &v_state.salt);

        // Decrypt entries
        let v_state = self
            .vault_state
            .as_ref()
            .expect("The vault could not be locked");

        let stream = self.crypto.decrypt(&v_state.cipher, &v_state.nonce);

        // Deserialize entries into BTreeMap
        self.entries = wincode::deserialize_from(&mut stream?.as_slice())
            .expect("Error on entries deserialization");

        Ok(())
    }

    pub fn lock(&mut self) -> Result<(), String> {
        if self.is_locked() {
            return Err("This Vault is already locked!".into());
        }

        for entry in self.entries.values_mut() {
            entry.zeroize();
        }
        self.entries.clear();
        self.vault_state = None;

        Ok(())
    }

    pub fn add(&mut self, service: &str, username: &str, password: &str) -> Result<(), String> {
        if self.is_locked() {
            return Err("The vault is blocked!".into());
        }

        let entry = Entry::new(service.into(), username.into(), password.into());

        if self.entries.contains_key(service) {
            return Err("This entry already exists".into());
        }

        self.entries.insert(service.into(), entry);

        Ok(())
    }

    pub fn delete(&mut self, service: &str) -> Result<Entry, String> {
        self.entries
            .remove(service)
            .ok_or_else(|| "Entry not found".to_string())
    }

    pub fn get(&self, service: &str) -> Result<&Entry, String> {
        self.entries
            .get(service)
            .ok_or_else(|| "Entry not found".into())
    }

    pub fn get_entries(&self) -> Result<Vec<String>, String> {
        if self.is_locked() {
            return Err("The vault is locked!".into());
        }
        Ok(self.entries.keys().cloned().collect())
    }

    pub fn get_vaults(&self) -> Result<Vec<String>, String> {
        self.storage.list_vaults()
    }    
}
