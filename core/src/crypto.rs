//! AEAD 加解密（XChaCha20-Poly1305）与随机字节生成。

use chacha20poly1305::{
    aead::{Aead, Payload},
    KeyInit, XChaCha20Poly1305,
};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{CoreError, Result};

use chacha20poly1305::{Key, XNonce};

/// 对称密钥长度（字节）。
pub const KEY_LEN: usize = 32;
/// nonce 长度（字节，XChaCha20 使用 24 字节 nonce）。
pub const NONCE_LEN: usize = 24;
/// AEAD 认证标签长度（字节）。
pub const TAG_LEN: usize = 16;
/// 推荐盐长度（字节）。
pub const SALT_LEN: usize = 16;

/// 内存中的对称密钥（主密钥 MK 或数据密钥 DEK）。
///
/// Drop 时自动用 `zeroize` 擦除内容；Debug 输出已脱敏。
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct EncryptionKey([u8; KEY_LEN]);

impl From<[u8; KEY_LEN]> for EncryptionKey {
    fn from(bytes: [u8; KEY_LEN]) -> Self {
        Self(bytes)
    }
}

impl std::fmt::Debug for EncryptionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EncryptionKey([REDACTED])")
    }
}

impl EncryptionKey {
    /// 生成 32 字节随机密钥。
    pub(crate) fn generate() -> Result<Self> {
        let mut bytes = [0u8; KEY_LEN];
        getrandom::fill(&mut bytes).map_err(|e| CoreError::Rng(e.to_string()))?;
        Ok(Self(bytes))
    }

    /// 密钥字节引用。
    ///
    /// 仅用于与平台安全存储（如系统凭据库）等受信任边界交互；
    /// 调用方负责在使用后擦除拷贝。
    pub fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.0
    }

    fn cipher(&self) -> XChaCha20Poly1305 {
        XChaCha20Poly1305::new(Key::from_slice(&self.0))
    }
}

/// 生成推荐长度的随机盐。
pub(crate) fn random_salt() -> Result<[u8; SALT_LEN]> {
    let mut salt = [0u8; SALT_LEN];
    getrandom::fill(&mut salt).map_err(|e| CoreError::Rng(e.to_string()))?;
    Ok(salt)
}

/// 生成随机 nonce。
pub(crate) fn random_nonce() -> Result<[u8; NONCE_LEN]> {
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::fill(&mut nonce).map_err(|e| CoreError::Rng(e.to_string()))?;
    Ok(nonce)
}

/// AEAD 加密并追加认证标签（输出 = 密文 + 16 字节标签）。
pub(crate) fn seal(
    key: &EncryptionKey,
    nonce: &[u8; NONCE_LEN],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>> {
    key.cipher()
        .encrypt(
            XNonce::from_slice(nonce),
            Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|_| CoreError::Crypto("encryption failed".into()))
}

/// AEAD 解密并校验认证标签与关联数据。
pub(crate) fn open(
    key: &EncryptionKey,
    nonce: &[u8; NONCE_LEN],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>> {
    key.cipher()
        .decrypt(
            XNonce::from_slice(nonce),
            Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|_| CoreError::Crypto("decryption failed".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key(byte: u8) -> EncryptionKey {
        EncryptionKey::from([byte; KEY_LEN])
    }

    #[test]
    fn seal_open_roundtrip() {
        let key = test_key(1);
        let nonce = random_nonce().unwrap();
        let plaintext = b"secret payload";
        let aad = b"header";
        let ct = seal(&key, &nonce, plaintext, aad).unwrap();
        assert_eq!(open(&key, &nonce, &ct, aad).unwrap(), plaintext);
    }

    #[test]
    fn tampered_ciphertext_rejected() {
        let key = test_key(1);
        let nonce = random_nonce().unwrap();
        let ct = seal(&key, &nonce, b"secret", b"aad").unwrap();
        let mut bad = ct.clone();
        *bad.last_mut().unwrap() ^= 1;
        assert!(open(&key, &nonce, &bad, b"aad").is_err());
    }

    #[test]
    fn wrong_key_rejected() {
        let nonce = random_nonce().unwrap();
        let ct = seal(&test_key(1), &nonce, b"secret", b"aad").unwrap();
        assert!(open(&test_key(2), &nonce, &ct, b"aad").is_err());
    }

    #[test]
    fn wrong_aad_rejected() {
        let key = test_key(1);
        let nonce = random_nonce().unwrap();
        let ct = seal(&key, &nonce, b"secret", b"aad").unwrap();
        assert!(open(&key, &nonce, &ct, b"other aad").is_err());
    }
}
