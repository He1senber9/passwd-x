# 架构设计

## 概述

基于 Tauri 2 的跨平台密码管理 App，一个工程覆盖桌面端（Windows、macOS、Linux）与移动端（Android、iOS）。核心功能为账号/密码记录的本地安全管理。

## 技术选型

- 框架：Tauri 2（Rust 核心 + 系统 WebView UI）
- 前端：React + TypeScript + Vite（待确认）
- Rust 核心依赖（规划）：`argon2`（Argon2id）、`chacha20poly1305` 或 `aes-gcm`、`zeroize`/`secrecy`、`serde`、`keyring-rs`（桌面凭据库）
- 存储：自建版本化加密文件；桌面端用系统凭据库保存包装后的密钥

## 目录结构

```text
passwd-x/
├── Cargo.toml          # workspace：core + app/src-tauri
├── core/               # 共享核心 crate（不依赖 Tauri）
│   ├── src/
│   └── tests/
├── app/
│   ├── src-tauri/      # Tauri Rust 后端与平台接入
│   │   └── gen/        # 生成的移动端工程（不进版本库）
│   └── ui/             # 前端源码
├── assets/             # 共享示例/演示数据
├── docs/               # 设计文档
└── .codex/rules/       # 项目级命令授权
```

## 安全架构

三段式密钥体系：

1. 主密码 → Argon2id（带盐、高内存参数）→ 主密钥（MK）
2. 随机生成数据密钥（DEK），由 MK 包装；改主密码仅重新包装 DEK
3. DEK 以 AEAD（AES-256-GCM 或 XChaCha20-Poly1305）加密整个保险库

加密文件格式（v1）：版本号 + KDF 参数与盐 + 包装后的 DEK + nonce + 密文 + 认证标签。密钥使用后立即以 `zeroize` 从内存擦除；日志与错误信息不得包含敏感内容。

## 数据模型（v1）

```text
Vault { version, kdf, entries[] }
Entry { id, title, username, password, urls[], notes, created_at, updated_at }
```

v1 整体加密一个保险库文件；若未来引入云同步，再迁移为条目级加密（同步协议另立文档）。

## V1 功能范围

- 新增、编辑、删除账号/密码记录（桌面端 + 移动端）
- 安全基线：本地加密保险库 + 主密码解锁
- 明确不做：自动填充、云同步、生物识别、搜索/分类/标签、TOTP（归入 V2 及以后）

## 平台接入

- 桌面端：`keyring-rs` 访问系统凭据库（Windows Credential Manager、macOS Keychain、Linux Secret Service）
- 移动端：Keychain / Keystore 方案待定（V1 仅需保存包装后的密钥）
- V2 再评估：生物识别解锁、自动填充（iOS Credential Provider / Android Autofill 原生扩展）
