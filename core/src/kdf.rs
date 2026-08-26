//! Argon2id 主密钥派生（KDF）。

use argon2::{Algorithm, Argon2, Params, Version};

use crate::crypto::{EncryptionKey, KEY_LEN};
use crate::error::{CoreError, Result};

/// 内存成本下限（KiB）。更低的值会让离线爆破成本过低。
pub const MIN_M_COST_KIB: u32 = 8;
/// 内存成本上限（KiB，1 GiB），防止解析恶意文件时过量分配内存。
pub const MAX_M_COST_KIB: u32 = 1_048_576;
/// Argon2 要求 `m_cost >= 8 * p_cost`。
pub const M_COST_PER_P_COST_MIN: u32 = 8;
/// 迭代次数下限。
pub const MIN_T_COST: u32 = 1;
/// 迭代次数上限。
pub const MAX_T_COST: u32 = 32;
/// 并行度下限。
pub const MIN_P_COST: u32 = 1;
/// 并行度上限。
pub const MAX_P_COST: u32 = 8;

/// Argon2id 参数。随加密文件头部存储，解锁时读取并校验。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KdfParams {
    /// 内存成本，单位 KiB。
    pub m_cost_kib: u32,
    /// 迭代次数。
    pub t_cost: u32,
    /// 并行度。
    pub p_cost: u32,
}

impl Default for KdfParams {
    /// OWASP 推荐基线：19 MiB、2 次迭代、1 路并行。
    fn default() -> Self {
        Self {
            m_cost_kib: 19_456,
            t_cost: 2,
            p_cost: 1,
        }
    }
}

impl KdfParams {
    pub const fn new(m_cost_kib: u32, t_cost: u32, p_cost: u32) -> Self {
        Self {
            m_cost_kib,
            t_cost,
            p_cost,
        }
    }

    /// 校验参数是否在安全范围内（也用于防御恶意构造的保险库文件）。
    pub fn validate(&self) -> Result<()> {
        if !(MIN_M_COST_KIB..=MAX_M_COST_KIB).contains(&self.m_cost_kib) {
            return Err(CoreError::InvalidParams(format!(
                "m_cost_kib must be in {MIN_M_COST_KIB}..={MAX_M_COST_KIB}, got {}",
                self.m_cost_kib
            )));
        }
        if !(MIN_T_COST..=MAX_T_COST).contains(&self.t_cost) {
            return Err(CoreError::InvalidParams(format!(
                "t_cost must be in {MIN_T_COST}..={MAX_T_COST}, got {}",
                self.t_cost
            )));
        }
        if !(MIN_P_COST..=MAX_P_COST).contains(&self.p_cost) {
            return Err(CoreError::InvalidParams(format!(
                "p_cost must be in {MIN_P_COST}..={MAX_P_COST}, got {}",
                self.p_cost
            )));
        }
        if self.m_cost_kib < M_COST_PER_P_COST_MIN * self.p_cost {
            return Err(CoreError::InvalidParams(format!(
                "m_cost_kib must be at least 8 * p_cost ({}), got {}",
                self.p_cost, self.m_cost_kib
            )));
        }
        Ok(())
    }
}

/// 用 Argon2id 将主密码派生成 32 字节主密钥。
///
/// 盐可任意长度，建议 16 字节随机值。返回的密钥在内存中零化（zeroize）。
pub fn derive_master_key(password: &str, salt: &[u8], params: &KdfParams) -> Result<EncryptionKey> {
    params.validate()?;
    let argon2_params = Params::new(
        params.m_cost_kib,
        params.t_cost,
        params.p_cost,
        Some(KEY_LEN),
    )
    .map_err(|e| CoreError::Kdf(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);
    let mut key = [0u8; KEY_LEN];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| CoreError::Kdf(e.to_string()))?;
    Ok(EncryptionKey::from(key))
}

#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn kdf_params_serde_roundtrip() {
        let params = KdfParams::default();
        let json = serde_json::to_string(&params).unwrap();
        let back: KdfParams = serde_json::from_str(&json).unwrap();
        assert_eq!(params, back);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_argon2id_vector() {
        // 参考实现（phc-winner-argon2 test.c）的 Argon2id v0x13 向量：
        // password="password"、salt="somesalt"、m=256 KiB、t=2、p=1。
        let params = KdfParams::new(256, 2, 1);
        let key = derive_master_key("password", b"somesalt", &params).unwrap();
        let expected = [
            0x9d, 0xfe, 0xb9, 0x10, 0xe8, 0x0b, 0xad, 0x03, 0x11, 0xfe, 0xe2, 0x0f, 0x9c, 0x0e,
            0x2b, 0x12, 0xc1, 0x79, 0x87, 0xb4, 0xca, 0xc9, 0x0c, 0x2e, 0xf5, 0x4d, 0x5b, 0x30,
            0x21, 0xc6, 0x8b, 0xfe,
        ];
        assert_eq!(key.as_bytes(), &expected);
    }

    #[test]
    fn derivation_is_deterministic() {
        let salt = [7u8; 16];
        let params = KdfParams::new(16, 1, 1);
        let a = derive_master_key("correct horse battery staple", &salt, &params).unwrap();
        let b = derive_master_key("correct horse battery staple", &salt, &params).unwrap();
        assert_eq!(a.as_bytes(), b.as_bytes());
    }

    #[test]
    fn different_salt_changes_key() {
        let params = KdfParams::new(16, 1, 1);
        let a = derive_master_key("correct horse battery staple", &[1u8; 16], &params).unwrap();
        let b = derive_master_key("correct horse battery staple", &[2u8; 16], &params).unwrap();
        assert_ne!(a.as_bytes(), b.as_bytes());
    }

    #[test]
    fn invalid_params_rejected() {
        assert!(KdfParams::new(0, 1, 1).validate().is_err());
        assert!(KdfParams::new(16, 0, 1).validate().is_err());
        assert!(KdfParams::new(16, 1, 9).validate().is_err());
        assert!(KdfParams::new(4, 1, 1).validate().is_err());
        assert!(KdfParams::new(MAX_M_COST_KIB + 1, 1, 1).validate().is_err());
    }
}
