use crate::{
    adapters::{aes_crypto::AesGcmCrypto, file_storage::FileStorage},
    application::engine::VaultEngine,
    domain::commands::Command, utils::parser,
};

mod adapters;
mod application;
mod domain;
mod utils;

fn main() {
    // TODO: Implementar entrada de argumentos
    // Se o arquivo for apontado, encaminhar direto para a CLI
    // Se nao for apontado, procurar por arquivos no diretório .vault
    // Se nao houver nenhum, mensagem para criar um novo vault
    // Fazer fluxo de criar um novo vault (ele ja começa desbloqueado)

    let storage = FileStorage::new().custom_path("passwd.vault".into());
    let crypto = AesGcmCrypto::new();

    let mut vault_engine = VaultEngine::new(storage, crypto);
    vault_engine
        .unlock("oi")
        .expect("Não foi possível desbloquear o cofre!");

    loop {
        print!("vault> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if let Some(cmd) = parser::parse_command(&input) {
            match cmd {
                Command::Lock => {
                    vault_engine.lock().expect("Erro ao bloquear cofre");
                    std::process::exit(0);
                }
                _ => {}
            }
        } else {
            print!("Unkown command. Type help to see the options")
        }
    }
}
