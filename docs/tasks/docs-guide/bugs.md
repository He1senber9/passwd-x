# 缺陷清单：项目使用指南文档（docs/guide.md）

- 任务：项目使用指南文档
- 作者：QA（高级测试工程师）
- 状态：本任务交付物（`docs/guide.md`）未发现 P0/P1/P2 缺陷；以下为 P3（建议）级观察项，
  均为仓库既有问题、非本任务引入，不阻塞合并

---

## BUG-001（P3 · 建议）：Cargo.lock 版本漂移（既有问题）

- **级别**：P3（建议）
- **引入方**：既有（`origin/master` 即存在，非本任务引入）
- **复现步骤**：
  1. 在任意基于 `origin/master` 的干净 checkout 上执行任意 cargo 命令（如 `cargo check --workspace`）；
  2. 观察 `git status`。
- **期望**：`Cargo.lock` 与根 `Cargo.toml` 版本一致，工作区保持干净。
- **实际**：根 `Cargo.toml` 的 `[workspace.package].version = "0.2.0"`（release 0.2.0 已升版），
  但提交的 `Cargo.lock` 中 `passwd-x-app` / `passwd-x-core` 仍为 `0.1.0`；任何 cargo 命令都会把
  `Cargo.lock` 改回 `0.2.0`，使工作区变脏（本次门槛复跑即复现）。
- **影响**：每次构建都会产生未提交的 `Cargo.lock` 变更，容易污染无关提交；无功能影响。
- **建议**：随任一后续 PR 将 `Cargo.lock` 同步为 `0.2.0` 一并合入 master。

---

## BUG-002（P3 · 建议）：「记住本机」凭据存储描述与实现不一致（既有问题）

- **级别**：P3（建议）
- **引入方**：既有（`AGENTS.md` 与实现长期不一致，非本任务引入；本指南忠实引用 `AGENTS.md` 表述）
- **复现步骤**：
  1. 阅读 `AGENTS.md` / `docs/guide.md`：「桌面端用系统凭据库（keyring）保存**包装后的密钥**」；
  2. 阅读 `app/src-tauri/src/keyring_store.rs`：`RememberToken` 直接序列化保存
     `master_key: [u8; 32]`（**已派生的主密钥 MK**，注释亦写明「KDF 参数 + 已派生的主密钥」）；
  3. 对比 `docs/architecture.md`：「将已派生的主密钥（MK）存入系统凭据库」。
- **期望**：三处描述一致；若安全模型是「凭据库只存包装后的密钥」，实现应改；若模型是
  「OS 凭据库本身即保管层」，文档应统一为「保存派生主密钥，由 OS 凭据库负责访问控制」。
- **实际**：`AGENTS.md` / `docs/guide.md`（“包装后的密钥”）与 `keyring_store.rs`（派生 MK）、
  `docs/architecture.md`（派生 MK）描述不一致。
- **影响**：文档与实现的安全模型表述漂移，可能误导读者对「记住本机」凭据保护强度的认知；
  不构成可被利用的安全漏洞（凭据仍由 OS 凭据库保护）。
- **建议**：另立任务统一 `AGENTS.md` 与实现/architecture.md 的表述（本任务 PRD 明确不改动
  `AGENTS.md`，故不在此修复）。
