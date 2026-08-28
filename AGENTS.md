# 仓库指南（Repository Guidelines）

## 项目简介

本项目是一款跨平台密码管理 App，支持桌面端（Windows、macOS、Linux）与移动端（Android、iOS），核心功能是密码记录与安全管理。

## 技术栈与架构

- 基于 **Tauri 2**：一个工程覆盖全部 5 个平台，Rust 核心 + 系统 WebView UI
- UI 层采用 React + TypeScript + Vite（已落地于 `app/ui/`）
- 采用「共享核心 + 应用壳」结构：
  - `core/`：独立 Rust crate，不依赖 Tauri，包含加密、保险库格式、数据模型与业务逻辑
  - `app/`：Tauri 工程，`src-tauri/` 为 Rust 后端与平台接入，`ui/` 为前端源码
  - 移动端目标由 Tauri 生成（`src-tauri/gen/android`、`src-tauri/gen/apple`），不进版本库
- 安全架构：主密码 → Argon2id → 主密钥（MK）；MK 包装随机数据密钥（DEK），DEK 以 AEAD 加密整个保险库；改主密码仅重新包装 DEK；密钥使用后立即从内存擦除（zeroize）
- 存储：自建版本化加密文件；桌面端用系统凭据库（keyring）保存包装后的密钥

## 当前状态

仓库已具备可构建的 Cargo workspace 与 Tauri 应用壳：`core/` 已实现安全与数据层（Argon2id 派生、AEAD 加解密、版本化保险库格式、记录 CRUD、免密解锁所需的「已派生主密钥」API）并配有单元与集成测试；`app/` 已落地 Tauri 2 后端（会话状态、vault 命令层、keyring「记住本机」）与 React + TypeScript + Vite 前端（解锁/建库/记录 CRUD）。约定发生变化时，请同步更新本文档。

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

根目录为 Cargo workspace，Rust 命令统一在根目录运行：

- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --check`

Linux 桌面端编译 Tauri 需要系统开发包：`pkg-config`、`libwebkit2gtk-4.1-dev`、`libgtk-3-dev`、`libsoup-3.0-dev`、`libjavascriptcoregtk-4.1-dev`（含其传递依赖）。前端与 Tauri 命令在 `app/` 目录运行：

- `npm install`
- `npm run dev` — 仅启动 Vite 开发服务器
- `npm run build` — TypeScript 检查 + 前端产物构建
- `npm run format` / `npm run format:check` — Prettier 格式化/校验前端代码
- `npm run tauri dev` — 本地运行桌面应用

移动端目标由 Tauri 生成（`src-tauri/gen/`），不进版本库。

## 编码风格与命名约定

Rust 已配置 rustfmt 与 Clippy（workspace lints：`unsafe_code = forbid`、`clippy::all` 警告；提交前需通过 `cargo fmt --check` 与 `cargo clippy --workspace --all-targets -- -D warnings`）。前端使用 Prettier 统一格式化（`npm run format`，校验用 `npm run format:check`），ESLint 待引入。命名遵循语言生态（Rust `snake_case`；TypeScript `camelCase`、组件 `PascalCase`）。每个提交只包含一个逻辑变更。

## 测试指南

Rust 测试使用标准 `#[test]`：单元测试随源码模块存放，集成测试在 `core/tests/`。首批测试已覆盖 `core/`（Argon2id 派生含参考实现标准向量、AEAD 加解密、保险库格式版本化、CRUD 行为）。测试文件与源码路径对应，按行为命名（如 `test_short_password_rejected`）。

## 提交与拉取请求指南

提交遵循 Conventional Commits 规范：`feat:`、`fix:`、`docs:`、`test:`、`refactor:`、`chore:`。**提交信息（标题与正文）一律使用中文**，类型前缀按规范保留英文（如 `feat: 增加导出功能`）。提交应小而聚焦，标题使用祈使句并控制在 72 个字符以内。拉取请求应有清晰的标题与摘要，关联相关 issue；涉及界面或视觉变更时应附上截图，合并前必须通过测试与 lint 检查。提交前会自动执行 `cargo fmt` 与 Prettier 格式化（钩子在 `.githooks/pre-commit`，首次克隆后需执行 `git config core.hooksPath .githooks` 启用）。

## 版本管理

- 版本号唯一来源是根 `Cargo.toml` 的 `[workspace.package].version`；`app/src-tauri/tauri.conf.json` 不再写版本（打包时自动从 Cargo.toml 读取），`app/package.json` 由 release-please 同步
- 每次合并到 `master` 后，release-please（GitHub Actions）扫描 Conventional Commits，自动创建/更新 release PR：升版本号并维护根目录 `CHANGELOG.md`；合并该 PR 后自动打 `vX.Y.Z` tag 并创建 GitHub Release
- 版本策略：1.0 之前 `feat:` 与破坏性变更升 minor、`fix:` 升 patch；1.0 之后按标准 SemVer
- 界面显示的版本号通过 Tauri `getVersion()` 从构建产物读取，无需手工维护

## 安全与配置提示

密码管理类应用对安全要求高，请严格遵守：

- 严禁提交密钥、API Key、主密码或任何真实用户数据
- 敏感数据必须使用系统安全存储（如 iOS Keychain、Android Keystore、桌面系统凭据库）
- 在 `.gitignore` 中添加本地配置与环境文件的忽略规则（例如 `.env`、`*.local`）
- `assets/` 中的示例数据仅用于演示，绝不能包含真实凭据
- 日志与错误信息不得包含敏感内容
