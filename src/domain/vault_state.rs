use serde::{Deserialize, Serialize};
use wincode::{SchemaRead, SchemaWrite};

#[derive(Serialize, Deserialize, Clone, SchemaWrite, SchemaRead)]
pub struct VaultState {
    pub salt: [u8; 16],
	pub nonce: [u8; 12],
	pub cipher: Vec<u8>,
}
