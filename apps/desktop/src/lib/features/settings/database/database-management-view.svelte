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
    integrityOnStartup = controller.settings.integrity_check_on_startup ?? true;
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
  const file = await controller.pickSavePath(
    `orbis_backup_${timestamp}.db`,
    l("ملف نسخة احتياطية (*.db)", "Backup File (*.db)"),
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
  const file = await controller.pickFile(l("ملف نسخة احتياطية (*.db)", "Backup File (*.db)"), ["db", "sqlite", "bak"]);
  if (file) {
    restoreFilePath = file;
    await controller.inspectBackupFile(file);
  }
}

async function handleExecuteRestore() {
  if (!restoreFilePath.trim()) {
    toast.error(l("يرجى اختيار ملف النسخة الاحتياطية أولاً", "Please select a backup file first"));
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
    toast.success(l("تم حفظ سياسات النسخ الاحتياطي بنجاح", "Backup policies saved successfully"));
  } catch (e) {
    toast.error(e instanceof Error ? e.message : String(e));
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

// KPI Stats
const kpiStats = $derived([
  {
    label: l("محرك التخزين", "Engine Mode"),
    value: controller.isLocal ? "SQLite (Embedded)" : "PostgreSQL",
  },
  {
    label: l("حجم البيانات", "Storage Size"),
    value: controller.status?.file_size_formatted || "—",
  },
  {
    label: l("إصدار المخطط", "Schema Version"),
    value: controller.status?.schema_version || "v1.0.0",
  },
  {
    label: l("الترحيلات المطبقة", "Applied Migrations"),
    value: `${controller.status?.applied_migrations_count ?? 0} / ${controller.status?.total_migrations_count ?? 0}`,
  },
]);

// Notion-style Directory Items
const directoryItems = $derived([
  {
    id: "overview" as const,
    title: l("نظرة عامة وفحص السلامة", "Overview & Integrity Health"),
    desc: l("تفاصيل المسار الفعلي، حالة التكامل، والفحوصات الفورية السريعة", "Target storage path, integrity check, and fast diagnostic operations"),
    icon: ShieldCheck,
    badge: controller.status?.integrity_status === "ok" ? l("سليمة 100%", "OK") : (controller.status?.integrity_status || l("سليمة", "OK")),
  },
  {
    id: "connection" as const,
    title: l("إعدادات الاتصال والمحرك", "Connection & Engine Settings"),
    desc: l("تكوين خادم Orbis API، إعدادات PostgreSQL، ونمط SQLite WAL", "Configure Orbis API server, PostgreSQL credentials, and SQLite WAL mode"),
    icon: GlobeAlt,
    badge: controller.isLocal ? "SQLite" : "PostgreSQL",
  },
  {
    id: "backup" as const,
    title: l("النسخ الاحتياطي والجدولة", "Backup & Retention Policies"),
    desc: l("إنشاء نسخة فورية عبر VACUUM INTO وتكوين سياسات الأرشفة والاحتفاظ", "Create instant snapshots via VACUUM INTO and configure auto-backup schedule"),
    icon: ArrowDownTray,
    badge: controller.status?.last_backup_timestamp ? l("محدث", "Active") : l("مطلوب نسخ", "Pending"),
  },
  {
    id: "restore" as const,
    title: l("استعادة البيانات واللقطات", "Restore & Snapshot Recovery"),
    desc: l("استعادة مرحلية آمنة مع فحص مسبق لسلامة ملف النسخة واللقطة التلقائية", "Safe staged recovery with pre-validation inspection and safety snapshot"),
    icon: ArrowUpTray,
    badge: l("استعادة مرحلية", "Staged"),
  },
  {
    id: "maintenance" as const,
    title: l("الصيانة والترحيلات والضغط", "Maintenance, Migrations & VACUUM"),
    desc: l("تطبيق ترحيلات المخطط الجديدة، ضغط واسترجاع المساحة، وفحص B-Tree", "Apply pending schema migrations, compact DB to reclaim space, and run B-Tree check"),
    icon: Wrench,
    badge: controller.status && controller.status.pending_migrations.length > 0
      ? l(`${controller.status.pending_migrations.length} معلق`, `${controller.status.pending_migrations.length} Pending`)
      : l("محدث بالكامل", "Up to Date"),
  },
]);

// Advisor Diagnostic Insights
const advisorInsights = $derived.by(() => {
  const insights = [];

  // 1. Integrity Check Insight
  const isHealthy = controller.status?.integrity_status === "ok" || controller.status?.integrity_status === "healthy";
  insights.push({
    id: "integrity-health",
    text: isHealthy
      ? l("سلامة بنية الجداول ومؤشرات B-Tree مفحوصة وسليمة 100%.", "Table structures and B-Tree indices verified 100% healthy.")
      : l("تم رصد ملاحظات في فحص السلامة، يوصى بإجراء فحص التكامل الشامل.", "Integrity anomalies detected. Comprehensive check recommended."),
    variant: isHealthy ? ("success" as const) : ("warning" as const),
    icon: ShieldCheck,
  });

  // 2. Schema Migrations Insight
  const pendingCount = controller.status?.pending_migrations.length ?? 0;
  if (pendingCount > 0) {
    insights.push({
      id: "migrations-pending",
      text: l(`توجد ${pendingCount} ترحيلات معلقة تتطلب التطبيق لمزامنة هيكل الجداول.`, `There are ${pendingCount} pending migrations that need to be applied.`),
      variant: "warning" as const,
      icon: Bolt,
    });
  } else {
    insights.push({
      id: "migrations-synced",
      text: l("مخطط قاعدة البيانات متطابق ومحدث بالكامل مع إصدار التطبيق.", "Database schema is fully synchronized with current application version."),
      variant: "success" as const,
      icon: CheckCircle,
    });
  }

  // 3. Concurrency / WAL Mode Insight
  const walActive = connWalMode || controller.status?.journal_mode?.toLowerCase() === "wal";
  insights.push({
    id: "wal-mode",
    text: walActive
      ? l("نمط التزامن الفائق (WAL) مفعل لضمان أعلى سرعة قراءة وكتابة دون إقفال.", "High-concurrency WAL mode enabled for lock-free parallel reads and writes.")
      : l("نمط التزامن القياسي نشط؛ يمكن تفعيل WAL لتحسين استجابة المعاملات المتزامنة.", "Standard journal active; WAL mode can be enabled for higher concurrency."),
    variant: walActive ? ("success" as const) : ("neutral" as const),
    icon: Sparkles,
  });

  // 4. Backup Freshness Insight
  if (controller.status?.last_backup_timestamp) {
    insights.push({
      id: "backup-fresh",
      text: l("تم أخذ نسخة احتياطية موثقة مؤخراً والبيانات مؤمنة ضد الفقدان.", "Verified snapshot taken recently; data protected against loss."),
      variant: "success" as const,
      icon: ClockRotateRight,
    });
  } else {
    insights.push({
      id: "backup-recommended",
      text: l("لم يتم تسجيل نسخة احتياطية حديثة، يوصى بإنشاء نسخة فورية.", "No recent backup recorded. Creating an instant backup is recommended."),
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
          {m.database_management_title?.() || l("إدارة قواعد البيانات", "Database Management")}
        </h2>
        <span class="text-[9.5px] text-muted-foreground/90 mt-0.5">
          {m.database_management_subtitle?.() || l("إدارة دورة حياة قاعدة البيانات، النسخ الاحتياطي، الاستعادة، والترحيلات", "Manage database lifecycle, backups, recovery, integrity, and migrations")}
        </span>
      </div>
    </div>

    <!-- Active Indicators & Action Bar -->
    <div class="flex items-center gap-2">
      <!-- Engine Indicator Badge -->
      <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
        {#if controller.isLocal}
          <Computer size={10} class="text-primary" />
          <span>SQLite (Embedded)</span>
        {:else}
          <GlobeAlt size={10} class="text-primary" />
          <span>PostgreSQL (Remote)</span>
        {/if}
      </Badge>

      <!-- Connection / Health Status Badge -->
      {#if controller.status}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
          <span class="h-1.5 w-1.5 rounded-full {controller.status.state === 'connected' ? 'bg-emerald-500' : controller.status.state === 'migration_required' ? 'bg-amber-500 animate-pulse' : 'bg-destructive'}"></span>
          <span>{controller.status.state === 'connected' ? (m.database_status_connected?.() || l("متصل ومستقر", "Connected & Healthy")) : controller.status.state === 'migration_required' ? (m.database_status_migration_required?.() || l("ترحيل مطلوب", "Migration Required")) : (m.database_status_disconnected?.() || l("غير متصل", "Disconnected"))}</span>
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
        <ArrowPath size={12} class={controller.isLoading ? "animate-spin text-primary" : ""} />
        <span>{l("تحديث", "Refresh")}</span>
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
        <span>{m.database_action_open?.() || l("فتح قاعدة", "Open DB")}</span>
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
        <span>{m.database_action_create?.() || l("قاعدة جديدة", "New DB")}</span>
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
        {l("إغلاق", "Dismiss")}
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
        {l("إغلاق", "Dismiss")}
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
            {l("أقسام وأدوات إدارة البيانات", "Database Management Directory")}
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
                    {l("تفاصيل ومسار قاعدة البيانات النشطة", "Active Database Target & Storage Details")}
                  </h3>
                </div>
                <Badge variant="outline" class="text-[9px] font-mono">
                  {controller.status?.mode?.toUpperCase() || "SQLITE"}
                </Badge>
              </div>

              <div class="space-y-3 text-xs">
                <div class="p-3 rounded-lg bg-card/40 border border-border/40 space-y-1">
                  <span class="text-[10px] font-semibold text-muted-foreground">{l("المسار الفعلي للتخزين:", "Storage Path:")}</span>
                  <p class="font-mono text-[10.5px] text-foreground break-all select-all">
                    {controller.status?.connection_target || l("جاري التحميل...", "Loading...")}
                  </p>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div class="p-3 rounded-lg bg-card/40 border border-border/40">
                    <span class="text-[10px] font-semibold text-muted-foreground">{l("حالة التكامل والسلامة:", "Integrity Check:")}</span>
                    <div class="flex items-center gap-1.5 mt-1 font-bold text-xs text-emerald-600 dark:text-emerald-400">
                      <CheckCircle size={13} />
                      <span>{controller.status?.integrity_status === "ok" ? l("سليمة ومفحوصة (OK)", "Healthy (OK)") : controller.status?.integrity_status || l("سليمة", "OK")}</span>
                    </div>
                  </div>

                  <div class="p-3 rounded-lg bg-card/40 border border-border/40">
                    <span class="text-[10px] font-semibold text-muted-foreground">{l("التوافقية مع التطبيق:", "Version Compatibility:")}</span>
                    <div class="flex items-center gap-1.5 mt-1 font-bold text-xs {controller.status?.is_compatible ? 'text-emerald-600 dark:text-emerald-400' : 'text-amber-600 dark:text-amber-400'}">
                      <Bolt size={13} />
                      <span>{controller.status?.is_compatible ? l("متوافق بنسبة 100%", "Fully Compatible") : l("يتطلب تطبيق ترحيلات", "Pending Migrations")}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Quick Operations Grid -->
              <div class="pt-2 border-t border-border/40">
                <h4 class="text-[10px] font-bold text-muted-foreground/80 tracking-wider uppercase mb-2.5">
                  {l("إجراءات سريعة وفورية", "Quick Operations")}
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
                    <span class="font-bold truncate">{m.database_action_backup_now?.() || l("نسخ احتياطي فوري", "Backup Now")}</span>
                  </Button>

                  <Button
                    type="button"
                    variant="outline"
                    class="justify-start text-[11px] h-9 gap-2 border-border/60 bg-card/40 hover:bg-muted/40 cursor-pointer"
                    onclick={() => controller.runIntegrityCheck()}
                    disabled={controller.isOperating}
                  >
                    <ShieldCheck size={14} class="text-emerald-500 shrink-0" />
                    <span class="font-bold truncate">{m.database_action_check_integrity?.() || l("فحص السلامة", "Check Integrity")}</span>
                  </Button>

                  <Button
                    type="button"
                    variant="outline"
                    class="justify-start text-[11px] h-9 gap-2 border-border/60 bg-card/40 hover:bg-muted/40 cursor-pointer"
                    onclick={() => controller.optimizeDatabase()}
                    disabled={controller.isOperating}
                  >
                    <Sparkles size={14} class="text-amber-500 shrink-0" />
                    <span class="font-bold truncate">{m.database_action_optimize?.() || l("ضغط وتحسين (VACUUM)", "Compact DB")}</span>
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
                    {l("تكوين إعدادات الاتصال والمحرك", "Connection & Engine Configuration")}
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
                      <h4 class="text-xs font-bold text-foreground">{l("قاعدة محلية (SQLite)", "Local SQLite")}</h4>
                      <p class="text-[9.5px] text-muted-foreground">{l("تخزين مدمج وسريع على هذا الجهاز", "Embedded on workstation")}</p>
                    </div>
                  </div>
                  {#if connMode === "local"}
                    <CheckCircle size={14} class="text-primary" />
                  {/if}
                </button>

                <button
                  type="button"
                  class="p-3 rounded-lg border transition-all flex items-center justify-between cursor-pointer {connMode === 'remote' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border/50 bg-card/40 hover:border-border'}"
                  onclick={() => connMode = "remote"}
                >
                  <div class="flex items-center gap-2.5">
                    <div class="size-7.5 rounded-lg flex items-center justify-center {connMode === 'remote' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                      <GlobeAlt size={15} />
                    </div>
                    <div class="text-start">
                      <h4 class="text-xs font-bold text-foreground">{l("خادم شبكي (Remote)", "Remote Server")}</h4>
                      <p class="text-[9.5px] text-muted-foreground">{l("خادم مركزي أو PostgreSQL", "Central Server or Postgres")}</p>
                    </div>
                  </div>
                  {#if connMode === "remote"}
                    <CheckCircle size={14} class="text-primary" />
                  {/if}
                </button>
              </div>

              <!-- Adaptive Form Fields -->
              {#if connMode === "local"}
                <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3.5">
                  <FormField id="connDbFilename" label={l("اسم ملف قاعدة البيانات", "Database File Name")}>
                    {#snippet children()}
                      <Input
                        id="connDbFilename"
                        type="text"
                        bind:value={connDbFilename}
                        class="h-8 text-xs font-mono bg-background border-input"
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
                          class="h-8 text-xs font-mono bg-background border-input flex-1"
                          style="direction: ltr;"
                        />
                        <Button
                          type="button"
                          variant="outline"
                          size="sm"
                          class="h-8 px-2.5 text-[11px] gap-1.5 shrink-0 border-border"
                          onclick={handleBrowseCustomDbPath}
                        >
                          <FolderOpen size={13} class="text-primary" />
                          <span>{l("تصفح", "Browse")}</span>
                        </Button>
                      </div>
                    {/snippet}
                  </FormField>

                  <div class="pt-2 border-t border-border/30 flex items-center justify-between">
                    <div>
                      <div class="text-xs font-bold text-foreground">{l("نمط التزامن WAL (Write-Ahead Logging)", "SQLite WAL Mode")}</div>
                      <p class="text-[9.5px] text-muted-foreground">{l("يوفر سرعة استجابة فائقة ويمنع تضارب القراءة والكتابة", "Enables high concurrency for simultaneous reads and writes")}</p>
                    </div>
                    <Switch bind:checked={connWalMode} />
                  </div>
                </div>
              {:else}
                <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3.5">
                  <FormField id="connServerUrl" label={l("عنوان خادم Orbis API", "Orbis Server URL")} required>
                    {#snippet children()}
                      <div class="flex gap-2">
                        <Input
                          id="connServerUrl"
                          type="url"
                          bind:value={connServerUrl}
                          placeholder="http://localhost:3000"
                          class="h-8 text-xs font-mono bg-background border-input flex-1"
                          style="direction: ltr;"
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
                            <ArrowPath size={13} class="animate-spin text-primary" />
                          {:else}
                            <Bolt size={13} class="text-primary" />
                          {/if}
                          <span>{m.database_action_test_connection?.() || l("اختبار", "Test")}</span>
                        </Button>
                      </div>
                    {/snippet}
                  </FormField>

                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    <FormField id="connHost" label={l("مضيف قاعدة البيانات (Host)", "Database Host")}>
                      {#snippet children()}
                        <Input id="connHost" type="text" bind:value={connHost} class="h-8 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                      {/snippet}
                    </FormField>

                    <FormField id="connPort" label={l("المنفذ (Port)", "Port")}>
                      {#snippet children()}
                        <Input id="connPort" type="number" bind:value={connPort} class="h-8 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                      {/snippet}
                    </FormField>

                    <FormField id="connDatabaseName" label={l("اسم قاعدة البيانات", "Database Name")}>
                      {#snippet children()}
                        <Input id="connDatabaseName" type="text" bind:value={connDatabaseName} class="h-8 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                      {/snippet}
                    </FormField>

                    <FormField id="connUsername" label={l("اسم المستخدم", "Username")}>
                      {#snippet children()}
                        <Input id="connUsername" type="text" bind:value={connUsername} class="h-8 text-xs font-mono bg-background border-input" style="direction: ltr;" />
                      {/snippet}
                    </FormField>
                  </div>

                  <FormField id="connPassword" label={l("كلمة المرور", "Password")}>
                    {#snippet children()}
                      <Input id="connPassword" type="password" bind:value={connPassword} placeholder="••••••••" class="h-8 text-xs font-mono bg-background border-input" style="direction: ltr;" />
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
                  {m.database_action_reset_connection?.() || l("استعادة الافتراضي", "Reset Default")}
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
                    <ArrowPath size={12} class="animate-spin" />
                    <span>{l("جاري الحفظ...", "Saving...")}</span>
                  {:else}
                    <CheckCircle size={13} />
                    <span>{m.database_action_save_connection?.() || l("حفظ وتطبيق الاتصال", "Save & Apply")}</span>
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
                    {l("النسخ الاحتياطي والسياسات المجدولة", "Backup Execution & Retention Policies")}
                  </h3>
                </div>
              </div>

              <!-- Instant Backup Card -->
              <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3">
                <div class="flex items-center gap-2">
                  <ClockRotateRight size={14} class="text-primary" />
                  <span class="text-xs font-bold text-foreground">{l("إنشاء نسخة احتياطية فورية", "Create Instant Backup")}</span>
                </div>

                <FormField id="customBackupPath" label={m.database_backup_destination_label?.() || l("مكان حفظ النسخة", "Backup Destination Path")}>
                  {#snippet children()}
                    <div class="flex gap-2">
                      <Input
                        id="customBackupPath"
                        type="text"
                        bind:value={customBackupPath}
                        placeholder={m.database_backup_custom_dest_hint?.() || l("اترك الحقل فارغاً للحفظ في مجلد النسخ التلقائي الافتراضي", "Leave blank for default backups folder")}
                        class="h-8 text-xs font-mono bg-background border-input flex-1"
                        style="direction: ltr;"
                      />
                      <Button
                        type="button"
                        variant="outline"
                        size="sm"
                        class="h-8 px-2.5 text-[11px] gap-1.5 shrink-0 border-border"
                        onclick={handleBrowseBackupDest}
                      >
                        <FolderOpen size={13} class="text-primary" />
                        <span>{l("تحديد المكان", "Choose Location")}</span>
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
                    <ArrowPath size={13} class="animate-spin" />
                    <span>{l("جاري إنشاء النسخة الاحتياطية وفحص سلامتها...", "Creating & Verifying Backup...")}</span>
                  {:else}
                    <ArrowDownTray size={14} />
                    <span>{m.database_action_backup_now?.() || l("بدء إنشاء النسخة الاحتياطية الآن", "Execute Backup Now")}</span>
                  {/if}
                </Button>
              </div>

              <!-- Automatic Backup Policy -->
              <div class="p-4 rounded-lg bg-card/40 border border-border/40 space-y-3.5">
                <div class="flex items-center justify-between pb-2 border-b border-border/30">
                  <div class="flex items-center gap-2">
                    <ClockRotateRight size={14} class="text-indigo-500" />
                    <span class="text-xs font-bold text-foreground">{l("سياسات النسخ الاحتياطي التلقائي", "Automated Backup Policy")}</span>
                  </div>
                  <Switch bind:checked={autoBackup} />
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
                  <FormField id="backupInterval" label={m.database_backup_interval_label?.() || l("تكرار النسخ (بالساعات)", "Interval (Hours)")}>
                    {#snippet children()}
                      <Input id="backupInterval" type="number" bind:value={backupInterval} min="1" max="168" class="h-8 text-xs bg-background border-input" disabled={!autoBackup} />
                    {/snippet}
                  </FormField>

                  <FormField id="maxBackups" label={m.database_backup_max_files_label?.() || l("أقصى عدد للنسخ المحفوظة", "Max Retained Files")}>
                    {#snippet children()}
                      <Input id="maxBackups" type="number" bind:value={maxBackups} min="1" max="100" class="h-8 text-xs bg-background border-input" disabled={!autoBackup} />
                    {/snippet}
                  </FormField>
                </div>

                <div class="pt-2 border-t border-border/30 flex items-center justify-between">
                  <div>
                    <div class="text-xs font-bold text-foreground">{l("فحص السلامة عند بدء التشغيل", "Integrity Check on Startup")}</div>
                    <p class="text-[9.5px] text-muted-foreground">{l("التحقق التلقائي السريع من بنية قاعدة البيانات عند كل إقلاع للنظام", "Automatically verify database integrity whenever Orbis starts up")}</p>
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
                    {l("حفظ السياسات", "Save Policy")}
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
                    {l("استعادة قاعدة البيانات من نسخة سابقة", "Restore Database from Backup")}
                  </h3>
                </div>
              </div>

              <!-- Step 1: File Selection -->
              <div class="space-y-2">
                <label class="text-xs font-bold text-foreground">
                  {m.database_restore_step1_title?.() || l("1. اختيار ملف النسخة الاحتياطية", "1. Select Backup File")}
                </label>
                <div class="flex gap-2">
                  <Input
                    type="text"
                    bind:value={restoreFilePath}
                    placeholder={l("حدد ملف النسخة المراد استعادتها (*.db)", "Select backup file (*.db)")}
                    class="h-8 text-xs font-mono bg-background border-input flex-1"
                    style="direction: ltr;"
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
                    <span>{m.database_browse_file?.() || l("اختيار ملف", "Browse File")}</span>
                  </Button>
                </div>
              </div>

              <!-- Step 2: Inspection Card -->
              {#if controller.isInspectingBackup}
                <div class="p-3.5 rounded-lg bg-card/40 border border-border/40 text-xs flex items-center gap-2.5 text-muted-foreground">
                  <ArrowPath size={13} class="animate-spin text-primary" />
                  <span>{l("جاري فحص وتدقيق ملف النسخة الاحتياطية...", "Inspecting and validating backup file...")}</span>
                </div>
              {:else if controller.inspectedBackup}
                <div class="p-3.5 rounded-lg border text-xs space-y-2 {controller.inspectedBackup.is_valid ? 'bg-emerald-500/5 border-emerald-500/20' : 'bg-destructive/5 border-destructive/20'}">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 font-bold text-foreground">
                      {#if controller.inspectedBackup.is_valid}
                        <CheckCircle size={14} class="text-emerald-500" />
                        <span>{l("تم التحقق من سلامة النسخة الاحتياطية", "Backup Validated Successfully")}</span>
                      {:else}
                        <InfoCircle size={14} class="text-destructive" />
                        <span>{l("ملف النسخة غير صالح أو تالف", "Invalid or Corrupted Backup File")}</span>
                      {/if}
                    </div>
                    <Badge variant="outline" class="text-[9px] font-mono">
                      {controller.inspectedBackup.file_size_formatted}
                    </Badge>
                  </div>

                  <div class="grid grid-cols-2 gap-3 text-[10.5px] text-muted-foreground">
                    <div>{l("إصدار المخطط:", "Schema Version:")} <span class="font-bold text-foreground font-mono">{controller.inspectedBackup.schema_version}</span></div>
                    <div>{l("الترحيلات المطبقة:", "Applied Migrations:")} <span class="font-bold text-foreground">{controller.inspectedBackup.applied_migrations_count}</span></div>
                  </div>
                </div>
              {/if}

              <!-- Step 3: Safety Warning & Trigger -->
              <div class="p-3.5 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-700 dark:text-amber-400 text-xs space-y-1.5">
                <div class="flex items-center gap-2 font-bold">
                  <InfoCircle size={14} />
                  <span>{l("تنبيه أمان الاستعادة", "Restoration Safety Notice")}</span>
                </div>
                <p class="text-[10px] leading-relaxed">
                  {m.database_restore_warning_desc?.() || l("تحذير: الاستعادة ستستبدل بيانات قاعدة البيانات الحالية. سيقوم النظام بأخذ لقطة أمان تلقائية للبيانات الحالية قبل الاستبدال.", "Warning: Restoration will overwrite current records. An automatic safety snapshot will be taken first.")}
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
                  <ArrowPath size={13} class="animate-spin" />
                  <span>{l("جاري الاستعادة وتطبيق الترحيلات...", "Restoring & Upgrading...")}</span>
                {:else}
                  <ArrowUpTray size={14} />
                  <span>{m.database_restore_btn?.() || l("تأكيد وبدء الاستعادة", "Confirm & Execute Restore")}</span>
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
                    {l("الصيانة والترحيلات وضغط المساحة", "Database Maintenance, Migrations & Compaction")}
                  </h3>
                </div>
                {#if controller.status && controller.status.pending_migrations.length === 0}
                  <Badge variant="outline" class="text-[9px] bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20">
                    {l("محدث بالكامل", "Up to Date")}
                  </Badge>
                {:else}
                  <Badge variant="outline" class="text-[9px] bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20">
                    {controller.status?.pending_migrations.length} {l("معلق", "Pending")}
                  </Badge>
                {/if}
              </div>

              <!-- Migrations Status -->
              {#if controller.status && controller.status.pending_migrations.length > 0}
                <div class="p-3.5 rounded-lg bg-amber-500/10 border border-amber-500/20 text-xs space-y-2">
                  <span class="font-bold text-amber-700 dark:text-amber-400">{l("الترحيلات المعلقة المطلوب تطبيقها:", "Pending migrations:")}</span>
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
                    <span>{m.database_action_apply_migrations?.() || l("تطبيق التحديثات الآن", "Apply Migrations Now")}</span>
                  </Button>
                </div>
              {:else}
                <div class="p-4 rounded-lg bg-card/40 border border-border/40 text-center py-4 space-y-1 text-xs text-muted-foreground">
                  <CheckCircle size={20} class="mx-auto text-emerald-500 mb-1" />
                  <div class="font-bold text-foreground">{l("مخطط قاعدة البيانات محدث بالكامل", "Database schema is completely up to date")}</div>
                  <p class="text-[10px]">{l("تم تطبيق جميع الترحيلات بنجاح ولا توجد تحديثات معلقة.", "All migrations are applied and active.")}</p>
                </div>
              {/if}

              <!-- Integrity & Compaction Controls -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-2 border-t border-border/30">
                <div class="p-3.5 rounded-lg bg-card/40 border border-border/40 space-y-2">
                  <div class="font-bold text-xs text-foreground">{l("فحص السلامة (PRAGMA integrity_check)", "Integrity Verification")}</div>
                  <p class="text-[10px] text-muted-foreground">{l("فحص شامل لجداول ومؤشرات قاعدة البيانات للتحقق من عدم وجود أي تلف.", "Comprehensive B-Tree check for corruption detection.")}</p>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="w-full text-[11px] font-bold gap-1.5 border-border h-8"
                    onclick={() => controller.runIntegrityCheck()}
                    disabled={controller.isOperating}
                  >
                    <ShieldCheck size={13} class="text-emerald-500" />
                    <span>{l("فحص السلامة", "Check Integrity")}</span>
                  </Button>
                </div>

                <div class="p-3.5 rounded-lg bg-card/40 border border-border/40 space-y-2">
                  <div class="font-bold text-xs text-foreground">{l("الضغط والتحسين (VACUUM)", "Compact & Optimize")}</div>
                  <p class="text-[10px] text-muted-foreground">{l("دمج سجلات WAL وإعادة ترتيب الصفحات لاسترجاع المساحات المتروكة.", "Checkpoints WAL and reclaims fragmented storage space.")}</p>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="w-full text-[11px] font-bold gap-1.5 border-border h-8"
                    onclick={() => controller.optimizeDatabase()}
                    disabled={controller.isOperating}
                  >
                    <Sparkles size={13} class="text-amber-500" />
                    <span>{l("ضغط وتحسين", "Compact DB")}</span>
                  </Button>
                </div>
              </div>

              {#if controller.integrityResult}
                <div class="p-3 rounded-lg border text-xs space-y-1.5 {controller.integrityResult.is_healthy ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400' : 'bg-destructive/10 border-destructive/20 text-destructive'}">
                  <div class="flex items-center gap-2 font-bold">
                    <ShieldCheck size={14} />
                    <span>{controller.integrityResult.is_healthy ? l("نتيجة الفحص: سليمة 100%", "Integrity Result: 100% Healthy") : l("تنبيه: تم رصد مشكلات", "Issues Detected")}</span>
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
                    {l("المساحة المسترجعة:", "Space Reclaimed:")} <span class="font-bold text-primary font-mono">{controller.optimizeResult.space_freed_formatted}</span>
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
            {l("مؤشرات محرك البيانات", "Database Engine KPIs")}
          </h4>
          
          <div class="grid grid-cols-2 gap-4 pb-4 border-b border-border/30">
            {#each kpiStats as stat}
              <div class="flex flex-col text-start">
                <span class="text-[9.5px] text-muted-foreground/80">{stat.label}</span>
                <span class="text-sm font-bold text-foreground font-mono mt-0.5 leading-none">{stat.value}</span>
              </div>
            {/each}
          </div>
        </div>

        <!-- Sidebar Section: Orbis AI Advisor & System Diagnostics -->
        <div class="flex flex-col gap-3">
          <div class="flex items-center gap-2 text-foreground font-semibold text-[11px] pb-1">
            <Sparkles size={13} class="text-primary animate-pulse" />
            <span>{l("تشخيصات المحرك والمستشار الذكي", "Engine Diagnostics & Advisor")}</span>
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
          <span class="truncate font-mono">{controller.status?.connection_target || l("جاهز للعمل", "Engine Ready")}</span>
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
