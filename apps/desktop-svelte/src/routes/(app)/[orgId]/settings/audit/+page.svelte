<!--
  @page AuditLogsPage
  Lists all audit logs with OrbisGrid, showing user actions, entity changes, and timestamps.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui-svelte/dialog";
import { History, Info } from "lucide-svelte";
import { l, isAr } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

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
      commands.auditLogs.getAll(data.orgId),
      commands.users.getUsers(data.orgId),
    ]);
    auditLogs = logsList.sort(
      (a, b) =>
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
    user: m.audit_col_member?.() || "Member",
    action: m.audit_col_action?.() || "Action",
    timestamp: m.audit_col_time?.() || "Time",
    ip: m.audit_col_ip?.() || "IP",
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
      {m.audit_title?.() || "Audit Logs"}
    </h1>
    <div class="flex items-center gap-2 flex-nowrap shrink-0">
      <div class="w-44 text-start">
        <Select type="single" value={actionFilter} onValueChange={(v) => (actionFilter = v ?? "all")}>
          <SelectTrigger class="h-8 w-full text-xs bg-card border-border/60">
            {actionFilter === "all" ? (m.audit_filter_action?.() || "All Actions") : actionFilter}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.audit_filter_action?.() || "All Actions"}</SelectItem>
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
      emptyTitle={m.audit_empty?.() || "No audit logs found"}
      emptyDescription={m.audit_empty_desc?.() || "No operations have been recorded yet."}
      rtl={getLocale() === "ar"}
      refreshable
      onRefresh={loadData}
    />
  </div>
</div>

<!-- Details Dialog -->
{#if selectedLog}
  <Dialog open={isDetailsOpen} onOpenChange={(open) => { if (!open) { selectedLog = null; isDetailsOpen = false; } }}>
    <DialogContent class="max-w-2xl text-start font-sans">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Info size={18} class="text-primary" />
          <span>{m.audit_details_title?.() || "Audit Log Details"}</span>
        </DialogTitle>
        <DialogDescription>
          {formatDate(selectedLog.created_at)}
        </DialogDescription>
      </DialogHeader>

      <div class="grid grid-cols-2 gap-4 py-2 text-xs">
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">User:</span>
          <span class="text-foreground">{getUserEmail(selectedLog.user_id)}</span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">Action:</span>
          <span class="font-mono bg-muted/40 px-2 py-0.5 rounded-md border border-border/50 inline-block">{selectedLog.action}</span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">Entity Type:</span>
          <span class="text-foreground font-semibold">{selectedLog.entity_type}</span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">Entity ID:</span>
          <span class="text-foreground font-mono">{selectedLog.entity_id}</span>
        </div>
        {#if selectedLog.ip_address}
          <div class="flex flex-col gap-1">
            <span class="text-muted-foreground font-semibold">IP Address:</span>
            <span class="text-foreground font-mono">{selectedLog.ip_address}</span>
          </div>
        {/if}
      </div>

      {#if selectedLog.metadata}
        <div class="flex flex-col gap-1.5 text-xs text-start border-t border-border pt-4">
          <span class="text-muted-foreground font-semibold">Metadata:</span>
          <pre class="bg-muted p-3 rounded-lg text-[10px] font-mono overflow-x-auto text-foreground whitespace-pre-wrap">{selectedLog.metadata}</pre>
        </div>
      {/if}
    </DialogContent>
  </Dialog>
{/if}
