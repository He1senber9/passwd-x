//! passwd-x 共享核心。
//!
//! 这里存放与平台无关的安全与业务逻辑：加密、保险库格式、数据模型。
//! 本 crate 禁止依赖 Tauri，保持框架无关（为将来 UniFFI 绑定留退路）。

mod crypto;
mod error;
mod format;
mod kdf;
mod vault;

pub use error::{CoreError, Result};
pub use format::{create_vault, unlock_vault, UnlockedVault};
pub use kdf::KdfParams;
pub use vault::{Entry, EntryInput, Vault};

/// 当前保险库内容格式版本（v1：整体加密的单文件保险库）。
pub const VAULT_FORMAT_VERSION: u32 = 1;

/// 主密码最小长度（字符数）。
pub const MIN_PASSWORD_LEN: usize = 8;
/// 主密码最大长度（字节数）。
pub const MAX_PASSWORD_LEN: usize = 1024;

pub(crate) fn validate_password(password: &str) -> Result<()> {
    if password.chars().count() < MIN_PASSWORD_LEN {
        return Err(CoreError::Validation(format!(
            "master password must be at least {MIN_PASSWORD_LEN} characters"
        )));
    }
    if password.len() > MAX_PASSWORD_LEN {
        return Err(CoreError::Validation(format!(
            "master password must be at most {MAX_PASSWORD_LEN} bytes"
        )));
    }
    Ok(())
}
