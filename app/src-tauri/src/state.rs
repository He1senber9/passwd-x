use std::sync::Mutex;

use passwd_x_core::{EncryptionKey, KdfParams, UnlockedVault};

/// 解锁会话：持有内存中的保险库、主密钥与 KDF 参数。
///
/// `master_key` 在会话期间保留在内存中（Drop 时 zeroize），用于免密保存；
/// 若用户选择「记住本机」，同一主密钥还会写入系统凭据库。
pub struct Session {
    pub vault: UnlockedVault,
    pub master_key: EncryptionKey,
    pub params: KdfParams,
    pub remember: bool,
}

#[derive(Default)]
pub struct VaultState(pub Mutex<Option<Session>>);
