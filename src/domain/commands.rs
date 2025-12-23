pub enum Command {
    Unlock,
    Lock,
    Add {
        service: String,
        username: String,
        password: String,
    },
    List,
    Remove(String),
    Exit
}
