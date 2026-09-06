# 工程规范（Engineering Guidelines）

本文档承载项目具体工程约定，由 `AGENTS.md` 按角色路由指向。适合开发、测试、审核等需要动手/把关的角色阅读。

## 构建、测试与开发命令

根目录为 Cargo workspace，Rust 命令统一在根目录运行：

- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --check`

前端与 Tauri 命令在 `app/` 目录运行：

- `npm install`
- `npm run dev` — 仅启动 Vite 开发服务器
- `npm run build` — TypeScript 检查 + 前端产物构建
- `npm run format` / `npm run format:check` — Prettier 格式化/校验
- `npm run tauri dev` — 本地运行桌面应用

Linux 桌面端编译 Tauri 需要系统开发包（`pkg-config`、`libwebkit2gtk-4.1-dev`、`libgtk-3-dev`、`libsoup-3.0-dev`、`libjavascriptcoregtk-4.1-dev`）。构建安装包（`npm run tauri build`）还需签名私钥环境变量 `TAURI_SIGNING_PRIVATE_KEY`（或 `TAURI_SIGNING_PRIVATE_KEY_PATH`）。移动端目标由 Tauri 生成到 `src-tauri/gen/`，不进版本库。

## 编码风格与命名约定

- Rust：rustfmt + Clippy 零警告（workspace lints：`unsafe_code = forbid`、`clippy::all`）。
- 前端：Prettier 统一格式化；ESLint 待引入。
- 命名遵循语言生态：Rust `snake_case`；TypeScript `camelCase`、组件 `PascalCase`。
- 每个提交只包含一个逻辑变更。

## 测试指南

- Rust 单元测试随源码模块存放，集成测试在 `core/tests/`；前端测试与 E2E 在 `app/` 内。
- 测试按行为命名（如 `test_short_password_rejected`）。

## 质量门禁（合并前必须全部通过）

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run format:check`（`app/`）
- `npm run build`（`app/`）

测试与审核角色对 P0/P1 缺陷/问题一票否决。

## 提交与拉取请求指南

- 提交遵循 Conventional Commits：`feat:`、`fix:`、`docs:`、`test:`、`refactor:`、`chore:`。
- 提交信息（标题与正文）一律使用中文，类型前缀保留英文（如 `feat: 增加导出功能`）；标题祈使句、≤72 字符；每个提交只含一个逻辑变更。
- PR 标题与摘要使用中文，说明改动、关联任务文档与门禁结论；涉及界面/视觉变更附截图。
- 提交前自动执行 `cargo fmt` 与 Prettier（钩子在 `.githooks/pre-commit`，需 `git config core.hooksPath .githooks` 启用）。

## 版本与发布

- 版本号唯一来源：根 `Cargo.toml` 的 `[workspace.package].version`；`tauri.conf.json` 不再写版本。
- 版本策略：1.0 前 `feat:` 与破坏性变更升 minor、`fix:` 升 patch；1.0 后按标准 SemVer。
- 发布排程与发布分支由 `docs/team.md` 与 `.github/workflows/release*.yml` 定义。
- UI 版本号通过 Tauri `getVersion()` 从构建产物读取。

## 相关文档

- `docs/architecture.md` — 架构与安全设计
- `docs/format.md` — 保险库文件格式
