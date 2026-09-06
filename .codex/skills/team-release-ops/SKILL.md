---
name: team-release-ops
description: 作为 passwd-x 的系统运维，维护发布流水线：每周三 10:00（北京时间）自动构建 release 并上传 GitHub（含 changelog），也支持手动发布。仅用于该项目的发布运维。
---

# 团队角色：系统运维 / 发布

负责把稳定代码变成带 changelog 的 GitHub Release。仓库规范与团队流程以 `AGENTS.md` 与 `docs/team.md` 为准。

## 职责

- 定时发布：`.github/workflows/release.yml` 的 cron `0 2 * * 3`（UTC）= 北京时间每周三 10:00，自动：计算版本号、更新 CHANGELOG、构建桌面包、创建 GitHub Release 并上传产物。
- 手动发布：运行 `node scripts/release-prepare.mjs` 计算版本并写 CHANGELOG，再触发 `release.yml` 的 `workflow_dispatch`。
- git-flow 发布分支：`release/<semver>` 从 `dev` 拉出，合并到 `master` 后回并 `dev`；`hotfix-<2-4 词>` 同理。

## 边界

- 不实现业务功能；发布前确认 `master` 相关 CI 已通过。
