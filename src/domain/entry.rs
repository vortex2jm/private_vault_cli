use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    id: Uuid,
    service: String,
    username: String,
    passwd: String,
    annotation: Option<String>,
    url: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Entry {
    fn new(
        service: String,
        username: String,
        passwd: String,
        annotation: Option<String>,
        url: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            service,
            username,
            passwd,
            annotation,
            url,
            created_at: now,
            updated_at: now,
        }
    }
}
