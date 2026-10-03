/**
 * @module DatabaseManagerStore
 *
 * Reactive state machine and orchestration for Orbis Database Management.
 * Built with Svelte 5 Runes for instant, transport-agnostic reactivity.
 */

import { api } from "$lib/api/client";
import type {
  DatabaseStatusDto,
  ConnectionConfig,
  DatabaseSettingsDto,
  RemoteHealthResult,
  BackupInspectionDto,
  IntegrityResultDto,
  OptimizeResultDto,
  ConnectionMode,
} from "$lib/api/types";
import { confirmStore } from "$lib/stores";

export class DatabaseManagerController {
  status = $state<DatabaseStatusDto | null>(null);
  config = $state<ConnectionConfig | null>(null);
  settings = $state<DatabaseSettingsDto | null>(null);
  isLoading = $state(false);

  // Active operation state
  isOperating = $state(false);
  operationName = $state<string | null>(null);
  operationError = $state<string | null>(null);
  operationSuccess = $state<string | null>(null);

  // Health check & diagnostics
  isCheckingHealth = $state(false);
  healthResult = $state<RemoteHealthResult | null>(null);
  inspectedBackup = $state<BackupInspectionDto | null>(null);
  isInspectingBackup = $state(false);
  integrityResult = $state<IntegrityResultDto | null>(null);
  optimizeResult = $state<OptimizeResultDto | null>(null);

  // Dialog states
  isCreateDialogOpen = $state(false);

  // Derived statuses
  isConnected = $derived(this.status?.state === "connected");
  isMigrationRequired = $derived(this.status?.state === "migration_required" || (this.status?.pending_migrations.length ?? 0) > 0);
  isLocal = $derived(this.status?.mode === "local" || this.config?.mode === "local");

  async init(): Promise<void> {
    await this.refresh();
  }

  async refresh(): Promise<void> {
    this.isLoading = true;
    this.operationError = null;
    try {
      const [statusRes, configRes, settingsRes] = await Promise.allSettled([
        api.databaseGetStatus(),
        api.connectionGetConfig(),
        api.databaseGetSettings(),
      ]);

      if (statusRes.status === "fulfilled") {
        this.status = statusRes.value;
      }
      if (configRes.status === "fulfilled") {
        this.config = configRes.value;
      }
      if (settingsRes.status === "fulfilled") {
        this.settings = settingsRes.value;
      }
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async testRemoteHealth(serverUrl: string): Promise<RemoteHealthResult> {
    this.isCheckingHealth = true;
    this.healthResult = null;
    try {
      const result = await api.connectionCheckRemoteHealth(serverUrl);
      this.healthResult = result;
      return result;
    } finally {
      this.isCheckingHealth = false;
    }
  }

  async saveConnectionConfig(params: {
    mode: ConnectionMode;
    serverUrl?: string;
    dbFilename?: string;
    customPath?: string;
    host?: string;
    port?: number;
    databaseName?: string;
    username?: string;
    password?: string;
    sslMode?: string;
    walMode?: boolean;
    busyTimeoutMs?: number;
  }): Promise<void> {
    this.isOperating = true;
    this.operationName = "save_connection";
    this.operationError = null;
    this.operationSuccess = null;

    try {
      this.config = await api.connectionSetConfig(params);
      await api.connectionConnect();
      this.operationSuccess = "Connection configuration updated and connected successfully.";
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async resetConnectionConfig(): Promise<void> {
    const confirmed = await confirmStore.prompt({
      title: "Reset Connection Configuration?",
      description: "This will reset connection settings back to the default local SQLite database (orbis.db in AppData).",
      variant: "destructive",
      confirmText: "Reset Connection",
    });

    if (!confirmed) return;

    this.isOperating = true;
    this.operationName = "reset_connection";
    this.operationError = null;

    try {
      this.config = await api.connectionReset();
      await api.connectionConnect();
      this.operationSuccess = "Connection reset to default local database.";
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async createNewDatabase(params: { customDir?: string; filename: string }): Promise<void> {
    this.isOperating = true;
    this.operationName = "create_database";
    this.operationError = null;
    this.operationSuccess = null;

    try {
      const newStatus = await api.databaseCreateLocal(params);
      this.status = newStatus;
      this.isCreateDialogOpen = false;
      this.operationSuccess = `New database "${newStatus.database_name}" created and initialized successfully.`;
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async openExistingDatabase(filePath: string): Promise<void> {
    this.isOperating = true;
    this.operationName = "open_database";
    this.operationError = null;
    this.operationSuccess = null;

    try {
      const newStatus = await api.databaseOpenExisting(filePath);
      this.status = newStatus;
      this.operationSuccess = `Successfully connected to database "${newStatus.database_name}".`;
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async triggerBackup(destinationPath?: string): Promise<void> {
    this.isOperating = true;
    this.operationName = "backup";
    this.operationError = null;
    this.operationSuccess = null;

    try {
      const result = await api.databaseBackup(destinationPath);
      this.operationSuccess = `Backup successfully created at: ${result.backup_path} (${result.file_size_formatted})`;
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async inspectBackupFile(filePath: string): Promise<BackupInspectionDto | null> {
    this.isInspectingBackup = true;
    this.inspectedBackup = null;
    this.operationError = null;

    try {
      const result = await api.databaseValidateBackupFile(filePath);
      this.inspectedBackup = result;
      return result;
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
      return null;
    } finally {
      this.isInspectingBackup = false;
    }
  }

  async restoreFromBackup(backupFilePath: string): Promise<void> {
    const confirmed = await confirmStore.prompt({
      title: "Confirm Database Restoration",
      description: `Are you sure you want to restore the database from "${backupFilePath}"? This is a destructive operation that will replace active records. An automatic safety snapshot will be taken first.`,
      variant: "destructive",
      confirmText: "Proceed with Restore",
    });

    if (!confirmed) return;

    this.isOperating = true;
    this.operationName = "restore";
    this.operationError = null;
    this.operationSuccess = null;

    try {
      const result = await api.databaseRestore(backupFilePath);
      this.operationSuccess = `Database successfully restored! (Pre-restore safety snapshot saved at: ${result.pre_restore_backup_path ?? "backups directory"})`;
      this.inspectedBackup = null;
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async applyPendingMigrations(): Promise<void> {
    this.isOperating = true;
    this.operationName = "migrations";
    this.operationError = null;
    this.operationSuccess = null;

    try {
      const result = await api.databaseRunMigrations();
      this.operationSuccess = result.message;
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async runIntegrityCheck(): Promise<void> {
    this.isOperating = true;
    this.operationName = "integrity";
    this.operationError = null;

    try {
      const result = await api.databaseCheckIntegrity();
      this.integrityResult = result;
      if (result.is_healthy) {
        this.operationSuccess = "Integrity check passed with 0 errors. Database is healthy.";
      } else {
        this.operationError = `Integrity issues detected: ${result.details.join(", ")}`;
      }
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async optimizeDatabase(): Promise<void> {
    this.isOperating = true;
    this.operationName = "optimize";
    this.operationError = null;

    try {
      const result = await api.databaseOptimize();
      this.optimizeResult = result;
      this.operationSuccess = `Optimization complete. Freed space: ${result.space_freed_formatted}.`;
      await this.refresh();
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  async saveSettings(newSettings: DatabaseSettingsDto): Promise<void> {
    this.isOperating = true;
    this.operationName = "settings";
    this.operationError = null;

    try {
      this.settings = await api.databaseUpdateSettings(newSettings);
      this.operationSuccess = "Database settings saved successfully.";
    } catch (e) {
      this.operationError = e instanceof Error ? e.message : String(e);
    } finally {
      this.isOperating = false;
      this.operationName = null;
    }
  }

  // ── Native Picker Helpers ──

  async pickFile(filterName?: string, extensions?: string[]): Promise<string | null> {
    try {
      return await api.databasePickFile({ filterName, extensions });
    } catch {
      return null;
    }
  }

  async pickFolder(): Promise<string | null> {
    try {
      return await api.databasePickFolder();
    } catch {
      return null;
    }
  }

  async pickSavePath(defaultFilename?: string, filterName?: string, extensions?: string[]): Promise<string | null> {
    try {
      return await api.databasePickSavePath({ defaultFilename, filterName, extensions });
    } catch {
      return null;
    }
  }
}
