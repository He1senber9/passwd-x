# 仓库指南（Repository Guidelines）

## 项目简介

本项目是一款跨平台密码管理 App，支持桌面端（Windows、macOS、Linux）与移动端（Android、iOS），核心功能是密码记录与安全管理。

## 技术栈与架构

- 基于 **Tauri 2**：一个工程覆盖全部 5 个平台，Rust 核心 + 系统 WebView UI
- UI 层采用 React + TypeScript + Vite（方案建议，最终选型待确认）
- 采用「共享核心 + 应用壳」结构：
  - `core/`：独立 Rust crate，不依赖 Tauri，包含加密、保险库格式、数据模型与业务逻辑
  - `app/`：Tauri 工程，`src-tauri/` 为 Rust 后端与平台接入，`ui/` 为前端源码
  - 移动端目标由 Tauri 生成（`src-tauri/gen/android`、`src-tauri/gen/apple`），不进版本库
- 安全架构：主密码 → Argon2id → 主密钥（MK）；MK 包装随机数据密钥（DEK），DEK 以 AEAD 加密整个保险库；改主密码仅重新包装 DEK；密钥使用后立即从内存擦除（zeroize）
- 存储：自建版本化加密文件；桌面端用系统凭据库（keyring）保存包装后的密钥

## 当前状态

仓库刚起步：已初始化 Git 并关联远程，已提交 AGENTS.md 与 `.codex/rules/` 命令授权，尚无源代码与工具链。约定发生变化时，请同步更新本文档。

## 项目结构与模块组织

- `core/` — 跨平台共享代码（禁止依赖 Tauri）
- `app/src-tauri/` — Tauri Rust 后端与平台接入
- `app/ui/` — 前端源码
- 测试归属各包：Rust 集成测试在 `core/tests/`，前端测试与 E2E 在 `app/` 内，顶层不设统一 `tests/` 目录
- `assets/` — 仅放共享示例/演示数据；应用图标在 `src-tauri/icons/`，UI 图片在 `app/ui/assets/`
- `docs/` — 设计文档（`architecture.md`、数据格式等）
- 根目录 — `Cargo.toml`（workspace，成员 `core` 与 `app/src-tauri`）、`README.md`、`.gitignore`、`.codex/rules/`

核心规则：安全与业务逻辑只进 `core/`，UI 与平台代码不得包含密码逻辑；`core/` 保持框架无关，为将来以 UniFFI 绑定原生移动壳留退路。

## V1 功能范围

V1 只做最小可用的密码记录 CRUD，覆盖桌面端与移动端：

- 新增账号/密码记录
- 编辑账号/密码信息
- 删除记录

安全基线（非可选功能，所有版本的基础）：本地加密保险库 + 主密码解锁，遵循上文安全架构。

V2 及以后：自动填充、云同步、生物识别解锁、搜索/分类/标签、TOTP 等。

## 构建、测试与开发命令

根目录为 Cargo workspace，Rust 命令统一在根目录运行（`cargo check`、`cargo test`）；前端与 Tauri 命令随 `app/` 落地后记录（预期 `tauri dev` 本地运行）。在此之前，不要假定默认命令。

## 编码风格与命名约定

尚未配置格式化与 lint 工具，第一个代码 PR 应引入：Rust 用 rustfmt + Clippy，前端用 ESLint + Prettier。命名遵循语言生态（Rust `snake_case`；TypeScript `camelCase`、组件 `PascalCase`）。每个提交只包含一个逻辑变更。

## 测试指南

尚未配置测试框架。首批测试优先覆盖 `core/`（Argon2id 派生、AEAD 加解密、保险库格式版本化）。测试文件与源码路径对应，按行为命名（如 `test_short_password_rejected`）。

## 提交与拉取请求指南

已有提交遵循 Conventional Commits 规范：`feat:`、`fix:`、`docs:`、`test:`、`refactor:`、`chore:`。提交应小而聚焦，标题使用祈使句并控制在 72 个字符以内。拉取请求应有清晰的标题与摘要，关联相关 issue；涉及界面或视觉变更时应附上截图，合并前必须通过测试与 lint 检查。

## 安全与配置提示

密码管理类应用对安全要求高，请严格遵守：

- 严禁提交密钥、API Key、主密码或任何真实用户数据
- 敏感数据必须使用系统安全存储（如 iOS Keychain、Android Keystore、桌面系统凭据库）
- 在 `.gitignore` 中添加本地配置与环境文件的忽略规则（例如 `.env`、`*.local`）
- `assets/` 中的示例数据仅用于演示，绝不能包含真实凭据
- 日志与错误信息不得包含敏感内容
