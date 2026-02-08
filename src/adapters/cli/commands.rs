pub enum Command {
    Lock,
    List,
    Help,
    Exit,
    Commit,
    Clear,
    Get(String),
    Unlock(String),
    Create(String),
    Remove(String),
    Add { service: String, username: String },
}
