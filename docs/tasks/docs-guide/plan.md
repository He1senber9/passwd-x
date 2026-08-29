# 项目计划：项目使用指南文档（docs/guide.md）

- 任务：项目使用指南文档
- 作者：PjM
- 输入：`docs/tasks/docs-guide/prd.md`（PM 已产出）
- 状态：待设计 / 实现 / 验证

## 1. 任务概述

按 PRD 新增单一文档 `docs/guide.md`，作为项目第一份「先读我」使用指南：项目简介、快速开始（环境要求 + 构建/运行/测试命令与执行目录）、目录结构说明、相关文档导航。目标读者为刚加入项目、尚未熟悉仓库的开发者（含 AI 团队角色）。本任务为**纯文档任务**（`needsBackend=false`、`needsFrontend=false`），不修改任何 Rust / TypeScript 源码，不修改 `README.md`、`AGENTS.md` 或既有 `docs/` 文档，不做技术选型、不新增依赖。

## 2. WBS 任务分解（对应 AC-1..AC-9）

| 编号 | 任务 | 对应 AC | 交付物 / 验证方式 | 可独立完成 |
| --- | --- | --- | --- | --- |
| W1 | 事实核对：通读 `AGENTS.md`、`README.md`、`docs/architecture.md`、`docs/format.md`、`docs/task-list.md`，核对仓库实际目录布局与命令执行目录 | AC-4、AC-7、AC-9（信息基础） | 事实清单（命令 × 目录、目录职责、版本号来源、安全架构、V1 范围），写入实现时的草稿 | 是 |
| W2 | 创建 `docs/guide.md` 骨架并完成「项目简介」章节：Tauri 2 + Rust 核心、桌面（Windows/macOS/Linux）+ 移动（Android/iOS）、密码记录与安全管理、当前早期开发阶段 | AC-1、AC-2 | `docs/guide.md` 存在（Markdown、中文）；QA 核对 AC-2 表述 | 是 |
| W3 | 「快速开始：环境要求」章节：Rust 工具链、Node.js / npm，以及 Linux 桌面端 Tauri 系统开发包（`pkg-config`、`libwebkit2gtk-4.1-dev`、`libgtk-3-dev`、`libsoup-3.0-dev`、`libjavascriptcoregtk-4.1-dev` 等）或指向 `AGENTS.md` 对应章节 | AC-3 | 章节内容与 `AGENTS.md`「构建、测试与开发命令」一致 | 是 |
| W4 | 「快速开始：构建 / 运行 / 测试命令」章节：构建（根目录 `cargo check --workspace` / `cargo build`；`app/` 目录 `npm install`、`npm run build`）、运行（`app/` 下 `npm run tauri dev`，可选 `npm run dev` 及用途说明）、测试（根目录 `cargo test --workspace`；`app/` 下 `npm run format:check`），每条标注执行目录 | AC-4、AC-5、AC-6 | 章节内容与 `AGENTS.md` 一致；QA 逐条核对命令与目录 | 是 |
| W5 | 「目录结构说明」章节：`core/`（共享 Rust 核心、不依赖 Tauri）、`app/src-tauri/`（Tauri 后端与平台接入）、`app/ui/`（React 前端）、`assets/`、`docs/` 等职责；说明测试归属（Rust 集成测试在 `core/tests/`、前端测试在 `app/` 内）与 `src-tauri/gen/` 不进版本库等事实 | AC-7 | 章节与仓库实际布局一致，无虚构目录 | 是 |
| W6 | 「相关文档导航」章节：相对链接指向 `docs/architecture.md`、`docs/format.md`、`docs/task-list.md`，并说明各文档适用场景 | AC-8 | 三个链接目标文件在仓库中存在（提交前 `ls` 核对） | 是 |
| W7 | 一致性交叉核对与修订：全文与 `AGENTS.md`、`README.md`、`docs/` 现有文档交叉核对，消除冲突表述（版本号来源、安全架构、V1 范围、命令行为），不承诺未实现功能/时间表、不引入新技术决策 | AC-9 | 核对清单与修订记录；QA 逐条核对 AC-9 | 是 |
| W8 | 质量门槛验证与提交：按第 5 节 DoD 跑适用门槛，确认仅变更 `docs/guide.md`，提交 `docs: 新增 项目使用指南文档` | G1–G7 | 门槛全部通过；单一逻辑提交 | 是 |

WBS 共 **8 个条目**（W1–W8），与 AC-1..AC-9 一一对应（W1 为 W2–W7 的事实基础，W8 为收口）。

## 3. 分支 / worktree 命名

| 角色 | 分支 | worktree 路径 | 用途 |
| --- | --- | --- | --- |
| 集成分支 | `task-docs-guide` | `/home/vann/Work/passwd-x/.worktrees/task-docs-guide` | 需求/规划/设计/实现/QA/Review 均在此，最终合并目标 |
| 后端（约定保留） | `task-docs-guide-be` | `/home/vann/Work/passwd-x/.worktrees/task-docs-guide-be` | 仅当出现 Rust 侧改动时启用 |
| 前端（约定保留） | `task-docs-guide-fe` | `/home/vann/Work/passwd-x/.worktrees/task-docs-guide-fe` | 仅当出现 `app/ui/` 改动时启用 |

> **执行说明**：本任务为纯文档任务，按 `docs/team/flow.md` 特殊情形（`needsBackend=false` 且 `needsFrontend=false`），由「通用实现」角色**直接在集成分支产出交付物**，默认**不创建** `-be` / `-fe` worktree（当前 `git worktree list` 亦仅存在集成分支）。上述 `-be`/`-fe` 命名作为约定保留：若实现或验证过程中出现需改动 Rust / 前端源码的情形（如真实验证命令需要），再按本表创建并遵循并行策略。

## 4. 并行策略与集成方式

- **并行边界**（作为约定，供任何出现跨端改动的场景使用）：
  - 后端：只动 `core/` + `app/src-tauri/`（Rust 侧），在 `task-docs-guide-be` worktree 独立提交；
  - 前端：只动 `app/ui/`，在 `task-docs-guide-fe` worktree 独立提交；
  - 两侧基于集成分支 `task-docs-guide` 拉出，独立 worktree、独立提交（每个提交只含一个逻辑变更，中文 Conventional Commits），互不阻塞。
- **集成方式**：集成阶段由 PjM（兼任发布）将 `-be` / `-fe` 分支合并回集成分支 `task-docs-guide`，解决冲突后进入 QA → Review。集成冲突原则上由对应开发角色修复，PjM 仅做 trivial 修复。
- **本任务实际执行**：无代码变更，无并行开发需求；W2–W7 由通用实现角色在集成分支顺序产出，W8 收口提交。风险与产物（`design.md`、`test-report.md`、`review.md`）均落在集成分支 `docs/tasks/docs-guide/`。

## 5. DoD（通用门槛 G1–G7）

合并前必须全部通过（引用 `docs/team/flow.md`）：

| # | 门槛 | 命令 / 说明 | 本任务适用性 |
| --- | --- | --- | --- |
| G1 | Rust 格式 | `cargo fmt --check`（仓库根目录） | 回归验证（不涉 Rust 变更，仍需通过） |
| G2 | Clippy 零警告 | `cargo clippy --workspace --all-targets -- -D warnings` | 回归验证 |
| G3 | Rust 测试 | `cargo test --workspace` | 回归验证 |
| G4 | 前端格式 | `npm run format:check`（`app/` 目录） | 回归验证（`docs/guide.md` 不在 Prettier 覆盖范围，仍跑确认工作区一致） |
| G5 | 前端构建 | `npm run build`（`app/` 目录） | 回归验证 |
| G6 | 验收标准 | prd.md 的 AC-1..AC-9 100% 覆盖（QA 逐条核对） | 核心门槛 |
| G7 | 缺陷 | 无未解决 P0/P1 缺陷（QA 与 Review 双否决） | 核心门槛 |

安全硬性要求（沿用 flow.md）：`core/` 禁止依赖 Tauri；严禁提交密钥/API Key/真实用户数据；日志与错误信息不含敏感内容。本任务为纯文档，上述要求以「不虚构、不承诺未实现功能」为落实重点。

## 6. 风险清单与缓解

| # | 风险 | 影响 | 缓解措施 |
| --- | --- | --- | --- |
| R1 | 指南内容与仓库现状漂移（如与 `AGENTS.md` 命令、版本号来源、安全架构、V1 范围不一致） | AC-9 不通过 | W1 先做事实核对并以 `AGENTS.md` / 既有 `docs/` 为唯一事实来源；W7 交叉核对 + QA 逐条核对 AC-9 |
| R2 | 命令或执行目录写错（如 cargo 命令标到 `app/`、npm 命令标到根目录） | AC-4/AC-5/AC-6 不通过 | W1 逐条摘录命令 × 目录；W8 实际执行 G1–G5 验证命令可用；QA 复核 |
| R3 | 虚构目录或描述不存在的结构（如写入 `src-tauri/gen/` 进版本库、杜撰目录） | AC-7 不通过 | W1 以 `git ls-files` / 实际目录为据；只写仓库真实存在的目录与文件；QA 与仓库布局核对 |
| R4 | 承诺未实现功能或时间表（如 V2 功能时间表、未实现能力） | AC-9 不通过 | 严格以 `docs/task-list.md` V1 范围为界；不写 V2 时间表；W7 终检 |
| R5 | 文档相对链接失效 | AC-8 不通过 | 链接统一指向仓库内实际文件；W6 提交前 `ls` 核对三个目标文件存在 |
| R6 | 误改其他文档 / 误动源码（纯文档任务越界） | G7 / 任务范围违背 | 提交前 `git status` 确认仅变更 `docs/guide.md`；已在 worktree 确认 `core.hooksPath=.githooks`，提交钩子自动跑格式化 |
| R7 | 门槛命令环境缺失（Linux 系统开发包未装导致 G5 构建失败） | G5 阻塞 | 先跑 `cargo check --workspace` 与 `npm run build` 探测；若系统包缺失，在 test-report 中如实记录环境限制并交由 Review 判断（文档任务不触及构建产物） |

## 7. 里程碑与出口条件

1. **规划完成**（本文件）：PjM 产出 plan.md 并提交。
2. **设计完成**：架构师产出 `design.md`（章节组织、目录树呈现形式等）。
3. **实现完成**：通用实现角色在集成分支产出 `docs/guide.md`，通过 W8 门槛。
4. **QA 通过**：`test-report.md` 逐条核对 AC-1..AC-9 全部通过，无 P0/P1。
5. **Review 通过**：`review.md` 结论 approve。
6. **合并清理**：PjM 推送、建 PR、合并到 `master`，删除 worktree 与分支。
