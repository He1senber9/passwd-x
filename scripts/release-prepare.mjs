#!/usr/bin/env node
// 计算下一个版本号，并更新 Cargo.toml / app/package.json / Cargo.lock / CHANGELOG.md。
// 无新提交时输出 NOCHANGE（供 release 工作流跳过发布）。
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";

function git(args) {
  try {
    return execFileSync("git", args, {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();
  } catch {
    return "";
  }
}

const cargo = readFileSync("Cargo.toml", "utf8");
const match = cargo.match(/^version\s*=\s*"([^"]+)"/m);
if (!match) {
  throw new Error("未找到 [workspace.package].version");
}
const current = match[1];

const lastTag = git(["describe", "--tags", "--abbrev=0"]);
const subjects = git([
  "log",
  "--format=%s",
  lastTag ? `${lastTag}..HEAD` : "HEAD",
])
  .split("\n")
  .filter(Boolean);

if (subjects.length === 0) {
  console.log("NOCHANGE");
  process.exit(0);
}

const breaking = (s) => /^[a-z]+!:/i.test(s) || /BREAKING CHANGE/i.test(s);
const feat = (s) => /^feat([!(]|:)/i.test(s);
const [major, minor, patch] = current.split(".").map(Number);

let next;
if (major === 0) {
  next = subjects.some((s) => breaking(s) || feat(s))
    ? [major, minor + 1, 0]
    : [major, minor, patch + 1];
} else if (subjects.some(breaking)) {
  next = [major + 1, 0, 0];
} else if (subjects.some(feat)) {
  next = [major, minor + 1, 0];
} else {
  next = [major, minor, patch + 1];
}
const version = next.join(".");

writeFileSync(
  "Cargo.toml",
  cargo.replace(/^version\s*=\s*"[^"]+"/m, `version = "${version}"`),
);
writeFileSync(
  "app/package.json",
  readFileSync("app/package.json", "utf8").replace(
    /"version":\s*"[^"]+"/,
    `"version": "${version}"`,
  ),
);
writeFileSync(
  "Cargo.lock",
  readFileSync("Cargo.lock", "utf8").replaceAll(
    `version = "${current}"`,
    `version = "${version}"`,
  ),
);

const date = new Date().toISOString().slice(0, 10);
const section = `## ${version} (${date})\n\n${subjects
  .map((s) => `- ${s}`)
  .join("\n")}\n`;
let changelog = "# Changelog\n\n";
try {
  changelog = readFileSync("CHANGELOG.md", "utf8");
} catch {
  // 首次生成
}
writeFileSync(
  "CHANGELOG.md",
  changelog.startsWith("# Changelog")
    ? changelog.replace(/^(# Changelog\n\n)/, `$1${section}`)
    : `${changelog}\n${section}`,
);

console.log(version);
