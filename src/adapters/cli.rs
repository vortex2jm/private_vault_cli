use anyhow::{Context, Result};
use std::io::{self, Write};
use zeroize::Zeroize;

use crate::{
    application::{commands::Command, engine::VaultEngine},
    domain::ports::{CryptoPort, StoragePort},
    utils::parser,
};

pub struct VaultCli<S: StoragePort, C: CryptoPort> {
    engine: VaultEngine<S, C>,
}

impl<S: StoragePort, C: CryptoPort> VaultCli<S, C> {
    pub fn new(engine: VaultEngine<S, C>) -> Self {
        Self { engine }
    }

    /// Main entry point for the CLI loop
    pub fn run(&mut self) -> Result<()> {
        println!("--- Vault CLI v1.0 ---");

        loop {
            self.flush_prompt()?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .context("Failed to read user input from stdin")?;

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            // Parse commands
            let cmd = match parser::parse_command(input) {
                Some(c) => c,
                None => {
                    println!("Unknown command. Type 'help' to see available options.");
                    continue;
                }
            };

            // Exit is the only command that breaks the loop directly
            if let Command::Exit = cmd {
                println!("👋 Goodbye!");
                break;
            }

            // Process the command and capture any errors
            if let Err(e) = self.handle_command(cmd) {
                // Anyhow's {:?} prints the full error chain/context
                eprintln!("Error: {:?}", e);
            }
        }

        Ok(())
    }

    /// Command dispatcher (returns Result to enable use of the '?' operator)
    fn handle_command(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Unlock(vault) => {
                let mut password = parser::request_password();
                let res = self.engine.unlock(&vault, &password).with_context(|| {
                    format!(
                        "Could not open vault '{}'. Please check the name and password.",
                        vault
                    )
                });
                password.zeroize();
                res?; // Propagate error after clearing sensitive data
                println!("🔓 Vault '{}' unlocked successfully!", vault);
            }

            Command::Create(name) => {
                let mut password = parser::request_password();
                let res = self
                    .engine
                    .create_vault(&name, &password)
                    .context("Failed to create the new vault");
                password.zeroize();
                res?;
                println!("✅ Vault '{}' created successfully.", name);
            }

            Command::Add {
                service,
                username,
                password,
            } => {
                self.engine
                    .add(&service, &username, &password)
                    .with_context(|| format!("Failed to add service '{}' to memory.", service))?;
                println!("➕ Entry for '{}' added to memory buffer.", service);
            }

            Command::Commit => {
                self.engine
                    .commit()
                    .context("Critical failure while persisting data to disk")?;
                println!("💾 Changes successfully committed to disk!");
            }

            Command::List => self.handle_list()?,

            Command::Get(service) => {
                let entry = self.engine.get(&service).with_context(|| {
                    format!("Service '{}' not found in the current vault.", service)
                })?;
                println!(
                    "🔑 [{}] User: {} | Pass: {}",
                    entry.service, entry.username, entry.passwd
                );
            }

            Command::Remove(service) => {
                self.engine.delete(&service).with_context(|| {
                    format!("Error while removing service '{}' from memory.", service)
                })?;
                println!("🗑️ Entry '{}' removed from memory.", service);
            }

            Command::Lock => {
                self.engine
                    .lock()
                    .context("Error while locking the vault")?;
                println!("🔒 Vault locked and memory cleared.");
            }

            Command::Help => Self::print_help(),

            Command::Exit => unreachable!(), // Handled in the main loop
        }

        Ok(())
    }

    // --- Support Helpers ---

    fn handle_list(&self) -> Result<()> {
        if self.engine.is_locked() {
            let vaults = self
                .engine
                .get_vaults()
                .context("Error listing vault files from storage")?;
            println!("AVAILABLE VAULTS ON DISK:");
            if vaults.is_empty() {
                println!("  (No vaults found)");
            }
            for v in vaults {
                println!("  -> {}", v);
            }
        } else {
            let entries = self
                .engine
                .get_entries()
                .context("Error reading entries from memory")?;
            println!("ENTRIES IN CURRENT VAULT:");
            if entries.is_empty() {
                println!("  (Vault is currently empty)");
            }
            for entry in entries {
                println!("  -> {}", entry);
            }
        }
        Ok(())
    }

    fn flush_prompt(&self) -> Result<()> {
        print!("vault> ");
        io::stdout().flush().context("Error flushing stdout buffer")
    }

    fn print_help() {
        println!(
            r#"
AVAILABLE COMMANDS:
  create <name>          Create a new vault file (.vault)
  unlock <name>          Open and decrypt an existing vault
  add <svc> <usr> <pw>   Add an entry (memory only, use commit to save)
  commit                 Persist memory entries to disk
  list                   List available vaults (if locked) or entries (if open)
  get <svc>              Show details of a specific entry
  remove <svc>           Remove an entry from memory
  lock                   Lock current vault and wipe memory
  help                   Show this help message
  exit                   Close the application
        "#
        );
    }
}
