<!--
  @component DatabaseManagementView
  Master control subsystem for database lifecycle, backups, restore, migrations, and health checks.
  Built for Orbis ERP Settings with Svelte 5 Runes, complete type-safety, and native OS capabilities.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { onMount } from "svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Switch } from "@orbis/ui/switch";
import { toast } from "@orbis/ui";
import {
  ArchiveBox,
  ArrowDownTray,
  ArrowPath,
  ArrowUpTray,
  Bolt,
  CheckCircle,
  ClockRotateRight,
  Computer,
  Database,
  DatabaseSettings,
  FolderOpen,
  GlobeAlt,
  InfoCircle,
  Plus,
  Scale,
  ShieldCheck,
  Sparkles,
  Wrench,
} from "@orbis/icons";
import { DatabaseManagerController } from "./database-manager.svelte";
import CreateDatabaseDialog from "./create-database-dialog.svelte";
import type { ConnectionMode } from "$lib/api/types";

let { orgId }: { orgId: string } = $props();

const controller = new DatabaseManagerController();
const isRTL = $derived(getLocale().startsWith("ar"));

let activeTab = $state<"overview" | "connection" | "backup" | "restore" | "maintenance">("overview");

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
    if (controller.config.remote) {
      connServerUrl = controller.config.remote.server_url || "";
      connHost = controller.config.remote.host || "localhost";
      connPort = controller.config.remote.port || 5432;
      connDatabaseName = controller.config.remote.database_name || "orbis_db";
      connUsername = controller.config.remote.username || "postgres";
      connPassword = controller.config.remote.password || "";
      connSslMode = controller.config.remote.ssl_mode || "prefer";
    }
  }

  if (controller.settings) {
    autoBackup = controller.settings.auto_backup_enabled;
    backupInterval = controller.settings.auto_backup_interval_hours;
    maxBackups = controller.settings.max_backup_files;
    customBackupDir = controller.settings.backup_dir || "";
  }
}

async function handleTestRemote() {
  if (!connServerUrl.trim()) {
    toast.error(l("يرجى إدخال عنوان الخادم", "Please enter a server URL"));
    return;
  }
  const result = await controller.testRemoteHealth(connServerUrl.trim());
  if (result.reachable) {
    toast.success(l(`الخادم متصل بنجاح (${result.latency_ms}ms)`, `Server connected successfully (${result.latency_ms}ms)`));
  } else {
    toast.error(result.error || l("تعذر الاتصال بالخادم", "Unable to connect to server"));
  }
}

async function handleSaveConnection() {
  try {
    await controller.saveConnectionConfig({
      mode: connMode,
      serverUrl: connMode === "remote" ? connServerUrl.trim() : undefined,
      dbFilename: connMode === "local" ? connDbFilename.trim() : undefined,
      customPath: connMode === "local" && connCustomPath.trim() ? connCustomPath.trim() : undefined,
      host: connMode === "remote" ? connHost.trim() : undefined,
      port: connMode === "remote" ? connPort : undefined,
      databaseName: connMode === "remote" ? connDatabaseName.trim() : undefined,
      username: connMode === "remote" ? connUsername.trim() : undefined,
      password: connMode === "remote" && connPassword ? connPassword : undefined,
      sslMode: connMode === "remote" ? connSslMode : undefined,
      walMode: connMode === "local" ? connWalMode : undefined,
      busyTimeoutMs: connMode === "local" ? connBusyTimeout : undefined,
    });
    toast.success(l("تم حفظ إعدادات الاتصال بنجاح", "Connection settings saved successfully"));
  } catch (e) {
    toast.error(e instanceof Error ? e.message : String(e));
  }
}

async function handleBrowseCustomDbPath() {
  const file = await controller.pickFile(l("قواعد بيانات أوربس (*.db)", "Orbis Databases (*.db)"), ["db", "sqlite", "sqlite3"]);
  if (file) {
    connCustomPath = file;
  }
}

async function handleBrowseBackupDest() {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
  const savePath = await controller.pickSavePath(`orbis_backup_${timestamp}.db`, l("نسخة احتياطية (*.db, *.bak)", "Backup (*.db, *.bak)"), ["db", "sqlite", "bak"]);
  if (savePath) {
    customBackupPath = savePath;
  }
}

async function handleTriggerBackup() {
  await controller.triggerBackup(customBackupPath.trim() || undefined);
  if (controller.operationSuccess) {
    toast.success(controller.operationSuccess);
    customBackupPath = "";
  } else if (controller.operationError) {
    toast.error(controller.operationError);
  }
}

async function handleBrowseRestoreFile() {
  const file = await controller.pickFile(l("ملفات النسخ الاحتياطي (*.db, *.sqlite, *.bak)", "Backup Files (*.db, *.sqlite, *.bak)"), ["db", "sqlite", "sqlite3", "bak"]);
  if (file) {
    restoreFilePath = file;
    await controller.inspectBackupFile(file);
  }
}

async function handleExecuteRestore() {
  if (!restoreFilePath) return;
  await controller.restoreFromBackup(restoreFilePath);
  if (controller.operationSuccess) {
    toast.success(controller.operationSuccess);
    restoreFilePath = "";
  } else if (controller.operationError) {
    toast.error(controller.operationError);
  }
}

async function handleSaveSettings() {
  await controller.saveSettings({
    auto_backup_enabled: autoBackup,
    auto_backup_interval_hours: backupInterval,
    max_backup_files: maxBackups,
    backup_dir: customBackupDir.trim() || undefined,
    integrity_check_on_startup: true,
  });
  if (controller.operationSuccess) {
    toast.success(controller.operationSuccess);
  }
}

async function handleOpenExistingDatabase() {
  const file = await controller.pickFile(l("قاعدة بيانات SQLite (*.db)", "SQLite Database (*.db)"), ["db", "sqlite", "sqlite3"]);
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
</script>

<div class="h-full flex flex-col overflow-hidden bg-background">
  <!-- Top Header Ribbon -->
  <div class="px-6 py-4 border-b border-border/60 bg-card/60 backdrop-blur-md flex flex-wrap items-center justify-between gap-4 shrink-0">
    <div class="flex items-center gap-3">
      <div class="size-10 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shadow-sm shadow-primary/10">
        <Database size={22} />
      </div>
      <div class="text-start">
        <div class="flex items-center gap-2">
          <h1 class="text-base sm:text-lg font-bold text-foreground leading-tight">
            {m.database_management_title?.() || l("إدارة قواعد البيانات", "Database Management")}
          </h1>
          {#if controller.status}
            {#if controller.status.state === "connected"}
              <Badge variant="outline" class="text-[10px] px-2 py-0.5 gap-1.5 font-medium border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400">
                <span class="size-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                <span>{m.database_status_connected?.() || l("متصل ومستقر", "Connected & Healthy")}</span>
              </Badge>
            {:else if controller.status.state === "migration_required"}
              <Badge variant="outline" class="text-[10px] px-2 py-0.5 gap-1.5 font-medium border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400">
                <Bolt size={10} class="text-amber-500" />
                <span>{m.database_status_migration_required?.() || l("تحديث مطلوب", "Migration Required")}</span>
              </Badge>
            {:else}
              <Badge variant="outline" class="text-[10px] px-2 py-0.5 gap-1.5 font-medium border-destructive/30 bg-destructive/10 text-destructive">
                <InfoCircle size={10} />
                <span>{m.database_status_disconnected?.() || l("غير متصل", "Disconnected")}</span>
              </Badge>
            {/if}
          {/if}
        </div>
        <p class="text-xs text-muted-foreground mt-0.5 font-medium">
          {m.database_management_subtitle?.() || l("إدارة دورة حياة قاعدة البيانات، النسخ الاحتياطي، الاستعادة، والترحيلات", "Manage database lifecycle, backups, recovery, integrity, and migrations")}
        </p>
      </div>
    </div>

    <!-- Quick Action Bar -->
    <div class="flex items-center gap-2">
      <Button
        type="button"
        variant="outline"
        size="sm"
        class="h-8 px-3 text-xs gap-1.5 border-border shadow-xs"
        onclick={() => controller.refresh()}
        disabled={controller.isLoading || controller.isOperating}
      >
        <ArrowPath size={13} class={controller.isLoading ? "animate-spin text-primary" : "text-muted-foreground"} />
        <span>{l("تحديث", "Refresh")}</span>
      </Button>

      <Button
        type="button"
        variant="outline"
        size="sm"
        class="h-8 px-3 text-xs gap-1.5 border-border shadow-xs"
        onclick={handleOpenExistingDatabase}
        disabled={controller.isOperating}
      >
        <FolderOpen size={13} class="text-primary" />
        <span>{m.database_action_open?.() || l("فتح قاعدة أخرى", "Open DB")}</span>
      </Button>

      <Button
        type="button"
        variant="default"
        size="sm"
        class="h-8 px-3.5 text-xs font-bold gap-1.5 shadow-xs shadow-primary/20"
        onclick={() => controller.isCreateDialogOpen = true}
        disabled={controller.isOperating}
      >
        <Plus size={14} />
        <span>{m.database_action_create?.() || l("إنشاء قاعدة جديدة", "New DB")}</span>
      </Button>
    </div>
  </div>

  <!-- Navigation Tabs Bar -->
  <div class="px-6 border-b border-border/40 bg-muted/10 shrink-0">
    <div class="flex gap-1 overflow-x-auto no-scrollbar py-2">
      <button
        type="button"
        class="px-3.5 py-1.5 rounded-xl text-xs font-semibold transition-all flex items-center gap-2 cursor-pointer {activeTab === 'overview' ? 'bg-primary text-primary-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground hover:bg-muted/40'}"
        onclick={() => activeTab = "overview"}
      >
        <Database size={14} />
        <span>{m.database_tab_overview?.() || l("نظرة عامة والحالة", "Overview & Health")}</span>
      </button>

      <button
        type="button"
        class="px-3.5 py-1.5 rounded-xl text-xs font-semibold transition-all flex items-center gap-2 cursor-pointer {activeTab === 'connection' ? 'bg-primary text-primary-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground hover:bg-muted/40'}"
        onclick={() => activeTab = "connection"}
      >
        <GlobeAlt size={14} />
        <span>{m.database_tab_connection?.() || l("إعدادات الاتصال", "Connection Settings")}</span>
      </button>

      <button
        type="button"
        class="px-3.5 py-1.5 rounded-xl text-xs font-semibold transition-all flex items-center gap-2 cursor-pointer {activeTab === 'backup' ? 'bg-primary text-primary-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground hover:bg-muted/40'}"
        onclick={() => activeTab = "backup"}
      >
        <ArrowDownTray size={14} />
        <span>{m.database_tab_backup?.() || l("النسخ الاحتياطي", "Backup")}</span>
      </button>

      <button
        type="button"
        class="px-3.5 py-1.5 rounded-xl text-xs font-semibold transition-all flex items-center gap-2 cursor-pointer {activeTab === 'restore' ? 'bg-primary text-primary-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground hover:bg-muted/40'}"
        onclick={() => activeTab = "restore"}
      >
        <ArrowUpTray size={14} />
        <span>{m.database_tab_restore?.() || l("الاستعادة", "Restore")}</span>
      </button>

      <button
        type="button"
        class="px-3.5 py-1.5 rounded-xl text-xs font-semibold transition-all flex items-center gap-2 cursor-pointer {activeTab === 'maintenance' ? 'bg-primary text-primary-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground hover:bg-muted/40'}"
        onclick={() => activeTab = "maintenance"}
      >
        <Wrench size={14} />
        <span>{m.database_tab_maintenance?.() || l("الصيانة والترحيلات", "Maintenance & Migrations")}</span>
        {#if controller.status && controller.status.pending_migrations.length > 0}
          <span class="size-2 rounded-full bg-amber-500"></span>
        {/if}
      </button>
    </div>
  </div>

  <!-- Main Scrollable Content Area -->
  <div class="flex-1 overflow-y-auto p-6 space-y-6">

    <!-- Active Global Notifications / Alerts -->
    {#if controller.operationError}
      <div class="p-3.5 rounded-2xl bg-destructive/10 border border-destructive/20 text-destructive text-xs flex items-center justify-between gap-3 animate-in fade-in">
        <div class="flex items-center gap-2.5">
          <InfoCircle size={16} class="shrink-0" />
          <span>{controller.operationError}</span>
        </div>
        <Button variant="ghost" size="sm" class="h-7 px-2 text-xs text-destructive hover:bg-destructive/20" onclick={() => controller.operationError = null}>
          {l("إغلاق", "Dismiss")}
        </Button>
      </div>
    {/if}

    {#if controller.operationSuccess}
      <div class="p-3.5 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 dark:text-emerald-400 text-xs flex items-center justify-between gap-3 animate-in fade-in">
        <div class="flex items-center gap-2.5">
          <CheckCircle size={16} class="shrink-0" />
          <span>{controller.operationSuccess}</span>
        </div>
        <Button variant="ghost" size="sm" class="h-7 px-2 text-xs text-emerald-600 hover:bg-emerald-500/20" onclick={() => controller.operationSuccess = null}>
          {l("إغلاق", "Dismiss")}
        </Button>
      </div>
    {/if}

    <!-- ═══════════════════════════════════════════════════════════ -->
    <!-- TAB 1: OVERVIEW & HEALTH -->
    <!-- ═══════════════════════════════════════════════════════════ -->
    {#if activeTab === "overview"}
      <div class="space-y-6 animate-in fade-in duration-200">
        <!-- Diagnostic Summary Cards Grid -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <!-- Engine & Target -->
          <div class="p-4 rounded-2xl bg-card border border-border/70 shadow-xs space-y-2 text-start">
            <div class="flex items-center justify-between">
              <span class="text-[11px] font-semibold text-muted-foreground">{m.database_card_engine?.() || l("محرك البيانات", "Engine")}</span>
              <div class="size-7 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
                {#if controller.isLocal}
                  <Computer size={14} />
                {:else}
                  <GlobeAlt size={14} />
                {/if}
              </div>
            </div>
            <div class="text-sm font-bold text-foreground truncate">
              {controller.isLocal ? "SQLite (Embedded)" : "PostgreSQL (Remote)"}
            </div>
            <p class="text-[11px] text-muted-foreground font-mono truncate" title={controller.status?.connection_target}>
              {controller.status?.database_name || "orbis.db"}
            </p>
          </div>

          <!-- DB Size -->
          <div class="p-4 rounded-2xl bg-card border border-border/70 shadow-xs space-y-2 text-start">
            <div class="flex items-center justify-between">
              <span class="text-[11px] font-semibold text-muted-foreground">{m.database_card_size?.() || l("حجم البيانات", "Size")}</span>
              <div class="size-7 rounded-lg bg-indigo-500/10 flex items-center justify-center text-indigo-500">
                <Database size={14} />
              </div>
            </div>
            <div class="text-sm font-bold text-foreground">
              {controller.status?.file_size_formatted || l("غير محدد", "N/A")}
            </div>
            <p class="text-[11px] text-muted-foreground truncate">
              {controller.status?.journal_mode ? `WAL Mode: ${controller.status.journal_mode}` : l("متصل ومفهرس", "Indexed & Ready")}
            </p>
          </div>

          <!-- Schema Version & Migrations -->
          <div class="p-4 rounded-2xl bg-card border border-border/70 shadow-xs space-y-2 text-start">
            <div class="flex items-center justify-between">
              <span class="text-[11px] font-semibold text-muted-foreground">{m.database_card_version?.() || l("إصدار المخطط", "Schema")}</span>
              <div class="size-7 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-500">
                <Sparkles size={14} />
              </div>
            </div>
            <div class="text-sm font-bold text-foreground">
              {controller.status?.schema_version || "v1.0.0"}
            </div>
            <p class="text-[11px] text-muted-foreground truncate">
              {controller.status?.applied_migrations_count ?? 0} / {controller.status?.total_migrations_count ?? 0} {l("ترحيل مطبق", "applied")}
            </p>
          </div>

          <!-- Last Backup -->
          <div class="p-4 rounded-2xl bg-card border border-border/70 shadow-xs space-y-2 text-start">
            <div class="flex items-center justify-between">
              <span class="text-[11px] font-semibold text-muted-foreground">{m.database_card_last_backup?.() || l("آخر نسخة احتياطية", "Last Backup")}</span>
              <div class="size-7 rounded-lg bg-amber-500/10 flex items-center justify-center text-amber-500">
                <ClockRotateRight size={14} />
              </div>
            </div>
            <div class="text-xs font-bold text-foreground truncate">
              {controller.status?.last_backup_timestamp ? new Date(controller.status.last_backup_timestamp).toLocaleString() : l("لا توجد نسخ سابقة", "Never")}
            </div>
            <p class="text-[11px] text-muted-foreground truncate" title={controller.status?.last_backup_path}>
              {controller.status?.last_backup_path ? l("نسخة مؤمنة", "Verified snapshot") : l("يوصى بإنشاء نسخة الآن", "Backup recommended")}
            </p>
          </div>
        </div>

        <!-- Health & Fast Actions Panel -->
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
          <!-- Left Column: Details & Paths (7 cols) -->
          <div class="lg:col-span-7 rounded-2xl bg-card border border-border/70 p-5 space-y-4 text-start">
            <div class="flex items-center justify-between pb-3 border-b border-border/40">
              <div class="flex items-center gap-2">
                <ShieldCheck size={16} class="text-primary" />
                <h3 class="text-xs sm:text-sm font-bold text-foreground">
                  {l("تفاصيل ومسار قاعدة البيانات النشطة", "Active Database Target & Details")}
                </h3>
              </div>
              <Badge variant="outline" class="text-[10px] font-mono">
                {controller.status?.mode?.toUpperCase() || "SQLITE"}
              </Badge>
            </div>

            <div class="space-y-3 text-xs">
              <div class="p-3 rounded-xl bg-muted/20 border border-border/50 space-y-1">
                <span class="text-[11px] font-semibold text-muted-foreground">{l("المسار الفعلي للتخزين:", "Storage Path:")}</span>
                <p class="font-mono text-[11px] text-foreground break-all select-all">
                  {controller.status?.connection_target || l("جاري التحميل...", "Loading...")}
                </p>
              </div>

              <div class="grid grid-cols-2 gap-3">
                <div class="p-3 rounded-xl bg-muted/20 border border-border/50">
                  <span class="text-[11px] font-semibold text-muted-foreground">{l("حالة التكامل والسلامة:", "Integrity Check:")}</span>
                  <div class="flex items-center gap-1.5 mt-1 font-bold text-emerald-600 dark:text-emerald-400">
                    <CheckCircle size={13} />
                    <span>{controller.status?.integrity_status === "ok" ? l("سليمة ومفحوصة (OK)", "Healthy (OK)") : controller.status?.integrity_status || l("سليمة", "OK")}</span>
                  </div>
                </div>

                <div class="p-3 rounded-xl bg-muted/20 border border-border/50">
                  <span class="text-[11px] font-semibold text-muted-foreground">{l("التوافقية مع التطبيق:", "Version Compatibility:")}</span>
                  <div class="flex items-center gap-1.5 mt-1 font-bold {controller.status?.is_compatible ? 'text-emerald-600 dark:text-emerald-400' : 'text-amber-600 dark:text-amber-400'}">
                    <Bolt size={13} />
                    <span>{controller.status?.is_compatible ? l("متوافق بنسبة 100%", "Fully Compatible") : l("يتطلب تطبيق ترحيلات", "Pending Migrations")}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Right Column: Quick Maintenance Actions (5 cols) -->
          <div class="lg:col-span-5 rounded-2xl bg-card border border-border/70 p-5 space-y-3 text-start">
            <h3 class="text-xs sm:text-sm font-bold text-foreground pb-2 border-b border-border/40">
              {l("إجراءات سريعة وفورية", "Quick Operations")}
            </h3>

            <div class="space-y-2">
              <Button
                type="button"
                variant="outline"
                class="w-full justify-start text-xs h-10 gap-2.5 border-border hover:bg-muted/40 cursor-pointer"
                onclick={handleTriggerBackup}
                disabled={controller.isOperating}
              >
                <ArrowDownTray size={15} class="text-primary shrink-0" />
                <div class="text-start">
                  <div class="font-bold">{m.database_action_backup_now?.() || l("إنشاء نسخة احتياطية فورية", "Backup Database Now")}</div>
                </div>
              </Button>

              <Button
                type="button"
                variant="outline"
                class="w-full justify-start text-xs h-10 gap-2.5 border-border hover:bg-muted/40 cursor-pointer"
                onclick={() => controller.runIntegrityCheck()}
                disabled={controller.isOperating}
              >
                <ShieldCheck size={15} class="text-emerald-500 shrink-0" />
                <div class="text-start">
                  <div class="font-bold">{m.database_action_check_integrity?.() || l("فحص السلامة الشامل", "Run Integrity Check")}</div>
                </div>
              </Button>

              <Button
                type="button"
                variant="outline"
                class="w-full justify-start text-xs h-10 gap-2.5 border-border hover:bg-muted/40 cursor-pointer"
                onclick={() => controller.optimizeDatabase()}
                disabled={controller.isOperating}
              >
                <Sparkles size={15} class="text-amber-500 shrink-0" />
                <div class="text-start">
                  <div class="font-bold">{m.database_action_optimize?.() || l("ضغط وتحسين الأداء (VACUUM)", "Compact & Optimize DB")}</div>
                </div>
              </Button>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- ═══════════════════════════════════════════════════════════ -->
    <!-- TAB 2: CONNECTION SETTINGS -->
    <!-- ═══════════════════════════════════════════════════════════ -->
    {#if activeTab === "connection"}
      <div class="max-w-4xl mx-auto space-y-6 animate-in fade-in duration-200">
        <div class="p-6 rounded-2xl bg-card border border-border/70 space-y-6 text-start">
          
          <!-- Mode Selector Switcher Cards -->
          <div class="space-y-2">
            <label class="text-xs font-bold text-foreground">
              {l("اختر وضع الاتصال النشط", "Select Active Connection Mode")}
            </label>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <button
                type="button"
                class="p-4 rounded-2xl border-2 transition-all flex items-center justify-between cursor-pointer {connMode === 'local' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border/60 bg-muted/5 hover:border-border'}"
                onclick={() => connMode = "local"}
              >
                <div class="flex items-center gap-3">
                  <div class="size-9 rounded-xl flex items-center justify-center {connMode === 'local' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                    <Computer size={18} />
                  </div>
                  <div class="text-start">
                    <h4 class="text-xs font-bold text-foreground">{l("قاعدة بيانات محلية (SQLite)", "Local SQLite")}</h4>
                    <p class="text-[11px] text-muted-foreground mt-0.5">{l("تخزين مدمج وسريع على هذا الجهاز", "Embedded on this workstation")}</p>
                  </div>
                </div>
                {#if connMode === "local"}
                  <CheckCircle size={16} class="text-primary" />
                {/if}
              </button>

              <button
                type="button"
                class="p-4 rounded-2xl border-2 transition-all flex items-center justify-between cursor-pointer {connMode === 'remote' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border/60 bg-muted/5 hover:border-border'}"
                onclick={() => connMode = "remote"}
              >
                <div class="flex items-center gap-3">
                  <div class="size-9 rounded-xl flex items-center justify-center {connMode === 'remote' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                    <GlobeAlt size={18} />
                  </div>
                  <div class="text-start">
                    <h4 class="text-xs font-bold text-foreground">{l("خادم سحابي / شبكي (Remote)", "Remote Server / Cloud")}</h4>
                    <p class="text-[11px] text-muted-foreground mt-0.5">{l("الاتصال بخادم مدرسي مركزي أو PostgreSQL", "Central Orbis Server or Postgres")}</p>
                  </div>
                </div>
                {#if connMode === "remote"}
                  <CheckCircle size={16} class="text-primary" />
                {/if}
              </button>
            </div>
          </div>

          <!-- Adaptive Form Fields -->
          {#if connMode === "local"}
            <div class="p-5 rounded-2xl bg-muted/20 border border-border/50 space-y-4">
              <FormField id="connDbFilename" label={l("اسم ملف قاعدة البيانات", "Database File Name")}>
                {#snippet children()}
                  <Input
                    id="connDbFilename"
                    type="text"
                    bind:value={connDbFilename}
                    class="h-9 text-xs font-mono bg-background border-input"
                    style="direction: ltr;"
                  />
                {/snippet}
              </FormField>

              <FormField id="connCustomPath" label={l("مسار ملف مخصص (اختياري)", "Custom Absolute File Path (Optional)")}>
                {#snippet children()}
                  <div class="flex gap-2">
                    <Input
                      id="connCustomPath"
                      type="text"
                      bind:value={connCustomPath}
                      placeholder={l("اترك فارغاً لاستخدام مسار AppData الافتراضي", "Leave blank for default AppData path")}
                      class="h-9 text-xs font-mono bg-background border-input flex-1"
                      style="direction: ltr;"
                    />
                    <Button
                      type="button"
                      variant="outline"
                      size="sm"
                      class="h-9 px-3 text-xs gap-1.5 shrink-0 border-border"
                      onclick={handleBrowseCustomDbPath}
                    >
                      <FolderOpen size={14} class="text-primary" />
                      <span>{l("تصفح", "Browse")}</span>
                    </Button>
                  </div>
                {/snippet}
              </FormField>

              <div class="pt-2 border-t border-border/40 flex items-center justify-between">
                <div>
                  <div class="text-xs font-bold text-foreground">{l("نمط التزامن WAL (Write-Ahead Logging)", "SQLite WAL Mode")}</div>
                  <p class="text-[11px] text-muted-foreground">{l("يوفر سرعة استجابة فائقة ويمنع تضارب القراءة والكتابة", "Enables high concurrency for simultaneous reads and writes")}</p>
                </div>
                <Switch bind:checked={connWalMode} />
              </div>
            </div>
          {:else}
            <!-- Remote Server / PostgreSQL Form -->
            <div class="p-5 rounded-2xl bg-muted/20 border border-border/50 space-y-4">
              <FormField id="connServerUrl" label={l("عنوان خادم Orbis API", "Orbis Server URL")} required>
                {#snippet children()}
                  <div class="flex gap-2">
                    <Input
                      id="connServerUrl"
                      type="url"
                      bind:value={connServerUrl}
                      placeholder="http://localhost:3000"
                      class="h-9 text-xs font-mono bg-background border-input flex-1"
                      style="direction: ltr;"
                    />
                    <Button
                      type="button"
                      variant="outline"
                      size="sm"
                      class="h-9 px-3 text-xs gap-1.5 shrink-0 border-border"
                      onclick={handleTestRemote}
                      disabled={controller.isCheckingHealth}
                    >
                      {#if controller.isCheckingHealth}
                        <ArrowPath size={14} class="animate-spin text-primary" />
                      {:else}
                        <Bolt size={14} class="text-primary" />
                      {/if}
                      <span>{m.database_action_test_connection?.() || l("اختبار", "Test")}</span>
                    </Button>
                  </div>
                {/snippet}
              </FormField>

              <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <FormField id="connHost" label={l("مضيف قاعدة البيانات (Host)", "Database Host")}>
                  {#snippet children()}
                    <Input id="connHost" type="text" bind:value={connHost} class="h-9 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                  {/snippet}
                </FormField>

                <FormField id="connPort" label={l("المنفذ (Port)", "Port")}>
                  {#snippet children()}
                    <Input id="connPort" type="number" bind:value={connPort} class="h-9 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                  {/snippet}
                </FormField>

                <FormField id="connDatabaseName" label={l("اسم قاعدة البيانات", "Database Name")}>
                  {#snippet children()}
                    <Input id="connDatabaseName" type="text" bind:value={connDatabaseName} class="h-9 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                  {/snippet}
                </FormField>

                <FormField id="connUsername" label={l("اسم المستخدم", "Username")}>
                  {#snippet children()}
                    <Input id="connUsername" type="text" bind:value={connUsername} class="h-9 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                  {/snippet}
                </FormField>
              </div>

              <FormField id="connPassword" label={l("كلمة المرور", "Password")}>
                {#snippet children()}
                  <Input id="connPassword" type="password" bind:value={connPassword} placeholder="••••••••" class="h-9 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                {/snippet}
              </FormField>
            </div>
          {/if}

          <!-- Form Actions -->
          <div class="flex items-center justify-between pt-2 border-t border-border/40">
            <Button
              type="button"
              variant="outline"
              size="sm"
              class="h-9 px-3 text-xs text-muted-foreground hover:text-destructive border-border"
              onclick={() => controller.resetConnectionConfig()}
              disabled={controller.isOperating}
            >
              {m.database_action_reset_connection?.() || l("استعادة الافتراضي", "Reset Default")}
            </Button>

            <Button
              type="button"
              variant="default"
              size="sm"
              class="h-9 px-5 text-xs font-bold gap-1.5 shadow-xs shadow-primary/20"
              onclick={handleSaveConnection}
              disabled={controller.isOperating}
            >
              {#if controller.isOperating && controller.operationName === "save_connection"}
                <ArrowPath size={14} class="animate-spin" />
                <span>{l("جاري الحفظ والاتصال...", "Saving & Connecting...")}</span>
              {:else}
                <CheckCircle size={14} />
                <span>{m.database_action_save_connection?.() || l("حفظ وتطبيق الاتصال", "Save & Apply")}</span>
              {/if}
            </Button>
          </div>
        </div>
      </div>
    {/if}

    <!-- ═══════════════════════════════════════════════════════════ -->
    <!-- TAB 3: BACKUP -->
    <!-- ═══════════════════════════════════════════════════════════ -->
    {#if activeTab === "backup"}
      <div class="max-w-4xl mx-auto space-y-6 animate-in fade-in duration-200">
        
        <!-- Manual Backup Card -->
        <div class="p-6 rounded-2xl bg-card border border-border/70 space-y-5 text-start">
          <div class="flex items-center gap-3 pb-3 border-b border-border/40">
            <div class="size-9 rounded-xl bg-primary/10 flex items-center justify-center text-primary">
              <ArrowDownTray size={18} />
            </div>
            <div>
              <h3 class="text-sm font-bold text-foreground">{l("إنشاء نسخة احتياطية فورية", "Create Instant Backup")}</h3>
              <p class="text-[11px] text-muted-foreground">{l("توليد نسخة متكاملة وذاتية الفحص عبر محرك SQLite VACUUM INTO", "Generate a consistent, verified online snapshot without table locks")}</p>
            </div>
          </div>

          <FormField id="customBackupPath" label={m.database_backup_destination_label?.() || l("مكان حفظ النسخة", "Backup Destination Path")}>
            {#snippet children()}
              <div class="flex gap-2">
                <Input
                  id="customBackupPath"
                  type="text"
                  bind:value={customBackupPath}
                  placeholder={m.database_backup_custom_dest_hint?.() || l("اترك الحقل فارغاً للحفظ في مجلد النسخ التلقائي الافتراضي", "Leave blank for default backups folder")}
                  class="h-9 text-xs font-mono bg-background border-input flex-1"
                  style="direction: ltr;"
                />
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  class="h-9 px-3 text-xs gap-1.5 shrink-0 border-border"
                  onclick={handleBrowseBackupDest}
                >
                  <FolderOpen size={14} class="text-primary" />
                  <span>{l("تحديد المكان", "Choose Location")}</span>
                </Button>
              </div>
            {/snippet}
          </FormField>

          <Button
            type="button"
            variant="default"
            class="w-full h-10 text-xs font-bold gap-2 cursor-pointer shadow-xs shadow-primary/20"
            onclick={handleTriggerBackup}
            disabled={controller.isOperating}
          >
            {#if controller.isOperating && controller.operationName === "backup"}
              <ArrowPath size={14} class="animate-spin" />
              <span>{l("جاري إنشاء النسخة الاحتياطية وفحص سلامتها...", "Creating & Verifying Backup...")}</span>
            {:else}
              <ArrowDownTray size={15} />
              <span>{m.database_action_backup_now?.() || l("بدء إنشاء النسخة الاحتياطية الآن", "Execute Backup Now")}</span>
            {/if}
          </Button>
        </div>

        <!-- Automatic Backup Policy Settings Card -->
        <div class="p-6 rounded-2xl bg-card border border-border/70 space-y-5 text-start">
          <div class="flex items-center justify-between pb-3 border-b border-border/40">
            <div class="flex items-center gap-3">
              <div class="size-9 rounded-xl bg-indigo-500/10 flex items-center justify-center text-indigo-500">
                <ClockRotateRight size={18} />
              </div>
              <div>
                <h3 class="text-sm font-bold text-foreground">{l("سياسات النسخ الاحتياطي التلقائي", "Automated Backup Policy")}</h3>
                <p class="text-[11px] text-muted-foreground">{l("جدولة إنشاء النسخ الاحتياطية وإدارة فترات الاحتفاظ", "Schedule recurring backups and retention limits")}</p>
              </div>
            </div>
            <Switch bind:checked={autoBackup} />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs">
            <FormField id="backupInterval" label={m.database_backup_interval_label?.() || l("تكرار النسخ (بالساعات)", "Interval (Hours)")}>
              {#snippet children()}
                <Input id="backupInterval" type="number" bind:value={backupInterval} min="1" max="168" class="h-9 text-xs bg-background border-input" disabled={!autoBackup} />
              {/snippet}
            </FormField>

            <FormField id="maxBackups" label={m.database_backup_max_files_label?.() || l("أقصى عدد للنسخ المحفوظة", "Max Retained Files")}>
              {#snippet children()}
                <Input id="maxBackups" type="number" bind:value={maxBackups} min="1" max="100" class="h-9 text-xs bg-background border-input" disabled={!autoBackup} />
              {/snippet}
            </FormField>
          </div>

          <div class="pt-2 border-t border-border/40 flex justify-end">
            <Button
              type="button"
              variant="outline"
              size="sm"
              class="h-8 px-4 text-xs font-bold border-border"
              onclick={handleSaveSettings}
              disabled={controller.isOperating}
            >
              {l("حفظ السياسات", "Save Policy")}
            </Button>
          </div>
        </div>
      </div>
    {/if}

    <!-- ═══════════════════════════════════════════════════════════ -->
    <!-- TAB 4: RESTORE -->
    <!-- ═══════════════════════════════════════════════════════════ -->
    {#if activeTab === "restore"}
      <div class="max-w-4xl mx-auto space-y-6 animate-in fade-in duration-200">
        <div class="p-6 rounded-2xl bg-card border border-border/70 space-y-6 text-start">
          
          <div class="flex items-center gap-3 pb-3 border-b border-border/40">
            <div class="size-9 rounded-xl bg-amber-500/10 flex items-center justify-center text-amber-500">
              <ArrowUpTray size={18} />
            </div>
            <div>
              <h3 class="text-sm font-bold text-foreground">{l("استعادة قاعدة البيانات من نسخة سابقة", "Restore Database from Backup")}</h3>
              <p class="text-[11px] text-muted-foreground">{l("إجراء استعادة مرحلي آمن مع فحص السلامة المسبق وأخذ لقطة أمان تلقائية", "Staged safe recovery workflow with pre-validation and automatic safety snapshot")}</p>
            </div>
          </div>

          <!-- Step 1: File Picker -->
          <div class="space-y-2">
            <label class="text-xs font-bold text-foreground">
              {m.database_restore_step1_title?.() || l("1. اختيار ملف النسخة الاحتياطية", "1. Select Backup File")}
            </label>
            <div class="flex gap-2">
              <Input
                type="text"
                bind:value={restoreFilePath}
                placeholder={l("حدد ملف النسخة المراد استعادتها (*.db)", "Select backup file (*.db)")}
                class="h-9 text-xs font-mono bg-background border-input flex-1"
                style="direction: ltr;"
                readonly
              />
              <Button
                type="button"
                variant="outline"
                size="sm"
                class="h-9 px-3.5 text-xs gap-1.5 shrink-0 border-border"
                onclick={handleBrowseRestoreFile}
                disabled={controller.isOperating}
              >
                <FolderOpen size={14} class="text-primary" />
                <span>{m.database_browse_file?.() || l("اختيار ملف", "Browse File")}</span>
              </Button>
            </div>
          </div>

          <!-- Step 2: Backup File Inspection Card -->
          {#if controller.isInspectingBackup}
            <div class="p-4 rounded-xl bg-muted/20 border border-border/50 text-xs flex items-center gap-2.5 text-muted-foreground">
              <ArrowPath size={14} class="animate-spin text-primary" />
              <span>{l("جاري فحص وتدقيق ملف النسخة الاحتياطية...", "Inspecting and validating backup file...")}</span>
            </div>
          {:else if controller.inspectedBackup}
            <div class="p-4 rounded-2xl border text-xs space-y-3 {controller.inspectedBackup.is_valid ? 'bg-emerald-500/5 border-emerald-500/20' : 'bg-destructive/5 border-destructive/20'}">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2 font-bold text-foreground">
                  {#if controller.inspectedBackup.is_valid}
                    <CheckCircle size={15} class="text-emerald-500" />
                    <span>{l("تم التحقق من سلامة النسخة الاحتياطية", "Backup Validated Successfully")}</span>
                  {:else}
                    <InfoCircle size={15} class="text-destructive" />
                    <span>{l("ملف النسخة غير صالح أو تالف", "Invalid or Corrupted Backup File")}</span>
                  {/if}
                </div>
                <Badge variant="outline" class="text-[10px] font-mono">
                  {controller.inspectedBackup.file_size_formatted}
                </Badge>
              </div>

              <div class="grid grid-cols-2 gap-3 text-[11px] text-muted-foreground">
                <div>{l("إصدار المخطط:", "Schema Version:")} <span class="font-bold text-foreground font-mono">{controller.inspectedBackup.schema_version}</span></div>
                <div>{l("الترحيلات المطبقة:", "Applied Migrations:")} <span class="font-bold text-foreground">{controller.inspectedBackup.applied_migrations_count}</span></div>
              </div>
            </div>
          {/if}

          <!-- Step 3: Safety Warning & Trigger -->
          <div class="p-4 rounded-2xl bg-amber-500/10 border border-amber-500/20 text-amber-700 dark:text-amber-400 text-xs space-y-2">
            <div class="flex items-center gap-2 font-bold">
              <InfoCircle size={15} />
              <span>{l("تنبيه أمان الاستعادة", "Restoration Safety Notice")}</span>
            </div>
            <p class="text-[11px] leading-relaxed">
              {m.database_restore_warning_desc?.() || l("تحذير: الاستعادة عملية حساسة ستستبدل بيانات قاعدة البيانات الحالية. سيقوم النظام بأخذ لقطة أمان تلقائية للبيانات الحالية قبل الاستبدال.", "Warning: Restoration will overwrite current records. An automatic safety snapshot will be taken first.")}
            </p>
          </div>

          <Button
            type="button"
            variant="destructive"
            class="w-full h-10 text-xs font-bold gap-2 cursor-pointer shadow-xs"
            onclick={handleExecuteRestore}
            disabled={!controller.inspectedBackup?.is_valid || controller.isOperating}
          >
            {#if controller.isOperating && controller.operationName === "restore"}
              <ArrowPath size={14} class="animate-spin" />
              <span>{l("جاري الاستعادة وتطبيق الترحيلات...", "Restoring & Upgrading...")}</span>
            {:else}
              <ArrowUpTray size={15} />
              <span>{m.database_restore_btn?.() || l("تأكيد وبدء الاستعادة", "Confirm & Execute Restore")}</span>
            {/if}
          </Button>
        </div>
      </div>
    {/if}

    <!-- ═══════════════════════════════════════════════════════════ -->
    <!-- TAB 5: MAINTENANCE & MIGRATIONS -->
    <!-- ═══════════════════════════════════════════════════════════ -->
    {#if activeTab === "maintenance"}
      <div class="max-w-4xl mx-auto space-y-6 animate-in fade-in duration-200">
        
        <!-- Migrations Panel -->
        <div class="p-6 rounded-2xl bg-card border border-border/70 space-y-5 text-start">
          <div class="flex items-center justify-between pb-3 border-b border-border/40">
            <div class="flex items-center gap-3">
              <div class="size-9 rounded-xl bg-primary/10 flex items-center justify-center text-primary">
                <Sparkles size={18} />
              </div>
              <div>
                <h3 class="text-sm font-bold text-foreground">{l("ترحيلات وتحديثات المخطط (Schema Migrations)", "Database Schema Migrations")}</h3>
                <p class="text-[11px] text-muted-foreground">{l("مزامنة وتطبيق تحديثات الجداول وحقول النظام الجديدة", "Synchronize table structures with the current Orbis version")}</p>
              </div>
            </div>

            {#if controller.status && controller.status.pending_migrations.length === 0}
              <Badge variant="outline" class="text-[10px] bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20">
                {l("محدث بالكامل", "Up to Date")}
              </Badge>
            {:else}
              <Badge variant="outline" class="text-[10px] bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20">
                {controller.status?.pending_migrations.length} {l("تحديث معلق", "Pending")}
              </Badge>
            {/if}
          </div>

          {#if controller.status && controller.status.pending_migrations.length > 0}
            <div class="p-4 rounded-xl bg-amber-500/10 border border-amber-500/20 text-xs space-y-2">
              <span class="font-bold text-amber-700 dark:text-amber-400">{l("الترحيلات المعلقة المطلوب تطبيقها:", "Pending migrations:")}</span>
              <ul class="list-disc list-inside space-y-1 font-mono text-[11px] text-foreground">
                {#each controller.status.pending_migrations as migration}
                  <li>{migration}</li>
                {/each}
              </ul>
            </div>

            <Button
              type="button"
              variant="default"
              class="w-full h-9 text-xs font-bold gap-2 cursor-pointer shadow-xs"
              onclick={() => controller.applyPendingMigrations()}
              disabled={controller.isOperating}
            >
              <Bolt size={14} />
              <span>{m.database_action_apply_migrations?.() || l("تطبيق التحديثات الآن", "Apply Migrations Now")}</span>
            </Button>
          {:else}
            <div class="p-4 rounded-xl bg-muted/20 border border-border/40 text-center py-6 space-y-1 text-xs text-muted-foreground">
              <CheckCircle size={22} class="mx-auto text-emerald-500 mb-2" />
              <div class="font-bold text-foreground">{l("مخطط قاعدة البيانات محدث بالكامل", "Database schema is completely up to date")}</div>
              <p class="text-[11px]">{l("تم تطبيق جميع الترحيلات بنجاح ولا توجد تحديثات معلقة.", "All migrations are applied and active.")}</p>
            </div>
          {/if}
        </div>

        <!-- Integrity & Compaction Panel -->
        <div class="p-6 rounded-2xl bg-card border border-border/70 space-y-5 text-start">
          <div class="flex items-center gap-3 pb-3 border-b border-border/40">
            <div class="size-9 rounded-xl bg-emerald-500/10 flex items-center justify-center text-emerald-500">
              <ShieldCheck size={18} />
            </div>
            <div>
              <h3 class="text-sm font-bold text-foreground">{l("فحص السلامة وضغط الملف (Integrity & Compaction)", "Integrity Check & Compaction")}</h3>
              <p class="text-[11px] text-muted-foreground">{l("فحص مؤشرات B-Tree وإعادة ترتيب الصفحات واسترجاع المساحة غير المستخدمة", "Verify B-Tree indices and reclaim unallocated space via SQLite VACUUM")}</p>
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="p-4 rounded-xl bg-muted/20 border border-border/50 space-y-3">
              <div class="font-bold text-xs text-foreground">{l("فحص السلامة (PRAGMA integrity_check)", "Integrity Verification")}</div>
              <p class="text-[11px] text-muted-foreground">{l("يقوم بفحص شامل لجداول ومؤشرات قاعدة البيانات للتحقق من عدم وجود أي تلف.", "Comprehensive B-Tree check for corruption detection.")}</p>
              <Button
                type="button"
                variant="outline"
                size="sm"
                class="w-full text-xs font-bold gap-1.5 border-border"
                onclick={() => controller.runIntegrityCheck()}
                disabled={controller.isOperating}
              >
                <ShieldCheck size={14} class="text-emerald-500" />
                <span>{l("فحص السلامة", "Check Integrity")}</span>
              </Button>
            </div>

            <div class="p-4 rounded-xl bg-muted/20 border border-border/50 space-y-3">
              <div class="font-bold text-xs text-foreground">{l("الضغط والتحسين (VACUUM)", "Compact & Optimize")}</div>
              <p class="text-[11px] text-muted-foreground">{l("يقوم بدمج سجلات WAL وإعادة ترتيب الصفحات لاسترجاع المساحات المتروكة.", "Checkpoints WAL and reclaims fragmented storage space.")}</p>
              <Button
                type="button"
                variant="outline"
                size="sm"
                class="w-full text-xs font-bold gap-1.5 border-border"
                onclick={() => controller.optimizeDatabase()}
                disabled={controller.isOperating}
              >
                <Sparkles size={14} class="text-amber-500" />
                <span>{l("ضغط وتحسين", "Compact DB")}</span>
              </Button>
            </div>
          </div>

          {#if controller.integrityResult}
            <div class="p-4 rounded-xl border text-xs space-y-2 {controller.integrityResult.is_healthy ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400' : 'bg-destructive/10 border-destructive/20 text-destructive'}">
              <div class="flex items-center gap-2 font-bold">
                <ShieldCheck size={15} />
                <span>{controller.integrityResult.is_healthy ? l("نتيجة الفحص: سليمة 100%", "Integrity Result: 100% Healthy") : l("تنبيه: تم رصد مشكلات", "Issues Detected")}</span>
              </div>
              <ul class="list-disc list-inside font-mono text-[11px]">
                {#each controller.integrityResult.details as detail}
                  <li>{detail}</li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if controller.optimizeResult}
            <div class="p-4 rounded-xl bg-primary/10 border border-primary/20 text-xs space-y-1">
              <div class="font-bold text-foreground">{controller.optimizeResult.message}</div>
              <div class="text-[11px] text-muted-foreground">
                {l("المساحة المسترجعة:", "Space Reclaimed:")} <span class="font-bold text-primary font-mono">{controller.optimizeResult.space_freed_formatted}</span>
              </div>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Creation Wizard Dialog -->
<CreateDatabaseDialog
  bind:open={controller.isCreateDialogOpen}
  {controller}
/>
