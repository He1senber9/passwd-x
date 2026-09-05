# 项目使用指南

本文档是 passwd-x 项目的第一份「先读我」使用指南，面向刚加入项目、尚未熟悉仓库的开发者（含 AI 团队角色）。它介绍项目是什么、如何从零把项目构建并跑起来、仓库的目录结构，以及按需深入时该看哪份文档。

## 项目简介

passwd-x 是一款**跨平台密码管理 App**，核心功能是密码记录与安全管理。

- **平台覆盖**：一个工程覆盖全部 5 个平台——桌面端（Windows、macOS、Linux）与移动端（Android、iOS）。
- **技术形态**：基于 **Tauri 2**（Rust 核心 + 系统 WebView UI）；UI 层采用 React + TypeScript + Vite（已落地于 `app/ui/`）。
- **架构**：「共享核心 + 应用壳」结构——`core/` 是独立 Rust crate（不依赖 Tauri），包含加密、保险库格式、数据模型与业务逻辑；`app/` 是 Tauri 工程，`src-tauri/` 为 Rust 后端与平台接入，`ui/` 为前端源码。
- **核心功能**：本地加密保险库 + 主密码解锁；V1 已落地账号/密码记录的**新增、编辑、删除与列表**（CRUD）。
- **安全架构**：主密码 → Argon2id → 主密钥（MK）；MK 包装随机数据密钥（DEK），DEK 以 AEAD 加密整个保险库；修改主密码仅重新包装 DEK；密钥使用后立即从内存擦除（zeroize）。桌面端用系统凭据库（keyring）保存包装后的密钥以实现「记住本机」免密解锁。
- **当前阶段**：早期开发阶段。V1 记录 CRUD 与桌面端解锁 / 建库流程已实现；移动端构建与发布、Windows / macOS 打包、以及 V2 功能（自动填充、云同步、生物识别解锁等）**尚未实现**，功能状态以 `docs/task-list.md` 为准。

## 快速开始

### 环境要求

- **Rust 工具链**：通过 [rustup](https://rustup.rs/) 安装 stable 版本；Rust / Cargo 命令统一在**仓库根目录**执行。
- **Node.js 与 npm**：前端构建与 Tauri CLI 依赖；相关命令在 **`app/` 目录**执行。
- **Linux 桌面端系统开发包**（编译 Tauri 需要）：`pkg-config`、`libwebkit2gtk-4.1-dev`、`libgtk-3-dev`、`libsoup-3.0-dev`、`libjavascriptcoregtk-4.1-dev`（含其传递依赖）。
- **移动端目标**：Android / iOS 工程由 Tauri 生成到 `app/src-tauri/gen/`，不进版本库，无需手动准备。

### 构建

仓库根目录为 Cargo workspace，Rust 构建命令在根目录执行：

```bash
# 仓库根目录
cargo check --workspace   # 快速编译检查（推荐先跑这个）
cargo build               # 完整构建
```

前端与 Tauri 相关命令在 `app/` 目录执行：

```bash
# app/ 目录
npm install        # 首次克隆后安装依赖
npm run build      # TypeScript 检查（tsc --noEmit）+ 前端产物构建（vite build）
```

### 运行

在 `app/` 目录执行：

```bash
# app/ 目录
npm run tauri dev   # 本地运行桌面应用：编译 Rust 后端并启动系统 WebView 窗口
npm run dev         # 仅启动 Vite 前端开发服务器（不启动桌面窗口，适合纯前端调试）
```

两者用途不同：`npm run tauri dev` 是完整的桌面应用开发体验（后端命令 + 前端热更新）；`npm run dev` 只起前端静态页面，无法调用 Tauri 后端能力。

### 测试与质量检查

Rust 测试与 lint 在仓库根目录执行：

```bash
# 仓库根目录
cargo test --workspace                        # 全部 Rust 单元与集成测试
cargo fmt --check                             # 格式校验
cargo clippy --workspace --all-targets -- -D warnings   # lint（零警告）
```

前端格式校验在 `app/` 目录执行：

```bash
# app/ 目录
npm run format:check   # Prettier 校验
npm run format         # Prettier 格式化
```

提交前会自动执行 `cargo fmt` 与 Prettier（钩子在 `.githooks/pre-commit`）；首次克隆后执行 `git config core.hooksPath .githooks` 启用。

## 目录结构说明

```text
passwd-x/
├── Cargo.toml            # Cargo workspace（成员：core 与 app/src-tauri）；版本号唯一来源
├── Cargo.lock
├── README.md             # 极简入口
├── AGENTS.md             # 仓库规范（开发流程、命令、编码约定、安全要求）
├── CHANGELOG.md          # 由发布流水线自动维护
├── core/                 # 共享 Rust 核心 crate（禁止依赖 Tauri）
│   ├── src/              #   加密、保险库格式、数据模型与业务逻辑
│   └── tests/            #   Rust 集成测试
├── app/                  # Tauri 应用壳
│   ├── src-tauri/        #   Tauri Rust 后端与平台接入（会话状态、vault 命令层、keyring）
│   │   └── gen/          #     生成的移动端工程（Android/iOS，不进版本库）
│   ├── ui/               #   React + TypeScript + Vite 前端源码
│   ├── package.json      #   前端脚本（dev / build / format 等）
│   └── vite.config.ts
├── assets/               # 共享示例 / 演示数据（约定位置，见 AGENTS.md）
├── docs/                 # 设计文档
│   ├── architecture.md   #   架构与安全设计
│   ├── format.md         #   加密保险库文件格式
│   ├── task-list.md      #   功能任务清单
│   ├── tasks/            #   各任务的需求 / 规划 / 设计 / QA 产物
│   └── team/             #   AI 团队协作流程与角色定义
├── .githooks/            # 提交钩子（提交前自动格式化）
├── .github/              # CI 与发布工作流（ci、release）
└── .codex/rules/         # 项目级命令授权规则
```

各关键目录职责：

- **`core/`**：跨平台共享代码，**禁止依赖 Tauri**；安全与业务逻辑（加密、保险库格式、数据模型）只进这里。UI 与平台代码不得包含密码逻辑。
- **`app/src-tauri/`**：Tauri 2 后端与平台接入——会话状态、vault 命令层、keyring「记住本机」。
- **`app/ui/`**：React 前端，只消费 Tauri 命令，不承载任何密码 / 加密逻辑。
- **`assets/`**：仅放共享示例 / 演示数据（当前仓库暂无内容，为约定目录）。
- **`docs/`**：设计文档与任务产物。
- **测试归属**：Rust 单元测试随源码模块存放，集成测试在 `core/tests/`；前端测试与 E2E 在 `app/` 内；顶层不设统一 `tests/` 目录。
- **`app/src-tauri/gen/`**：Tauri 生成的移动端工程，**不进版本库**。

## 相关文档导航

| 文档 | 适用场景 |
| --- | --- |
| [architecture.md](architecture.md) | 架构与安全设计：技术选型、目录结构、安全架构、数据模型、V1 功能范围 |
| [format.md](format.md) | 加密保险库文件格式 v1：文件布局、密钥体系、安全性质与参数限制 |
| [task-list.md](task-list.md) | 功能任务清单：已完成 / 未实现功能状态，判断某功能是否已落地 |

仓库规范（开发流程、完整命令清单、安全与配置提示）见 `AGENTS.md`；`README.md` 为极简入口。
