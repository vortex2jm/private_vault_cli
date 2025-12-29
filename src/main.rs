use crate::{
    adapters::{aes_crypto::AesGcmCrypto, file_storage::FileStorage},
    application::engine::VaultEngine,
    domain::commands::Command,
    utils::parser::{self},
};

use zeroize::Zeroize;

mod adapters;
mod application;
mod domain;
mod utils;

fn main() {
    let storage = FileStorage::new();
    let crypto = AesGcmCrypto::new();
    let mut vault_engine = VaultEngine::new(storage, crypto);

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
                        .expect("Could not unlock the Vault!");

                    password.zeroize();
                }

                Command::Lock => {
                    vault_engine.lock().expect("Erro ao bloquear cofre");
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
                        let vaults = vault_engine.get_vaults().expect("get vaults error");

                        println!("AVAILABLE VAULTS");
                        for vault in vaults {
                            println!("-> {}", vault);
                        }
                        continue;
                    }

                    let entries = vault_engine.get_entries().expect("get error!");
                    for entry in entries {
                        println!("-> {}", entry);
                    }
                }

                Command::Get(service) => {
                    let entry = vault_engine.get(&service).expect("get single error!");
                    println!(
                        "service: {} - user: {} - password: {}",
                        entry.service, entry.username, entry.passwd
                    );
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
