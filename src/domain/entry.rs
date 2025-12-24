use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wincode::{SchemaRead, SchemaWrite};
use zeroize::Zeroize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, SchemaWrite, SchemaRead, Zeroize)]
pub struct Entry {
    id: [u8; 16],
    service: String,
    username: String,
    passwd: String,
    created_at: i64,
    updated_at: i64,
}

impl Entry {
    fn new(service: String, username: String, passwd: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: *Uuid::new_v4().as_bytes(),
            service,
            username,
            passwd,
            created_at: now,
            updated_at: now,
        }
    }
}
