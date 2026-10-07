<!--
  @component AuditLogViewer
  Authoritative System Audit Log & Security Trail Viewer.
  Uses TanStack Query and OrbisGrid for high-performance virtualized audit inspection.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { createQuery } from "@tanstack/svelte-query";
import { api, type AuditLogDto, type UserDto } from "$lib/api";
import { Refresh } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { l } from "$lib/core/i18n";
import { formatDate as fmtDate } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import AuditDetailsDialog from "./components/audit-details-dialog.svelte";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { auditFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

// Reactive TanStack Queries
const auditLogsQuery = createQuery(() => ({
  queryKey: ["audit-logs", orgId],
  queryFn: async () => {
    const logs = await api.listAuditLogs();
    return (logs || []).sort(
      (a: AuditLogDto, b: AuditLogDto) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
    );
  },
  enabled: !!orgId,
  staleTime: 60 * 1000,
}));

const usersQuery = createQuery(() => ({
  queryKey: ["users", orgId],
  queryFn: () => api.listUsers(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const auditLogs = $derived<AuditLogDto[]>(auditLogsQuery.data || []);
const users = $derived<UserDto[]>(usersQuery.data || []);
const usersById = $derived(
  new Map(users.map((user) => [String(user.id), user])),
);

const isLoading = $derived(auditLogsQuery.isLoading || usersQuery.isLoading);
const isFetching = $derived(auditLogsQuery.isFetching);

// Selected Log for Details Sheet/Dialog
let selectedLog = $state<AuditLogDto | null>(null);
let isDetailsOpen = $state(false);

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

function handleGridAction(event: GridActionEvent) {
  if (
    event.actionId === "dblclick" ||
    event.actionId === "click" ||
    event.actionId === "view" ||
    event.actionId === "edit"
  ) {
    selectedLog = event.row as unknown as AuditLogDto;
    isDetailsOpen = true;
  }
}

function getUserName(userId: string | null | undefined): string {
  if (!userId) return m.settings_ui_system();
  const user = usersById.get(String(userId));
  if (!user) return userId;
  return l(user.full_name_ar, user.full_name) || userId;
}

function getUserEmail(userId: string | null | undefined): string {
  if (!userId) return m.settings_ui_system_account_email();
  const user = usersById.get(String(userId));
  return user?.email || "system@orbis.internal";
}

function formatDate(dateStr: string): string {
  return fmtDate(dateStr, "datetime");
}

// Authoritative Grid Source
const gridSource = $derived(
  auditLogs.map((log, i) => ({
    ...log,
    __idx: i + 1,
    user_name: getUserName(log.user_id),
    formatted_created_at: formatDate(log.created_at),
    ip_address_display: log.ip_address || "127.0.0.1",
    entity_display: log.entity_type
      ? `${log.entity_type} ${log.entity_id ? `(#${log.entity_id.slice(0, 8)})` : ""}`
      : "—",
    details_summary:
      typeof log.details === "object" && log.details
        ? JSON.stringify(log.details)
        : log.metadata || "—",
  })),
);

// Columns directly reflecting backend audit structure
const columns = $derived<OrbisColumnDef[]>([
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    pin: "left",
    align: "center",
  },
  {
    id: "user_name",
    prop: "user_name",
    header: m.ui_copy_member_916bd8ec(),
    name: m.settings_ui_audit_user(),
    width: 190,
    size: 190,
    readonly: true,
    pin: "left",
  },
  {
    id: "action",
    prop: "action",
    header: m.ui_copy_action_b6f7953e(),
    name: m.settings_ui_audit_action(),
    width: 140,
    size: 140,
    type: "badge",
    align: "center",
    readonly: true,
  },
  {
    id: "entity_display",
    prop: "entity_display",
    header: m.ui_copy_target_entity_5e64064a(),
    name: m.settings_ui_audit_target_entity(),
    width: 180,
    size: 180,
    readonly: true,
    cellClass: "font-mono text-xs",
  },
  {
    id: "formatted_created_at",
    prop: "formatted_created_at",
    header: m.ui_copy_timestamp_0a682aad(),
    name: m.ui_copy_timestamp_0a682aad(),
    width: 160,
    size: 160,
    readonly: true,
    cellClass: "font-mono text-xs text-muted-foreground",
  },
  {
    id: "ip_address_display",
    prop: "ip_address_display",
    header: m.ui_copy_ip_address_36eeef84(),
    name: m.settings_ui_audit_ip_address(),
    width: 120,
    size: 120,
    readonly: true,
    cellClass: "font-mono text-xs text-muted-foreground",
  },
  {
    id: "details_summary",
    prop: "details_summary",
    header: m.ui_copy_log_details_a2799095(),
    name: m.common_audit_details_title(),
    width: 260,
    size: 260,
    readonly: true,
  },
]);
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide text-start">
  <!-- Header -->
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <div class="flex flex-col">
      <h1 class="text-lg font-bold text-foreground truncate">
        {m.common_nav_audit?.() || m.ui_copy_audit_logs_security_trail_64efdffc()}
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.ui_copy_immutable_record_of_system_operations_and__1591c0b6()}
      </p>
    </div>
    
    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={auditFilterFields}
      />
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={() => auditLogsQuery.refetch()}
        disabled={isFetching}
        class="text-muted-foreground hover:text-foreground"
        title={m.ui_copy_refresh_9d82f148()}
      >
        {#if isFetching}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
      </Button>
    </div>
  </div>

  <!-- OrbisGrid Container -->
  <div class="flex-1 min-h-0 rounded-xl border border-border/80 bg-background overflow-hidden">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      filter={activeFilter}
      emptyTitle={m.ui_copy_no_audit_logs_found_6a299836()}
      emptyDescription={m.ui_copy_no_operations_have_been_recorded_yet_a16f9eb7()}
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
