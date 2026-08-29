# 代码审核报告：项目使用指南文档（docs/guide.md）

- 任务：项目使用指南文档
- 作者：Review（代码审核）
- 输入：集成分支 `task-docs-guide` 差异（`git diff origin/master...HEAD`）+ `prd.md` / `design.md` / `test-report.md` / `bugs.md`
- 审核对象：`docs/guide.md`（新增 124 行）+ `docs/tasks/docs-guide/` 任务产物（prd/plan/design/test-report/bugs）
- 结论：**verdict = approve**（securityPassed=true；无未解决 P0/P1 问题；3 项 P3 观察项，均为仓库既有或文档措辞级，不阻塞合并）

---

## 0. 审核范围与性质

本任务为**纯文档任务**：差异仅含 `docs/guide.md` 与任务产物文档，共 6 个文件、560 行新增，
**零 Rust / TypeScript 源码变更**（`core/`、`app/src-tauri/`、`app/ui/` 均未触碰）。
因此安全清单与门槛按「内容核对 + 回归验证」执行，与 test-report.md 第 1 节口径一致。

集成分支提交链（均为中文 Conventional Commits、单逻辑变更）：

| 提交 | 类型 | 变更 |
| --- | --- | --- |
| `9b9a388` | docs | 需求文档（prd.md） |
| `8bc25ee` | docs | 项目计划（plan.md） |
| `bbf7a04` | docs | 架构设计（design.md） |
| `3a3287b` | docs | 新增 项目使用指南文档（`docs/guide.md`，唯一实现交付物） |
| `7b78432` | test | 测试报告与缺陷记录（test-report.md + bugs.md） |

---

## 1. 安全清单逐项核对

| # | 检查项 | 结果 | 说明 |
| --- | --- | --- | --- |
| S-1 | Argon2id 参数（内存/迭代/并行度）与盐随机性 | ✅ 通过（N/A 代码面） | 零代码变更；指南**未复述**具体参数（正确做法：避免与 `docs/format.md` 重复/漂移），仅以「主密码 → Argon2id → MK」概述，与 `AGENTS.md` / `format.md` 表述无冲突 |
| S-2 | AEAD 用法（认证标签、nonce、密钥轮换路径） | ✅ 通过（N/A 代码面） | 零代码变更；指南「DEK 以 AEAD 加密整个保险库」「改密仅重包装 DEK」与 `AGENTS.md`、`docs/task-list.md` 已完成行一致 |
| S-3 | 密钥 zeroize 覆盖 | ✅ 通过（N/A 代码面） | 零代码变更；指南「密钥使用后立即从内存擦除（zeroize）」与 `AGENTS.md` 逐字一致 |
| S-4 | 密钥/凭据只进系统安全存储（keyring/Keychain/Keystore） | ✅ 通过（内容层） | 指南表述「桌面端用系统凭据库（keyring）保存…以实现「记住本机」免密解锁」与 `AGENTS.md` 一致；**实现层措辞差异为仓库既有问题**（`keyring_store.rs` 实际保存已派生 MK，见 bugs.md BUG-002，P3、非本任务引入、不构成可利用漏洞） |
| S-5 | 日志与错误信息无敏感内容 | ✅ 通过（N/A 代码面） | 零代码变更；对 `docs/guide.md` 全文 grep（password / secret / token / api_key / 私钥 / 示例口令等）**零命中**，无任何明文密码、密钥或凭据示例（S5 与 QA 报告一致） |
| S-6 | 依赖漏洞（cargo audit） | ✅ 通过（N/A） | 本机 `cargo audit` 不可用（未安装）；本次差异**零新增依赖**，无新增依赖风险敞口，故不阻塞 |
| S-7 | 不承诺未实现安全功能 / 无时间表 | ✅ 通过 | 指南将 V2 功能（自动填充、云同步、生物识别解锁等）、移动端构建与发布、Windows/macOS 打包一律标注「**尚未实现**」，并声明「以 `docs/task-list.md` 为准」；经与 `docs/task-list.md`「未实现」表逐行核对，未宣称自动锁定、剪贴板自动清除等未实现能力已具备，无任何时间表承诺（对应 design.md T1 缓解） |

**安全清单结论：全项通过，无 P0/P1。**

## 2. 规范风格核对

| # | 检查项 | 结果 | 说明 |
| --- | --- | --- | --- |
| C-1 | 提交信息中文（Conventional Commits） | ✅ 通过 | 5 个提交标题、正文均为中文，类型前缀规范（docs:/test:）；标题均 ≤72 字符、祈使句 |
| C-2 | 每个提交只含一个逻辑变更 | ✅ 通过 | 逐提交核对：prd / plan / design / guide.md 各单文件提交；test-report + bugs 为 QA 产物一个逻辑单元（2 文件） |
| C-3 | 命名规范（Rust snake_case / TS camelCase） | ✅ 通过（N/A） | 零代码变更，无命名可审 |
| C-4 | `core/` 无 Tauri 依赖 | ✅ 通过（N/A） | 差异未触碰 `core/` 任何文件；指南明确写「`core/` 是独立 Rust crate（**不依赖 Tauri**）」「禁止依赖 Tauri」，与架构约定一致 |
| C-5 | 中文文档、Markdown、仓库文档风格 | ✅ 通过 | `docs/guide.md` 全文中文，与既有 `docs/` 标题层级、表格、代码块风格一致 |

## 3. 一致性核对

### 3.1 实现 vs design.md

| design.md 决策 | 实现落实情况 | 结果 |
| --- | --- | --- |
| §6.1 固定四章：项目简介 / 快速开始 / 目录结构说明 / 相关文档导航 | `docs/guide.md` 恰为四章（快速开始含环境要求、构建、运行、测试与质量检查子节） | ✅ 一致 |
| §6.2 目录树用 ` ```text ` 纯文本树 | 目录树使用 ` ```text ` 代码块，风格同 `architecture.md` | ✅ 一致 |
| §6.2 命令用 ` ```bash ` 代码块、标注执行目录、以 AGENTS.md 为事实来源 | 每条命令均标注「仓库根目录 / app/ 目录」，命令与 `AGENTS.md` 及 `app/package.json` 实际脚本一致（`build = "tsc --noEmit && vite build"`、`dev = "vite"`、`format:check` 等，逐项核对无误） | ✅ 一致 |
| §6.3 不动 README、不合并/拆分、不加入口链接 | 差异中无 `README.md` 变更 | ✅ 一致 |
| §5.1 T1/T2/T3 缓解：不承诺未实现功能、零敏感示例、安全表述以既有文档为事实来源 | 见安全清单 S-5/S-7；指南安全概述与 `AGENTS.md`/`format.md` 无冲突（未复述参数细节，仅概述并指向 `docs/format.md` 场景说明） | ✅ 一致 |
| §2 归属边界：零代码变更 | 差异 100% 落在 `docs/` | ✅ 一致 |
| §4 数据模型：指南如需提及仅概述并指向 format.md | 指南未展开数据模型，仅链接 `format.md` | ✅ 一致 |

### 3.2 测试 vs prd.md AC（独立复核）

QA test-report 声称 AC-1..AC-9 覆盖 9/9。Review 对关键事实**独立复核**：

| AC | 独立复核结果 |
| --- | --- |
| AC-1 | `docs/guide.md` 存在，Markdown、中文 ✅ |
| AC-2 | 项目简介与 `AGENTS.md` / `task-list.md` 已完成行一致（Tauri 2 + Rust 核心、5 平台、CRUD、早期阶段）✅ |
| AC-3 | 环境要求含 Rust 工具链、Node.js/npm、五个 Linux 系统开发包，与 `AGENTS.md` 逐项一致 ✅ |
| AC-4 | 构建命令与目录：`cargo check --workspace` / `cargo build`（根目录）、`npm install` / `npm run build`（`app/`），与 `AGENTS.md`、`package.json` 一致 ✅ |
| AC-5 | 运行命令：`npm run tauri dev`（桌面完整体验）、`npm run dev`（仅前端静态页、不可调用后端），用途区分准确 ✅ |
| AC-6 | 测试命令与目录：根目录 `cargo test --workspace` 等；`app/` 下 `npm run format:check` ✅ |
| AC-7 | 目录结构：`core/src`、`core/tests`、`app/src-tauri`、`app/ui`、`docs/`（含 tasks/、team/）、`.githooks/`、`.github/`（release-please、format-check 工作流）、`.codex/rules/` 均真实存在；测试归属表述准确；`src-tauri/gen/` 不进版本库（`git ls-files` 为空、`.gitignore` 含 `app/src-tauri/gen/`）✅（**`assets/` 例外，见 ISS-001**） |
| AC-8 | 三个相对链接 `architecture.md` / `format.md` / `task-list.md` 目标文件均存在于 `docs/` ✅ |
| AC-9 | 与 `AGENTS.md`、`README.md`、`docs/task-list.md` 交叉核对：版本号唯一来源、安全架构、V1 范围、命令行为无冲突；V2 未实现、无时间表、无新技术决策 ✅ |

**AC 覆盖率 9/9 成立（G6 通过）。**

### 3.3 门槛独立复跑（回归验证）

| # | 门槛 | 结果 |
| --- | --- | --- |
| G1 | `cargo fmt --check`（根目录） | ✅ 通过（Review 独立复跑） |
| G2 | `cargo clippy --workspace --all-targets -- -D warnings`（根目录） | ✅ 通过（Review 独立复跑） |
| G3 | `cargo test --workspace`（根目录） | ✅ 通过（Review 独立复跑：13 集成 + 16 单元 + 0 doc，0 失败，与 QA 报告一致） |
| G4 | `npm run format:check`（`app/`） | ✅ 通过（Review 独立复跑） |
| G5 | `npm run build`（`app/`） | ✅ 通过（Review 独立复跑：tsc --noEmit + vite build 成功） |

> 复跑确认：`core.hooksPath=.githooks` 已在 worktree 生效（`git config core.hooksPath` 返回 `.githooks`）。

## 4. Issues

### ISS-001（P3 · 建议）：`assets/` 目录实际不存在，QA 报告表述有误

- **现象**：`docs/guide.md` 目录树与职责条目列出 `assets/`（「共享示例 / 演示数据（约定位置，见 AGENTS.md）」「当前仓库暂无内容，为约定目录」）；但 `assets/` 在集成分支与 master 的**文件系统与 git 树中均不存在**（`ls` 无此目录、`git ls-files` / `git ls-tree origin/master assets/` 均为空）。
- **严重性**：P3。指南以「约定位置」「为约定目录」「当前仓库暂无内容」如实标注，且 PRD AC-7 明确要求说明 `assets/`、`AGENTS.md` 亦将其列为约定目录，故**不构成误导**、不违反 AC-7 实质（非虚构「存在内容」的目录）。
- **次要问题**：QA test-report.md AC-7 行声称 `assets/`（空目录）「**均真实存在**」——该核实表述**不准确**（空目录不可被 git 跟踪，且物理上亦未创建）。
- **建议**：① 后续任务创建 `assets/` 目录（如加 `.gitkeep`）使约定落盘；② 或将指南措辞保持「约定位置」并在 test-report 中修正「真实存在」的表述。

### ISS-002（P3 · 既有）：`Cargo.lock` 版本漂移（bugs.md BUG-001）

- 复验确认：根 `Cargo.toml` = 0.2.0，`Cargo.lock` 中 `passwd-x-core` / `passwd-x-app` 仍为 0.1.0；**任何 cargo 命令都会弄脏工作区**（本审核复跑 G1–G3 即复现，已 `git checkout -- Cargo.lock` 还原，未纳入本审核提交）。
- 既有问题、非本任务引入、无功能影响。建议随任一后续 PR 同步 Cargo.lock。

### ISS-003（P3 · 既有）：「记住本机」凭据存储描述与实现不一致（bugs.md BUG-002）

- 指南/`AGENTS.md` 表述「keyring 保存**包装后的密钥**」，实现（`app/src-tauri/src/keyring_store.rs`）与 `docs/architecture.md` 为「保存**已派生的主密钥（MK）**」；指南忠实引用 `AGENTS.md`，属仓库既有文档漂移。
- 凭据仍由 OS 凭据库保护，不构成可利用漏洞；建议另立任务统一三处表述。

### 备注：cargo audit

- 本机未安装 `cargo audit`，无法运行；本次差异**零新增依赖**，无新增依赖风险敞口，不构成阻塞项。

## 5. 判定

| 维度 | 结果 |
| --- | --- |
| 安全清单 S-1..S-7 | 全部通过（securityPassed=true） |
| 规范风格 C-1..C-5 | 全部通过 |
| 一致性：实现 vs design.md / 测试 vs prd.md AC | 全部一致；AC 9/9 |
| 门槛 G1–G5 | 全部通过（Review 独立复跑） |
| 缺陷 | **无 P0/P1/P2**；3 项 P3（ISS-001 本任务文档表述级 + ISS-002/003 仓库既有，均不阻塞） |

**Review verdict = approve**，可以进入合并流程。

> 审核边界声明：本审核未修改任何代码或文档文件（除本报告 `review.md` 本身外）；`Cargo.lock` 复跑产生的漂移已还原。
