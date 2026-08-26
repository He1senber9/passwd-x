# passwd-x

跨平台密码管理 App（Tauri 2 + Rust 核心），当前处于早期开发阶段。

## 项目结构

- `core/` — 与平台无关的安全与业务逻辑（加密、保险库格式、数据模型）
- `app/` — Tauri 应用（后端 `src-tauri/`，前端 `ui/` 待落地）
- `docs/` — 架构与数据格式文档

## 开发命令

根目录为 Cargo workspace，Rust 命令统一在根目录运行：

- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --check`

详见 `AGENTS.md` 与 `docs/architecture.md`。
