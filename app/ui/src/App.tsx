import { useCallback, useEffect, useState } from "react";

import { api } from "./api";
import type { Entry, EntryInput, VaultStatus } from "./types";

type LockedMode = "unlock" | "create";

const emptyForm: EntryInput = {
  title: "",
  username: "",
  password: "",
  urls: [],
  notes: "",
};

export default function App() {
  const [status, setStatus] = useState<VaultStatus | null>(null);
  const [entries, setEntries] = useState<Entry[]>([]);

  const [mode, setMode] = useState<LockedMode>("unlock");
  const [password, setPassword] = useState("");
  const [confirm, setConfirm] = useState("");
  const [remember, setRemember] = useState(false);

  const [editing, setEditing] = useState<Entry | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [form, setForm] = useState<EntryInput>(emptyForm);

  const [changingPassword, setChangingPassword] = useState(false);
  const [newPassword, setNewPassword] = useState("");
  const [newConfirm, setNewConfirm] = useState("");

  const [error, setError] = useState("");
  const [busy, setBusy] = useState(false);

  const refresh = useCallback(async () => {
    const next = await api.status();
    setStatus(next);
    setEntries(next.unlocked ? await api.listEntries() : []);
  }, []);

  useEffect(() => {
    refresh().catch((err) => setError(String(err)));
  }, [refresh]);

  const run = useCallback(async (action: () => Promise<unknown>) => {
    setBusy(true);
    setError("");
    try {
      await action();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, []);

  const submitLocked = () =>
    run(async () => {
      if (mode === "create") {
        if (password !== confirm) throw new Error("两次输入的密码不一致");
        if (password.length < 8) throw new Error("主密码至少需要 8 个字符");
        await api.createVault(password, remember);
      } else {
        await api.unlockVault(password, remember);
      }
      setPassword("");
      setConfirm("");
      await refresh();
    });

  const unlockRemembered = () =>
    run(async () => {
      await api.unlockRemembered();
      await refresh();
    });

  const lock = () =>
    run(async () => {
      await api.lock();
      setShowForm(false);
      setEditing(null);
      await refresh();
    });

  const openNew = () => {
    setForm(emptyForm);
    setEditing(null);
    setShowForm(true);
  };

  const openEdit = (entry: Entry) => {
    setForm({
      title: entry.title,
      username: entry.username,
      password: entry.password,
      urls: entry.urls,
      notes: entry.notes,
    });
    setEditing(entry);
    setShowForm(true);
  };

  const submitForm = () =>
    run(async () => {
      const urls = form.urls
        .join("\n")
        .split("\n")
        .map((url) => url.trim())
        .filter(Boolean);
      const input = { ...form, urls };
      if (editing) {
        await api.updateEntry(editing.id, input);
      } else {
        await api.addEntry(input);
      }
      setShowForm(false);
      setEditing(null);
      await refresh();
    });

  const removeEntry = (entry: Entry) => {
    if (!window.confirm(`确定删除「${entry.title}」吗？`)) return;
    run(async () => {
      await api.deleteEntry(entry.id);
      await refresh();
    });
  };

  const submitChangePassword = () =>
    run(async () => {
      if (newPassword !== newConfirm) throw new Error("两次输入的新密码不一致");
      if (newPassword.length < 8) throw new Error("主密码至少需要 8 个字符");
      await api.changePassword(newPassword, remember);
      setChangingPassword(false);
      setNewPassword("");
      setNewConfirm("");
      await refresh();
    });

  const forget = () => {
    if (!window.confirm("确定忘记本机保存的解锁凭据吗？")) return;
    run(async () => {
      await api.forget();
    });
  };

  if (status === null) {
    return <div className="center">载入中…</div>;
  }

  if (!status.unlocked) {
    return (
      <div className="screen">
        <div className="card">
          <h1>passwd-x</h1>
          <p className="muted">本地加密保险库</p>

          <div className="tabs">
            <button
              className={mode === "unlock" ? "active" : ""}
              onClick={() => setMode("unlock")}
            >
              解锁
            </button>
            <button
              className={mode === "create" ? "active" : ""}
              onClick={() => setMode("create")}
            >
              新建保险库
            </button>
          </div>

          <input
            type="password"
            placeholder="主密码"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            autoFocus
          />
          {mode === "create" && (
            <input
              type="password"
              placeholder="再次输入主密码"
              value={confirm}
              onChange={(e) => setConfirm(e.target.value)}
            />
          )}

          <label className="row">
            <input
              type="checkbox"
              checked={remember}
              onChange={(e) => setRemember(e.target.checked)}
            />
            记住本机（使用系统凭据库免密解锁）
          </label>

          <button disabled={busy} onClick={submitLocked} className="primary">
            {mode === "create" ? "创建并解锁" : "解锁"}
          </button>
          {mode === "unlock" && (
            <button disabled={busy} onClick={unlockRemembered} className="ghost">
              使用已保存的本机凭据
            </button>
          )}

          {error && <p className="error">{error}</p>}
        </div>
      </div>
    );
  }

  return (
    <div className="screen">
      <header className="toolbar">
        <h1>passwd-x</h1>
        <div className="spacer" />
        <button onClick={() => setChangingPassword(true)}>修改主密码</button>
        <button onClick={forget}>忘记本机凭据</button>
        <button onClick={lock}>锁定</button>
      </header>

      <main>
        <div className="row between">
          <h2>记录（{entries.length}）</h2>
          <button onClick={openNew} className="primary">
            新增记录
          </button>
        </div>

        {error && <p className="error">{error}</p>}

        {entries.length === 0 ? (
          <p className="muted empty">还没有记录，点击「新增记录」开始。</p>
        ) : (
          <table>
            <thead>
              <tr>
                <th>标题</th>
                <th>用户名</th>
                <th>网站</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {entries.map((entry) => (
                <tr key={entry.id}>
                  <td>{entry.title}</td>
                  <td>{entry.username}</td>
                  <td>{entry.urls.join(", ") || "—"}</td>
                  <td className="row">
                    <button onClick={() => openEdit(entry)}>编辑</button>
                    <button onClick={() => removeEntry(entry)} className="danger">
                      删除
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </main>

      {showForm && (
        <div className="overlay">
          <div className="card">
            <h2>{editing ? "编辑记录" : "新增记录"}</h2>
            <input
              placeholder="标题"
              value={form.title}
              onChange={(e) => setForm({ ...form, title: e.target.value })}
              autoFocus
            />
            <input
              placeholder="用户名"
              value={form.username}
              onChange={(e) => setForm({ ...form, username: e.target.value })}
            />
            <input
              placeholder="密码"
              value={form.password}
              onChange={(e) => setForm({ ...form, password: e.target.value })}
            />
            <textarea
              placeholder="网址（每行一个）"
              value={form.urls.join("\n")}
              onChange={(e) =>
                setForm({ ...form, urls: e.target.value.split("\n") })
              }
            />
            <textarea
              placeholder="备注"
              value={form.notes}
              onChange={(e) => setForm({ ...form, notes: e.target.value })}
            />
            {error && <p className="error">{error}</p>}
            <div className="row">
              <button
                onClick={() => {
                  setShowForm(false);
                  setEditing(null);
                }}
              >
                取消
              </button>
              <button onClick={submitForm} disabled={busy} className="primary">
                保存
              </button>
            </div>
          </div>
        </div>
      )}

      {changingPassword && (
        <div className="overlay">
          <div className="card">
            <h2>修改主密码</h2>
            <input
              type="password"
              placeholder="新主密码"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
              autoFocus
            />
            <input
              type="password"
              placeholder="再次输入新主密码"
              value={newConfirm}
              onChange={(e) => setNewConfirm(e.target.value)}
            />
            <label className="row">
              <input
                type="checkbox"
                checked={remember}
                onChange={(e) => setRemember(e.target.checked)}
              />
              记住本机
            </label>
            {error && <p className="error">{error}</p>}
            <div className="row">
              <button
                onClick={() => {
                  setChangingPassword(false);
                  setNewPassword("");
                  setNewConfirm("");
                }}
              >
                取消
              </button>
              <button
                onClick={submitChangePassword}
                disabled={busy}
                className="primary"
              >
                确认修改
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
