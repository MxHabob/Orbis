<!--
@component AuditLogViewer
Lists all audit logs with OrbisGrid, showing user actions, entity changes, and timestamps.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { ClockRotateRight, Refresh } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { l, isAr } from "$lib/core/i18n";
import { formatDate as fmtDate } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import AuditDetailsDialog from "./components/audit-details-dialog.svelte";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { auditFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

// State
let auditLogs = $state<any[]>([]);
let users = $state<any[]>([]);
const usersById = $derived(
  new Map(users.map((user) => [String(user.id), user])),
);
let isLoading = $state(true);
let selectedLog = $state<any | null>(null);
let isDetailsOpen = $state(false);

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

async function loadData() {
  isLoading = true;
  try {
    const [logsList, usersList] = await Promise.all([
      api.listAuditLogs(),
      api.listUsers().catch(() => []),
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
  if (orgId) loadData();
});

function handleGridAction(event: any) {
  if (event.actionId === "dblclick" || event.actionId === "click" || event.actionId === "view" || event.actionId === "edit") {
    selectedLog = event.row;
    isDetailsOpen = true;
  }
}

function getUserName(userId: string | null): string {
  if (!userId) return "System";
  const user = usersById.get(String(userId));
  if (!user) return "System";
  return l(user.name_ar, user.full_name) || user.username || "System";
}

function getUserEmail(userId: string | null): string {
  if (!userId) return "system@orbis.local";
  const user = usersById.get(String(userId));
  return user?.email || "system@orbis.local";
}

function formatDate(dateStr: string): string {
  return fmtDate(dateStr, "datetime", { month: "short", day: "numeric" });
}

const gridSource = $derived(
  auditLogs.map((log, i) => ({
    ...log,
    __idx: i + 1,
    user_name: getUserName(log.user_id),
    created_at: formatDate(log.created_at),
    ip_address: log.ip_address || "127.0.0.1",
    details: log.entity_type
      ? `${log.entity_type} · ${log.entity_id?.slice(0, 8) || ""}`
      : "",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "user_name", prop: "user_name", header: m.common_audit_col_member?.() || l("المستخدم", "Member"), name: "Member", width: 180, size: 180, type: "avatar", readonly: true, pin: "left" },
  { id: "action", prop: "action", header: m.common_audit_col_action?.() || l("العملية", "Action"), name: "Action", width: 140, size: 140, type: "badge", align: "center", readonly: true },
  { id: "created_at", prop: "created_at", header: l("الوقت", "Time"), name: "Time", width: 160, size: 160, readonly: true, cellClass: "font-mono text-muted-foreground" },
  { id: "ip_address", prop: "ip_address", header: l("عنوان IP", "IP"), name: "IP", width: 120, size: 120, readonly: true, cellClass: "font-mono text-muted-foreground" },
  { id: "details", prop: "details", header: l("التفاصيل", "Details"), name: "Details", width: 220, size: 220, readonly: true },
];
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide">
  <!-- Header -->
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <h1 class="text-lg font-bold text-foreground truncate">
      {m.common_nav_audit?.() || l("سجل التدقيق", "Audit Logs")}
    </h1>
    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={auditFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
    </div>
  </div>

  <!-- OrbisGrid -->
  <div class="flex-1 min-h-0">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      filter={activeFilter}
      emptyTitle={l("لا توجد سجلات تدقيق", "No audit logs found")}
      emptyDescription={l("لم يتم تسجيل أي عمليات بعد.", "No operations have been recorded yet.")}
      rtl={getLocale() === "ar"}
      readonly
      onaction={handleGridAction}
    />
  </div>
</div>

<AuditDetailsDialog
  bind:open={isDetailsOpen}
  log={selectedLog}
  userEmail={selectedLog ? getUserEmail(selectedLog.user_id) : ""}
  {formatDate}
  onclose={() => { selectedLog = null; isDetailsOpen = false; }}
/>
