use anyhow::{Context, Result};
use rustyline::{Editor, error::ReadlineError, history::DefaultHistory};
use std::io::{self, Write};
use zeroize::Zeroize;

use crate::{
    application::engine::VaultEngine,
    domain::ports::{CryptoPort, StoragePort},
};

const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

enum Command {
    Lock,
    List,
    Help,
    Exit,
    Commit,
    Get(String),
    Unlock(String),
    Create(String),
    Remove(String),
    Add { service: String, username: String },
}

pub struct VaultCli<S: StoragePort, C: CryptoPort> {
    engine: VaultEngine<S, C>,
    rl: Editor<(), DefaultHistory>,
}

impl<S: StoragePort, C: CryptoPort> VaultCli<S, C> {
    pub fn new(engine: VaultEngine<S, C>) -> Result<Self> {
        Ok(Self {
            engine,
            rl: Editor::<(), DefaultHistory>::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        println!("--- Vault CLI v1.1 ---");
        println!("Type 'help' to see available commands.\n");

        loop {
            let prompt = self.prompt();
            let line = match self.rl.readline(&prompt) {
                Ok(line) => {
                    self.rl.add_history_entry(line.as_str())?;
                    line
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("exit");
                    break;
                }
                Err(err) => return Err(err.into()),
            };

            let input = line.trim();
            if input.is_empty() {
                continue;
            }

            let cmd = match self.parse_command(input) {
                Some(cmd) => cmd,
                None => {
                    println!("Unknown command. Type 'help'.");
                    continue;
                }
            };

            if let Command::Exit = cmd {
                println!("Closing vault CLI.");
                break;
            }

            if let Err(e) = self.handle_command(cmd) {
                eprintln!("Error: {:#}", e);
            }
        }

        Ok(())
    }

    // =========================
    // Parsing
    // =========================

    fn parse_command(&self, input: &str) -> Option<Command> {
        let mut parts = input.split_whitespace();
        let cmd = parts.next()?;

        match cmd {
            "unlock" => Some(Command::Unlock(parts.next()?.to_string())),
            "lock" => Some(Command::Lock),
            "ls" | "list" => Some(Command::List),
            "get" => Some(Command::Get(parts.next()?.to_string())),
            "create" => Some(Command::Create(parts.next()?.to_string())),
            "add" => Some(Command::Add {
                service: parts.next()?.to_string(),
                username: parts.next()?.to_string(),
            }),
            "commit" => Some(Command::Commit),
            "rm" | "remove" => Some(Command::Remove(parts.next()?.to_string())),
            "exit" | "quit" => Some(Command::Exit),
            "help" => Some(Command::Help),
            _ => None,
        }
    }

    // =========================
    // Command handling
    // =========================

    fn handle_command(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Unlock(vault) => {
                let mut password = self.request_password("Vault password: ");
                let res = self
                    .engine
                    .unlock(&vault, &password)
                    .with_context(|| format!("Could not unlock vault '{}'", vault));
                password.zeroize();
                res?;
                println!("Vault '{}' unlocked.", vault);
            }

            Command::Create(name) => {
                let mut password = self.request_password("New vault password: ");
                let res = self
                    .engine
                    .create_vault(&name, &password)
                    .context("Failed to create vault");
                password.zeroize();
                res?;
                println!("Vault '{}' created.", name);
            }

            Command::Add { service, username } => {
                let mut password = self.request_password("Service password: ");
                self.engine
                    .add(&service, &username, &password)
                    .with_context(|| format!("Failed to add '{}'", service))?;
                password.zeroize();
                println!("Entry '{}' added to memory.", service);
            }

            Command::Commit => {
                self.engine.commit().context("Failed to persist data")?;
                println!("Changes committed to disk.");
            }

            Command::List => self.handle_list()?,

            Command::Get(service) => {
                let entry = self
                    .engine
                    .get(&service)
                    .with_context(|| format!("Service '{}' not found", service))?;
                println!(
                    "[{}]\n  user: {}\n  pass: {}",
                    entry.service, entry.username, entry.passwd
                );
            }

            Command::Remove(service) => {
                if !self.confirm(&format!("Remove '{}'?", service)) {
                    println!("Aborted.");
                    return Ok(());
                }
                self.engine
                    .delete(&service)
                    .with_context(|| format!("Failed to remove '{}'", service))?;
                println!("Entry '{}' removed.", service);
            }

            Command::Lock => {
                self.engine.lock().context("Failed to lock vault")?;
                println!("Vault locked and memory wiped.");
            }

            Command::Help => Self::print_help(),

            Command::Exit => unreachable!(),
        }

        Ok(())
    }

    // =========================
    // Helpers
    // =========================

    fn prompt(&self) -> String {
        if self.engine.is_locked() {
            format!("{BLUE}vault{RESET}[{RED}locked{RESET}]> ",)
        } else {
            format!("{BLUE}vault{RESET}[{GREEN}unlocked{RESET}]> ",)
        }
    }

    fn request_password(&self, label: &str) -> String {
        print!("{}", label);
        io::stdout().flush().unwrap();
        rpassword::read_password().unwrap()
    }

    fn confirm(&self, msg: &str) -> bool {
        print!("{} (y/N): ", msg);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        matches!(input.trim(), "y" | "Y" | "yes" | "YES")
    }

    fn handle_list(&self) -> Result<()> {
        if self.engine.is_locked() {
            let vaults = self.engine.get_vaults().context("Failed to list vaults")?;
            println!("Vaults on disk:");
            if vaults.is_empty() {
                println!("  (none)");
            }
            for v in vaults {
                println!("  - {}", v);
            }
        } else {
            let entries = self
                .engine
                .get_entries()
                .context("Failed to list entries")?;
            println!("Entries:");
            if entries.is_empty() {
                println!("  (empty)");
            }
            for e in entries {
                println!("  - {}", e);
            }
        }
        Ok(())
    }

    fn print_help() {
        println!(
            r#"
AVAILABLE COMMANDS:

  create <name>            Create a new vault
  unlock <name>            Unlock an existing vault
  lock                     Lock vault and wipe memory

  add <service> <user>     Add entry (password prompted securely)
  get <service>            Show entry details
  rm <service>             Remove entry (with confirmation)
  commit                   Persist changes to disk

  ls | list                List vaults or entries
  help                     Show this message
  exit | quit              Exit the application
"#
        );
    }
}
