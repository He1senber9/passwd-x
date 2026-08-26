//! 核心层统一的错误类型。

use std::fmt;

/// 核心层统一的错误类型。错误消息不包含任何密码或密钥材料。
#[derive(Debug)]
pub enum CoreError {
    /// KDF 参数不合法（包括恶意文件头中的参数）。
    InvalidParams(String),
    /// 输入校验失败（空标题、主密码过短等）。
    Validation(String),
    /// 文件头不合法或版本不受支持。
    InvalidFormat(String),
    /// 主密码错误或数据已损坏（AEAD 认证失败，两者不可区分）。
    WrongPasswordOrCorrupted,
    /// 主密钥派生失败。
    Kdf(String),
    /// 加解密失败。
    Crypto(String),
    /// 序列化失败。
    Serialization(String),
    /// 随机数生成失败。
    Rng(String),
    /// 记录不存在。
    NotFound(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::InvalidParams(msg)
            | CoreError::Validation(msg)
            | CoreError::InvalidFormat(msg)
            | CoreError::Kdf(msg)
            | CoreError::Crypto(msg)
            | CoreError::Serialization(msg)
            | CoreError::Rng(msg)
            | CoreError::NotFound(msg) => f.write_str(msg),
            CoreError::WrongPasswordOrCorrupted => {
                f.write_str("wrong password or corrupted vault data")
            }
        }
    }
}

impl std::error::Error for CoreError {}

pub type Result<T> = std::result::Result<T, CoreError>;
