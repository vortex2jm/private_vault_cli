pub enum Command {
    Unlock(String),
    Lock,
    Create(String),
    Add {
        service: String,
        username: String,
        password: String,
    },
    Remove(String),
    List,
    Get(String),
    Commit,
    Help,
    Exit
}
