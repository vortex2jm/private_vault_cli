use std::collections::BTreeMap;

use zeroize::Zeroize;

use crate::domain::{
    entry::Entry,
    errors::VaultError,
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

    pub fn create_vault(&mut self, name: &str, password: &str) -> Result<(), VaultError> {
        if !self.is_locked() {
            return Err(VaultError::Unlocked);
        }

        let salt = self.crypto.salt_gen();
        self.vault_state = Some(VaultState::new(&salt));

        self.storage.set_path(name.into());
        self.crypto.init(password, &salt)?;

        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), VaultError> {
        let vault_state = self.vault_state.as_mut().ok_or(VaultError::Locked)?;

        let mut entries_buffer = Vec::new();
        wincode::serialize_into(&mut entries_buffer, &self.entries)
            .map_err(|_| VaultError::Serialization)?;

        let (cipher, nonce) = self.crypto.encrypt(&entries_buffer)?;

        vault_state.cipher = cipher;
        vault_state.nonce = nonce;

        let mut vault_buffer = Vec::new();
        wincode::serialize_into(&mut vault_buffer, vault_state)
            .map_err(|_| VaultError::Serialization)?;

        self.storage.save(&vault_buffer)?;

        Ok(())
    }

    pub fn unlock(&mut self, vault: &str, password: &str) -> Result<(), VaultError> {
        self.storage.set_path(vault.into());

        if !self.storage.exists() {
            return Err(VaultError::VaultNotFound);
        }

        // Do not unlock if it's already unlocked
        if !self.is_locked() {
            return Err(VaultError::Unlocked);
        }

        // Load file bytes
        let buffer = self.storage.load()?;

        // Deserialize into vault state
        let v_state: VaultState = wincode::deserialize_from(&mut buffer.as_slice())
            .map_err(|_| VaultError::Serialization)?;
        self.vault_state = Some(v_state.clone());

        // Derive key
        self.crypto.init(password, &v_state.salt)?;

        // Decrypt entries
        // let v_state = self.vault_state.as_ref().ok_or(VaultError::Locked)?;

        let stream = self
            .crypto
            .decrypt(&v_state.cipher, &v_state.nonce)
            .map_err(|_| {
                self.vault_state = None;    //Invalid password, keeps vault locked
                VaultError::InvalidPassword
            });

        // Deserialize entries into BTreeMap
        self.entries = wincode::deserialize_from(&mut stream?.as_slice())
            .map_err(|_| VaultError::Serialization)?;

        Ok(())
    }

    pub fn lock(&mut self) -> Result<(), VaultError> {
        if self.is_locked() {
            return Err(VaultError::Locked);
        }

        for entry in self.entries.values_mut() {
            entry.zeroize();
        }

        self.entries.clear();
        self.vault_state = None;

        Ok(())
    }

    pub fn add(&mut self, service: &str, username: &str, password: &str) -> Result<(), VaultError> {
        if self.is_locked() {
            return Err(VaultError::Locked);
        }

        let entry = Entry::new(service.into(), username.into(), password.into());

        if self.entries.contains_key(service) {
            return Err(VaultError::EntryExists);
        }

        self.entries.insert(service.into(), entry);

        Ok(())
    }

    pub fn delete(&mut self, service: &str) -> Result<Entry, VaultError> {
        if self.is_locked() {
            return Err(VaultError::Locked);
        }
        self.entries
            .remove(service)
            .ok_or(VaultError::EntryNotFound)
    }

    pub fn get(&self, service: &str) -> Result<&Entry, VaultError> {
        if self.is_locked() {
            return Err(VaultError::Locked);
        }
        self.entries
            .get(service)
            .ok_or(VaultError::EntryNotFound)
    }

    pub fn get_entries(&self) -> Result<Vec<String>, VaultError> {
        if self.is_locked() {
            return Err(VaultError::Locked);
        }
        Ok(self.entries.keys().cloned().collect())
    }

    pub fn get_vaults(&self) -> Result<Vec<String>, VaultError> {
        let vaults = self.storage.list_vaults()?;
        Ok(vaults)
    }
}
