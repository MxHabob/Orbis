<!--
  @page AuditLogsPage
  Lists all audit logs with OrbisGrid, showing user actions, entity changes, and timestamps.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { History } from "@lucide/svelte";
import { l, isAr } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import AuditDetailsDialog from "$lib/features/settings/components/audit-details-dialog.svelte";

import { DataTable } from "$lib/tables";
import { auditLogs as auditDef, withIndex } from "$lib/tables/definitions";

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────
let auditLogs = $state<any[]>([]);
let users = $state<any[]>([]);
let isLoading = $state(true);
let actionFilter = $state("all");
let selectedLog = $state<any | null>(null);
let isDetailsOpen = $state(false);

// ── Data Loading ────────────────────────────────────────────────────────

async function loadData() {
  isLoading = true;
  try {
    const [logsList, usersList] = await Promise.all([
      commands.iam.listAuditLogs(),
      commands.iam.listUsers(),
    ]);
    auditLogs = logsList.sort(
      (a: any, b: any) =>
        new Date(b.created_at as string).getTime() -
        new Date(a.created_at as string).getTime(),
    );
    users = usersList;
  } catch (e) {
    console.error("Error loading audit logs:", e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadData();
});

// ── Helpers ─────────────────────────────────────────────────────────────

function getUserName(userId: string | null): string {
  if (!userId) return "System";
  const user = users.find((u) => u.id === userId);
  if (!user) return "System";
  return l(user.name_ar, user.full_name) || user.username || "System";
}

function getUserEmail(userId: string | null): string {
  if (!userId) return "system@orbis.local";
  const user = users.find((u) => u.id === userId);
  return user?.email || "system@orbis.local";
}

function formatDate(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleDateString(isAr() ? "ar-EG" : "en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

// ── Filtering ───────────────────────────────────────────────────────────

const uniqueActions = $derived([
  ...new Set(auditLogs.map((l) => l.action).filter(Boolean)),
]);

const filteredLogs = $derived(
  auditLogs.filter(
    (log) => actionFilter === "all" || log.action === actionFilter,
  ),
);

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    filteredLogs.map((log) => ({
      ...log,
      user_name: getUserName(log.user_id),
      created_at: formatDate(log.created_at),
      ip_address: log.ip_address || "127.0.0.1",
      details: log.entity_type
        ? `${log.entity_type} · ${log.entity_id?.slice(0, 8) || ""}`
        : "",
    })),
  ),
);

const columns = $derived(
  auditDef.columns({
    idx: "#",
    user: m.common_audit_col_member?.() || "Member",
    action: m.common_audit_col_action?.() || "Action",
    timestamp: m.common_audit_col_time?.() || "Time",
    ip: m.common_audit_col_ip?.() || "IP",
    details: "Details",
    entity: "Entity",
  }),
);

// ── Details Dialog ──────────────────────────────────────────────────────

function openDetails(log: any) {
  // Find original log by matching index
  const original = filteredLogs.find((l) => l.id === log.id);
  if (original) {
    selectedLog = original;
    isDetailsOpen = true;
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide">
  <!-- Header -->
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <h1 class="text-lg font-bold text-foreground truncate">
      {m.common_nav_audit?.() || "Audit Logs"}
    </h1>
    <div class="flex items-center gap-2 flex-nowrap shrink-0">
      <div class="w-44 text-start">
        <Select type="single" value={actionFilter} onValueChange={(v) => (actionFilter = v ?? "all")}>
          <SelectTrigger class="h-8 w-full text-xs bg-card border-border/60">
            {actionFilter === "all" ? (m.common_audit_filter_action?.() || "All Actions") : actionFilter}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.common_audit_filter_action?.() || "All Actions"}</SelectItem>
            {#each uniqueActions as act}
              <SelectItem value={act}>{act}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>
  </div>

  <!-- DataTable -->
  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      emptyIcon={History}
      emptyTitle={m.common_audit_empty?.() || "No audit logs found"}
      emptyDescription={m.common_audit_empty_desc?.() || "No operations have been recorded yet."}
      rtl={getLocale() === "ar"}
      refreshable
      onRefresh={loadData}
    />
  </div>
</div>

<AuditDetailsDialog bind:open={isDetailsOpen} log={selectedLog} userEmail={selectedLog ? getUserEmail(selectedLog.user_id) : ""} formatDate={formatDate} onclose={() => { selectedLog = null; isDetailsOpen = false; }} />
