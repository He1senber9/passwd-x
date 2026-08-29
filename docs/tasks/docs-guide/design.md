# 架构设计：项目使用指南文档（docs/guide.md）

- 任务：项目使用指南文档
- 作者：架构师
- 输入：`docs/tasks/docs-guide/prd.md`、`docs/tasks/docs-guide/plan.md`；参考 `docs/architecture.md`、`docs/format.md`、`docs/task-list.md`、`AGENTS.md`、`README.md`
- 状态：待实现 / 验证

## 1. 任务定性

本任务为**纯文档任务**（`needsBackend=false`、`needsFrontend=false`，见 plan.md 第 1 节）：

- 唯一交付物是新增单个文档 `docs/guide.md`；
- 不修改任何 Rust / TypeScript 源码，不修改 `README.md`、`AGENTS.md` 或既有 `docs/` 文档；
- 不做技术选型、不新增依赖、不引入新技术决策；
- 按 flow.md「特殊情形」，由通用实现角色直接在集成分支 `task-docs-guide` 产出，默认不创建 `-be` / `-fe` worktree。

因此本设计不含代码层面的接口或数据格式变更，重点在于：明确变更归属边界、给出指南的内容组织与呈现形式决策、并完成安全影响分析（纯文档任务同样适用安全硬性要求）。

## 2. 变更归属层

| 归属层 | 变更 | 理由 |
| --- | --- | --- |
| `core/` | 无变更 | 本任务不涉及加密、保险库格式、数据模型或业务逻辑；不触碰 `core/` 任何文件。`core/` 保持框架无关、不依赖 Tauri 的既有约定不受影响（指南描述安全架构时不得暗示 `core/` 依赖平台层） |
| `app/src-tauri/` | 无变更 | 不涉及 Tauri 后端、命令层、会话状态或 keyring 平台接入；不新增、不修改任何 Tauri command |
| `app/ui/` | 无变更 | 不涉及 React / TypeScript 界面代码 |
| `docs/`（文档层） | **新增 `docs/guide.md`** | 任务唯一交付物；`docs/tasks/docs-guide/` 下的 prd/plan/design/test-report/review 随集成分支一并入库，属文档层既有惯例（见 flow.md「任务产物位置」） |
| `README.md` / `AGENTS.md` / 既有 `docs/` 文档 | 无变更 | PRD「明确非目标」与 plan.md W2–W7 约束：README 与指南的去重、整合或增加入口链接，另立任务处理，本任务一律不动 |

> 归属边界结论：本任务**不产生任何代码变更**，交付物全部落在文档层。该结论与 plan.md 第 3、4 节的分支策略一致——无跨端改动，不需要 `-be` / `-fe` 分支。

## 3. Tauri command 签名

**不涉及。** 本任务不新增、不修改、不删除任何 Tauri command；指南按 PRD「明确非目标」不写内部 API 或代码级说明，仅从用户/开发者视角描述命令行为（如 `npm run tauri dev` 的用途），不涉及 `app/src-tauri/` 内部签名。

## 4. 数据模型变更

**不涉及。** 保险库文件格式保持 v1（`docs/format.md`），无版本化格式变更、无 KDF 参数调整、无兼容性迁移需求。指南如需提及数据模型，仅做与 `docs/format.md` 一致的概述（如 `Vault { version, kdf, entries[] }`），细节一律指向 `docs/format.md`，不在此引入新表述。

## 5. 安全影响分析

本任务**不触碰加密、密钥、凭据库或日志相关代码**，无新增攻击面、无密钥生命周期变更。但作为密码管理类项目的文档，仍需评估内容层面的安全影响：

### 5.1 威胁与缓解

| # | 威胁 | 影响 | 缓解措施 |
| --- | --- | --- | --- |
| T1 | 指南承诺未实现的安全功能（如生物识别解锁、云同步加密、自动填充），或为 V2 功能给出时间表 | 用户/开发者产生虚假安全感或错误预期；AC-9 不通过 | 严格以 `docs/task-list.md` 为功能现状事实来源：V2 功能一律以「未实现 / V2 规划」呈现，不写任何实现时间表；「快速开始」只描述已落地能力（本地加密保险库 + 主密码解锁、CRUD、免密解锁凭据保存等） |
| T2 | 指南泄露敏感内容：写入示例主密码、真实凭据、密钥或凭据库条目 | 违反「严禁提交密钥、API Key、真实用户数据」硬性要求；安全声誉受损 | 全文零敏感示例：不出现任何形式的密码/密钥占位符之外的明文示例；涉及安全架构的描述仅引用 `docs/architecture.md` 的密钥体系叙述（Argon2id → MK → 包装 DEK → AEAD、zeroize、日志不含敏感内容），不展开实现细节；提交前按 plan W7 交叉核对 |
| T3 | 指南与现有文档的安全表述冲突（如 Argon2id 参数、AEAD 算法、keyring 用途描述不一致） | 误导读者，削弱文档可信度；AC-9 不通过 | 以 `AGENTS.md`、`docs/architecture.md`、`docs/format.md` 为唯一事实来源；若指南概述安全架构，参数与算法引用必须与 format.md 一致（Argon2id 默认 m=19456 KiB、t=2、p=1；XChaCha20-Poly1305；MK/DEK 使用后 zeroize；解锁失败统一报「密码错误或数据损坏」等），详细参数一律链接 `docs/format.md` 而非在指南中复制全文 |

### 5.2 安全基线一致性

指南「项目简介 / 快速开始」中涉及安全的表述必须与安全基线一致：V1 安全基线是「本地加密保险库 + 主密码解锁」，桌面端「记住本机」凭据存入系统凭据库（keyring）需用户显式勾选——这些是**已实现**能力（见 `docs/task-list.md` 已完成清单），可在指南中如实呈现，但不得夸大（如不得称「自动锁定」「剪贴板自动清除」等未实现项已具备）。

### 5.3 结论

本任务安全影响为**内容层、零代码面变更**：无新增威胁模型输入，不改变密钥派生、加密、存储或凭据库行为；主要风险是文档内容与现状漂移或敏感信息泄露，通过「以既有文档为唯一事实来源 + 不承诺未实现功能 + 零敏感示例」三项缓解措施控制，并纳入 AC-9 由 QA 逐条核对。

## 6. 指南设计决策（架构师职权，对应 PRD 第 5 节委托项）

### 6.1 章节组织

固定四章，与 PRD V1 范围一一对应，便于 QA 按 AC-1..AC-9 核对：

1. **项目简介** —— 定位（Tauri 2 + Rust 核心、跨平台密码管理 App）、平台覆盖（桌面 Windows/macOS/Linux + 移动 Android/iOS）、核心功能（密码记录与安全管理）、当前阶段（早期开发阶段，V1 记录 CRUD 已落地）。对应 AC-2。
2. **快速开始** —— 环境要求（Rust 工具链、Node.js/npm、Linux 桌面端 Tauri 系统开发包）、构建命令（根目录 `cargo check --workspace` / `cargo build`；`app/` 目录 `npm install`、`npm run build`）、运行命令（`app/` 目录 `npm run tauri dev`；可选纯前端 `npm run dev` 及用途说明）、测试命令（根目录 `cargo test --workspace`；`app/` 目录 `npm run format:check`）。每条命令标注执行目录。对应 AC-3..AC-6。
3. **目录结构说明** —— `core/`（共享 Rust 核心、不依赖 Tauri）、`app/src-tauri/`（Tauri 后端与平台接入）、`app/ui/`（React 前端）、`assets/`、`docs/` 等职责；测试归属（Rust 集成测试在 `core/tests/`、前端测试在 `app/` 内）；`src-tauri/gen/` 不进版本库。对应 AC-7。
4. **相关文档导航** —— 相对链接指向 `architecture.md`、`format.md`、`task-list.md`，各附一句适用场景说明（架构与安全设计 / 加密保险库文件格式 / 功能任务清单）。对应 AC-8。

不新增 CONTRIBUTING、FAQ、故障排查、用户操作手册等章节（PRD 非目标）。

### 6.2 呈现形式

- **目录树**：使用 Markdown 代码块内的纯文本树（` ```text `），与 `docs/architecture.md` 第 14-29 行的文本树风格一致，不引入图表工具或 Mermaid（不新增依赖、保持纯 Markdown）。
- **命令**：使用 ` ```bash ` 代码块逐条列出，命令与执行目录以 `AGENTS.md`「构建、测试与开发命令」章节为唯一事实来源（plan W1/W2 事实核对项）。
- **文档风格**：全文中文、Markdown；标题层级、表格、列表风格与既有 `docs/` 文档保持一致；文中不写内部 API、代码级说明（PRD 非目标）。

### 6.3 README 入口决策

本任务**不在 README 中增加指向指南的链接**，也**不合并 / 拆分** README：PRD「明确非目标」禁止修改 `README.md`，与 README 的去重整合、入口链接另立任务处理（PRD 第 5 节委托项在本任务内的裁决为「不动 README」）。`docs/guide.md` 作为 `docs/` 内的先读文档独立存在。

### 6.4 命令验证流程

实现角色按 plan W8 实际执行 G1–G5（`cargo fmt --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`、`app/` 下 `npm run format:check`、`npm run build`）作为回归验证，确认指南中写明的命令可用；`docs/guide.md` 不在 Prettier 覆盖范围（`app/` 内），G4 仅确认工作区一致。若系统开发包缺失导致 G5 阻塞，按 plan R7 在 test-report 中如实记录，交 Review 判断。

## 7. 与 plan.md 的一致性

| plan.md 约定 | 本设计落实 |
| --- | --- |
| 纯文档任务，集成分支直接产出，不建 `-be` / `-fe` | 第 2 节归属边界：无代码变更，无需开发分支 |
| W1 事实核对 | 第 6.2 节：命令以 `AGENTS.md` 为唯一事实来源；目录以实际仓库布局为据 |
| W2–W7 章节实现 | 第 6.1 节章节组织与 AC 映射 |
| W8 门槛验证 | 第 6.4 节验证流程，DoD 映射见 plan 第 5 节 G1–G7 |
| 风险 R1–R7 | 第 5.1 节（T1–T3 对应 R1/R4 内容风险）、第 6.4 节（R7）逐项覆盖 |
| 提交规范（中文 Conventional Commits、单一逻辑提交） | 第 8 节 |

## 8. 交付物与提交

- 交付物：`docs/guide.md`（由通用实现角色在集成分支产出）。
- 本设计文档提交：`docs: 新增 项目使用指南文档 架构设计（design.md）`（docs 类型、中文标题，单一逻辑变更）。
- 提交前确认 `core.hooksPath=.githooks` 已配置（已在 worktree 生效），提交钩子自动执行 cargo fmt / Prettier。
