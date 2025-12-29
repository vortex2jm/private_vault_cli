use zeroize::Zeroize;

use crate::{
    application::engine::VaultEngine,
    domain::{
        commands::Command,
        ports::{CryptoPort, StoragePort},
    },
    utils::parser,
};

// src/adapters/cli.rs
pub struct VaultCli<S: StoragePort, C: CryptoPort> {
    engine: VaultEngine<S, C>,
}

impl<S: StoragePort, C: CryptoPort> VaultCli<S, C> {
    pub fn new(engine: VaultEngine<S, C>) -> Self {
        Self { engine }
    }

    fn print_help() {
        println!(
            r#"
			COMANDOS DISPONÍVEIS:
			create <nome>          Cria um novo vault
			unlock <nome>          Abre um vault existente
			add <svc> <user> <pw>  Adiciona entrada na memória
			commit                 Salva as alterações no disco
			list                   Lista vaults (se trancado) ou entradas (se aberto)
			get <svc>              Mostra dados de uma entrada
			remove <svc>           Deleta uma entrada
			lock                   Fecha o vault atual
			exit                   Sai do programa
			  "#
        );
    }

    pub fn run(&mut self) {
        println!("--- Vault CLI v1.0 ---");
        loop {
            print!("vault> ");
            if let Err(e) = std::io::Write::flush(&mut std::io::stdout()) {
                eprintln!("❌ Erro fatal de IO: {}", e);
                break;
            }

            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                eprintln!("❌ Erro ao ler input: {}", e);
                continue;
            }

            let cmd = match parser::parse_command(&input) {
                Some(c) => c,
                None => {
                    println!("❓ Comando desconhecido. Digite 'help' para opções.");
                    continue;
                }
            };

            // Processamento dos comandos
            match cmd {
                Command::Unlock(vault) => {
                    let mut password = parser::request_password();
                    if let Err(e) = self.engine.unlock(&vault, &password) {
                        eprintln!("❌ Erro ao abrir: {}", e);
                    } else {
                        println!("🔓 Vault '{}' aberto com sucesso.", vault);
                    }
                    password.zeroize();
                }

                Command::Create(name) => {
                    let mut password = parser::request_password();
                    if let Err(e) = self.engine.create_vault(&name, &password) {
                        eprintln!("❌ Erro ao criar vault: {}", e);
                    } else {
                        println!("✅ Vault '{}' criado. Não esqueça de dar 'commit'!", name);
                    }
                    password.zeroize();
                }

                Command::Add {
                    service,
                    username,
                    password,
                } => {
                    if let Err(e) = self.engine.add(&service, &username, &password) {
                        eprintln!("❌ Erro ao adicionar: {}", e);
                    } else {
                        println!("➕ Entrada para '{}' adicionada à memória.", service);
                    }
                }

                Command::Commit => {
                    if let Err(e) = self.engine.commit() {
                        eprintln!("❌ Falha ao salvar no disco: {}", e);
                    } else {
                        println!("💾 Alterações persistidas com sucesso!");
                    }
                }

                Command::List => {
                    if self.engine.is_locked() {
                        match self.engine.get_vaults() {
                            Ok(vaults) => {
                                println!("VAULTS DISPONÍVEIS:");
                                for v in vaults {
                                    println!("  -> {}", v);
                                }
                            }
                            Err(e) => eprintln!("❌ Erro ao listar diretório: {}", e),
                        }
                    } else {
                        match self.engine.get_entries() {
                            Ok(entries) => {
                                println!("ENTRADAS NO VAULT:");
                                for entry in entries {
                                    println!("  -> {}", entry);
                                }
                            }
                            Err(e) => eprintln!("❌ Erro ao listar entradas: {}", e),
                        }
                    }
                }

                Command::Get(service) => match self.engine.get(&service) {
                    Ok(entry) => println!(
                        "🔑 [{}] User: {} | Pass: {}",
                        entry.service, entry.username, entry.passwd
                    ),
                    Err(e) => eprintln!("❌ {}", e),
                },

                Command::Remove(service) => {
                    if let Err(e) = self.engine.delete(&service) {
                        eprintln!("❌ {}", e);
                    } else {
                        println!("🗑️ Entrada '{}' removida.", service);
                    }
                }

                Command::Lock => {
                    if let Err(e) = self.engine.lock() {
                        eprintln!("❌ {}", e);
                    } else {
                        println!("🔒 Vault fechado e memória limpa.");
                    }
                }

                Command::Help => Self::print_help(),
                Command::Exit => {
                    println!("Tchau!");
                    break;
                }
            }
        }
    }
}
