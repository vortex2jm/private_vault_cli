use crate::{
    adapters::{aes_crypto::AesGcmCrypto, file_storage::FileStorage},
    application::engine::VaultEngine,
    domain::commands::Command,
    utils::parser::{self},
};

use clap::Parser;
use zeroize::Zeroize;

mod adapters;
mod application;
mod domain;
mod utils;

#[derive(Debug, Parser)]
#[command(name = "vault")]
#[command(about="Vault CLI", long_about=None)]
pub struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let storage = FileStorage::new();
    let crypto = AesGcmCrypto::new();
    let mut vault_engine = VaultEngine::new(storage, crypto);

    // Unlock logic
    if let Some(path) = args.file {
        print!("password: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut password = rpassword::read_password().unwrap();

        vault_engine
            .unlock(&path, &password)
            .expect("Não foi possível desbloquear o cofre");
        password.zeroize();
    }

    loop {
        print!("vault> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if let Some(cmd) = parser::parse_command(&input) {
            match cmd {
                Command::Unlock(vault) => {
                    let mut password = parser::request_password();

                    vault_engine
                        .unlock(&vault, &password)
                        .expect("Não foi possível desbloquear o cofre!");

                    password.zeroize();
                }

                Command::Lock => {
                    vault_engine.lock().expect("Erro ao bloquear cofre");
                    std::process::exit(0);
                }

                Command::Create(name) => {
                    let mut password = parser::request_password();
                    vault_engine.create_vault(&name, &password);
                    password.zeroize();
                }

                Command::Add {
                    service,
                    username,
                    password,
                } => {
                    vault_engine
                        .add(&service, &username, &password)
                        .expect("Erro ao adicionar entrada!");
                }

                Command::Commit => {
                    // let mut password = parser::request_password();
                    vault_engine.commit().expect("Erro ao commitar vault");
                }

                Command::List => {
                    if vault_engine.is_locked() {
                        let vaults = vault_engine.get_vaults();
                        // print vaults
                        return;
                    }

                    let entries = vault_engine.get_entries();
                    // print entries
                }

                Command::Remove(service) => {
                    vault_engine
                        .delete(&service)
                        .expect("Erro ao deletar entrada!");
                }

                Command::Help => {
                    println!("Help command");
                }

                Command::Exit => break,
            }
        } else {
            println!("Unkown command. Type help to see the options")
        }
    }
}
