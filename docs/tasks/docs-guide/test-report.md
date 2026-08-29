# 测试报告：项目使用指南文档（docs/guide.md）

- 任务：项目使用指南文档
- 作者：QA（高级测试工程师）
- 验收标准：`docs/tasks/docs-guide/prd.md`（AC-1..AC-9）
- 实现依据：`docs/tasks/docs-guide/design.md`、`docs/tasks/docs-guide/plan.md`
- 被验收对象：集成分支 `task-docs-guide`，提交 `3a3287b docs: 新增 项目使用指南文档`
- 结论：**verdict = pass**（AC 覆盖率 9/9 = 100%，无未解决 P0/P1 缺陷；存在 2 项 P3 建议，见 `bugs.md`）

## 1. 验收范围与性质

本任务为**纯文档任务**（`needsBackend=false`、`needsFrontend=false`）：唯一交付物是新增
`docs/guide.md`，不产生任何 Rust / TypeScript 源码变更（实现提交仅含 `docs/guide.md`，
124 行）。因此：

- AC 核对以文档内容与仓库事实（`AGENTS.md`、`README.md`、`docs/architecture.md`、
  `docs/format.md`、`docs/task-list.md`、实际目录布局、`app/package.json`）交叉比对为主；
- 门槛 G1–G5 与负面安全测试按 plan.md 第 5 节以**回归验证**方式复跑；
- 本任务无可执行测试用例（无代码变更），AC 的「测试用例」维度由回归测试套件
  （core 单元 + 集成测试）与文档事实核对共同支撑。

## 2. AC 覆盖矩阵（9/9 = 100%）

| AC | 验收内容 | 验证方式 | 结果 |
| --- | --- | --- | --- |
| AC-1 | `docs/guide.md` 存在（Markdown、中文） | `ls docs/guide.md`；通读全文（中文、Markdown 语法合法） | ✅ 通过 |
| AC-2 | 项目简介准确：Tauri 2 + Rust 核心、跨平台密码管理 App、桌面（Windows/macOS/Linux）+ 移动（Android/iOS）、核心功能密码记录与安全管理、早期开发阶段 | 与 `AGENTS.md`、`docs/architecture.md`、`README.md`、`docs/task-list.md` 交叉核对（guide.md 第 5–14 行） | ✅ 通过 |
| AC-3 | 快速开始给出环境要求：Rust 工具链、Node.js/npm、Linux 桌面端 Tauri 系统开发包 | guide.md 第 18–23 行；五个系统包清单与 `AGENTS.md`「构建、测试与开发命令」逐项一致 | ✅ 通过 |
| AC-4 | 构建命令与执行目录正确：根目录 `cargo check --workspace` / `cargo build`；`app/` 目录 `npm install` / `npm run build` | guide.md 第 25–41 行；与 `AGENTS.md` 一致；`app/package.json` 脚本 `build = "tsc --noEmit && vite build"`、`dev = "vite"` 与指南注释一致；G1–G5 实测命令可用 | ✅ 通过 |
| AC-5 | 运行命令与执行目录正确：`app/` 下 `npm run tauri dev`、可选 `npm run dev`，并说明各自用途 | guide.md 第 43–53 行；与 `AGENTS.md`、`README.md` 一致；用途区分表述准确（tauri dev = 桌面应用完整体验；dev = 仅前端静态页、不可调用后端） | ✅ 通过 |
| AC-6 | 测试命令与执行目录正确：根目录 `cargo test --workspace`；`app/` 目录 `npm run format:check` | guide.md 第 55–74 行；与 `AGENTS.md` 一致；G3 / G4 实测通过 | ✅ 通过 |
| AC-7 | 目录结构说明与仓库实际一致：`core/`（不依赖 Tauri）、`app/src-tauri/`、`app/ui/`、`assets/`、`docs/` 等职责；测试归属；`src-tauri/gen/` 不进版本库；无虚构目录 | guide.md 第 76–114 行目录树与各职责条目；逐项对照实际目录（`core/src|tests`、`app/src-tauri/src`、`app/ui/src`、`assets/`（空目录，指南如实标注「暂无内容，为约定目录」）、`docs/`、`.githooks/`、`.github/workflows/`、`.codex/rules/` 均真实存在）；`git ls-files app/src-tauri/gen` 为空（不进版本库，`.gitignore` 含 `app/src-tauri/gen/`） | ✅ 通过 |
| AC-8 | 链接既有文档且目标存在：相对链接指向 `architecture.md`、`format.md`、`task-list.md` | guide.md 第 116–124 行；三个目标文件均存在于 `docs/`，链接为同目录相对路径；各文档适用场景描述与文档实际内容一致 | ✅ 通过 |
| AC-9 | 内容与现有文档一致、不引入新承诺：版本号来源、安全架构、V1 功能范围、命令行为无冲突；不承诺未实现功能/时间表；不引入新技术决策 | 全文与 `AGENTS.md`、`README.md`、`docs/` 交叉核对（详见第 4 节核对清单）；未发现冲突表述或新承诺 | ✅ 通过 |

**覆盖率：9/9 = 100%**，满足 G6。

## 3. 门槛复跑结果（G1–G5）

在集成分支 worktree（`/home/vann/Work/passwd-x/.worktrees/task-docs-guide`）复跑：

| # | 门槛 | 命令 | 结果 |
| --- | --- | --- | --- |
| G1 | Rust 格式 | `cargo fmt --check`（根目录） | ✅ 通过 |
| G2 | Clippy 零警告 | `cargo clippy --workspace --all-targets -- -D warnings`（根目录） | ✅ 通过 |
| G3 | Rust 测试 | `cargo test --workspace`（根目录） | ✅ 通过：16 单元 + 13 集成（`vault_flow.rs`）+ 0 doc = 29 项，0 失败 |
| G4 | 前端格式 | `npm run format:check`（`app/`） | ✅ 通过：All matched files use Prettier code style! |
| G5 | 前端构建 | `npm run build`（`app/`） | ✅ 通过：tsc --noEmit + vite build 成功 |

门槛 G1–G5 全部通过，与 plan.md 第 5 节 DoD 一致（本任务为纯文档，门槛为回归验证）。

## 4. AC-9 一致性交叉核对清单

| 核对项 | 指南表述 | 事实来源 | 结论 |
| --- | --- | --- | --- |
| 项目定位 | Tauri 2 + Rust 核心、跨平台密码管理 App、5 平台 | `AGENTS.md`、`docs/architecture.md` | 一致 |
| 核心功能 | 密码记录与安全管理；V1 记录 CRUD（新增/编辑/删除/列表） | `AGENTS.md`、`docs/task-list.md`（已完成行） | 一致 |
| 安全架构 | 主密码 → Argon2id → MK；MK 包装 DEK；AEAD 加密全库；改密仅重包装 DEK；zeroize | `AGENTS.md`、`docs/architecture.md`、`docs/format.md` | 一致 |
| 记住本机 | 桌面端用系统凭据库（keyring）保存密钥免密解锁 | `AGENTS.md` | 一致（措辞与 AGENTS.md 相同；代码侧表述差异见 bugs.md BUG-002，P3、非本任务引入） |
| 当前阶段 | 早期开发阶段；移动端构建/发布、Windows/macOS 打包、V2 功能（自动填充、云同步、生物识别）未实现 | `docs/task-list.md`（未实现行） | 一致，且明确「以 task-list.md 为准」，无时间表承诺 |
| 版本号来源 | 根 `Cargo.toml` 唯一来源；CHANGELOG 由 release-please 维护 | `AGENTS.md`「版本管理」 | 一致 |
| 命令与目录 | G1–G5 涉及命令均标注执行目录 | `AGENTS.md`；G1–G5 实测 | 一致且可用 |
| 系统依赖 | 五个开发包清单 | `AGENTS.md` | 逐项一致 |
| 测试归属 | 集成测试在 `core/tests/`、前端测试在 `app/` 内、顶层无统一 `tests/` | `AGENTS.md`、实际布局 | 一致 |
| gen/ 不进版本库 | 移动端工程由 Tauri 生成、不进版本库 | `AGENTS.md`、`.gitignore`、`git ls-files` | 一致 |
| 新技术决策 | 无技术选型、无新依赖、未承诺 V2 时间表 | 全文扫描 | 无 |

## 5. 负面安全测试（本变更可用部分）

本任务零代码变更，负面安全测试以 `core/` 既有安全层与 `app/src-tauri/` 命令层的**回归验证** +
**指南内容安全扫描**落实（测试均随 G3 实际执行）：

| # | 测试项 | 验证方式 | 结果 |
| --- | --- | --- | --- |
| S1 | 错误主密码解锁失败且不泄露信息 | 集成测试 `wrong_password_rejected`、`change_password_rewraps_dek_and_old_password_stops_working`、`master_key_must_match_file_salt`、`short_password_rejected_on_create_and_unlock` 全部通过；错误统一映射为 `CoreError::WrongPasswordOrCorrupted`（密码错误与数据损坏不可区分） | ✅ 通过 |
| S2 | 篡改保险库文件被检测（认证失败） | 集成测试 `tampered_ciphertext_rejected`（翻转密文末字节）、`tampered_header_rejected`（翻转盐字节）、`truncated_file_rejected`、`unreasonable_kdf_params_in_file_rejected`（恶意 m_cost 在派生前被拒）全部通过，均报认证失败 | ✅ 通过 |
| S3 | 格式版本兼容 | 当前格式 v1 无迁移需求；`unsupported_file_version_rejected`（version=2 → `InvalidFormat`）通过；`custom_params_roundtrip`、`salt_is_stable_across_saves` 通过；`docs/format.md` 版本策略（拒绝未知版本）与实现一致 | ✅ 通过 |
| S4 | 日志 / 错误信息脱敏 | `core/` 与 `app/src-tauri/` 全量 grep：无 `println!` / `eprintln!` / `log::` / `tracing` / `dbg!`（仅测试内 `unwrap()`）；`core/src/error.rs` 明确「错误消息不包含任何密码或密钥材料」，`WrongPasswordOrCorrupted` 为通用文案；单元测试 `debug_output_redacts_password` 验证 `Debug` 输出脱敏密码；keyring 层错误仅含 OS 级通用文案 | ✅ 通过 |
| S5 | 指南内容不含敏感信息（T2 威胁） | 对 `docs/guide.md` 全文 grep（password / secret / token / api_key / 私钥 / 示例口令等）：零命中；全文无任何明文密码、密钥或凭据示例 | ✅ 通过 |
| S6 | 指南不承诺未实现安全功能（T1 威胁） | AC-9 核对：V2 功能一律标注「尚未实现」，无时间表；未宣称自动锁定、剪贴板自动清除等未实现能力 | ✅ 通过 |

负面安全测试 S1–S6 全部通过。

## 6. 缺陷列表

本任务未发现 P0 / P1 / P2 缺陷。发现 2 项 P3（建议）级观察项，均为**仓库既有问题、非本任务引入**，
不影响本次判定，详见 `docs/tasks/docs-guide/bugs.md`：

- **BUG-001（P3）**：根 `Cargo.toml`（version 0.2.0）与提交的 `Cargo.lock`（0.1.0）版本漂移，
  任何 cargo 命令都会弄脏工作区（本报告撰写时已 `git checkout -- Cargo.lock` 还原，未纳入 QA 提交）。
- **BUG-002（P3）**：`AGENTS.md` / 本指南表述「keyring 保存**包装后的密钥**」，而 `app/src-tauri/src/keyring_store.rs`
  实际将**已派生的主密钥（MK）**直接存入系统凭据库（由 OS 凭据库保管）；指南忠实引用 `AGENTS.md`，
  但建议后续任务统一描述。

## 7. 判定

- AC 覆盖率：**9/9 = 100%**（G6 通过）
- 门槛 G1–G5：全部通过
- 负面安全测试 S1–S6：全部通过
- 缺陷：无 P0/P1/P2；2 项 P3（既有、非本任务引入）

**QA verdict = pass**，可以进入 Review 阶段。
