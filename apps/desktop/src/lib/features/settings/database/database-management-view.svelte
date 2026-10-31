<!-- @component Database management for connection, backup, restore, maintenance, and diagnostics. -->
<script lang="ts">
import type { ConnectionMode } from "$lib/api/types";
import * as m from "$lib/paraglide/messages";
import {
  ArrowDownTray,
  ArrowPath,
  ArrowUpTray,
  Bolt,
  ChartBar,
  CheckCircle,
  ClockRotateRight,
  Computer,
  Database,
  FolderOpen,
  GlobeAlt,
  InfoCircle,
  Plus,
  ShieldCheck,
  Sparkles,
} from "@orbis/icons";
import { FormField, LoadingIndicator, toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Skeleton } from "@orbis/ui/skeleton";
import { Switch } from "@orbis/ui/switch";
import { onMount } from "svelte";
import {
  getDesktopCommandPerformance,
  isDesktopCommandPerformanceActive,
  resetDesktopCommandPerformance,
  startDesktopCommandPerformance,
  stopDesktopCommandPerformance,
  type DesktopCommandPerformance,
} from "$lib/api/desktop-command-performance";
import CreateDatabaseDialog from "./create-database-dialog.svelte";
import DatabaseManagementHeader from "./database-management-header.svelte";
import DatabaseManagementMetrics from "./database-management-metrics.svelte";
import DatabaseManagementNavigation from "./database-management-navigation.svelte";
import type {
  DatabaseManagementSection,
  DatabaseNavigationItem,
} from "./database-management-types";
import { DatabaseManagerController } from "./database-manager.svelte";

let { orgId }: { orgId: string } = $props();

const controller = new DatabaseManagerController();

let activeSection = $state<DatabaseManagementSection>("overview");
let commandPerformance = $state<DesktopCommandPerformance[]>([]);
let isRecordingPerformance = $state(isDesktopCommandPerformanceActive());

// Connection form state
let connMode = $state<ConnectionMode>("local");
let connServerUrl = $state("");
let connDbFilename = $state("orbis.db");
let connCustomPath = $state("");
let connHost = $state("localhost");
let connPort = $state<number>(5432);
let connDatabaseName = $state("orbis_db");
let connUsername = $state("postgres");
let connPassword = $state("");
let connSslMode = $state("prefer");
let connWalMode = $state(true);
let connBusyTimeout = $state(5000);

// Backup form state
let customBackupPath = $state("");

// Restore form state
let restoreFilePath = $state("");

// Settings state
let autoBackup = $state(false);
let backupInterval = $state(24);
let maxBackups = $state(10);
let customBackupDir = $state("");
let integrityOnStartup = $state(true);

onMount(async () => {
  await controller.init();
  syncFormValues();
});

onMount(() => () => stopDesktopCommandPerformance());

function syncFormValues() {
  if (controller.config) {
    connMode = controller.config.mode;
    if (controller.config.local) {
      connDbFilename = controller.config.local.db_filename || "orbis.db";
      connCustomPath = controller.config.local.custom_path || "";
      connWalMode = controller.config.local.wal_mode ?? true;
      connBusyTimeout = controller.config.local.busy_timeout_ms ?? 5000;
    }
    if (controller.config.api) {
      connServerUrl = controller.config.api.server_url || "";
    }
    if (controller.config.direct) {
      connHost = controller.config.direct.host || "localhost";
      connPort = controller.config.direct.port || 5432;
      connDatabaseName = controller.config.direct.database_name || "orbis_db";
      connUsername = controller.config.direct.username || "";
      connPassword = "";
      connSslMode = controller.config.direct.ssl_mode || "prefer";
    }
  }

  if (controller.settings) {
    autoBackup = controller.settings.auto_backup_enabled;
    backupInterval = controller.settings.auto_backup_interval_hours;
    maxBackups = controller.settings.max_backup_files;
    customBackupDir = controller.settings.backup_dir || "";
    integrityOnStartup = controller.settings.integrity_check_on_startup ?? true;
  }
}

function formatDuration(durationMs: number): string {
  return `${durationMs.toFixed(durationMs < 10 ? 2 : 1)} ms`;
}

function clearCommandPerformance() {
  stopDesktopCommandPerformance();
  isRecordingPerformance = false;
  resetDesktopCommandPerformance();
  commandPerformance = [];
}

function refreshCommandPerformance() {
  commandPerformance = getDesktopCommandPerformance();
}

function startPerformanceRecording() {
  startDesktopCommandPerformance();
  isRecordingPerformance = true;
  commandPerformance = [];
}

function stopPerformanceRecording() {
  stopDesktopCommandPerformance();
  isRecordingPerformance = false;
  refreshCommandPerformance();
}

function selectSection(section: DatabaseManagementSection) {
  if (activeSection === "performance" && section !== "performance") {
    stopPerformanceRecording();
  }
  activeSection = section;
  if (section === "performance") refreshCommandPerformance();
}

function commandBarWidth(durationMs: number): string {
  const maximum = Math.max(
    ...commandPerformance.map((metric) => metric.p95Ms),
    1,
  );
  return `${Math.max(4, Math.min(100, (durationMs / maximum) * 100))}%`;
}

const performanceSummary = $derived.by(() => ({
  commands: commandPerformance.length,
  calls: commandPerformance.reduce((sum, metric) => sum + metric.totalCalls, 0),
  failures: commandPerformance.reduce(
    (sum, metric) => sum + metric.failedCalls,
    0,
  ),
  p95: commandPerformance[0]?.p95Ms ?? null,
}));

async function handleTestRemote() {
  if (!connServerUrl.trim()) {
    toast.error(m.ui_copy_please_enter_a_server_url_4a739f83());
    return;
  }
  const result = await controller.testRemoteHealth(connServerUrl.trim());
  if (result.reachable) {
    toast.success(
      m.ui_server_connected_latency({ latency: result.latency_ms }),
    );
  } else {
    toast.error(
      result.error || m.ui_copy_unable_to_connect_to_server_147a38a8(),
    );
  }
}

async function handleSaveConnection() {
  try {
    await controller.saveConnectionConfig({
      mode: connMode,
      serverUrl: connMode === "api" ? connServerUrl.trim() : undefined,
      dbFilename: connMode === "local" ? connDbFilename.trim() : undefined,
      customPath:
        connMode === "local" && connCustomPath.trim()
          ? connCustomPath.trim()
          : undefined,
      databaseEngine: connMode === "direct" ? "postgres" : undefined,
      host: connMode === "direct" ? connHost.trim() : undefined,
      port: connMode === "direct" ? connPort : undefined,
      databaseName: connMode === "direct" ? connDatabaseName.trim() : undefined,
      username: connMode === "direct" ? connUsername.trim() : undefined,
      password:
        connMode === "direct" && connPassword ? connPassword : undefined,
      sslMode: connMode === "direct" ? connSslMode : undefined,
      walMode: connMode === "local" ? connWalMode : undefined,
      busyTimeoutMs: connMode === "local" ? connBusyTimeout : undefined,
    });
    toast.success(m.ui_copy_connection_settings_saved_successfully_bb97a224());
  } catch (e) {
    toast.error(e instanceof Error ? e.message : String(e));
  }
}

async function handleBrowseCustomDbPath() {
  const file = await controller.pickFile(
    m.ui_copy_orbis_databases_db_2636ac20(),
    ["db", "sqlite", "sqlite3"],
  );
  if (file) {
    connCustomPath = file;
  }
}

async function handleBrowseBackupDest() {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
  const file = await controller.pickSavePath(
    `orbis_backup_${timestamp}.db`,
    m.ui_copy_backup_file_db_8e8097bf(),
    ["db", "sqlite", "bak"],
  );
  if (file) {
    customBackupPath = file;
  }
}

async function handleTriggerBackup() {
  await controller.triggerBackup(
    customBackupPath.trim() ? customBackupPath.trim() : undefined,
  );
  if (controller.operationSuccess) {
    toast.success(controller.operationSuccess);
    customBackupPath = "";
  } else if (controller.operationError) {
    toast.error(controller.operationError);
  }
}

async function handleBrowseRestoreFile() {
  const file = await controller.pickFile(m.ui_copy_backup_file_db_8e8097bf(), [
    "db",
    "sqlite",
    "bak",
  ]);
  if (file) {
    restoreFilePath = file;
    await controller.inspectBackupFile(file);
  }
}

async function handleExecuteRestore() {
  if (!restoreFilePath.trim()) {
    toast.error(m.ui_copy_please_select_a_backup_file_first_8b8a6c8d());
    return;
  }
  await controller.restoreFromBackup(restoreFilePath.trim());
  if (controller.operationSuccess) {
    toast.success(controller.operationSuccess);
    restoreFilePath = "";
    syncFormValues();
  } else if (controller.operationError) {
    toast.error(controller.operationError);
  }
}

async function handleSaveSettings() {
  try {
    await controller.saveSettings({
      auto_backup_enabled: autoBackup,
      auto_backup_interval_hours: backupInterval,
      max_backup_files: maxBackups,
      backup_dir: customBackupDir.trim() ? customBackupDir.trim() : undefined,
      integrity_check_on_startup: integrityOnStartup,
      last_backup_timestamp: controller.settings?.last_backup_timestamp,
      last_backup_path: controller.settings?.last_backup_path,
    });
    toast.success(m.ui_copy_backup_policies_saved_successfully_78b2a35b());
  } catch (e) {
    toast.error(e instanceof Error ? e.message : String(e));
  }
}

async function handleOpenExistingDatabase() {
  const file = await controller.pickFile(
    m.ui_copy_sqlite_database_db_08cc781c(),
    ["db", "sqlite", "sqlite3"],
  );
  if (file) {
    await controller.openExistingDatabase(file);
    if (controller.operationSuccess) {
      toast.success(controller.operationSuccess);
      syncFormValues();
    } else if (controller.operationError) {
      toast.error(controller.operationError);
    }
  }
}

// KPI Stats
const kpiStats = $derived([
  {
    label: m.ui_copy_engine_mode_7a8fe27f(),
    value: controller.isLocal
      ? m.settings_ui_database_sqlite_embedded()
      : connMode === "direct"
        ? m.database_direct_postgres()
        : m.ui_copy_remote_server_efd114c0(),
    sub: controller.isLocal
      ? m.ui_copy_local_embedded_36bd9bbd()
      : connMode === "direct"
        ? m.database_direct_network()
        : m.ui_copy_remote_cloud_51770fc6(),
  },
  {
    label: m.ui_copy_storage_size_8122fef2(),
    value: controller.status?.file_size_formatted || "—",
    sub: controller.status?.journal_mode
      ? `WAL: ${controller.status.journal_mode}`
      : m.ui_copy_indexed_da85078a(),
  },
  {
    label: m.database_card_version(),
    value: controller.status?.applied_migrations_count
      ? `v${controller.status.applied_migrations_count}.0`
      : controller.status?.schema_version || "v1.0.0",
    sub: controller.status?.schema_version
      ? controller.status.schema_version
          .replace(/^m\d+_\d+_/, "")
          .replace(/_/g, " ")
      : m.ui_copy_current_7b958d7b(),
    fullTitle: controller.status?.schema_version || "v1.0.0",
  },
  {
    label: m.ui_copy_applied_migrations_b57f13d8(),
    value: `${controller.status?.applied_migrations_count ?? 0} / ${controller.status?.total_migrations_count ?? 0}`,
    sub:
      (controller.status?.pending_migrations.length ?? 0) > 0
        ? m.ui_pending_migrations_count({
            count: controller.status?.pending_migrations.length ?? 0,
          })
        : m.ui_copy_100_synced_085c434f(),
  },
]);

const directoryItems: DatabaseNavigationItem[] = $derived([
  {
    id: "overview" as const,
    title: m.database_tab_overview(),
  },
  {
    id: "connection" as const,
    title: m.database_tab_connection(),
  },
  ...(controller.isLocal
    ? [
        {
          id: "backup" as const,
          title: m.database_tab_backup(),
        },
        {
          id: "restore" as const,
          title: m.database_tab_restore(),
        },
        {
          id: "maintenance" as const,
          title: m.database_tab_maintenance(),
        },
      ]
    : []),
  { id: "performance" as const, title: m.database_tab_performance() },
]);

const connectionStatus = $derived.by(() => {
  const status = controller.status?.state;
  if (!status) return { label: null, tone: null };
  if (status === "connected") {
    return { label: m.database_status_connected(), tone: "connected" as const };
  }
  if (status === "migration_required") {
    return {
      label: m.database_status_migration_required(),
      tone: "warning" as const,
    };
  }
  return {
    label: m.database_status_disconnected(),
    tone: "disconnected" as const,
  };
});
</script>

<div class="flex h-full w-full min-w-0 flex-col overflow-hidden bg-background font-sans">
  <DatabaseManagementHeader
    connectionMode={connMode}
    isLocal={controller.isLocal}
    isLoading={controller.isLoading}
    isOperating={controller.isOperating}
    statusLabel={connectionStatus.label}
    statusTone={connectionStatus.tone}
    onRefresh={() => controller.refresh()}
    onOpenExistingDatabase={handleOpenExistingDatabase}
    onCreateDatabase={() => (controller.isCreateDialogOpen = true)}
  />

  <div class="min-h-0 flex-1 overflow-y-auto scrollbar-hide">
    {#if controller.operationError}
    <div class="mx-4 mt-3 flex shrink-0 items-center justify-between gap-3 rounded-lg border border-destructive/25 bg-destructive/10 px-3 py-2.5 text-xs text-destructive sm:mx-5" role="alert">
      <div class="flex min-w-0 items-center gap-2.5">
        <InfoCircle size={15} class="shrink-0" />
        <span>{controller.operationError}</span>
      </div>
      <Button variant="ghost" size="sm" class="h-7 shrink-0 px-2 text-[10px] text-destructive hover:bg-destructive/20" onclick={() => controller.operationError = null}>
        {m.ui_copy_dismiss_f18c09a6()}
      </Button>
    </div>
    {/if}

    {#if controller.operationSuccess}
    <div class="mx-4 mt-3 flex shrink-0 items-center justify-between gap-3 rounded-lg border border-emerald-500/20 bg-emerald-500/10 px-3 py-2.5 text-xs text-emerald-600 sm:mx-5 dark:text-emerald-400" role="status">
      <div class="flex min-w-0 items-center gap-2.5">
        <CheckCircle size={15} class="shrink-0" />
        <span>{controller.operationSuccess}</span>
      </div>
      <Button variant="ghost" size="sm" class="h-7 shrink-0 px-2 text-[10px] text-emerald-600 hover:bg-emerald-500/20" onclick={() => controller.operationSuccess = null}>
        {m.ui_copy_dismiss_f18c09a6()}
      </Button>
    </div>
    {/if}

    {#if controller.isLoading && !controller.status}
      <div class="mx-auto grid w-full max-w-7xl gap-5 px-4 py-5 sm:px-6">
        <div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
          {#each Array(4) as _, index (index)}
            <Skeleton class="h-16 rounded-md" />
          {/each}
        </div>
        <Skeleton class="h-10 w-full rounded-md" />
        <Skeleton class="h-64 w-full rounded-md" />
      </div>
    {:else}
      <div class="mx-auto flex w-full max-w-7xl flex-col gap-5 px-4 py-4 sm:px-6 sm:py-5">
        <DatabaseManagementMetrics stats={kpiStats} />
        <DatabaseManagementNavigation
          {activeSection}
          items={directoryItems}
          onSelect={selectSection}
        />

        <main
          id="database-management-content"
          class="min-w-0 text-start"
        >
        <section class="space-y-4">
          
          <!-- ═══════════════════════════════════════════════════════════ -->
          <!-- SUBVIEW 1: OVERVIEW & HEALTH -->
          <!-- ═══════════════════════════════════════════════════════════ -->
          {#if activeSection === "overview"}
            <div class="space-y-4 text-start">
              <div class="flex items-center justify-between pb-3 border-b border-border/40">
                <h3 class="text-xs font-bold text-foreground">
                  {m.ui_copy_active_database_target_storage_details_e83fa545()}
                </h3>
                <Badge variant="outline" class="text-[9px] font-mono">
                  {controller.status?.mode?.toUpperCase() || "SQLITE"}
                </Badge>
              </div>

              <div class="space-y-3 text-xs">
                <div class="p-3 rounded-lg bg-card/40 border border-border/40 space-y-1">
                  <span class="text-[10px] font-semibold text-muted-foreground">{m.ui_copy_storage_path_d44cf816()}</span>
                  <p class="font-mono text-[10.5px] text-foreground break-all select-all">
                    {controller.status?.connection_target || m.common_loading()}
                  </p>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div class="p-3 rounded-lg bg-card/40 border border-border/40">
                    <span class="text-[10px] font-semibold text-muted-foreground">{m.ui_copy_integrity_check_53c0a8af()}</span>
                    <div class="flex items-center gap-1.5 mt-1 font-bold text-xs text-emerald-600 dark:text-emerald-400">
                      <CheckCircle size={13} />
                      <span>{controller.status?.integrity_status === "ok" ? m.ui_copy_healthy_ok_6c002014() : controller.status?.integrity_status || m.ui_copy_ok_217bf3e2()}</span>
                    </div>
                  </div>

                  <div class="p-3 rounded-lg bg-card/40 border border-border/40">
                    <span class="text-[10px] font-semibold text-muted-foreground">{m.ui_copy_version_compatibility_c24b4981()}</span>
                    <div class="flex items-center gap-1.5 mt-1 font-bold text-xs {controller.status?.is_compatible ? 'text-emerald-600 dark:text-emerald-400' : 'text-amber-600 dark:text-amber-400'}">
                      <Bolt size={13} />
                      <span>{controller.status?.is_compatible ? m.ui_copy_fully_compatible_a73d9d43() : m.ui_copy_pending_migrations_51b08493()}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Quick Operations Grid -->
              <div class="pt-2 border-t border-border/40">
                <h4 class="text-[10px] font-bold text-muted-foreground/80 tracking-wider uppercase mb-2.5">
                  {m.ui_copy_quick_operations_5a6750ff()}
                </h4>
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
                  <Button
                    type="button"
                    variant="outline"
                    class="justify-start text-[11px] h-9 gap-2 border-border/60 bg-card/40 hover:bg-muted/40 cursor-pointer"
                    onclick={handleTriggerBackup}
                    disabled={controller.isOperating}
                  >
                    <ArrowDownTray size={14} class="text-primary shrink-0" />
                    <span class="font-bold truncate">{m.database_action_backup_now?.() || m.ui_copy_backup_now_6ec7f13d()}</span>
                  </Button>

                  <Button
                    type="button"
                    variant="outline"
                    class="justify-start text-[11px] h-9 gap-2 border-border/60 bg-card/40 hover:bg-muted/40 cursor-pointer"
                    onclick={() => controller.runIntegrityCheck()}
                    disabled={controller.isOperating}
                  >
                    <ShieldCheck size={14} class="text-emerald-500 shrink-0" />
                    <span class="font-bold truncate">{m.database_action_check_integrity?.() || m.ui_copy_check_integrity_bbcabc81()}</span>
                  </Button>

                  <Button
                    type="button"
                    variant="outline"
                    class="justify-start text-[11px] h-9 gap-2 border-border/60 bg-card/40 hover:bg-muted/40 cursor-pointer"
                    onclick={() => controller.optimizeDatabase()}
                    disabled={controller.isOperating}
                  >
                    <Sparkles size={14} class="text-amber-500 shrink-0" />
                    <span class="font-bold truncate">{m.database_action_optimize?.() || m.ui_copy_compact_db_c94fa31d()}</span>
                  </Button>
                </div>
              </div>
            </div>
          {/if}

          <!-- ═══════════════════════════════════════════════════════════ -->
          <!-- SUBVIEW 2: CONNECTION SETTINGS -->
          <!-- ═══════════════════════════════════════════════════════════ -->
          {#if activeSection === "connection"}
            <div class="space-y-4 text-start">
              <div class="flex items-center justify-between pb-3 border-b border-border/40">
                <h3 class="text-xs font-bold text-foreground">
                  {m.ui_copy_connection_engine_configuration_c7d4c473()}
                </h3>
              </div>

              <!-- Mode Selector -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <button
                  type="button"
                  class="p-3 rounded-lg border transition-all flex items-center justify-between cursor-pointer {connMode === 'local' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border/50 bg-card/40 hover:border-border'}"
                  onclick={() => connMode = "local"}
                >
                  <div class="flex items-center gap-2.5">
                    <div class="size-7.5 rounded-lg flex items-center justify-center {connMode === 'local' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                      <Computer size={15} />
                    </div>
                    <div class="text-start">
                      <h4 class="text-xs font-bold text-foreground">{m.ui_copy_local_sqlite_baee2e7a()}</h4>
                      <p class="text-[9.5px] text-muted-foreground">{m.ui_copy_embedded_on_workstation_2e5f5f62()}</p>
                    </div>
                  </div>
                  {#if connMode === "local"}
                    <CheckCircle size={14} class="text-primary" />
                  {/if}
                </button>

                <button
                  type="button"
                  class="p-3 rounded-lg border transition-all flex items-center justify-between cursor-pointer {connMode === 'direct' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border/50 bg-card/40 hover:border-border'}"
                  onclick={() => connMode = "direct"}
                >
                  <div class="flex items-center gap-2.5">
                    <div class="size-7.5 rounded-lg flex items-center justify-center {connMode === 'direct' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                      <Database size={15} />
                    </div>
                    <div class="text-start">
                      <h4 class="text-xs font-bold text-foreground">{m.database_direct_postgres()}</h4>
                      <p class="text-[9.5px] text-muted-foreground">{m.database_direct_postgres_description()}</p>
                    </div>
                  </div>
                  {#if connMode === "direct"}
                    <CheckCircle size={14} class="text-primary" />
                  {/if}
                </button>

                <button
                  type="button"
                  class="p-3 rounded-lg border transition-all flex items-center justify-between cursor-pointer {connMode === 'api' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border/50 bg-card/40 hover:border-border'}"
                  onclick={() => connMode = "api"}
                >
                  <div class="flex items-center gap-2.5">
                    <div class="size-7.5 rounded-lg flex items-center justify-center {connMode === 'api' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                      <GlobeAlt size={15} />
                    </div>
                    <div class="text-start">
                      <h4 class="text-xs font-bold text-foreground">{m.ui_copy_remote_server_efd114c0()}</h4>
                      <p class="text-[9.5px] text-muted-foreground">{m.ui_copy_connects_to_a_central_orbis_api_server_for_ed966b7f()}</p>
                    </div>
                  </div>
                  {#if connMode === "api"}
                    <CheckCircle size={14} class="text-primary" />
                  {/if}
                </button>
              </div>

              <!-- Adaptive Form Fields -->
              {#if connMode === "local"}
                <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3.5">
                  <FormField id="connDbFilename" label={m.database_create_filename_label()}>
                    {#snippet children()}
                      <Input
                        id="connDbFilename"
                        type="text"
                        bind:value={connDbFilename}
                        class="h-8 text-xs font-mono bg-background border-input"
                      />
                    {/snippet}
                  </FormField>

                  <FormField id="connCustomPath" label={m.ui_copy_custom_absolute_file_path_optional_0c3adc12()}>
                    {#snippet children()}
                      <div class="flex gap-2">
                        <Input
                          id="connCustomPath"
                          type="text"
                          bind:value={connCustomPath}
                          placeholder={m.ui_copy_leave_blank_for_default_appdata_path_a40f6b20()}
                          class="h-8 text-xs font-mono bg-background border-input flex-1"
                        />
                        <Button
                          type="button"
                          variant="outline"
                          size="sm"
                          class="h-8 px-2.5 text-[11px] gap-1.5 shrink-0 border-border"
                          onclick={handleBrowseCustomDbPath}
                        >
                          <FolderOpen size={13} class="text-primary" />
                          <span>{m.ui_copy_browse_f7b87051()}</span>
                        </Button>
                      </div>
                    {/snippet}
                  </FormField>

                  <div class="pt-2 border-t border-border/30 flex items-center justify-between">
                    <div>
                      <div class="text-xs font-bold text-foreground">{m.ui_copy_sqlite_wal_mode_e1e34536()}</div>
                      <p class="text-[9.5px] text-muted-foreground">{m.ui_copy_enables_high_concurrency_for_simultaneous__51ac4597()}</p>
                    </div>
                    <Switch bind:checked={connWalMode} />
                  </div>
                </div>
              {:else if connMode === "api"}
                <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3.5">
                  <FormField id="connServerUrl" label={m.ui_copy_orbis_server_url_b7cff22f()} required>
                    {#snippet children()}
                      <div class="flex gap-2">
                        <Input
                          id="connServerUrl"
                          type="url"
                          bind:value={connServerUrl}
                          placeholder="http://localhost:3000"
                          class="h-8 text-xs font-mono bg-background border-input flex-1"
                        />
                        <Button
                          type="button"
                          variant="outline"
                          size="sm"
                          class="h-8 px-2.5 text-[11px] gap-1.5 shrink-0 border-border"
                          onclick={handleTestRemote}
                          disabled={controller.isCheckingHealth}
                        >
                          {#if controller.isCheckingHealth}
                            <LoadingIndicator class="text-primary"  size={13} />
                          {:else}
                            <Bolt size={13} class="text-primary" />
                          {/if}
                          <span>{m.database_action_test_connection?.() || m.ui_copy_test_cd34d3e9()}</span>
                        </Button>
                      </div>
                    {/snippet}
                  </FormField>

                </div>
              {:else}
                <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3.5">
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    <FormField id="connHost" label={m.ui_copy_database_host_218c7a89()}>
                      {#snippet children()}
                        <Input id="connHost" type="text" bind:value={connHost} class="h-8 text-xs font-mono bg-background border-input" />
                      {/snippet}
                    </FormField>

                    <FormField id="connPort" label={m.ui_copy_port_966ae516()}>
                      {#snippet children()}
                        <Input id="connPort" type="number" bind:value={connPort} class="h-8 text-xs font-mono bg-background border-input" />
                      {/snippet}
                    </FormField>

                    <FormField id="connDatabaseName" label={m.ui_copy_database_name_72beaf93()}>
                      {#snippet children()}
                        <Input id="connDatabaseName" type="text" bind:value={connDatabaseName} class="h-8 text-xs font-mono bg-background border-input" />
                      {/snippet}
                    </FormField>

                    <FormField id="connUsername" label={m.ui_copy_username_f733f5b3()}>
                      {#snippet children()}
                        <Input id="connUsername" type="text" bind:value={connUsername} class="h-8 text-xs font-mono bg-background border-input" />
                      {/snippet}
                    </FormField>
                  </div>

                  <FormField id="connPassword" label={m.common_user_form_password()}>
                    {#snippet children()}
                      <Input id="connPassword" type="password" bind:value={connPassword} placeholder="••••••••" class="h-8 text-xs font-mono bg-background border-input" />
                    {/snippet}
                  </FormField>
                </div>
              {/if}

              <!-- Actions -->
              <div class="flex items-center justify-between pt-2 border-t border-border/40">
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  class="h-7.5 px-2.5 text-[10.5px] text-muted-foreground hover:text-destructive border-border"
                  onclick={() => controller.resetConnectionConfig()}
                  disabled={controller.isOperating}
                >
                  {m.database_action_reset_connection?.() || m.ui_copy_reset_default_a114b24e()}
                </Button>

                <Button
                  type="button"
                  variant="default"
                  size="sm"
                  class="h-7.5 px-4 text-[10.5px] font-bold gap-1.5 shadow-xs shadow-primary/20"
                  onclick={handleSaveConnection}
                  disabled={controller.isOperating}
                >
                  {#if controller.isOperating && controller.operationName === "save_connection"}
                    <LoadingIndicator   size={12} />
                    <span>{m.common_state_saving()}</span>
                  {:else}
                    <CheckCircle size={13} />
                    <span>{m.database_action_save_connection?.() || m.ui_copy_save_apply_ff1d269f()}</span>
                  {/if}
                </Button>
              </div>
            </div>
          {/if}

          <!-- ═══════════════════════════════════════════════════════════ -->
          <!-- SUBVIEW 3: BACKUP -->
          <!-- ═══════════════════════════════════════════════════════════ -->
          {#if activeSection === "backup"}
            <div class="space-y-4 text-start">
              <div class="flex items-center justify-between pb-3 border-b border-border/40">
                <h3 class="text-xs font-bold text-foreground">
                  {m.ui_copy_backup_execution_retention_policies_6d81af81()}
                </h3>
              </div>

              <!-- Instant Backup Card -->
              <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3">
                <div class="flex items-center gap-2">
                  <ClockRotateRight size={14} class="text-primary" />
                  <span class="text-xs font-bold text-foreground">{m.ui_copy_create_instant_backup_ab00a324()}</span>
                </div>

                <FormField id="customBackupPath" label={m.database_backup_destination_label?.() || m.ui_copy_backup_destination_path_f84c9ca4()}>
                  {#snippet children()}
                    <div class="flex gap-2">
                      <Input
                        id="customBackupPath"
                        type="text"
                        bind:value={customBackupPath}
                        placeholder={m.database_backup_custom_dest_hint?.() || m.ui_copy_leave_blank_for_default_backups_folder_aa0c2051()}
                        class="h-8 text-xs font-mono bg-background border-input flex-1"
                      />
                      <Button
                        type="button"
                        variant="outline"
                        size="sm"
                        class="h-8 px-2.5 text-[11px] gap-1.5 shrink-0 border-border"
                        onclick={handleBrowseBackupDest}
                      >
                        <FolderOpen size={13} class="text-primary" />
                        <span>{m.ui_copy_choose_location_a03e8da9()}</span>
                      </Button>
                    </div>
                  {/snippet}
                </FormField>

                <Button
                  type="button"
                  variant="default"
                  class="w-full h-8.5 text-xs font-bold gap-1.5 cursor-pointer shadow-xs shadow-primary/20"
                  onclick={handleTriggerBackup}
                  disabled={controller.isOperating}
                >
                  {#if controller.isOperating && controller.operationName === "backup"}
                    <LoadingIndicator   size={13} />
                    <span>{m.ui_copy_creating_verifying_backup_47acb457()}</span>
                  {:else}
                    <ArrowDownTray size={14} />
                    <span>{m.database_action_backup_now?.() || m.ui_copy_execute_backup_now_5faedbbd()}</span>
                  {/if}
                </Button>
              </div>

              <!-- Automatic Backup Policy -->
              <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3.5">
                <div class="flex items-center justify-between pb-2 border-b border-border/30">
                  <div class="flex items-center gap-2">
                    <ClockRotateRight size={14} class="text-indigo-500" />
                    <span class="text-xs font-bold text-foreground">{m.ui_copy_automated_backup_policy_ecc1317b()}</span>
                  </div>
                  <Switch bind:checked={autoBackup} />
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
                  <FormField id="backupInterval" label={m.database_backup_interval_label?.() || m.ui_copy_interval_hours_23ad5bfc()}>
                    {#snippet children()}
                      <Input id="backupInterval" type="number" bind:value={backupInterval} min="1" max="168" class="h-8 text-xs bg-background border-input" disabled={!autoBackup} />
                    {/snippet}
                  </FormField>

                  <FormField id="maxBackups" label={m.database_backup_max_files_label?.() || m.ui_copy_max_retained_files_02a1c239()}>
                    {#snippet children()}
                      <Input id="maxBackups" type="number" bind:value={maxBackups} min="1" max="100" class="h-8 text-xs bg-background border-input" disabled={!autoBackup} />
                    {/snippet}
                  </FormField>
                </div>

                <div class="pt-2 border-t border-border/30 flex items-center justify-between">
                  <div>
                    <div class="text-xs font-bold text-foreground">{m.ui_copy_integrity_check_on_startup_92991c81()}</div>
                    <p class="text-[9.5px] text-muted-foreground">{m.ui_copy_automatically_verify_database_integrity_wh_1270bffe()}</p>
                  </div>
                  <Switch bind:checked={integrityOnStartup} />
                </div>

                <div class="pt-2 border-t border-border/30 flex justify-end">
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="h-7.5 px-3 text-[10.5px] font-bold border-border"
                    onclick={handleSaveSettings}
                    disabled={controller.isOperating}
                  >
                    {m.ui_copy_save_policy_5fce5a59()}
                  </Button>
                </div>
              </div>
            </div>
          {/if}

          <!-- ═══════════════════════════════════════════════════════════ -->
          <!-- SUBVIEW 4: RESTORE -->
          <!-- ═══════════════════════════════════════════════════════════ -->
          {#if activeSection === "restore"}
            <div class="space-y-4 text-start">
              <div class="flex items-center justify-between pb-3 border-b border-border/40">
                <h3 class="text-xs font-bold text-foreground">
                  {m.ui_copy_restore_database_from_backup_9f021a21()}
                </h3>
              </div>

              <!-- Step 1: File Selection -->
              <div class="space-y-2">
                <p class="text-xs font-bold text-foreground">
                  {m.database_restore_step1_title?.() || m.database_restore_step1_title()}
                </p>
                <div class="flex gap-2">
                  <Input
                    type="text"
                    bind:value={restoreFilePath}
                    placeholder={m.ui_copy_select_backup_file_db_f97eaf03()}
                    class="h-8 text-xs font-mono bg-background border-input flex-1"
                    readonly
                  />
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="h-8 px-2.5 text-[11px] gap-1.5 shrink-0 border-border"
                    onclick={handleBrowseRestoreFile}
                    disabled={controller.isOperating}
                  >
                    <FolderOpen size={13} class="text-primary" />
                    <span>{m.database_browse_file?.() || m.ui_copy_browse_file_6c69bea3()}</span>
                  </Button>
                </div>
              </div>

              <!-- Step 2: Inspection Card -->
              {#if controller.isInspectingBackup}
                <div class="p-3.5 rounded-lg bg-card/40 border border-border/40 text-xs flex items-center gap-2.5 text-muted-foreground">
                  <LoadingIndicator class="text-primary"  size={13} />
                  <span>{m.ui_copy_inspecting_and_validating_backup_file_8f710a33()}</span>
                </div>
              {:else if controller.inspectedBackup}
                <div class="p-3.5 rounded-lg border text-xs space-y-2 {controller.inspectedBackup.is_valid ? 'bg-emerald-500/5 border-emerald-500/20' : 'bg-destructive/5 border-destructive/20'}">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 font-bold text-foreground">
                      {#if controller.inspectedBackup.is_valid}
                        <CheckCircle size={14} class="text-emerald-500" />
                        <span>{m.ui_copy_backup_validated_successfully_9d213b23()}</span>
                      {:else}
                        <InfoCircle size={14} class="text-destructive" />
                        <span>{m.ui_copy_invalid_or_corrupted_backup_file_8aafa626()}</span>
                      {/if}
                    </div>
                    <Badge variant="outline" class="text-[9px] font-mono">
                      {controller.inspectedBackup.file_size_formatted}
                    </Badge>
                  </div>

                  <div class="grid grid-cols-2 gap-3 text-[10.5px] text-muted-foreground">
                    <div>{m.ui_copy_schema_version_4bf60d19()} <span class="font-bold text-foreground font-mono">{controller.inspectedBackup.schema_version}</span></div>
                    <div>{m.ui_copy_applied_migrations_c2079a5a()} <span class="font-bold text-foreground">{controller.inspectedBackup.applied_migrations_count}</span></div>
                  </div>
                </div>
              {/if}

              <!-- Step 3: Safety Warning & Trigger -->
              <div class="p-3.5 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-700 dark:text-amber-400 text-xs space-y-1.5">
                <div class="flex items-center gap-2 font-bold">
                  <InfoCircle size={14} />
                  <span>{m.ui_copy_restoration_safety_notice_42d0e578()}</span>
                </div>
                <p class="text-[10px] leading-relaxed">
                  {m.database_restore_warning_desc?.() || m.ui_copy_warning_restoration_will_overwrite_current_786916c2()}
                </p>
              </div>

              <Button
                type="button"
                variant="destructive"
                class="w-full h-8.5 text-xs font-bold gap-1.5 cursor-pointer shadow-xs"
                onclick={handleExecuteRestore}
                disabled={!controller.inspectedBackup?.is_valid || controller.isOperating}
              >
                {#if controller.isOperating && controller.operationName === "restore"}
                  <LoadingIndicator   size={13} />
                  <span>{m.ui_copy_restoring_upgrading_30e27930()}</span>
                {:else}
                  <ArrowUpTray size={14} />
                  <span>{m.database_restore_btn?.() || m.ui_copy_confirm_execute_restore_8d40c357()}</span>
                {/if}
              </Button>
            </div>
          {/if}

          <!-- ═══════════════════════════════════════════════════════════ -->
          <!-- SUBVIEW 5: MAINTENANCE & MIGRATIONS -->
          <!-- ═══════════════════════════════════════════════════════════ -->
          {#if activeSection === "maintenance"}
            <div class="space-y-4 text-start">
              <div class="flex items-center justify-between pb-3 border-b border-border/40">
                <h3 class="text-xs font-bold text-foreground">
                  {m.ui_copy_database_maintenance_migrations_compaction_07f28bbc()}
                </h3>
                {#if controller.status && controller.status.pending_migrations.length === 0}
                  <Badge variant="outline" class="text-[9px] bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20">
                    {m.ui_copy_up_to_date_ccf363f4()}
                  </Badge>
                {:else}
                  <Badge variant="outline" class="text-[9px] bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20">
                    {controller.status?.pending_migrations.length} {m.common_calendar_status_pending()}
                  </Badge>
                {/if}
              </div>

              <!-- Migrations Status -->
              {#if controller.status && controller.status.pending_migrations.length > 0}
                <div class="p-3.5 rounded-lg bg-amber-500/10 border border-amber-500/20 text-xs space-y-2">
                  <span class="font-bold text-amber-700 dark:text-amber-400">{m.ui_copy_pending_migrations_00e88713()}</span>
                  <ul class="list-disc list-inside space-y-1 font-mono text-[10.5px] text-foreground">
                    {#each controller.status.pending_migrations as migration (migration)}
                      <li>{migration}</li>
                    {/each}
                  </ul>
                  <Button
                    type="button"
                    variant="default"
                    class="w-full h-8 text-xs font-bold gap-1.5 cursor-pointer shadow-xs mt-2"
                    onclick={() => controller.applyPendingMigrations()}
                    disabled={controller.isOperating}
                  >
                    <Bolt size={13} />
                    <span>{m.database_action_apply_migrations?.() || m.ui_copy_apply_migrations_now_5f07d44c()}</span>
                  </Button>
                </div>
              {:else}
                <div class="p-4 rounded-lg bg-card/40 border border-border/40 text-center py-4 space-y-1 text-xs text-muted-foreground">
                  <CheckCircle size={20} class="mx-auto text-emerald-500 mb-1" />
                  <div class="font-bold text-foreground">{m.ui_copy_database_schema_is_completely_up_to_date_ba874d2a()}</div>
                  <p class="text-[10px]">{m.ui_copy_all_migrations_are_applied_and_active_3057c243()}</p>
                </div>
              {/if}

              <!-- Integrity & Compaction Controls -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-2 border-t border-border/30">
                <div class="p-3.5 rounded-lg bg-card/40 border border-border/40 space-y-2">
                  <div class="font-bold text-xs text-foreground">{m.ui_copy_integrity_verification_ba15d509()}</div>
                  <p class="text-[10px] text-muted-foreground">{m.ui_copy_comprehensive_b_tree_check_for_corruption__9f64a6c1()}</p>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="w-full text-[11px] font-bold gap-1.5 border-border h-8"
                    onclick={() => controller.runIntegrityCheck()}
                    disabled={controller.isOperating}
                  >
                    <ShieldCheck size={13} class="text-emerald-500" />
                    <span>{m.ui_copy_check_integrity_bbcabc81()}</span>
                  </Button>
                </div>

                <div class="p-3.5 rounded-lg bg-card/40 border border-border/40 space-y-2">
                  <div class="font-bold text-xs text-foreground">{m.ui_copy_compact_optimize_63f0fa7f()}</div>
                  <p class="text-[10px] text-muted-foreground">{m.ui_copy_checkpoints_wal_and_reclaims_fragmented_st_31ef8a0a()}</p>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="w-full text-[11px] font-bold gap-1.5 border-border h-8"
                    onclick={() => controller.optimizeDatabase()}
                    disabled={controller.isOperating}
                  >
                    <Sparkles size={13} class="text-amber-500" />
                    <span>{m.ui_copy_compact_db_b5dcdd6c()}</span>
                  </Button>
                </div>
              </div>

              {#if controller.integrityResult}
                <div class="p-3 rounded-lg border text-xs space-y-1.5 {controller.integrityResult.is_healthy ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400' : 'bg-destructive/10 border-destructive/20 text-destructive'}">
                  <div class="flex items-center gap-2 font-bold">
                    <ShieldCheck size={14} />
                    <span>{controller.integrityResult.is_healthy ? m.ui_copy_integrity_result_100_healthy_41f833b9() : m.ui_copy_issues_detected_d91351f2()}</span>
                  </div>
                  <ul class="list-disc list-inside font-mono text-[10.5px]">
                    {#each controller.integrityResult.details as detail, index (index)}
                      <li>{detail}</li>
                    {/each}
                  </ul>
                </div>
              {/if}

              {#if controller.optimizeResult}
                <div class="p-3 rounded-lg bg-primary/10 border border-primary/20 text-xs space-y-1">
                  <div class="font-bold text-foreground">{controller.optimizeResult.message}</div>
                  <div class="text-[10px] text-muted-foreground">
                    {m.ui_copy_space_reclaimed_e53845f4()} <span class="font-bold text-primary font-mono">{controller.optimizeResult.space_freed_formatted}</span>
                  </div>
                </div>
              {/if}
            </div>
          {/if}

          {#if activeSection === "performance"}
            <section class="space-y-5 text-start">
              <div class="flex flex-col gap-3 border-b border-border/40 pb-4 sm:flex-row sm:items-center sm:justify-between">
                <h2 class="text-sm font-semibold text-foreground">
                  {m.database_performance_title()}
                </h2>
                <div class="flex flex-wrap items-center gap-2">
                  {#if isRecordingPerformance}
                    <Button
                      type="button"
                      variant="destructive"
                      size="sm"
                      class="h-8 gap-1.5 px-3 text-[10px]"
                      onclick={stopPerformanceRecording}
                    >
                      <span class="size-1.5 rounded-full bg-current"></span>
                      {m.database_performance_stop()}
                    </Button>
                  {:else}
                    <Button
                      type="button"
                      variant="default"
                      size="sm"
                      class="h-8 gap-1.5 px-3 text-[10px]"
                      onclick={startPerformanceRecording}
                    >
                      <ChartBar size={13} />
                      {m.database_performance_start()}
                    </Button>
                  {/if}
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="h-8 gap-1.5 px-2.5 text-[10px]"
                    onclick={refreshCommandPerformance}
                  >
                    <ArrowPath size={13} />
                    {m.common_refresh()}
                  </Button>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="h-8 px-2.5 text-[10px]"
                    onclick={clearCommandPerformance}
                    disabled={commandPerformance.length === 0}
                  >
                    {m.database_performance_reset()}
                  </Button>
                </div>
              </div>

              {#if isRecordingPerformance}
                <div class="flex items-center gap-2 rounded-lg border border-primary/20 bg-primary/5 px-3 py-2 text-[10px] text-primary" role="status">
                  <span class="relative flex size-2">
                    <span class="absolute inline-flex size-full animate-ping rounded-full bg-primary opacity-50"></span>
                    <span class="relative inline-flex size-2 rounded-full bg-primary"></span>
                  </span>
                  {m.database_performance_recording()}
                </div>
              {/if}

              <div class="grid grid-cols-2 gap-3 xl:grid-cols-4">
                {#each [
                  { label: m.database_performance_commands(), value: performanceSummary.commands },
                  { label: m.database_performance_calls_total(), value: performanceSummary.calls },
                  {
                    label: m.database_performance_p95(),
                    value:
                      performanceSummary.p95 === null
                        ? "—"
                        : formatDuration(performanceSummary.p95),
                  },
                  { label: m.database_performance_failures(), value: performanceSummary.failures },
                ] as metric (metric.label)}
                  <div class="rounded-lg border border-border/50 bg-background/60 p-3">
                    <p class="text-[10px] text-muted-foreground">{metric.label}</p>
                    <p class="mt-1.5 font-mono text-lg font-semibold text-foreground">{metric.value}</p>
                  </div>
                {/each}
              </div>

              <div class="rounded-lg border border-border/50 bg-background/40 p-4">
                <div class="mb-4 flex items-center justify-between gap-3">
                  <h3 class="text-xs font-semibold text-foreground">{m.database_performance_slowest_commands()}</h3>
                  <span class="text-[9px] text-muted-foreground">{m.database_performance_p95()}</span>
                </div>
                {#if commandPerformance.length === 0}
                  <div class="rounded-md border border-dashed border-border/60 px-4 py-8 text-center text-xs text-muted-foreground">
                    {m.database_performance_empty()}
                  </div>
                {:else}
                  <div class="space-y-3">
                    {#each commandPerformance.slice(0, 8) as metric (metric.command)}
                      <div class="grid grid-cols-[minmax(7rem,1fr)_minmax(5rem,2fr)_4.5rem] items-center gap-3">
                        <span class="truncate font-mono text-[10px] text-muted-foreground" title={metric.command}>{metric.command}</span>
                        <div class="h-2 overflow-hidden rounded-full bg-muted">
                          <div
                            class="h-full rounded-full {metric.p95Ms >= 500 ? 'bg-amber-500' : 'bg-primary'}"
                            style:width={commandBarWidth(metric.p95Ms)}
                          ></div>
                        </div>
                        <span class="text-end font-mono text-[10px] {metric.p95Ms >= 500 ? 'text-amber-600 dark:text-amber-400' : 'text-foreground'}">{formatDuration(metric.p95Ms)}</span>
                      </div>
                    {/each}
                  </div>
                {/if}
              </div>

              {#if commandPerformance.length > 0}
                <div class="overflow-x-auto rounded-lg border border-border/50">
                  <table class="w-full min-w-[34rem] text-start text-[10px]">
                    <thead class="bg-muted/50 text-muted-foreground">
                      <tr>
                        <th class="px-3 py-2.5 font-medium">{m.database_performance_command()}</th>
                        <th class="px-3 py-2.5 text-end font-medium">{m.database_performance_calls()}</th>
                        <th class="px-3 py-2.5 text-end font-medium">{m.database_performance_average()}</th>
                        <th class="px-3 py-2.5 text-end font-medium">{m.database_performance_p95()}</th>
                        <th class="px-3 py-2.5 text-end font-medium">{m.database_performance_failures()}</th>
                      </tr>
                    </thead>
                    <tbody>
                      {#each commandPerformance as metric (metric.command)}
                        <tr class="border-t border-border/40">
                          <td class="px-3 py-2.5 font-mono text-foreground">{metric.command}</td>
                          <td class="px-3 py-2.5 text-end font-mono text-muted-foreground">
                            {metric.totalCalls} <span class="text-muted-foreground/60">({metric.sampleCount})</span>
                          </td>
                          <td class="px-3 py-2.5 text-end font-mono text-muted-foreground">{formatDuration(metric.averageMs)}</td>
                          <td class="px-3 py-2.5 text-end font-mono {metric.p95Ms >= 500 ? 'text-amber-600 dark:text-amber-400' : 'text-muted-foreground'}">{formatDuration(metric.p95Ms)}</td>
                          <td class="px-3 py-2.5 text-end font-mono {metric.failedCalls > 0 ? 'text-destructive' : 'text-muted-foreground'}">{metric.failedCalls}</td>
                        </tr>
                      {/each}
                    </tbody>
                  </table>
                </div>
              {/if}

              <p class="text-[10px] leading-relaxed text-muted-foreground">
                <span>{m.database_performance_privacy_note()}</span>
                <span> {m.database_performance_window_note()}</span>
              </p>
            </section>
          {/if}

        </section>
        </main>
      </div>
    {/if}
  </div>
</div>

<!-- Database Creation Dialog -->
<CreateDatabaseDialog
  bind:open={controller.isCreateDialogOpen}
  {controller}
/>
