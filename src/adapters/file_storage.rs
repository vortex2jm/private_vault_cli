use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, create_dir_all},
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use crate::domain::{errors::StorageError, ports::StoragePort};

pub struct FileStorage {
    base_path: PathBuf,
    path: PathBuf,
}

impl FileStorage {
    pub fn new() -> Self {
        let home = dirs_2::home_dir().expect("Could not find home dir!");
        let base_path = home.join(".vault");

        let path = base_path.join("default.vault");
        Self {
            path: path,
            base_path: base_path,
        }
    }

    fn hash_file(path: &PathBuf) -> Result<Vec<u8>, StorageError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        std::io::copy(&mut reader, &mut hasher)?;
        Ok(hasher.finalize().to_vec())
    }
}

impl StoragePort for FileStorage {
    fn set_path(&mut self, mut path: String) {
        path.push_str(".vault");
        self.path = self.base_path.join(path);
    }

    fn save(&self, data: &[u8]) -> Result<(), StorageError> {
        // Checks dir
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent)?;
        }

        // Create backup file
        let backup_path = self.path.with_extension("bkp");
        if self.path.exists() {
            std::fs::copy(&self.path, &backup_path)?;

            // Valida integridade do backup
            let orig_hash = FileStorage::hash_file(&self.path)?;
            let bkp_hash = FileStorage::hash_file(&backup_path)?;

            if orig_hash != bkp_hash {
                return Err(StorageError::IntegrityError);
            }
        }

        let mut file = File::create(&self.path)?;

        if let Err(_) = file.write_all(data).and_then(|_| file.sync_all()) {
            // Se falhar, restaura backup
            if backup_path.exists() {
                std::fs::copy(&backup_path, &self.path)?;
            }
            return Err(StorageError::IntegrityError);
        }

        Ok(())
    }

    fn load(&self) -> Result<std::vec::Vec<u8>, StorageError> {
        let mut file = File::open(&self.path)?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn list_vaults(&self) -> Result<Vec<String>, StorageError> {
        let home = dirs_2::home_dir()
            .ok_or(StorageError::DirNotFound("Could not found home dir".into()))?;

        let vault_dir = home.join(".vault");

        if !vault_dir.exists() {
            return Ok(Vec::new());
        }

        let mut vaults = Vec::new();

        for entry in fs::read_dir(&vault_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Filtra apenas arquivos com extensão ".vault"
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("vault") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    vaults.push(stem.to_string());
                }
            }
        }

        Ok(vaults)
    }
}
