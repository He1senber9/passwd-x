# passwd-x

跨平台密码管理 App（Tauri 2 + Rust 核心），当前处于早期开发阶段。

## 项目结构

- `core/` — 与平台无关的安全与业务逻辑（加密、保险库格式、数据模型）
- `app/` — Tauri 2 应用（后端 `src-tauri/`，前端 `ui/`）
- `docs/` — 架构与数据格式文档

## 开发命令

根目录为 Cargo workspace，Rust 命令统一在根目录运行：

- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --check`

前端与桌面应用命令在 `app/` 目录运行：

- `npm install`
- `npm run tauri dev` — 本地运行桌面应用

Linux 桌面端需先安装 Tauri 的系统开发包（webkit2gtk-4.1、gtk-3、libsoup-3 等，见 `AGENTS.md`）。

详见 `AGENTS.md` 与 `docs/architecture.md`。
