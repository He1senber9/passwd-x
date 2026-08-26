//! 版本化加密保险库文件格式（v1）。
//!
//! 布局（多字节整数一律小端）：
//!
//! ```text
//! magic(4) | version(4) | m_cost(4) | t_cost(4) | p_cost(4)
//! | salt(16) | wrap_nonce(24) | wrapped_dek(48) | vault_nonce(24)
//! | ciphertext(可变)
//! ```
//!
//! `wrapped_dek` 是 DEK 经 MK 用 AEAD 加密的结果（32 字节密钥 + 16 字节标签）。
//! `ciphertext` 是整个保险库 JSON 经 DEK 用 AEAD 加密的结果。
//! 两次 AEAD 都以文件头作为关联数据绑定，防止头部被替换或篡改后仍能通过认证。
//! 详见 `docs/format.md`。

use crate::crypto::{self, EncryptionKey, NONCE_LEN, SALT_LEN, TAG_LEN};
use crate::error::{CoreError, Result};
use crate::kdf::{derive_master_key, KdfParams};
use crate::vault::{Entry, EntryInput, Vault};
use crate::{validate_password, VAULT_FORMAT_VERSION};

const MAGIC: [u8; 4] = *b"PWDX";
const FORMAT_VERSION: u32 = 1;
const WRAPPED_DEK_LEN: usize = crypto::KEY_LEN + TAG_LEN;
const HEADER_LEN: usize = 4 + 4 + 3 * 4 + SALT_LEN + NONCE_LEN + WRAPPED_DEK_LEN + NONCE_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Header {
    params: KdfParams,
    salt: [u8; SALT_LEN],
    wrap_nonce: [u8; NONCE_LEN],
    wrapped_dek: [u8; WRAPPED_DEK_LEN],
    vault_nonce: [u8; NONCE_LEN],
}

/// 已解锁的保险库：持有内存中的记录与 DEK。
///
/// DEK 在 Drop 时自动零化；`save` 用当前主密码重新包装 DEK 并整体重新加密，
/// 因此修改主密码等价于用新密码调用 `save`。DEK 在解锁期间保持不变。
pub struct UnlockedVault {
    vault: Vault,
    dek: EncryptionKey,
    salt: [u8; SALT_LEN],
    params: KdfParams,
}

impl std::fmt::Debug for UnlockedVault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnlockedVault")
            .field("vault", &self.vault)
            .field("dek", &self.dek)
            .finish()
    }
}

impl UnlockedVault {
    pub fn vault(&self) -> &Vault {
        &self.vault
    }

    /// 该保险库文件头部使用的 KDF 盐（配合公开的 `derive_master_key` 使用）。
    pub fn salt(&self) -> [u8; SALT_LEN] {
        self.salt
    }

    /// 该保险库文件头部使用的 KDF 参数。
    pub fn params(&self) -> KdfParams {
        self.params
    }

    pub fn add_entry(&mut self, input: EntryInput) -> Result<Entry> {
        self.vault.add(input)
    }

    pub fn update_entry(&mut self, id: &str, input: EntryInput) -> Result<Entry> {
        self.vault.update(id, input)
    }

    pub fn delete_entry(&mut self, id: &str) -> Result<Entry> {
        self.vault.delete(id)
    }

    /// 用当前主密码（或新密码，即改密）整体加密保存保险库。
    pub fn save(&self, password: &str, params: &KdfParams) -> Result<Vec<u8>> {
        validate_password(password)?;
        let master_key = derive_master_key(password, &self.salt, params)?;
        self.save_with_key(&master_key, params)
    }

    /// 用已派生的主密钥保存保险库（用于「记住本机」等免密流程）。
    ///
    /// 主密钥必须由本保险库文件头部中的盐派生，否则保存后的文件将无法用主密码解锁。
    pub fn save_with_key(&self, master_key: &EncryptionKey, params: &KdfParams) -> Result<Vec<u8>> {
        params.validate()?;
        seal_vault(&self.vault, &self.dek, master_key, params, self.salt)
    }
}

/// 创建空保险库并立即加密，返回已解锁实例与文件字节。
pub fn create_vault(password: &str, params: &KdfParams) -> Result<(UnlockedVault, Vec<u8>)> {
    validate_password(password)?;
    params.validate()?;
    let salt = crypto::random_salt()?;
    let master_key = derive_master_key(password, &salt, params)?;
    let dek = EncryptionKey::generate()?;
    let vault = Vault::new();
    let bytes = seal_vault(&vault, &dek, &master_key, params, salt)?;
    Ok((
        UnlockedVault {
            vault,
            dek,
            salt,
            params: *params,
        },
        bytes,
    ))
}

/// 用主密码解锁加密保险库文件。
pub fn unlock_vault(password: &str, bytes: &[u8]) -> Result<UnlockedVault> {
    validate_password(password)?;
    let (header, ciphertext) = parse_file(bytes)?;
    let master_key = derive_master_key(password, &header.salt, &header.params)?;
    unlock_with_header(&header, ciphertext, &master_key)
}

/// 用已派生的主密钥解锁保险库（用于「记住本机」等免密流程）。
pub fn unlock_vault_with_key(bytes: &[u8], master_key: &EncryptionKey) -> Result<UnlockedVault> {
    let (header, ciphertext) = parse_file(bytes)?;
    unlock_with_header(&header, ciphertext, master_key)
}

fn unlock_with_header(
    header: &Header,
    ciphertext: &[u8],
    master_key: &EncryptionKey,
) -> Result<UnlockedVault> {
    let aad_wrap = header_bytes(
        &header.params,
        header.salt,
        header.wrap_nonce,
        header.vault_nonce,
        None,
    )?;
    let dek_bytes = crypto::open(
        master_key,
        &header.wrap_nonce,
        &header.wrapped_dek,
        &aad_wrap,
    )
    .map_err(|_| CoreError::WrongPasswordOrCorrupted)?;
    let dek_bytes: [u8; crypto::KEY_LEN] = dek_bytes
        .try_into()
        .map_err(|_| CoreError::WrongPasswordOrCorrupted)?;
    let dek = EncryptionKey::from(dek_bytes);

    let aad_vault = header_bytes(
        &header.params,
        header.salt,
        header.wrap_nonce,
        header.vault_nonce,
        Some(&header.wrapped_dek),
    )?;
    let payload = crypto::open(&dek, &header.vault_nonce, ciphertext, &aad_vault)
        .map_err(|_| CoreError::WrongPasswordOrCorrupted)?;
    let vault: Vault =
        serde_json::from_slice(&payload).map_err(|e| CoreError::Serialization(e.to_string()))?;
    if vault.version() != VAULT_FORMAT_VERSION {
        return Err(CoreError::InvalidFormat(format!(
            "unsupported vault content version {}",
            vault.version()
        )));
    }
    Ok(UnlockedVault {
        vault,
        dek,
        salt: header.salt,
        params: header.params,
    })
}

fn seal_vault(
    vault: &Vault,
    dek: &EncryptionKey,
    master_key: &EncryptionKey,
    params: &KdfParams,
    salt: [u8; SALT_LEN],
) -> Result<Vec<u8>> {
    let wrap_nonce = crypto::random_nonce()?;
    let vault_nonce = crypto::random_nonce()?;

    let aad_wrap = header_bytes(params, salt, wrap_nonce, vault_nonce, None)?;
    let wrapped_dek = crypto::seal(master_key, &wrap_nonce, dek.as_bytes(), &aad_wrap)?;
    let wrapped_dek: [u8; WRAPPED_DEK_LEN] = wrapped_dek
        .try_into()
        .map_err(|_| CoreError::Crypto("unexpected wrapped key length".into()))?;

    let aad_vault = header_bytes(params, salt, wrap_nonce, vault_nonce, Some(&wrapped_dek))?;
    let payload = serde_json::to_vec(vault).map_err(|e| CoreError::Serialization(e.to_string()))?;
    let ciphertext = crypto::seal(dek, &vault_nonce, &payload, &aad_vault)?;

    let mut out = Vec::with_capacity(HEADER_LEN + ciphertext.len());
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    out.extend_from_slice(&params.m_cost_kib.to_le_bytes());
    out.extend_from_slice(&params.t_cost.to_le_bytes());
    out.extend_from_slice(&params.p_cost.to_le_bytes());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&wrap_nonce);
    out.extend_from_slice(&wrapped_dek);
    out.extend_from_slice(&vault_nonce);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

fn parse_file(bytes: &[u8]) -> Result<(Header, &[u8])> {
    if bytes.len() < HEADER_LEN {
        return Err(CoreError::InvalidFormat("vault file is too short".into()));
    }
    let mut off = 0;

    let magic = read_array::<4>(bytes, &mut off)?;
    if magic != MAGIC {
        return Err(CoreError::InvalidFormat("bad magic bytes".into()));
    }

    let version = u32::from_le_bytes(read_array(bytes, &mut off)?);
    if version != FORMAT_VERSION {
        return Err(CoreError::InvalidFormat(format!(
            "unsupported vault file version {version}"
        )));
    }

    let m_cost_kib = u32::from_le_bytes(read_array(bytes, &mut off)?);
    let t_cost = u32::from_le_bytes(read_array(bytes, &mut off)?);
    let p_cost = u32::from_le_bytes(read_array(bytes, &mut off)?);
    let params = KdfParams::new(m_cost_kib, t_cost, p_cost);
    params.validate()?;

    let salt = read_array(bytes, &mut off)?;
    let wrap_nonce = read_array(bytes, &mut off)?;
    let wrapped_dek = read_array(bytes, &mut off)?;
    let vault_nonce = read_array(bytes, &mut off)?;
    let ciphertext = &bytes[off..];

    if ciphertext.len() < TAG_LEN {
        return Err(CoreError::InvalidFormat(
            "vault ciphertext is too short".into(),
        ));
    }

    Ok((
        Header {
            params,
            salt,
            wrap_nonce,
            wrapped_dek,
            vault_nonce,
        },
        ciphertext,
    ))
}

fn header_bytes(
    params: &KdfParams,
    salt: [u8; SALT_LEN],
    wrap_nonce: [u8; NONCE_LEN],
    vault_nonce: [u8; NONCE_LEN],
    wrapped_dek: Option<&[u8]>,
) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_LEN);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    out.extend_from_slice(&params.m_cost_kib.to_le_bytes());
    out.extend_from_slice(&params.t_cost.to_le_bytes());
    out.extend_from_slice(&params.p_cost.to_le_bytes());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&wrap_nonce);
    if let Some(dek) = wrapped_dek {
        out.extend_from_slice(dek);
    }
    out.extend_from_slice(&vault_nonce);
    Ok(out)
}

fn read_array<const N: usize>(bytes: &[u8], off: &mut usize) -> Result<[u8; N]> {
    let end = off
        .checked_add(N)
        .ok_or_else(|| CoreError::InvalidFormat("vault file length overflow".into()))?;
    if bytes.len() < end {
        return Err(CoreError::InvalidFormat("vault file is too short".into()));
    }
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes[*off..end]);
    *off = end;
    Ok(out)
}
