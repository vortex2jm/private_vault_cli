use sha2::{Digest, Sha256};
use std::{
    fs::{File, create_dir_all},
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use crate::domain::traits::StoragePort;

pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    pub fn new() -> Self {
        let home = dirs_2::home_dir().expect("Could not find home dir!");
        let path = home.join(std::env::var("VAULT_PATH").unwrap_or(".vault/vault.bin".into()));
        Self { path }
    }

    pub fn custom_path(mut self, path: String) -> Self {
        self.path = path.into();
        self
    }

    pub fn get_path(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }

    fn hash_file(path: &PathBuf) -> Result<Vec<u8>, String> {
        let file = File::open(path).map_err(|e| format!("Open file error: {}", e))?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        std::io::copy(&mut reader, &mut hasher).map_err(|e| format!("Hash copy error: {}", e))?;
        Ok(hasher.finalize().to_vec())
    }
}

impl StoragePort for FileStorage {
    fn save(&self, data: &[u8]) -> Result<(), String> {
        // Checks dir
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent).map_err(|e| format!("Dir error: {}", e))?;
        }

        // Create backup file
        let backup_path = self.path.with_extension("bkp");
        if self.path.exists() {
            std::fs::copy(&self.path, &backup_path)
                .map_err(|e| format!("Temporary backup error: {}", e))?;    //TODO: error in first try

            // Valida integridade do backup
            let orig_hash = FileStorage::hash_file(&self.path)?;
            let bkp_hash = FileStorage::hash_file(&backup_path)?;
            if orig_hash != bkp_hash {
                return Err("Backup integrity check failed!".into());
            }
        }

        let mut file = File::create(&self.path).map_err(|e| format!("File create error: {}", e))?;
        
        if let Err(e) = file.write_all(data).and_then(|_| file.sync_all()) {
            // Se falhar, restaura backup
            if backup_path.exists() {
                std::fs::copy(&backup_path, &self.path)
                    .map_err(|e2| format!("Restore backup failed: {e2}"))?;
            }
            return Err(format!("Write error: {e}"));
        }

        Ok(())
    }

    fn load(&self) -> Result<Vec<u8>, String> {
        let mut file = File::open(&self.path)
            .map_err(|e| format!("File open error: {e}"))?;
        
        let mut buffer = Vec::new();
        
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Read error: {e}"))?;
        
        Ok(buffer)
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }
}
