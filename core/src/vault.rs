//! 保险库数据模型与记录 CRUD。

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{CoreError, Result};
use crate::VAULT_FORMAT_VERSION;

const MAX_ENTRY_TITLE_LEN: usize = 256;
const MAX_ENTRY_USERNAME_LEN: usize = 256;
const MAX_ENTRY_PASSWORD_LEN: usize = 1024;
const MAX_ENTRY_NOTES_LEN: usize = 4096;
const MAX_ENTRY_URLS: usize = 16;
const MAX_ENTRY_URL_LEN: usize = 2048;

/// 一条账号/密码记录。密码字段仅在解锁后的内存中出现。
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub notes: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .field("urls", &self.urls)
            .field("notes", &self.notes)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}

/// 新增/编辑记录时的输入字段（不含 id 与时间戳）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EntryInput {
    pub title: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub notes: String,
}

impl EntryInput {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }
}

/// 内存中的保险库：版本号 + 记录列表。
///
/// v1 采用整体加密：整个结构序列化后由 DEK 加密落盘。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vault {
    version: u32,
    entries: Vec<Entry>,
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}

impl Vault {
    pub fn new() -> Self {
        Self {
            version: VAULT_FORMAT_VERSION,
            entries: Vec::new(),
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn get(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    /// 新增记录，返回带 id 与时间戳的完整记录。
    pub fn add(&mut self, input: EntryInput) -> Result<Entry> {
        self.add_at(input, now_secs())
    }

    /// 编辑记录（id 不变，刷新 `updated_at`），返回更新后的记录。
    pub fn update(&mut self, id: &str, input: EntryInput) -> Result<Entry> {
        self.update_at(id, input, now_secs())
    }

    /// 删除记录，返回被删除的记录。
    pub fn delete(&mut self, id: &str) -> Result<Entry> {
        let index = self
            .entries
            .iter()
            .position(|entry| entry.id == id)
            .ok_or_else(|| CoreError::NotFound(id.to_owned()))?;
        Ok(self.entries.remove(index))
    }

    pub(crate) fn add_at(&mut self, input: EntryInput, now: u64) -> Result<Entry> {
        validate_input(&input)?;
        let entry = Entry {
            id: Uuid::new_v4().to_string(),
            title: input.title.trim().to_owned(),
            username: input.username.trim().to_owned(),
            password: input.password,
            urls: normalize_urls(input.urls)?,
            notes: input.notes,
            created_at: now,
            updated_at: now,
        };
        self.entries.push(entry.clone());
        Ok(entry)
    }

    pub(crate) fn update_at(&mut self, id: &str, input: EntryInput, now: u64) -> Result<Entry> {
        validate_input(&input)?;
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| entry.id == id)
            .ok_or_else(|| CoreError::NotFound(id.to_owned()))?;
        entry.title = input.title.trim().to_owned();
        entry.username = input.username.trim().to_owned();
        entry.password = input.password;
        entry.urls = normalize_urls(input.urls)?;
        entry.notes = input.notes;
        entry.updated_at = now;
        Ok(entry.clone())
    }
}

fn validate_input(input: &EntryInput) -> Result<()> {
    if input.title.trim().is_empty() {
        return Err(CoreError::Validation("title must not be empty".into()));
    }
    if input.title.chars().count() > MAX_ENTRY_TITLE_LEN {
        return Err(CoreError::Validation(format!(
            "title must be at most {MAX_ENTRY_TITLE_LEN} characters"
        )));
    }
    if input.username.chars().count() > MAX_ENTRY_USERNAME_LEN {
        return Err(CoreError::Validation(format!(
            "username must be at most {MAX_ENTRY_USERNAME_LEN} characters"
        )));
    }
    if input.password.len() > MAX_ENTRY_PASSWORD_LEN {
        return Err(CoreError::Validation(format!(
            "password must be at most {MAX_ENTRY_PASSWORD_LEN} bytes"
        )));
    }
    if input.notes.chars().count() > MAX_ENTRY_NOTES_LEN {
        return Err(CoreError::Validation(format!(
            "notes must be at most {MAX_ENTRY_NOTES_LEN} characters"
        )));
    }
    if input.urls.len() > MAX_ENTRY_URLS {
        return Err(CoreError::Validation(format!(
            "at most {MAX_ENTRY_URLS} urls are allowed"
        )));
    }
    Ok(())
}

fn normalize_urls(urls: Vec<String>) -> Result<Vec<String>> {
    let mut normalized = Vec::with_capacity(urls.len());
    for url in urls {
        let url = url.trim();
        if url.is_empty() {
            continue;
        }
        if url.chars().count() > MAX_ENTRY_URL_LEN {
            return Err(CoreError::Validation(format!(
                "url must be at most {MAX_ENTRY_URL_LEN} characters"
            )));
        }
        normalized.push(url.to_owned());
    }
    Ok(normalized)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(title: &str) -> EntryInput {
        EntryInput {
            title: title.into(),
            username: "alice".into(),
            password: "hunter2secret".into(),
            ..EntryInput::default()
        }
    }

    #[test]
    fn add_update_delete_lifecycle() {
        let mut vault = Vault::new();
        let created = vault.add_at(input("GitHub"), 100).unwrap();
        assert_eq!(vault.count(), 1);
        assert_eq!(created.created_at, 100);
        assert_eq!(created.updated_at, 100);

        let id = created.id.clone();
        let mut change = input("GitHub");
        change.password = "new-password-123".into();
        let updated = vault.update_at(&id, change, 200).unwrap();
        assert_eq!(updated.id, id);
        assert_eq!(updated.created_at, 100);
        assert_eq!(updated.updated_at, 200);
        assert_eq!(updated.password, "new-password-123");

        let removed = vault.delete(&id).unwrap();
        assert_eq!(removed.id, id);
        assert_eq!(vault.count(), 0);
    }

    #[test]
    fn ids_are_unique() {
        let mut vault = Vault::new();
        let a = vault.add_at(input("A"), 1).unwrap();
        let b = vault.add_at(input("B"), 1).unwrap();
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn empty_title_rejected() {
        let mut vault = Vault::new();
        let err = vault.add_at(input("   "), 1).unwrap_err();
        assert!(matches!(err, CoreError::Validation(_)));
    }

    #[test]
    fn update_missing_entry_returns_not_found() {
        let mut vault = Vault::new();
        let err = vault.update_at("nope", input("X"), 1).unwrap_err();
        assert!(matches!(err, CoreError::NotFound(_)));
    }

    #[test]
    fn delete_missing_entry_returns_not_found() {
        let mut vault = Vault::new();
        let err = vault.delete("nope").unwrap_err();
        assert!(matches!(err, CoreError::NotFound(_)));
    }

    #[test]
    fn whitespace_normalized_and_empty_urls_dropped() {
        let mut vault = Vault::new();
        let mut input = input("GitHub");
        input.urls = vec!["  https://github.com  ".into(), "  ".into()];
        let entry = vault.add_at(input, 1).unwrap();
        assert_eq!(entry.urls, vec!["https://github.com".to_owned()]);
    }

    #[test]
    fn debug_output_redacts_password() {
        let entry = Entry {
            id: "id".into(),
            title: "GitHub".into(),
            username: "alice".into(),
            password: "super-secret".into(),
            urls: vec![],
            notes: String::new(),
            created_at: 1,
            updated_at: 1,
        };
        let debug = format!("{entry:?}");
        assert!(!debug.contains("super-secret"));
        assert!(debug.contains("[REDACTED]"));
    }
}
