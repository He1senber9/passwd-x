# AI 团队（passwd-x）

7 个 AI 角色组成的开发团队，由 DSH workflow 编排，闭环完成「需求 → 规划 → 设计 → 开发 → 测试 → 审核 → 合并」。

## 角色一览

| # | 角色 | 产出物 | 关键职责 | 否决权 |
| --- | --- | --- | --- | --- |
| 1 | 产品经理（PM） | `docs/tasks/<task>/prd.md` | 需求、Given-When-Then 验收标准、V1/V2 范围与非目标 | — |
| 2 | 项目经理（PjM） | `docs/tasks/<task>/plan.md` | WBS、分支/worktree 命名、DoD、风险、并行策略 | — |
| 3 | 架构师 | `docs/tasks/<task>/design.md` | 层归属（core / src-tauri / ui）、命令签名、数据模型、安全影响分析 | — |
| 4 | 后端开发（Rust / Node） | 代码 | `core/` 安全与业务逻辑、`src-tauri/` 会话/命令/keyring；Node 用于工具链与 CI | — |
| 5 | 前端开发 | 代码 | `app/ui/` React + TypeScript | — |
| 6 | 高级测试（QA） | `test-report.md`、`bugs.md` | AC 100% 覆盖、负面安全测试、缺陷 P0–P3 分级 | P0/P1 缺陷一票否决 |
| 7 | 代码审核（Review） | `review.md` | 安全清单 + 规范风格审查、approve / request-changes | P0/P1 问题一票否决 |

## 目录

- `roles/` — 7 个角色的职责定义与可直接使用的 Prompt 模板
- `flow.md` — 协作流程、通用门槛、运维命令（worktree / 推送 / PR / 合并 / 清理）
- `workflow.mjs` — 可执行的 DSH workflow 编排脚本（团队闭环的自动化入口）

## 运行方式

1. 确保 `master` 干净、已 fetch 最新，且无残留 worktree（`git worktree prune`）。
2. 在 DSH 的 workflow 工具中填写三个参数（示例见 `workflow.mjs` 头部注释）：
   - `meta`：`name` / `description` / `phases`
   - `args`：`{ "task": { "slug", "title", "desc", "needsBackend", "needsFrontend" }, "maxRounds" }`
   - `script`：`workflow.mjs` 的脚本体（不含注释块）。
3. 运行后脚本返回各阶段产出物路径、QA 判定、审核判定与合并结果。

## 与个人 skill 的关系

单人小改动仍沿用 `~/.codex/skills/` 的 `task-developer` + `pr-approver` 双代理流程；
多角色、跨层级的正式任务交给本团队（DSH workflow 编排）。两者共用同一套仓库规范（`AGENTS.md`）。
