/**
 * Types pour l'interface Tauri
 */

export interface CommandResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface VaultEntry {
  id: string;
  service_name: string;
  username: string;
  password: string;
  url?: string;
  notes?: string;
  tags: string[];
  created_at: number;
  updated_at: number;
}

export interface PasswordConfig {
  length: number;
  use_uppercase: boolean;
  use_lowercase: boolean;
  use_digits: boolean;
  use_symbols: boolean;
  exclude_ambiguous: boolean;
}

export interface LockStatus {
  is_locked: boolean;
  vault_path?: string;
}
