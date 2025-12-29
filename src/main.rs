use crate::{
    adapters::{aes_crypto::AesGcmCrypto, cli::VaultCli, file_storage::FileStorage},
    application::engine::VaultEngine,
};

mod adapters;
mod application;
mod domain;
mod utils;

fn main() {
    let storage = FileStorage::new();
    let crypto = AesGcmCrypto::new();
    let engine = VaultEngine::new(storage, crypto);
    let mut cli = VaultCli::new(engine);

    cli.run();

    // REPL
    // loop {
    //     print!("vault> ");
    //     std::io::Write::flush(&mut std::io::stdout()).unwrap();

    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();

    //     // Commands interpreter
    //     if let Some(cmd) = parser::parse_command(&input) {
    //         match cmd {
    //             Command::Unlock(vault) => {
    //                 let mut password = parser::request_password();
    //                 vault_engine
    //                     .unlock(&vault, &password)
    //                     .expect("Could not unlock the Vault!");
    //                 password.zeroize();
    //             }

    //             Command::Lock => {
    //                 vault_engine.lock().expect("Could not lock the Vault!");
    //             }

    //             Command::Create(name) => {
    //                 let mut password = parser::request_password();
    //                 vault_engine
    //                     .create_vault(&name, &password)
    //                     .expect("Could not create the Vault!");
    //                 password.zeroize();
    //             }

    //             Command::Add {
    //                 service,
    //                 username,
    //                 password,
    //             } => {
    //                 vault_engine
    //                     .add(&service, &username, &password)
    //                     .expect("Could not add entry to Vault!");
    //             }

    //             Command::Commit => {
    //                 vault_engine.commit().expect("Could not commit!");
    //             }

    //             Command::List => {
    //                 if vault_engine.is_locked() {
    //                     let vaults = vault_engine.get_vaults().expect("Could not get avaiable Vaults");

    //                     println!("AVAILABLE VAULTS");
    //                     for vault in vaults {
    //                         println!("-> {}", vault);
    //                     }
    //                     continue;
    //                 }

    //                 let entries = vault_engine.get_entries().expect("Could not get Vault entries!");
    //                 for entry in entries {
    //                     println!("-> {}", entry);
    //                 }
    //             }

    //             Command::Get(service) => {
    //                 let entry = vault_engine.get(&service).expect("Could not get this entry");
    //                 println!(
    //                     "service: {} - user: {} - password: {}",
    //                     entry.service, entry.username, entry.passwd
    //                 );
    //             }

    //             Command::Remove(service) => {
    //                 vault_engine
    //                     .delete(&service)
    //                     .expect("Could not delete entry");
    //             }

    //             Command::Help => {
    //                 println!("Help command");
    //             }

    //             Command::Exit => break,
    //         }
    //     } else {
    //         println!("Unkown command. Type help to see the options")
    //     }
    // }
}
