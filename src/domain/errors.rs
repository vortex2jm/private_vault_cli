#[derive(Debug)]
pub enum VaultError {
    /// O vault já está desbloqueado
    AlreadyUnlocked,

    /// O vault está bloqueado e a operação exige unlock
    Locked,

    /// Tentativa de lock quando já está locked
    AlreadyLocked,

    /// Senha inválida
    InvalidPassword,

    /// Dados do vault estão corrompidos ou inválidos
    CorruptedVault,

    /// Versão do arquivo não suportada
    UnsupportedVersion(u8),

    /// Erro ao serializar ou desserializar o vault
    Serialization,

    /// Falha ao criptografar ou descriptografar
    Crypto,

    /// Erro ao acessar o storage (disco, permissão, etc)
    Storage,

    /// Entrada já existe
    EntryAlreadyExists(String),

    /// Entrada não encontrada
    EntryNotFound(String),
}
