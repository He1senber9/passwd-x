# Changelog

## 0.4.0 (2026-09-06)

- Merge pull request #27 from He1senber9/feature-slim-agents
- docs: 精简 AGENTS.md 并下沉工程规范到 docs/engineering.md
- Merge pull request #26 from He1senber9/feature-role-decouple
- refactor: 角色 skill 与项目流程解耦为通用模板
- Merge pull request #25 from He1senber9/feature-flow-dev-base
- docs: 明确 feature 分支基于 dev 且 PR 合入 dev
- Merge pull request #24 from He1senber9/feature-master-sync
- Merge pull request #23 from He1senber9/feature-dev-sync
- merge: 同步 dev 的 AI 团队 skill 入库到 master
- Merge pull request #22 from He1senber9/feature-team-intake
- docs: 约定所有需求经 AI 团队对接
- Merge pull request #21 from He1senber9/feature-vault-create-flow
- chore: 同步锁文件版本到 0.3.0
- feat: 首次使用才显示创建密码库，已有保险库仅解锁
- Merge pull request #20 from He1senber9/feature-ci-deps
- fix: ci 的 Rust 步骤安装 Tauri 系统依赖
- Merge pull request #19 from He1senber9/feature-test-badges
- feat: 首页增加 CI 与 Rust 测试通过率徽章
- chore: 将 AI 团队角色 skill 纳入仓库版本管理 (#18)
- Merge pull request #17 from He1senber9/feature-promote-dev
- Merge pull request #16 from He1senber9/feature-sync-master
- merge: 同步 master 自动更新功能并统一发布工作流
- chore: 强制约定 feature-/hotfix- 分支命名并增加 CI 校验 (#15)
- docs: 将分支流程简化为轻量 Git Flow (#14)
- fix: 修复 README 分支图的主分支名 (#13)
- docs: 在 README 增加分支开发流程图 (#12)
- chore: 从零重配 AI 团队与 git-flow 发布流程 (#11)
## [0.3.0](https://github.com/He1senber9/passwd-x/compare/v0.2.0...v0.3.0) (2026-09-04)


### Features

* 增加应用内自动更新 ([dadc611](https://github.com/He1senber9/passwd-x/commit/dadc61166adbd81bb0751981430fe0bd35d64343))
* 增加应用内自动更新 ([1720e4a](https://github.com/He1senber9/passwd-x/commit/1720e4a9964b696a79a7fafe0d5f57027d36bc47))

## [0.2.0](https://github.com/He1senber9/passwd-x/compare/v0.1.0...v0.2.0) (2026-08-29)


### Features

* add remember-device key APIs to core ([7c4b96e](https://github.com/He1senber9/passwd-x/commit/7c4b96e563bf263a6c8706fcdefb389ec62cd628))
* default first launch to vault creation ([9c13219](https://github.com/He1senber9/passwd-x/commit/9c132193e704a7367ffe10c81fc266ab0923b37d))
* implement core vault security layer ([6e28250](https://github.com/He1senber9/passwd-x/commit/6e28250c42b5d814d72b4732ce9bf08b63b209a9))
* scaffold tauri app shell with keyring and react ui ([f912cb2](https://github.com/He1senber9/passwd-x/commit/f912cb2abfe6af290faff71637dba9a2bc043156))
* 界面显示应用版本号 ([fa74168](https://github.com/He1senber9/passwd-x/commit/fa7416820b72c24eed0be4a64ca8e175b8e65de3))


### Bug Fixes

* add square app icons for linux bundling ([8de11e3](https://github.com/He1senber9/passwd-x/commit/8de11e3366ba3daa15061e6ed0d559818db2683b))
