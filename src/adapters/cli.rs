use crate::{domain::commands::Command};

pub struct Cli<E> {
    engine: E
}

impl<E> Cli<E> {

}







pub fn parse_command(input: &str) -> Option<Command> {

    let mut parts = input.trim().split_whitespace();
    let cmd  = parts.next()?;

    match cmd {
        "unlock" => Some(Command::Unlock),
        "lock" => Some(Command::Lock),
        "list" => Some(Command::List),
        "add" => {
            let service: String = parts.next()?.to_string();
            let username: String = parts.next()?.to_string();
            let password: String = parts.next()?.to_string();
            Some(Command::Add { service, username, password })
        },
        "edit" => None,
        "remove" => Some(Command::Remove(parts.next()?.to_string())),
        "exit" => Some(Command::Exit),
        _ => None
    }
}
