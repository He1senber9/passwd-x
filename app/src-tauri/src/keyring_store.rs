use passwd_x_core::KdfParams;
use serde::{Deserialize, Serialize};

const KEYRING_SERVICE: &str = "passwd-x";
const KEYRING_ACCOUNT: &str = "vault";
const TOKEN_VERSION: u32 = 1;

/// 保存在系统凭据库中的「记住本机」凭据：
/// KDF 参数 + 已派生的主密钥（由 OS 凭据库负责保管与访问控制）。
#[derive(Serialize, Deserialize)]
pub struct RememberToken {
    version: u32,
    params: KdfParams,
    master_key: [u8; 32],
}

impl RememberToken {
    pub fn new(params: KdfParams, master_key: [u8; 32]) -> Self {
        Self {
            version: TOKEN_VERSION,
            params,
            master_key,
        }
    }

    pub fn params(&self) -> KdfParams {
        self.params
    }

    pub fn master_key(&self) -> [u8; 32] {
        self.master_key
    }
}

pub fn store_token(token: &RememberToken) -> Result<(), String> {
    let json = serde_json::to_vec(token).map_err(|e| e.to_string())?;
    let json = String::from_utf8(json).map_err(|e| e.to_string())?;
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT).map_err(|e| e.to_string())?;
    entry
        .set_password(&json)
        .map_err(|e| format!("failed to store credentials in system keychain: {e}"))
}

pub fn load_token() -> Result<Option<RememberToken>, String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT).map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(json) => {
            let token: RememberToken = serde_json::from_str(&json).map_err(|e| e.to_string())?;
            if token.version != TOKEN_VERSION {
                return Err("unsupported remembered credential version".into());
            }
            Ok(Some(token))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("failed to read system keychain: {e}")),
    }
}

pub fn delete_token() -> Result<(), String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT).map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("failed to clear system keychain: {e}")),
    }
}
