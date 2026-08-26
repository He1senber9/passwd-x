use std::path::PathBuf;

use passwd_x_core::{validate_password, EncryptionKey, EntryInput, KdfParams};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use crate::keyring_store::{self, RememberToken};
use crate::state::{Session, VaultState};

const VAULT_FILE_NAME: &str = "vault.pwx";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultRequest {
    pub password: String,
    pub remember: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockVaultRequest {
    pub password: String,
    pub remember: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub new_password: String,
    pub remember: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryInputPayload {
    pub title: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub notes: String,
}

impl From<EntryInputPayload> for EntryInput {
    fn from(payload: EntryInputPayload) -> Self {
        Self {
            title: payload.title,
            username: payload.username,
            password: payload.password,
            urls: payload.urls,
            notes: payload.notes,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryPayload {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub notes: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<&passwd_x_core::Entry> for EntryPayload {
    fn from(entry: &passwd_x_core::Entry) -> Self {
        Self {
            id: entry.id.clone(),
            title: entry.title.clone(),
            username: entry.username.clone(),
            password: entry.password.clone(),
            urls: entry.urls.clone(),
            notes: entry.notes.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct VaultStatus {
    pub unlocked: bool,
    pub entry_count: usize,
}

fn status_of(session: &Option<Session>) -> VaultStatus {
    VaultStatus {
        unlocked: session.is_some(),
        entry_count: session
            .as_ref()
            .map(|s| s.vault.vault().count())
            .unwrap_or(0),
    }
}

fn vault_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("failed to create data dir: {e}"))?;
    Ok(dir.join(VAULT_FILE_NAME))
}

fn read_vault_file(app: &AppHandle) -> Result<Vec<u8>, String> {
    let path = vault_path(app)?;
    std::fs::read(&path).map_err(|e| format!("cannot read vault file: {e}"))
}

fn write_vault_file(app: &AppHandle, bytes: &[u8]) -> Result<(), String> {
    let path = vault_path(app)?;
    std::fs::write(&path, bytes).map_err(|e| format!("cannot write vault file: {e}"))
}

fn save_session(app: &AppHandle, session: &Session) -> Result<(), String> {
    let bytes = session
        .vault
        .save_with_key(&session.master_key, &session.params)
        .map_err(|e| e.to_string())?;
    write_vault_file(app, &bytes)
}

#[tauri::command]
pub fn vault_status(state: State<'_, VaultState>) -> VaultStatus {
    let guard = state.0.lock().expect("vault state poisoned");
    status_of(&guard)
}

#[tauri::command]
pub fn create_vault(
    app: AppHandle,
    state: State<'_, VaultState>,
    request: CreateVaultRequest,
) -> Result<VaultStatus, String> {
    let params = KdfParams::default();
    let (vault, bytes) =
        passwd_x_core::create_vault(&request.password, &params).map_err(|e| e.to_string())?;
    write_vault_file(&app, &bytes)?;

    let master_key =
        passwd_x_core::derive_master_key(&request.password, &vault.salt(), &vault.params())
            .map_err(|e| e.to_string())?;
    if request.remember {
        keyring_store::store_token(&RememberToken::new(vault.params(), *master_key.as_bytes()))?;
    }

    let params = vault.params();
    let session = Session {
        vault,
        master_key,
        params,
        remember: request.remember,
    };
    let mut guard = state.0.lock().expect("vault state poisoned");
    *guard = Some(session);
    Ok(status_of(&guard))
}

#[tauri::command]
pub fn unlock_vault(
    app: AppHandle,
    state: State<'_, VaultState>,
    request: UnlockVaultRequest,
) -> Result<VaultStatus, String> {
    let bytes = read_vault_file(&app)?;
    let vault =
        passwd_x_core::unlock_vault(&request.password, &bytes).map_err(|e| e.to_string())?;

    let master_key =
        passwd_x_core::derive_master_key(&request.password, &vault.salt(), &vault.params())
            .map_err(|e| e.to_string())?;
    if request.remember {
        keyring_store::store_token(&RememberToken::new(vault.params(), *master_key.as_bytes()))?;
    }

    let params = vault.params();
    let session = Session {
        vault,
        master_key,
        params,
        remember: request.remember,
    };
    let mut guard = state.0.lock().expect("vault state poisoned");
    *guard = Some(session);
    Ok(status_of(&guard))
}

#[tauri::command]
pub fn unlock_remembered(
    app: AppHandle,
    state: State<'_, VaultState>,
) -> Result<VaultStatus, String> {
    let bytes = read_vault_file(&app)?;
    let token = keyring_store::load_token()?.ok_or("本机未保存解锁凭据")?;
    let master_key = EncryptionKey::from(token.master_key());
    let vault =
        passwd_x_core::unlock_vault_with_key(&bytes, &master_key).map_err(|e| e.to_string())?;

    let session = Session {
        vault,
        master_key,
        params: token.params(),
        remember: true,
    };
    let mut guard = state.0.lock().expect("vault state poisoned");
    *guard = Some(session);
    Ok(status_of(&guard))
}

#[tauri::command]
pub fn lock_vault(state: State<'_, VaultState>) {
    *state.0.lock().expect("vault state poisoned") = None;
}

#[tauri::command]
pub fn list_entries(state: State<'_, VaultState>) -> Result<Vec<EntryPayload>, String> {
    let guard = state
        .0
        .lock()
        .map_err(|_| "vault state poisoned".to_string())?;
    let session = guard.as_ref().ok_or("vault is locked")?;
    Ok(session
        .vault
        .vault()
        .entries()
        .iter()
        .map(EntryPayload::from)
        .collect())
}

#[tauri::command]
pub fn add_entry(
    app: AppHandle,
    state: State<'_, VaultState>,
    input: EntryInputPayload,
) -> Result<EntryPayload, String> {
    let mut guard = state
        .0
        .lock()
        .map_err(|_| "vault state poisoned".to_string())?;
    let session = guard.as_mut().ok_or("vault is locked")?;
    let entry = session
        .vault
        .add_entry(input.into())
        .map_err(|e| e.to_string())?;
    save_session(&app, session)?;
    Ok(EntryPayload::from(&entry))
}

#[tauri::command]
pub fn update_entry(
    app: AppHandle,
    state: State<'_, VaultState>,
    id: String,
    input: EntryInputPayload,
) -> Result<EntryPayload, String> {
    let mut guard = state
        .0
        .lock()
        .map_err(|_| "vault state poisoned".to_string())?;
    let session = guard.as_mut().ok_or("vault is locked")?;
    let entry = session
        .vault
        .update_entry(&id, input.into())
        .map_err(|e| e.to_string())?;
    save_session(&app, session)?;
    Ok(EntryPayload::from(&entry))
}

#[tauri::command]
pub fn delete_entry(
    app: AppHandle,
    state: State<'_, VaultState>,
    id: String,
) -> Result<(), String> {
    let mut guard = state
        .0
        .lock()
        .map_err(|_| "vault state poisoned".to_string())?;
    let session = guard.as_mut().ok_or("vault is locked")?;
    session.vault.delete_entry(&id).map_err(|e| e.to_string())?;
    save_session(&app, session)
}

#[tauri::command]
pub fn change_password(
    app: AppHandle,
    state: State<'_, VaultState>,
    request: ChangePasswordRequest,
) -> Result<(), String> {
    validate_password(&request.new_password).map_err(|e| e.to_string())?;

    let mut guard = state
        .0
        .lock()
        .map_err(|_| "vault state poisoned".to_string())?;
    let session = guard.as_mut().ok_or("vault is locked")?;
    let new_master_key = passwd_x_core::derive_master_key(
        &request.new_password,
        &session.vault.salt(),
        &session.params,
    )
    .map_err(|e| e.to_string())?;

    let bytes = session
        .vault
        .save_with_key(&new_master_key, &session.params)
        .map_err(|e| e.to_string())?;
    write_vault_file(&app, &bytes)?;

    session.master_key = new_master_key;
    session.remember = request.remember;
    if request.remember {
        keyring_store::store_token(&RememberToken::new(
            session.params,
            *session.master_key.as_bytes(),
        ))?;
    } else {
        keyring_store::delete_token()?;
    }
    Ok(())
}

#[tauri::command]
pub fn forget_vault(state: State<'_, VaultState>) -> Result<(), String> {
    keyring_store::delete_token()?;
    let mut guard = state
        .0
        .lock()
        .map_err(|_| "vault state poisoned".to_string())?;
    if let Some(session) = guard.as_mut() {
        session.remember = false;
    }
    Ok(())
}
