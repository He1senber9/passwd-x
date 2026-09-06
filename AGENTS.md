# 仓库指南（Repository Guidelines）

## 项目简介

跨平台密码管理 App（Tauri 2：Rust 核心 `core/` + 应用壳 `app/`，React + TypeScript UI）。处于早期阶段，V1 记录 CRUD + 本地加密保险库已落地。完整背景见 `docs/guide.md`、`docs/architecture.md`。

## 核心结构与红线（所有人必读）

- `core/`：安全与业务逻辑，**禁止依赖 Tauri**；`app/src-tauri/`：Tauri 后端与平台接入；`app/ui/`：React 界面（不含密码逻辑）。
- 版本号唯一来源：根 `Cargo.toml` 的 `[workspace.package].version`。
- 严禁提交密钥、API Key、主密码或真实用户数据；敏感凭据只入系统安全存储；日志与错误信息脱敏；签名私钥只存 GitHub Secret。

## 团队与协作（AI 团队）

- 项目变更类需求**默认经 AI 团队**处理，不直接实现：`team-pm` 登记 `docs/backlog.md` → `team-pjm` 分派 → `team-architect` 建分支与设计 → 测试/开发实现 → 创建 PR → `team-pr-reviewer` 审查合并。用户以自然语言提需求并参与验收。
- 7 个角色 skill 是通用模板（规范源码 `.codex/skills/`，需安装到 `~/.codex/skills/`）；职责与通用判断见各 skill。
- 分支模型、命名、PR 目标、门禁、发布排程等**项目流程约定见 `docs/team.md`**（按角色路由阅读下表）。

## 角色必读路由

| 角色 | 开工前阅读 |
| --- | --- |
| 所有人 | `docs/team.md`（团队与流程总览） |
| PM / PjM | `docs/backlog.md`（表头即待办格式） |
| 架构师 | `docs/team.md`（分支模型/命名）+ `docs/engineering.md` |
| 开发 / 测试 | `docs/engineering.md`（命令、风格、测试、门禁、提交规范） |
| PR 审核员 | `docs/engineering.md`（门禁/提交）+ `docs/team.md`（分支与合并规则） |
| 运维 / 发布 | `docs/team.md`（发布排程）+ `.github/workflows/release*.yml` |

常用检查命令、提交与 PR 细节都在 `docs/engineering.md`，不再在本文重复。

## 文档导航

- `docs/guide.md` — 项目使用指南（先读我）
- `docs/architecture.md` — 架构与安全设计
- `docs/format.md` — 加密保险库文件格式
- `docs/engineering.md` — 工程规范（命令/风格/测试/提交/门禁/版本）
- `docs/team.md` — AI 团队、分支模型与流程
- `docs/backlog.md` — 项目待办（需求入口）
- `docs/task-list.md` — 功能任务清单
