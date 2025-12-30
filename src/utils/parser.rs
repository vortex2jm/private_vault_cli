use crate::application::commands::Command;

pub fn parse_command(input: &str) -> Option<Command> {
    let mut parts = input.trim().split_whitespace();
    let cmd = parts.next()?;

    match cmd {
        "unlock" => Some(Command::Unlock(parts.next()?.to_string())),
        "lock" => Some(Command::Lock),
        "ls" => Some(Command::List),
        "get" => Some(Command::Get(parts.next()?.to_string())),
        "create" => Some(Command::Create(parts.next()?.to_string())),
        "add" => {
            let service: String = parts.next()?.to_string();
            let username: String = parts.next()?.to_string();
            let password: String = parts.next()?.to_string();
            Some(Command::Add {
                service,
                username,
                password,
            })
        }
        "commit" => Some(Command::Commit),
        // "edit" => None,
        "rm" => Some(Command::Remove(parts.next()?.to_string())),
        "exit" => Some(Command::Exit),
        "help" => Some(Command::Help),
        _ => None,
    }
}

pub fn request_password() -> String {
    print!("password: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let password = rpassword::read_password().unwrap();
    password
}
