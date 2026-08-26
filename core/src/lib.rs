//! passwd-x 共享核心。
//!
//! 这里存放与平台无关的安全与业务逻辑：加密、保险库格式、数据模型。
//! 本 crate 禁止依赖 Tauri，保持框架无关（为将来 UniFFI 绑定留退路）。

/// 当前保险库格式版本。
pub const VAULT_FORMAT_VERSION: u32 = 1;
