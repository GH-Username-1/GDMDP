import { invoke } from "@tauri-apps/api/tauri";
import { CommandResult, VaultEntry, PasswordConfig } from "../types";

/**
 * Service pour interagir avec le backend Tauri
 */
export class VaultService {
  /**
   * Crée un nouveau coffre-fort
   */
  static async createVault(
    path: string,
    masterPassword: string
  ): Promise<CommandResult<string>> {
    return await invoke("create_vault", {
      path,
      masterPassword,
    });
  }

  /**
   * Ouvre un coffre-fort existant
   */
  static async openVault(
    path: string,
    masterPassword: string
  ): Promise<CommandResult<VaultEntry[]>> {
    return await invoke("open_vault", {
      path,
      masterPassword,
    });
  }

  /**
   * Verrouille le coffre-fort
   */
  static async lockVault(): Promise<CommandResult<void>> {
    return await invoke("lock_vault");
  }

  /**
   * Vérifie si le coffre est verrouillé
   */
  static async isLocked(): Promise<CommandResult<boolean>> {
    return await invoke("is_locked");
  }

  /**
   * Liste toutes les entrées
   */
  static async listEntries(): Promise<CommandResult<VaultEntry[]>> {
    return await invoke("list_entries");
  }

  /**
   * Ajoute une nouvelle entrée
   */
  static async addEntry(
    serviceName: string,
    username: string,
    password: string,
    url?: string,
    notes?: string,
    tags: string[] = []
  ): Promise<CommandResult<VaultEntry>> {
    return await invoke("add_entry", {
      serviceName,
      username,
      password,
      url,
      notes,
      tags,
    });
  }

  /**
   * Met à jour une entrée existante
   */
  static async updateEntry(
    id: string,
    serviceName?: string,
    username?: string,
    password?: string,
    url?: string,
    notes?: string,
    tags?: string[]
  ): Promise<CommandResult<VaultEntry>> {
    return await invoke("update_entry", {
      id,
      serviceName,
      username,
      password,
      url,
      notes,
      tags,
    });
  }

  /**
   * Supprime une entrée
   */
  static async deleteEntry(id: string): Promise<CommandResult<void>> {
    return await invoke("delete_entry", { id });
  }

  /**
   * Recherche des entrées
   */
  static async searchEntries(
    query: string
  ): Promise<CommandResult<VaultEntry[]>> {
    return await invoke("search_entries", { query });
  }

  /**
   * Génère un mot de passe
   */
  static async generatePassword(
    config: PasswordConfig
  ): Promise<CommandResult<string>> {
    return await invoke("generate_password_cmd", {
      length: config.length,
      useUppercase: config.use_uppercase,
      useLowercase: config.use_lowercase,
      useDigits: config.use_digits,
      useSymbols: config.use_symbols,
      excludeAmbiguous: config.exclude_ambiguous,
    });
  }

  /**
   * Crée un backup
   */
  static async createBackup(): Promise<CommandResult<void>> {
    return await invoke("create_backup");
  }
}
