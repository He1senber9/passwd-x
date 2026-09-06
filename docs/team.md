# AI 团队与开发流程

本文件是 passwd-x AI 软件团队的宪法，定义角色、分支模型、协作阶段与发布排程。角色以 Codex skill 落地：规范源码在本仓库 `.codex/skills/`（随项目版本管理），本机使用需安装到 `~/.codex/skills/`；仓库始终生效的硬性规范见 `AGENTS.md`。

## 角色

| 角色 | skill | 职责 | 主要产出 |
| --- | --- | --- | --- |
| 产品经理 | `team-pm` | 实时接收需求，澄清并写入待办 | `docs/backlog.md` |
| 项目经理 | `team-pjm` | 拆分、排期、分派任务 | backlog 状态与任务卡 |
| 系统架构师 | `team-architect` | 分析任务、建分支、设计文档与测试计划 | `docs/tasks/<slug>/design.md`、`test-plan.md` |
| 开发（若干） | `team-developer` | 按设计实现；可多实例并行（后端/前端） | 代码 |
| 测试 | `team-tester` | 编写单元测试、跑门禁、出报告 | `docs/tasks/<slug>/test-report.md` |
| PR 审核员 | `team-pr-reviewer` | 代码质量与安全审查、合并 PR | `docs/tasks/<slug>/review.md` |
| 系统运维 | `team-release-ops` | 定时/手动发布，构建并上传 GitHub | `CHANGELOG.md`、Release 产物 |

## 角色 skill 安装

角色 skill 的规范源码随仓库维护在 `.codex/skills/`（更新规范后请同步到仓库并重新安装）。克隆仓库后本机安装方式：

- 用 Codex 的 skill-installer 安装：`install-skill-from-github.py --repo He1senber9/passwd-x --path .codex/skills/team-pm .codex/skills/team-pjm .codex/skills/team-architect .codex/skills/team-developer .codex/skills/team-tester .codex/skills/team-pr-reviewer .codex/skills/team-release-ops`
- 或直接把 `.codex/skills/team-*` 目录复制到 `~/.codex/skills/`。

## 分支模型（Git Flow 轻量）

```text
master ───────────────●──────────────●  稳定生产，只接收 release/* 与 hotfix-*
                        ↖            ↖
dev ─────●────●────●───┴────●───────┴  集成分支，接收 feature-*
         ↑    ↑    ↑        ↑
  feature-password-gen feature-export feature-search  release/0.3.0（自 dev）
```

- `feature-<2-4 词>`：功能任务，自 `dev` 拉出，完成后 PR 回 `dev`。
- `release/<semver>`：发布准备，自 `dev` 拉出，完成后 PR 到 `master`（打 tag），再回并 `dev`。
- `hotfix-<2-4 词>`：紧急修复，自 `master` 拉出，完成后 PR 到 `master`（打 tag），再回并 `dev`。

**轻量原则**：

- `release/*` 只在真正发版那一周从 `dev` 切出，合并后立即清理，不长期并行。
- `feature-*` 短命：一个任务一个分支，合并即删。
- `hotfix-*` 保留：从 `master` 出、修完合 `master` 并回并 `dev`，是版本化产品最有价值的部分。

**进一步简化（可选）**：若想更接近 trunk-based，可去掉长期 `dev`，以 `master` 为唯一主干，`feature-*` 直接 PR 回 `master`，仅在发版周切 `release/<semver>`。

## 阶段流转

1. 需求：`team-pm` 写入 `docs/backlog.md`（状态 `待规划`）。
2. 排期：`team-pjm` 拆分任务卡并分派（状态 `已规划`）。
3. 设计与分支：`team-architect` 建分支，产出 `design.md` + `test-plan.md`。
4. 实现与测试：`team-tester` 与 `team-developer` 并行推进（状态 `开发中`/`测试中`）。
5. 创建 PR：测试与开发都完成后，`feature-<2-4 词>` → `dev`。
6. 审核合并：`team-pr-reviewer` 审查质量与安全并合并（状态 `已发布`）。

## 质量门禁（合并前必须全部通过）

`cargo fmt --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`、`npm run format:check`、`npm run build`。PR 审核员对 P0/P1 问题一票否决。

## 发布排程

`team-release-ops` 维护发布流水线：`.github/workflows/release.yml` 每周三北京时间 10:00（UTC cron `0 2 * * 3`）自动计算版本号、更新 `CHANGELOG.md`、切 `release/<semver>`、打 tag 创建 GitHub Release 并开 PR 到 master；`.github/workflows/release-updater.yml` 在 Release 发布后构建签名安装包并上传。无新提交时自动跳过；需要立即发布可手动触发 `workflow_dispatch`。

## 与 AGENTS.md / skill 的关系

- `AGENTS.md`：所有角色共用的硬性规则（命令、安全、提交约定），每次会话自动加载。
- 本文件：团队结构与人读规范。
- `.codex/skills/team-*`：每个角色的可执行提示词（规范源码，随仓库版本管理）；安装到 `~/.codex/skills/` 后由 Codex 在触发对应任务时加载。
