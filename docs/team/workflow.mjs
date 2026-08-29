// ============================================================================
// AI 团队协作流程 — DSH workflow 编排脚本（passwd-x）
//
// 使用方式：将本文件内容整体粘贴到 DSH workflow 工具的 script 参数。
//
// meta 参数示例（name / description / phases）：
//   {
//     "name": "ai-team-flow",
//     "description": "7 角色 AI 团队闭环交付：需求→规划→设计→开发→测试→审核→合并",
//     "phases": [
//       { "title": "需求分析", "detail": "PM 编写 prd.md（Given-When-Then 验收标准）" },
//       { "title": "项目规划", "detail": "PjM 编写 plan.md（WBS/分支/DoD/风险）" },
//       { "title": "架构设计", "detail": "架构师编写 design.md（层归属/数据模型/安全影响分析）" },
//       { "title": "开发实现", "detail": "后端与前端并行开发（独立 worktree）" },
//       { "title": "集成", "detail": "合并开发分支到集成分支并跑全量门槛" },
//       { "title": "质量保障", "detail": "QA 验收 AC 100% + 负面安全测试，P0/P1 一票否决" },
//       { "title": "代码审核", "detail": "Review 安全清单 + 风格，approve 或 request-changes" },
//       { "title": "合并发布", "detail": "推送分支、创建中文 PR、合并、清理 worktree" }
//     ]
//   }
//
// args 参数示例：
//   {
//     "task": {
//       "slug": "password-generator",
//       "title": "密码生成器",
//       "desc": "在新建/编辑记录时提供随机强密码生成",
//       "needsBackend": true,
//       "needsFrontend": true
//     },
//     "maxRounds": 2
//   }
// ============================================================================

// ===== 配置 =====
const REPO = "/home/vann/Work/passwd-x";

const task = args.task;
if (!task || !task.slug || !task.title) {
  throw new Error(
    "args.task 必须包含 slug 与 title（可含 desc / needsBackend / needsFrontend）",
  );
}
const slug = task.slug;
const title = task.title;
const desc = task.desc || "";
const maxRounds = args.maxRounds || 2;
const needsBackend = task.needsBackend !== false;
const needsFrontend = task.needsFrontend !== false;

const WT = `${REPO}/.worktrees/task-${slug}`; // 集成分支 worktree
const WT_BE = `${REPO}/.worktrees/task-${slug}-be`; // 后端 worktree
const WT_FE = `${REPO}/.worktrees/task-${slug}-fe`; // 前端 worktree
const BRANCH = `task-${slug}`;
const BRANCH_BE = `task-${slug}-be`;
const BRANCH_FE = `task-${slug}-fe`;
const TASKDIR = `${WT}/docs/tasks/${slug}`; // 任务文档目录

// 每个角色的公共上下文
const READ_FIRST = `
先阅读以下文档再动手：
- ${REPO}/AGENTS.md（仓库规范：worktree 流程、中文提交、门槛命令、安全要求）
- ${REPO}/docs/team/flow.md（协作流程与运维命令）
- ${REPO}/docs/team/roles/（本角色定义与 Prompt 模板）

仓库根目录：${REPO}（可用 pwd 验证）。所有 worktree 必须建在 ${REPO}/.worktrees/ 下。
在 worktree 内先执行 git config core.hooksPath .githooks（提交时自动 cargo fmt / prettier）。
提交信息一律中文（Conventional Commits），每个提交只含一个逻辑变更。
`;

// ===== JSON Schema（agent 结构化返回） =====
const qaSchema = {
  type: "object",
  properties: {
    verdict: { type: "string", enum: ["pass", "fail"] },
    acCoverage: { type: "string" },
    defects: { type: "array", items: { type: "string" } },
    summary: { type: "string" },
  },
  required: ["verdict", "acCoverage", "defects"],
};

const reviewSchema = {
  type: "object",
  properties: {
    verdict: { type: "string", enum: ["approve", "request-changes"] },
    issues: { type: "array", items: { type: "string" } },
    securityPassed: { type: "boolean" },
    summary: { type: "string" },
  },
  required: ["verdict", "issues"],
};

const mergeSchema = {
  type: "object",
  properties: {
    merged: { type: "boolean" },
    prUrl: { type: "string" },
    prNumber: { type: "integer" },
  },
  required: ["merged"],
};

// ===== 阶段 1：需求分析（PM） =====
phase("需求分析");
const pmResult = await agent(
  `${READ_FIRST}
你是 passwd-x 的产品经理（PM）。任务：${title} —— ${desc}

工作区准备：目标 worktree ${WT}（分支 ${BRANCH}，基于 origin/master）。如不存在先创建：
git -C "${REPO}" worktree add "${WT}" -b "${BRANCH}" origin/master

产出（写入 ${TASKDIR}/prd.md 并提交）：
1. 背景与目标
2. 用户故事 + Given-When-Then 验收标准（逐条编号 AC-1, AC-2, ...）
3. V1 范围与明确非目标（V2 及以后）
4. 不做技术选型（留给架构师）
提交信息：docs: 新增 ${title} 需求文档（prd.md）

完成后汇报：prd.md 路径、验收标准条目数。`,
  { label: "产品经理", phase: "需求分析" },
);
if (!pmResult) {
  log("PM 阶段失败，终止。");
  return { task: { slug, title }, merged: false, reason: "PM 失败" };
}
log(`PM 完成：${pmResult}`);

// ===== 阶段 2：项目规划（PjM） =====
phase("项目规划");
const pjmResult = await agent(
  `${READ_FIRST}
你是 passwd-x 的项目经理（PjM）。任务：${title}
输入：${TASKDIR}/prd.md（PM 已产出）

产出（写入 ${TASKDIR}/plan.md 并提交）：
1. WBS 任务分解（与 prd.md 的 AC 对应）
2. 分支/worktree 命名：集成分支 ${BRANCH}（${WT}）、后端 ${BRANCH_BE}（${WT_BE}）、前端 ${BRANCH_FE}（${WT_FE}）
3. 并行策略：后端（core/ + app/src-tauri/）与前端（app/ui/）并行、独立 worktree、独立提交，由集成阶段合并
4. DoD（flow.md 通用门槛 G1–G7）与风险缓解
提交信息：docs: 新增 ${title} 项目计划（plan.md）

完成后汇报：plan.md 路径、WBS 条目数、风险清单。`,
  { label: "项目经理", phase: "项目规划" },
);
if (!pjmResult) {
  log("PjM 阶段失败，终止。");
  return { task: { slug, title }, merged: false, reason: "PjM 失败" };
}
log(`PjM 完成：${pjmResult}`);

// ===== 阶段 3：架构设计（架构师） =====
phase("架构设计");
const archResult = await agent(
  `${READ_FIRST}
你是 passwd-x 的架构师。任务：${title}
输入：${TASKDIR}/prd.md、${TASKDIR}/plan.md；参考 ${REPO}/docs/architecture.md、${REPO}/docs/format.md

产出（写入 ${TASKDIR}/design.md 并提交）：
1. 变更归属层（core / app/src-tauri / app/ui）与理由；core/ 禁止依赖 Tauri
2. Tauri command 签名（如涉及）
3. 数据模型变更（如涉及，说明版本化格式兼容，见 docs/format.md）
4. 安全影响分析：涉及加密/密钥/凭据库时必写威胁与缓解（Argon2id 参数、AEAD、zeroize、日志脱敏）
提交信息：docs: 新增 ${title} 架构设计（design.md）

完成后汇报：design.md 路径、归属层清单、安全影响分析结论。`,
  { label: "架构师", phase: "架构设计" },
);
if (!archResult) {
  log("架构设计阶段失败，终止。");
  return { task: { slug, title }, merged: false, reason: "架构师失败" };
}
log(`架构师完成：${archResult}`);

// ===== 阶段 4–6：开发 → 集成 → QA → Review（支持回环） =====
let qaResult = null;
let reviewResult = null;
let feedback = ""; // 上一轮缺陷/问题
let round = 0;

for (round = 1; round <= maxRounds; round++) {
  log(`======== 第 ${round}/${maxRounds} 轮 ========`);

  // 阶段 4：开发实现（后端 ∥ 前端，独立 worktree 并行）
  phase("开发实现");
  const dev = await parallel([
    async () => {
      if (!needsBackend) {
        return { role: "后端", skipped: true };
      }
      const out = await agent(
        `${READ_FIRST}
你是 passwd-x 的后端开发工程师（Rust / Node.js）。任务：${title}
实现依据：${TASKDIR}/design.md、${TASKDIR}/plan.md、${TASKDIR}/prd.md
工作目录：${WT_BE}（分支 ${BRANCH_BE}，基于 ${BRANCH}；不存在先创建：
git -C "${REPO}" worktree add "${WT_BE}" -b "${BRANCH_BE}" "${BRANCH}"）

要求：
- 安全与业务逻辑进 core/；会话/命令/keyring 在 app/src-tauri/；Node 仅限工具链/CI/脚本
- 遵守 design.md 的接口签名与数据模型
- 硬性要求：密钥 zeroize、日志与错误信息不含敏感内容、core/ 无 Tauri 依赖
- 只改 Rust 与工具链文件，不触碰 app/ui/（前端并行范围）
- 若 ${WT_BE}/target 不存在：ln -sfn ${REPO}/target ${WT_BE}/target（共享编译缓存）
- 每个提交只含一个逻辑变更，提交信息中文（feat:/fix:/refactor: 等）
- 门槛：cargo fmt --check、cargo clippy --workspace --all-targets -- -D warnings、cargo test --workspace
${feedback ? `上一轮 QA/审核意见（须逐一修复并补充回归验证）：\n${feedback}` : ""}
完成后汇报：变更文件清单、门槛结果、提交列表。`,
        { label: "后端开发", phase: "开发实现" },
      );
      return { role: "后端", skipped: false, out };
    },
    async () => {
      if (!needsFrontend) {
        return { role: "前端", skipped: true };
      }
      const out = await agent(
        `${READ_FIRST}
你是 passwd-x 的前端开发工程师。任务：${title}
实现依据：${TASKDIR}/design.md、${TASKDIR}/plan.md、${TASKDIR}/prd.md
工作目录：${WT_FE}（分支 ${BRANCH_FE}，基于 ${BRANCH}；不存在先创建：
git -C "${REPO}" worktree add "${WT_FE}" -b "${BRANCH_FE}" "${BRANCH}"）

要求：
- 只改 app/ui/（React + TypeScript），密码/加密逻辑一律通过 Tauri command 走后端
- 遵守 design.md 的界面与交互约定
- 若 ${WT_FE}/app/node_modules 不存在：ln -sfn ${REPO}/app/node_modules ${WT_FE}/app/node_modules
- 每个提交只含一个逻辑变更，提交信息中文（feat:/fix:/refactor: 等）
- 门槛：npm run format:check、npm run build（在 ${WT_FE}/app 目录）
${feedback ? `上一轮 QA/审核意见（须逐一修复并补充回归验证）：\n${feedback}` : ""}
完成后汇报：变更文件清单、门槛结果、提交列表。`,
        { label: "前端开发", phase: "开发实现" },
      );
      return { role: "前端", skipped: false, out };
    },
    async () => {
      // 纯文档/运维任务（BE 与 FE 均不需要）：通用实现角色直接在集成分支产出交付物
      if (needsBackend || needsFrontend) {
        return { role: "通用实现", skipped: true };
      }
      const out = await agent(
        `${READ_FIRST}
你是 passwd-x 的实现工程师（通用）。任务：${title} —— ${desc}
实现依据：${TASKDIR}/design.md、${TASKDIR}/plan.md、${TASKDIR}/prd.md
工作目录：${WT}（集成分支 ${BRANCH}，直接在此提交）

本任务为纯文档/运维类（needsBackend=false 且 needsFrontend=false），无独立开发分支：
按 design.md 与 prd.md 在集成分支直接产出交付物（如 docs/guide.md）并提交。

要求：
- 产出物须满足 prd.md 的全部 AC（QA 将逐条核对）
- 遵守 design.md 的结构与约定；若 ${WT}/app/node_modules 缺失：
  ln -sfn ${REPO}/app/node_modules ${WT}/app/node_modules
- 每个提交只含一个逻辑变更，提交信息中文（docs:/feat:/chore: 等）
- 门槛：cargo fmt --check、cargo clippy --workspace --all-targets -- -D warnings、
  cargo test --workspace、npm run format:check、npm run build
${feedback ? `上一轮 QA/审核意见（须逐一修复并补充回归验证）：\n${feedback}` : ""}
完成后汇报：交付物清单、门槛结果、提交列表。`,
        { label: "通用实现", phase: "开发实现" },
      );
      return { role: "通用实现", skipped: false, out };
    },
  ]);
  log(`开发结果：${JSON.stringify(dev)}`);
  if (dev.some((d) => d && !d.skipped && d.out === null)) {
    log("警告：有开发角色未交付结果，仍继续流程，由 QA 兜底判定。");
  }

  // 阶段 5：集成（合并分支 + 全量门槛）
  phase("集成");
  const integResult = await agent(
    `${READ_FIRST}
你是集成工程师（架构师兼任）。任务：${title}
工作目录：${WT}（集成分支 ${BRANCH}）

步骤：
1. 合并开发分支（某分支不存在或 Already up to date 属正常，跳过即可）：
   git -C "${WT}" merge "${BRANCH_BE}" -m "merge: 合入 ${title} 后端开发分支"
   git -C "${WT}" merge "${BRANCH_FE}" -m "merge: 合入 ${title} 前端开发分支"
   有冲突先解决（通常互不相交目录不会冲突），解决后继续。
2. 全量门槛（全部通过）：
   cd "${WT}" && cargo fmt --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cd "${WT}/app" && npm run format:check && npm run build
   （若 ${WT}/app/node_modules 缺失：ln -sfn ${REPO}/app/node_modules ${WT}/app/node_modules）
3. 仅允许修复集成层问题（合并冲突、门槛失败），不擅自改功能。
完成后汇报：合并结果与门槛结果。`,
    { label: "集成", phase: "集成" },
  );
  if (!integResult) {
    log("集成阶段失败，终止。");
    return {
      task: { slug, title },
      merged: false,
      reason: "集成失败",
      roundsUsed: round,
    };
  }
  log(`集成完成：${integResult}`);

  // 阶段 5b：质量保障（QA）
  phase("质量保障");
  qaResult = await agent(
    `${READ_FIRST}
你是 passwd-x 的高级测试工程师（QA）。任务：${title}
工作目录：${WT}（集成分支 ${BRANCH}）
验收标准：${TASKDIR}/prd.md；实现依据：${TASKDIR}/design.md

验收内容：
1. AC 覆盖率：逐条核对 prd.md 的 AC（代码实现 + 测试用例），统计 X/Y，要求 100%
2. 门槛复跑（全部通过）：cargo fmt --check、cargo clippy --workspace --all-targets -- -D warnings、
   cargo test --workspace、npm run format:check、npm run build
   （若 ${WT}/app/node_modules 缺失：ln -sfn ${REPO}/app/node_modules ${WT}/app/node_modules）
3. 负面安全测试（针对本变更可用的部分）：错误主密码、篡改保险库文件、格式版本兼容、日志/错误信息脱敏
4. 缺陷分级 P0–P3（P0 阻断发布 / P1 高优先级 / P2 一般 / P3 建议）

产出（提交到集成分支）：
- ${TASKDIR}/test-report.md（覆盖矩阵 + 门槛结果 + 缺陷列表）
- 如有缺陷同时写入 ${TASKDIR}/bugs.md（ID/级别/复现步骤/期望/实际）
- 提交信息：test: 提交 ${title} 测试报告与缺陷记录

判定（严格）：AC 覆盖率 100% 且无未解决 P0/P1 缺陷 → verdict=pass；
否则 verdict=fail 并在 defects 列出全部缺陷。`,
    { label: "QA", phase: "质量保障", schema: qaSchema },
  );
  if (!qaResult) {
    log("QA 阶段失败，终止。");
    return {
      task: { slug, title },
      merged: false,
      reason: "QA 失败",
      roundsUsed: round,
    };
  }
  log(
    `QA 判定：${qaResult.verdict}，AC 覆盖 ${qaResult.acCoverage}，缺陷 ${qaResult.defects.length} 条`,
  );
  if (qaResult.verdict === "fail") {
    feedback = `【QA 缺陷（第 ${round} 轮）】\n${qaResult.defects.join("\n")}`;
    continue;
  }

  // 阶段 6：代码审核（Review）
  phase("代码审核");
  reviewResult = await agent(
    `${READ_FIRST}
你是 passwd-x 的代码审核（Review）。任务：${title}
工作目录：${WT}（集成分支 ${BRANCH}）
审核差异：git -C "${WT}" diff origin/master...HEAD
依据：${TASKDIR}/prd.md、${TASKDIR}/design.md、${TASKDIR}/test-report.md、${TASKDIR}/bugs.md

审核内容：
1. 安全清单：Argon2id 参数、AEAD 用法、zeroize、密钥存储位置（keyring/Keychain/Keystore）、
   日志脱敏、依赖漏洞（cargo audit 可用则运行）
2. 规范风格：提交信息中文、单逻辑变更、命名（Rust snake_case / TS camelCase）、core/ 无 Tauri 依赖
3. 一致性：实现 vs design.md；测试 vs prd.md AC

产出（提交到集成分支）：
- ${TASKDIR}/review.md（检查清单逐项结果 + issues）
- 提交信息：docs: 提交 ${title} 代码审核报告

判定：无未解决 P0/P1 问题 → verdict=approve（securityPassed=true）；
否则 verdict=request-changes 并列出全部 issues。
注意：你只审核，绝不修改任何代码文件。`,
    { label: "Review", phase: "代码审核", schema: reviewSchema },
  );
  if (!reviewResult) {
    log("Review 阶段失败，终止。");
    return {
      task: { slug, title },
      merged: false,
      reason: "Review 失败",
      roundsUsed: round,
    };
  }
  log(
    `Review 判定：${reviewResult.verdict}，安全通过：${reviewResult.securityPassed}`,
  );
  if (reviewResult.verdict === "request-changes") {
    feedback = `【Review 问题（第 ${round} 轮）】\n${reviewResult.issues.join("\n")}`;
    continue;
  }
  break; // approve
}

// 达到最大轮次仍未通过
if (
  !qaResult ||
  qaResult.verdict !== "pass" ||
  !reviewResult ||
  reviewResult.verdict !== "approve"
) {
  log("任务未通过（达到最大轮次），不合并。详见 bugs.md / review.md。");
  return {
    task: { slug, title },
    merged: false,
    reason:
      qaResult && qaResult.verdict === "fail" ? "QA 未通过" : "Review 未通过",
    roundsUsed: round,
    artifacts: {
      prd: `${TASKDIR}/prd.md`,
      plan: `${TASKDIR}/plan.md`,
      design: `${TASKDIR}/design.md`,
      testReport: `${TASKDIR}/test-report.md`,
      review: `${TASKDIR}/review.md`,
    },
    qa: qaResult,
    review: reviewResult,
  };
}

// ===== 阶段 7：合并发布 =====
phase("合并发布");
const mergeResult = await agent(
  `${READ_FIRST}
你是发布工程师（PjM 兼任）。任务：${title} 已通过 QA 与 Review，执行发布。

步骤：
1. 推送集成分支（网络慢，务必加超时）：
   timeout 300 git -C "${REPO}" push -u origin "${BRANCH}"
2. 创建 PR（标题与摘要一律中文）。凭据从 ~/.git-credentials 读取：
   CRED=$(grep -oE 'https://[^@]+@github\\.com' ~/.git-credentials | head -n1 | sed -E 's#https://##; s#@github\\.com##')
   curl -s -u "$CRED" -H "Accept: application/vnd.github+json" \
     -d '{"title":"<中文标题，如：feat: 新增 ${title}>","head":"${BRANCH}","base":"master","body":"<中文摘要：变更说明 + ${TASKDIR}/prd.md、plan.md、design.md、test-report.md、review.md 路径 + 门槛结论>"}' \
     https://api.github.com/repos/He1senber9/passwd-x/pulls
   从响应中取出 "number" 与 "html_url"。
3. 合并 PR：
   curl -s -X PUT -u "$CRED" -H "Accept: application/vnd.github+json" \
     -d '{"merge_method":"merge"}' \
     https://api.github.com/repos/He1senber9/passwd-x/pulls/<number>/merge
4. 清理（命令见 flow.md）：
   git -C "${REPO}" worktree remove "${WT}" --force
   git -C "${REPO}" worktree remove "${WT_BE}" --force
   git -C "${REPO}" worktree remove "${WT_FE}" --force
   git -C "${REPO}" branch -d "${BRANCH}" "${BRANCH_BE}" "${BRANCH_FE}"
   timeout 300 git -C "${REPO}" push origin --delete "${BRANCH}"

完成后汇报 merged / prUrl / prNumber。`,
  { label: "发布", phase: "合并发布", schema: mergeSchema },
);

return {
  task: { slug, title },
  merged: mergeResult ? mergeResult.merged : false,
  prUrl: mergeResult ? mergeResult.prUrl : null,
  prNumber: mergeResult ? mergeResult.prNumber : null,
  roundsUsed: round,
  artifacts: {
    prd: `${TASKDIR}/prd.md`,
    plan: `${TASKDIR}/plan.md`,
    design: `${TASKDIR}/design.md`,
    testReport: `${TASKDIR}/test-report.md`,
    review: `${TASKDIR}/review.md`,
  },
  qa: qaResult,
  review: reviewResult,
};
