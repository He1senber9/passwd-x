export interface Entry {
  id: string;
  title: string;
  username: string;
  password: string;
  urls: string[];
  notes: string;
  createdAt: number;
  updatedAt: number;
}

export interface EntryInput {
  title: string;
  username: string;
  password: string;
  urls: string[];
  notes: string;
}

export interface VaultStatus {
  unlocked: boolean;
  entryCount: number;
}
