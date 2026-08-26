import { invoke } from "@tauri-apps/api/core";

import type { Entry, EntryInput, VaultStatus } from "./types";

export const api = {
  status: () => invoke<VaultStatus>("vault_status"),

  createVault: (password: string, remember: boolean) =>
    invoke<VaultStatus>("create_vault", { request: { password, remember } }),

  unlockVault: (password: string, remember: boolean) =>
    invoke<VaultStatus>("unlock_vault", { request: { password, remember } }),

  unlockRemembered: () => invoke<VaultStatus>("unlock_remembered"),

  lock: () => invoke<void>("lock_vault"),

  listEntries: () => invoke<Entry[]>("list_entries"),

  addEntry: (input: EntryInput) =>
    invoke<Entry>("add_entry", { input: toPayload(input) }),

  updateEntry: (id: string, input: EntryInput) =>
    invoke<Entry>("update_entry", { id, input: toPayload(input) }),

  deleteEntry: (id: string) => invoke<void>("delete_entry", { id }),

  changePassword: (newPassword: string, remember: boolean) =>
    invoke<void>("change_password", {
      request: { newPassword, remember },
    }),

  forget: () => invoke<void>("forget_vault"),
};

function toPayload(input: EntryInput) {
  return {
    title: input.title,
    username: input.username,
    password: input.password,
    urls: input.urls,
    notes: input.notes,
  };
}
