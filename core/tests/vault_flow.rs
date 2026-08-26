//! 保险库生命周期集成测试：创建、解锁、CRUD、改密与文件完整性。

use passwd_x_core::{
    create_vault, derive_master_key, unlock_vault, unlock_vault_with_key, CoreError, EntryInput,
    KdfParams,
};

const PASSWORD: &str = "correct horse battery staple";
const PASSWORD_2: &str = "another correct horse battery staple";

/// 测试用低开销参数（16 KiB、1 次迭代）。生产默认值见 `KdfParams::default()`。
fn test_params() -> KdfParams {
    KdfParams::new(16, 1, 1)
}

fn entry_input(title: &str) -> EntryInput {
    EntryInput {
        title: title.into(),
        username: "alice".into(),
        password: "hunter2secret".into(),
        ..EntryInput::default()
    }
}

#[test]
fn create_unlock_and_crud_roundtrip() {
    let (mut vault, bytes) = create_vault(PASSWORD, &test_params()).unwrap();

    // 刚创建的文件解锁后为空。
    let fresh = unlock_vault(PASSWORD, &bytes).unwrap();
    assert_eq!(fresh.vault().count(), 0);

    let created = vault.add_entry(entry_input("GitHub")).unwrap();

    // 保存后，新文件包含刚添加的记录。
    let saved = vault.save(PASSWORD, &test_params()).unwrap();
    let unlocked = unlock_vault(PASSWORD, &saved).unwrap();
    assert_eq!(unlocked.vault().count(), 1);
    assert_eq!(unlocked.vault().get(&created.id).unwrap().username, "alice");

    // 编辑与删除。
    let mut updated_input = entry_input("GitHub");
    updated_input.password = "new-password-123".into();
    let updated = vault.update_entry(&created.id, updated_input).unwrap();
    assert_eq!(updated.password, "new-password-123");

    vault.delete_entry(&created.id).unwrap();
    let saved_empty = vault.save(PASSWORD, &test_params()).unwrap();
    assert_eq!(
        unlock_vault(PASSWORD, &saved_empty)
            .unwrap()
            .vault()
            .count(),
        0
    );
}

#[test]
fn wrong_password_rejected() {
    let (_, bytes) = create_vault(PASSWORD, &test_params()).unwrap();
    let err = unlock_vault("wrong password", &bytes).unwrap_err();
    assert!(matches!(err, CoreError::WrongPasswordOrCorrupted));
}

#[test]
fn short_password_rejected_on_create_and_unlock() {
    let err = create_vault("short", &test_params()).unwrap_err();
    assert!(matches!(err, CoreError::Validation(_)));

    let (_, bytes) = create_vault(PASSWORD, &test_params()).unwrap();
    let err = unlock_vault("short", &bytes).unwrap_err();
    assert!(matches!(err, CoreError::Validation(_)));
}

#[test]
fn change_password_rewraps_dek_and_old_password_stops_working() {
    let (vault, _) = create_vault(PASSWORD, &test_params()).unwrap();
    let rekeyed = vault.save(PASSWORD_2, &test_params()).unwrap();

    unlock_vault(PASSWORD_2, &rekeyed).unwrap();
    let err = unlock_vault(PASSWORD, &rekeyed).unwrap_err();
    assert!(matches!(err, CoreError::WrongPasswordOrCorrupted));
}

#[test]
fn tampered_ciphertext_rejected() {
    let (_, bytes) = create_vault(PASSWORD, &test_params()).unwrap();
    let mut tampered = bytes.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0x01;
    let err = unlock_vault(PASSWORD, &tampered).unwrap_err();
    assert!(matches!(err, CoreError::WrongPasswordOrCorrupted));
}

#[test]
fn tampered_header_rejected() {
    let (_, bytes) = create_vault(PASSWORD, &test_params()).unwrap();
    let mut tampered = bytes.clone();
    // salt 位于偏移 20..36，翻转其中一个字节。
    tampered[20] ^= 0x01;
    let err = unlock_vault(PASSWORD, &tampered).unwrap_err();
    assert!(matches!(err, CoreError::WrongPasswordOrCorrupted));
}

#[test]
fn unsupported_file_version_rejected() {
    let (_, bytes) = create_vault(PASSWORD, &test_params()).unwrap();
    let mut bad = bytes.clone();
    // version 位于偏移 4..8（小端 u32）。
    bad[4..8].copy_from_slice(&2u32.to_le_bytes());
    let err = unlock_vault(PASSWORD, &bad).unwrap_err();
    assert!(matches!(err, CoreError::InvalidFormat(_)));
}

#[test]
fn truncated_file_rejected() {
    let (_, bytes) = create_vault(PASSWORD, &test_params()).unwrap();
    let err = unlock_vault(PASSWORD, &bytes[..100]).unwrap_err();
    assert!(matches!(err, CoreError::InvalidFormat(_)));
}

#[test]
fn unreasonable_kdf_params_in_file_rejected() {
    let (_, bytes) = create_vault(PASSWORD, &test_params()).unwrap();
    let mut bad = bytes.clone();
    // m_cost 位于偏移 8..12：改成 4 GiB（超过上限），应在派生前被拒绝。
    bad[8..12].copy_from_slice(&4_194_304u32.to_le_bytes());
    let err = unlock_vault(PASSWORD, &bad).unwrap_err();
    assert!(matches!(err, CoreError::InvalidParams(_)));
}

#[test]
fn custom_params_roundtrip() {
    let params = KdfParams::new(64, 2, 2);
    let (vault, bytes) = create_vault(PASSWORD, &params).unwrap();
    unlock_vault(PASSWORD, &bytes).unwrap();

    let saved = vault.save(PASSWORD, &params).unwrap();
    unlock_vault(PASSWORD, &saved).unwrap();
}

#[test]
fn remembered_key_unlock_and_save_roundtrip() {
    let params = test_params();
    let (vault, bytes) = create_vault(PASSWORD, &params).unwrap();
    let salt = vault.salt();
    let master_key = derive_master_key(PASSWORD, &salt, &params).unwrap();

    // 免密保存（如「记住本机」后的自动保存），并验证两种解锁路径。
    let rekeyed = vault.save_with_key(&master_key, &params).unwrap();
    unlock_vault(PASSWORD, &rekeyed).unwrap();
    unlock_vault_with_key(&rekeyed, &master_key).unwrap();

    // 原文件仍可用原密码解锁。
    unlock_vault(PASSWORD, &bytes).unwrap();
}

#[test]
fn salt_is_stable_across_saves() {
    let params = test_params();
    let (vault, _) = create_vault(PASSWORD, &params).unwrap();
    let salt = vault.salt();

    let first = vault.save(PASSWORD, &params).unwrap();
    let second = vault.save(PASSWORD, &params).unwrap();
    unlock_vault(PASSWORD, &first).unwrap();
    unlock_vault(PASSWORD, &second).unwrap();

    // 盐在创建与多次保存之间保持不变，保证派生出的 MK 始终有效。
    assert_eq!(salt, vault.salt());
}

#[test]
fn master_key_must_match_file_salt() {
    let params = test_params();
    let (vault, _) = create_vault(PASSWORD, &params).unwrap();

    // 用错误的盐派生 MK 保存后，原密码将无法解锁该文件。
    let wrong_master_key = derive_master_key(PASSWORD, &[9u8; 16], &params).unwrap();
    let saved = vault.save_with_key(&wrong_master_key, &params).unwrap();
    let err = unlock_vault(PASSWORD, &saved).unwrap_err();
    assert!(matches!(err, CoreError::WrongPasswordOrCorrupted));

    // 但该 MK 本人仍可解锁。
    unlock_vault_with_key(&saved, &wrong_master_key).unwrap();
}
