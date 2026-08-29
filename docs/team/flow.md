# 协作流程（flow）

本文档定义 AI 团队的阶段流转、质量门槛与运维命令。所有角色共用，`workflow.mjs` 按此流程编排。

## 阶段流转

```text
┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────────────┐
│ 1 需求    │ → │ 2 规划    │ → │ 3 设计    │ → │ 4 开发（BE ∥ FE）│
│ PM: prd  │   │ PjM: plan│   │ 架构师    │   │ 独立 worktree    │
└──────────┘   └──────────┘   │ : design │   └────────┬─────────┘
                              └──────────┘            ▼
                                            ┌──────────────────┐
                              ┌───────────── │ 5 集成（合并分支）│
                              │               └────────┬─────────┘
                              ▼                        ▼
┌──────────┐   ┌──────────┐   ┌─────────────────────────┐
│ 7 合并    │ ← │ 6 审核    │ ← │ 5 质量保障（QA）        │
│ PR+清理   │   │ Review   │   │ test-report + bugs      │
└──────────┘   └──────────┘   └─────────────────────────┘
```

回环规则：QA 判定 `fail`（AC 未 100% 或存在 P0/P1 缺陷）或 Review 判定
`request-changes` 时，缺陷/问题写入 `docs/tasks/<task>/bugs.md`，回到阶段 4 由对应
开发角色修复，再走集成 → QA → Review；**不新建任务**。同一任务最多重试 `maxRounds`
轮（默认 2），仍不通过则整单失败并报告。

## 通用门槛（DoD，合并前必须全部通过）

| # | 门槛 | 命令 |
| --- | --- | --- |
| G1 | Rust 格式 | `cargo fmt --check` |
| G2 | Clippy 零警告 | `cargo clippy --workspace --all-targets -- -D warnings` |
| G3 | Rust 测试 | `cargo test --workspace` |
| G4 | 前端格式 | `npm run format:check`（`app/` 目录） |
| G5 | 前端构建 | `npm run build`（`app/` 目录） |
| G6 | 验收标准 | prd.md 的 AC 100% 覆盖（QA 逐条核对） |
| G7 | 缺陷 | 无未解决 P0/P1 缺陷（QA 与 Review 双否决） |

安全硬性要求（所有角色）：密钥使用后立即 zeroize；日志与错误信息不得包含敏感内容；
`core/` 禁止依赖 Tauri；严禁提交密钥、API Key、真实用户数据。

## 工作区准备（worktree）

仓库根目录：`/home/vann/Work/passwd-x`（以实际 `pwd` 为准）。所有 worktree 建在
仓库内 `.worktrees/` 下（沙箱只允许写该目录），分支命名 `task-<2-3 个英文小写单词>`。

```bash
REPO=/home/vann/Work/passwd-x
# 集成分支（需求/规划/设计/集成/QA/审核都在这里）
git -C "$REPO" worktree add "$REPO/.worktrees/task-<slug>" -b "task-<slug>" origin/master
# 后端分支（基于集成分支）
git -C "$REPO" worktree add "$REPO/.worktrees/task-<slug>-be" -b "task-<slug>-be" "task-<slug>"
# 前端分支（基于集成分支）
git -C "$REPO" worktree add "$REPO/.worktrees/task-<slug>-fe" -b "task-<slug>-fe" "task-<slug>"
```

工作树内首次提交前需确认 hooks：`git config core.hooksPath .githooks`（提交时自动执行
cargo fmt / prettier）。若 worktree 内 `app/node_modules` 缺失，软链主仓库的：
`ln -sfn $REPO/app/node_modules <worktree>/app/node_modules`。

## 提交与 PR

- Conventional Commits，**标题与正文一律中文**，标题 ≤72 字符、祈使句；每个提交只含一个逻辑变更。
- 开发分支（`-be` / `-fe`）由开发角色各自提交；集成分支由集成阶段合并。
- QA 与 Review 的产物（`test-report.md` / `review.md` / `bugs.md`）也提交到集成分支。

## 推送 / 建 PR / 合并 / 清理（运维命令）

网络到 GitHub 较慢时，推送务必加 `timeout 300`。凭据从 `~/.git-credentials` 读取，
用 GitHub REST API（`gh` 未登录时用此方式）。

```bash
REPO=/home/vann/Work/passwd-x
BRANCH="task-<slug>"
CRED=$(grep -oE 'https://[^@]+@github\.com' ~/.git-credentials | head -n1 | sed -E 's#https://##; s#@github\.com##')

# 推送
timeout 300 git -C "$REPO" push -u origin "$BRANCH"

# 创建 PR（标题/摘要中文）
curl -s -u "$CRED" -H "Accept: application/vnd.github+json" \
  -d '{"title":"<中文标题>","head":"'"$BRANCH"'","base":"master","body":"<中文摘要，含 prd/plan/design/test-report/review 路径与门槛结论>"}' \
  https://api.github.com/repos/He1senber9/passwd-x/pulls

# 合并（merge_method=merge）
curl -s -X PUT -u "$CRED" -H "Accept: application/vnd.github+json" \
  -d '{"merge_method":"merge"}' \
  https://api.github.com/repos/He1senber9/passwd-x/pulls/<编号>/merge

# 清理：删 worktree、本地与远程分支
git -C "$REPO" worktree remove "$REPO/.worktrees/task-<slug>" --force
git -C "$REPO" worktree remove "$REPO/.worktrees/task-<slug>-be" --force
git -C "$REPO" worktree remove "$REPO/.worktrees/task-<slug>-fe" --force
git -C "$REPO" branch -d "task-<slug>" "task-<slug>-be" "task-<slug>-fe"
timeout 300 git -C "$REPO" push origin --delete "task-<slug>"
```

## 任务产物位置

每个任务的所有文档集中在 `docs/tasks/<task>/`：

| 文件 | 作者 | 说明 |
| --- | --- | --- |
| `prd.md` | PM | 需求与验收标准 |
| `plan.md` | PjM | WBS、分支、DoD、风险 |
| `design.md` | 架构师 | 设计决策与安全影响分析 |
| `bugs.md` | QA | 缺陷清单（失败回环时由开发补充修复记录） |
| `test-report.md` | QA | 测试报告 |
| `review.md` | Review | 审核报告与结论 |

`docs/tasks/` 与 `docs/team/` 一样属于文档层，随功能 PR 一并进入仓库。
