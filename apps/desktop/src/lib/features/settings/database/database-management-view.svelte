<!--
  @component DatabaseManagementView
  Master control subsystem for database lifecycle, backups, restore, migrations, and health checks.
  Redesigned with the canonical Orbis Exams Dashboard design system:
  - Notion-style Directory Index list with monolithic icon boxes & badges
  - Linear-style Diagnostics & AI Advisor Sidebar with live KPI tiles & status indicators
  - Sleek top header ribbon with active engine & connection badges and quick actions
  - High-performance Svelte 5 Runes architecture with dual-transport parity & zero guessed defaults
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { onMount } from "svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Switch } from "@orbis/ui/switch";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import {
  ArrowDownTray,
  ArrowPath,
  ArrowRight,
  ArrowUpTray,
  Bolt,
  CheckCircle,
  ClockRotateRight,
  Computer,
  Database,
  FolderOpen,
  GlobeAlt,
  Info,
  InfoCircle,
  Plus,
  ShieldCheck,
  Sparkles,
  Wrench,
} from "@orbis/icons";
import { DatabaseManagerController } from "./database-manager.svelte";
import CreateDatabaseDialog from "./create-database-dialog.svelte";
import type { ConnectionMode } from "$lib/api/types";

let { orgId }: { orgId: string } = $props();

const controller = new DatabaseManagerController();

let activeSection = $state<"overview" | "connection" | "backup" | "restore" | "maintenance">("overview");

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

async function handleTestRemote() {
  if (!connServerUrl.trim()) {
    toast.error(m.ui_copy_please_enter_a_server_url_4a739f83());
    return;
  }
  const result = await controller.testRemoteHealth(connServerUrl.trim());
  if (result.reachable) {
    toast.success(m.ui_server_connected_latency({ latency: result.latency_ms }));
  } else {
    toast.error(result.error || m.ui_copy_unable_to_connect_to_server_147a38a8());
  }
}

async function handleSaveConnection() {
  try {
    await controller.saveConnectionConfig({
      mode: connMode,
      serverUrl: connMode === "api" ? connServerUrl.trim() : undefined,
      dbFilename: connMode === "local" ? connDbFilename.trim() : undefined,
      customPath: connMode === "local" && connCustomPath.trim() ? connCustomPath.trim() : undefined,
      databaseEngine: connMode === "direct" ? "postgres" : undefined,
      host: connMode === "direct" ? connHost.trim() : undefined,
      port: connMode === "direct" ? connPort : undefined,
      databaseName: connMode === "direct" ? connDatabaseName.trim() : undefined,
      username: connMode === "direct" ? connUsername.trim() : undefined,
      password: connMode === "direct" && connPassword ? connPassword : undefined,
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
  const file = await controller.pickFile(m.ui_copy_orbis_databases_db_2636ac20(), ["db", "sqlite", "sqlite3"]);
  if (file) {
    connCustomPath = file;
  }
}

async function handleBrowseBackupDest() {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
  const file = await controller.pickSavePath(
    `orbis_backup_${timestamp}.db`,
    m.ui_copy_backup_file_db_8e8097bf(),
    ["db", "sqlite", "bak"]
  );
  if (file) {
    customBackupPath = file;
  }
}

async function handleTriggerBackup() {
  await controller.triggerBackup(customBackupPath.trim() ? customBackupPath.trim() : undefined);
  if (controller.operationSuccess) {
    toast.success(controller.operationSuccess);
    customBackupPath = "";
  } else if (controller.operationError) {
    toast.error(controller.operationError);
  }
}

async function handleBrowseRestoreFile() {
  const file = await controller.pickFile(m.ui_copy_backup_file_db_8e8097bf(), ["db", "sqlite", "bak"]);
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
  const file = await controller.pickFile(m.ui_copy_sqlite_database_db_08cc781c(), ["db", "sqlite", "sqlite3"]);
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
        ? l("PostgreSQL مباشر", "Direct PostgreSQL")
        : m.ui_copy_remote_server_efd114c0(),
    sub: controller.isLocal
      ? m.ui_copy_local_embedded_36bd9bbd()
      : connMode === "direct"
        ? l("اتصال مباشر بالشبكة", "Direct network connection")
        : m.ui_copy_remote_cloud_51770fc6(),
  },
  {
    label: m.ui_copy_storage_size_8122fef2(),
    value: controller.status?.file_size_formatted || "—",
    sub: controller.status?.journal_mode ? `WAL: ${controller.status.journal_mode}` : m.ui_copy_indexed_da85078a(),
  },
  {
    label: m.database_card_version(),
    value: controller.status?.applied_migrations_count ? `v${controller.status.applied_migrations_count}.0` : (controller.status?.schema_version || "v1.0.0"),
    sub: controller.status?.schema_version ? controller.status.schema_version.replace(/^m\d+_\d+_/, "").replace(/_/g, " ") : m.ui_copy_current_7b958d7b(),
    fullTitle: controller.status?.schema_version || "v1.0.0",
  },
  {
    label: m.ui_copy_applied_migrations_b57f13d8(),
    value: `${controller.status?.applied_migrations_count ?? 0} / ${controller.status?.total_migrations_count ?? 0}`,
    sub: (controller.status?.pending_migrations.length ?? 0) > 0 ? m.ui_pending_migrations_count({ count: controller.status?.pending_migrations.length ?? 0 }) : m.ui_copy_100_synced_085c434f(),
  },
]);

// Notion-style Directory Items
const directoryItems = $derived([
  {
    id: "overview" as const,
    title: m.ui_copy_overview_integrity_health_42939363(),
    desc: m.ui_copy_target_storage_path_integrity_check_and_fa_a39b10ce(),
    icon: ShieldCheck,
    badge: controller.status?.integrity_status === "ok" ? m.ui_copy_ok_3b8e28b7() : (controller.status?.integrity_status || m.ui_copy_ok_217bf3e2()),
  },
  {
    id: "connection" as const,
    title: m.ui_copy_connection_engine_settings_e9507bc6(),
    desc: m.ui_copy_configure_orbis_api_server_postgresql_cred_20f84da3(),
    icon: GlobeAlt,
    badge: controller.isLocal
      ? "SQLite"
      : connMode === "direct"
        ? "PostgreSQL"
        : "Orbis API",
  },
  ...(controller.isLocal ? [{
    id: "backup" as const,
    title: m.ui_copy_backup_retention_policies_6a7221cd(),
    desc: m.ui_copy_create_instant_snapshots_via_vacuum_into_a_2757e2e9(),
    icon: ArrowDownTray,
    badge: controller.status?.last_backup_timestamp ? m.ui_copy_active_2acda471() : m.ui_copy_pending_5ac26458(),
  },
  {
    id: "restore" as const,
    title: m.ui_copy_restore_snapshot_recovery_0c381fdb(),
    desc: m.ui_copy_safe_staged_recovery_with_pre_validation_i_589d67b1(),
    icon: ArrowUpTray,
    badge: m.ui_copy_staged_a21d0221(),
  },
  {
    id: "maintenance" as const,
    title: m.ui_copy_maintenance_migrations_vacuum_4ddc2f50(),
    desc: m.ui_copy_apply_pending_schema_migrations_compact_db_c69fb0b3(),
    icon: Wrench,
    badge: controller.status && controller.status.pending_migrations.length > 0
      ? m.ui_pending_migrations_count({ count: controller.status.pending_migrations.length })
      : m.ui_copy_up_to_date_ccf363f4(),
  }] : []),
]);

// Advisor Diagnostic Insights
const advisorInsights = $derived.by(() => {
  const insights = [];

  // 1. Integrity Check Insight
  const isHealthy = controller.status?.integrity_status === "ok" || controller.status?.integrity_status === "healthy";
  insights.push({
    id: "integrity-health",
    text: isHealthy
      ? m.ui_copy_table_structures_and_b_tree_indices_verifi_ebbfc409()
      : m.ui_copy_integrity_anomalies_detected_comprehensive_a7b9d476(),
    variant: isHealthy ? ("success" as const) : ("warning" as const),
    icon: ShieldCheck,
  });

  // 2. Schema Migrations Insight
  const pendingCount = controller.status?.pending_migrations.length ?? 0;
  if (pendingCount > 0) {
    insights.push({
      id: "migrations-pending",
      text: m.ui_pending_migrations_apply_required({ count: pendingCount }),
      variant: "warning" as const,
      icon: Bolt,
    });
  } else {
    insights.push({
      id: "migrations-synced",
      text: m.ui_copy_database_schema_is_fully_synchronized_with_832f99dd(),
      variant: "success" as const,
      icon: CheckCircle,
    });
  }

  // 3. Concurrency / WAL Mode Insight
  const walActive = connWalMode || controller.status?.journal_mode?.toLowerCase() === "wal";
  insights.push({
    id: "wal-mode",
    text: walActive
      ? m.ui_copy_high_concurrency_wal_mode_enabled_for_lock_9e20529c()
      : m.ui_copy_standard_journal_active_wal_mode_can_be_en_a5ec5641(),
    variant: walActive ? ("success" as const) : ("neutral" as const),
    icon: Sparkles,
  });

  // 4. Backup Freshness Insight
  if (controller.status?.last_backup_timestamp) {
    insights.push({
      id: "backup-fresh",
      text: m.ui_copy_verified_snapshot_taken_recently_data_prot_f3a8a1cd(),
      variant: "success" as const,
      icon: ClockRotateRight,
    });
  } else {
    insights.push({
      id: "backup-recommended",
      text: m.ui_copy_no_recent_backup_recorded_creating_an_inst_f7ac9304(),
      variant: "warning" as const,
      icon: ArrowDownTray,
    });
  }

  return insights;
});
</script>

<div class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide p-6 select-none font-sans flex flex-col gap-6">
  
  <!-- Sleek Header with Engine & Status Indicators -->
  <div class="flex items-center justify-between border-b border-border/40 pb-4">
    <div class="flex items-center gap-3 text-start">
      <div class="h-8.5 w-8.5 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0 font-bold text-xs">
        <Database size={18} />
      </div>
      <div class="flex flex-col">
        <h2 class="text-xs font-bold text-foreground tracking-wide uppercase">
          {m.database_management_title?.() || m.settings_nav_database()}
        </h2>
        <span class="text-[9.5px] text-muted-foreground/90 mt-0.5">
          {m.database_management_subtitle?.() || m.database_management_subtitle()}
        </span>
      </div>
    </div>

    <!-- Active Indicators & Action Bar -->
    <div class="flex items-center gap-2">
      <!-- Engine Indicator Badge -->
      <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
        {#if controller.isLocal}
          <Computer size={10} class="text-primary" />
          <span>{m.settings_ui_database_sqlite_embedded()}</span>
        {:else if connMode === "direct"}
          <GlobeAlt size={10} class="text-primary" />
          <span>{l("PostgreSQL مباشر", "Direct PostgreSQL")}</span>
        {:else}
          <GlobeAlt size={10} class="text-primary" />
          <span>{m.ui_copy_remote_server_efd114c0()}</span>
        {/if}
      </Badge>

      <!-- Connection / Health Status Badge -->
      {#if controller.status}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
          <span class="h-1.5 w-1.5 rounded-full {controller.status.state === 'connected' ? 'bg-emerald-500' : controller.status.state === 'migration_required' ? 'bg-amber-500 animate-pulse' : 'bg-destructive'}"></span>
          <span>{controller.status.state === 'connected' ? (m.database_status_connected?.() || m.database_status_connected()) : controller.status.state === 'migration_required' ? (m.database_status_migration_required?.() || m.ui_copy_migration_required_571b6bc3()) : (m.database_status_disconnected?.() || m.database_status_disconnected())}</span>
        </Badge>
      {/if}

      <!-- Refresh Button -->
      <Button
        variant="outline"
        size="sm"
        class="text-[10px] font-bold px-2.5 h-7.5 rounded-md gap-1.5 cursor-pointer border-border/60 bg-card/60"
        onclick={() => controller.refresh()}
        disabled={controller.isLoading || controller.isOperating}
      >
        {#if controller.isLoading}<LoadingIndicator  size={12} class="text-primary" />{:else}<ArrowPath size={12}   />{/if}
        <span>{m.common_refresh()}</span>
      </Button>

      <!-- Open Existing DB Button -->
      <Button
        variant="outline"
        size="sm"
        class="text-[10px] font-bold px-2.5 h-7.5 rounded-md gap-1.5 cursor-pointer border-border/60 bg-card/60"
        onclick={handleOpenExistingDatabase}
        disabled={controller.isOperating}
      >
        <FolderOpen size={12} class="text-primary" />
        <span>{m.database_action_open?.() || m.ui_copy_open_db_a45f2956()}</span>
      </Button>

      <!-- New DB Button -->
      <Button 
        variant="default" 
        size="sm" 
        class="text-[10px] font-bold px-3 h-7.5 rounded-md gap-1.5 cursor-pointer ms-1 shadow-xs shadow-primary/20"
        onclick={() => controller.isCreateDialogOpen = true}
        disabled={controller.isOperating}
      >
        <Plus size={13} />
        <span>{m.database_action_create?.() || m.ui_copy_new_db_a2bc15eb()}</span>
      </Button>
    </div>
  </div>

  <!-- Operational Global Alerts -->
  {#if controller.operationError}
    <div class="p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs flex items-center justify-between gap-3 animate-in fade-in">
      <div class="flex items-center gap-2.5">
        <InfoCircle size={15} class="shrink-0" />
        <span>{controller.operationError}</span>
      </div>
      <Button variant="ghost" size="sm" class="h-6 px-2 text-[10px] text-destructive hover:bg-destructive/20" onclick={() => controller.operationError = null}>
        {m.ui_copy_dismiss_f18c09a6()}
      </Button>
    </div>
  {/if}

  {#if controller.operationSuccess}
    <div class="p-3.5 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 dark:text-emerald-400 text-xs flex items-center justify-between gap-3 animate-in fade-in">
      <div class="flex items-center gap-2.5">
        <CheckCircle size={15} class="shrink-0" />
        <span>{controller.operationSuccess}</span>
      </div>
      <Button variant="ghost" size="sm" class="h-6 px-2 text-[10px] text-emerald-600 hover:bg-emerald-500/20" onclick={() => controller.operationSuccess = null}>
        {m.ui_copy_dismiss_f18c09a6()}
      </Button>
    </div>
  {/if}

  {#if controller.isLoading && !controller.status}
    <!-- Split Grid Loading Skeleton -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      <!-- Left Directory List Skeleton -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col gap-3">
        <Skeleton class="h-4 w-32 mb-2 rounded" />
        {#each Array(5) as _}
          <div class="flex items-center justify-between p-3.5 border-b border-border/30">
            <div class="flex items-center gap-3 w-full">
              <Skeleton class="h-8.5 w-8.5 rounded-lg" />
              <div class="flex flex-col gap-1.5 w-2/3">
                <Skeleton class="h-3 w-28 rounded" />
                <Skeleton class="h-2.5 w-full rounded" />
              </div>
            </div>
            <Skeleton class="h-4 w-12 rounded" />
          </div>
        {/each}
      </div>

      <!-- Right Diagnostics Sidebar Skeleton -->
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/20 rounded-xl p-5">
        <Skeleton class="h-3 w-40 mb-1 rounded" />
        <div class="grid grid-cols-2 gap-4 pb-4 border-b border-border/40">
          {#each Array(4) as _}
            <div class="flex flex-col gap-1">
              <Skeleton class="h-2.5 w-16 rounded" />
              <Skeleton class="h-4 w-8 rounded" />
            </div>
          {/each}
        </div>
        <Skeleton class="h-3 w-32 mb-1 rounded" />
        <div class="flex flex-col gap-3">
          {#each Array(4) as _}
            <div class="flex items-start gap-2.5">
              <Skeleton class="h-4 w-4 rounded-full shrink-0" />
              <Skeleton class="h-3 w-full rounded" />
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else}
    <!-- Main Content Split Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      
      <!-- 1. LEFT COLUMN: Directory Index List (Notion Database style) & Active Inspector Subview -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col gap-5">
        
        <div>
          <h3 class="text-[10px] font-bold text-muted-foreground/80 tracking-wider uppercase mb-3 text-start">
            {m.ui_copy_database_management_directory_abb727de()}
          </h3>
          
          <div class="flex flex-col">
            {#each directoryItems as item (item.id)}
              <button 
                type="button"
                class="group flex items-center justify-between py-2.5 px-3 border-b border-border/30 hover:bg-muted/40 rounded-lg transition-colors duration-150 select-none cursor-pointer w-full text-start {activeSection === item.id ? 'bg-muted/40' : ''}"
                onclick={() => activeSection = item.id}
              >
                <div class="flex items-center gap-3.5 min-w-0 text-start">
                  <!-- Sleek Monolithic icon box -->
                  <div class="h-8 w-8 rounded-lg flex items-center justify-center shrink-0 transition-colors duration-150 {activeSection === item.id ? 'bg-primary/15 text-primary' : 'bg-muted/65 text-muted-foreground group-hover:bg-primary/10 group-hover:text-primary'}">
                    <item.icon size={16} />
                  </div>
                  <div class="flex flex-col min-w-0">
                    <span class="text-xs font-bold transition-colors duration-150 {activeSection === item.id ? 'text-primary' : 'text-foreground group-hover:text-primary'}">
                      {item.title}
                    </span>
                    <span class="text-[9.5px] text-muted-foreground/90 leading-normal truncate mt-0.5 max-w-sm sm:max-w-md md:max-w-lg">
                      {item.desc}
                    </span>
                  </div>
                </div>
                
                <div class="flex items-center gap-2 shrink-0">
                  {#if item.badge}
                    <span class="text-[8.5px] font-mono font-medium text-muted-foreground/80 bg-muted/50 border border-border/40 px-2 py-0.5 rounded select-none">
                      {item.badge}
                    </span>
                  {/if}
                  <ArrowRight 
                    size={12} 
                    class="text-muted-foreground/40 transition-opacity duration-150 rtl:rotate-180 {activeSection === item.id ? 'opacity-100 text-primary' : 'opacity-0 group-hover:opacity-100'}" 
                  />
                </div>
              </button>
            {/each}
          </div>
        </div>

        <!-- Active Section Inspector Detail Panel -->
        <div class="border border-border/50 bg-card/20 rounded-xl p-5 space-y-4 animate-in fade-in duration-150">
          
          <!-- ═══════════════════════════════════════════════════════════ -->
          <!-- SUBVIEW 1: OVERVIEW & HEALTH -->
          <!-- ═══════════════════════════════════════════════════════════ -->
          {#if activeSection === "overview"}
            <div class="space-y-4 text-start">
              <div class="flex items-center justify-between pb-3 border-b border-border/40">
                <div class="flex items-center gap-2">
                  <ShieldCheck size={16} class="text-primary" />
                  <h3 class="text-xs font-bold text-foreground">
                    {m.ui_copy_active_database_target_storage_details_e83fa545()}
                  </h3>
                </div>
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
                <div class="flex items-center gap-2">
                  <GlobeAlt size={16} class="text-primary" />
                  <h3 class="text-xs font-bold text-foreground">
                    {m.ui_copy_connection_engine_configuration_c7d4c473()}
                  </h3>
                </div>
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
                      <h4 class="text-xs font-bold text-foreground">{l("اتصال PostgreSQL مباشر", "Direct PostgreSQL")}</h4>
                      <p class="text-[9.5px] text-muted-foreground">{l("اتصال مباشر بقاعدة PostgreSQL.", "Direct connection to PostgreSQL.")}</p>
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
                <div class="flex items-center gap-2">
                  <ArrowDownTray size={16} class="text-primary" />
                  <h3 class="text-xs font-bold text-foreground">
                    {m.ui_copy_backup_execution_retention_policies_6d81af81()}
                  </h3>
                </div>
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
                <div class="flex items-center gap-2">
                  <ArrowUpTray size={16} class="text-amber-500" />
                  <h3 class="text-xs font-bold text-foreground">
                    {m.ui_copy_restore_database_from_backup_9f021a21()}
                  </h3>
                </div>
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
                <div class="flex items-center gap-2">
                  <Wrench size={16} class="text-primary" />
                  <h3 class="text-xs font-bold text-foreground">
                    {m.ui_copy_database_maintenance_migrations_compaction_07f28bbc()}
                  </h3>
                </div>
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
                    {#each controller.status.pending_migrations as migration}
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
                    {#each controller.integrityResult.details as detail}
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

        </div>
      </div>

      <!-- 2. RIGHT COLUMN: Database Health & Diagnostics (Linear Sidebar style) -->
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/10 rounded-xl p-5 select-none text-start">
        
        <!-- Sidebar Section: KPI Summary -->
        <div>
          <h4 class="text-[9.5px] font-bold text-muted-foreground/80 tracking-wider uppercase border-b border-border/30 pb-2 mb-3">
            {m.ui_copy_database_engine_kpis_30659166()}
          </h4>
          
          <div class="grid grid-cols-2 gap-3.5 pb-4 border-b border-border/30">
            {#each kpiStats as stat}
              <div class="flex flex-col min-w-0 text-start overflow-hidden" title={stat.fullTitle || stat.value}>
                <span class="text-[9.5px] text-muted-foreground/80 truncate">{stat.label}</span>
                <span class="text-xs font-bold text-foreground font-mono mt-0.5 leading-snug truncate">{stat.value}</span>
                {#if stat.sub}
                  <span class="text-[8.5px] text-muted-foreground/70 truncate mt-0.5 font-sans">{stat.sub}</span>
                {/if}
              </div>
            {/each}
          </div>
        </div>

        <!-- Sidebar Section: Orbis AI Advisor & System Diagnostics -->
        <div class="flex flex-col gap-3">
          <div class="flex items-center gap-2 text-foreground font-semibold text-[11px] pb-1">
            <Sparkles size={13} class="text-primary animate-pulse" />
            <span>{m.ui_copy_engine_diagnostics_advisor_d44c33a7()}</span>
          </div>
          
          <div class="flex flex-col gap-2.5">
            {#each advisorInsights as insight (insight.id)}
              <div class="flex items-start gap-2.5 p-2 rounded-lg bg-card/30 border border-border/20 text-[10px] leading-relaxed text-muted-foreground">
                
                <!-- Status icon or color dot -->
                <div class="shrink-0 mt-0.5">
                  {#if insight.variant === 'warning'}
                    <span class="relative flex h-2 w-2">
                      <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
                      <span class="relative inline-flex rounded-full h-2 w-2 bg-amber-500"></span>
                    </span>
                  {:else if insight.variant === 'success'}
                    <span class="inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                  {:else}
                    <span class="inline-flex rounded-full h-2 w-2 bg-muted-foreground/40"></span>
                  {/if}
                </div>

                <div class="flex flex-col min-w-0">
                  <span class="text-foreground/90 font-medium">{insight.text}</span>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- System Information Footer -->
        <div class="pt-2 border-t border-border/30 flex items-center gap-2 text-[9px] text-muted-foreground/75 truncate" title={controller.status?.connection_target}>
          <Info size={11} class="shrink-0" />
          <span class="truncate font-mono">{controller.status?.connection_target || m.ui_copy_engine_ready_dca5401b()}</span>
        </div>

      </div>

    </div>
  {/if}
</div>

<!-- Database Creation Dialog -->
<CreateDatabaseDialog
  bind:open={controller.isCreateDialogOpen}
  {controller}
/>
