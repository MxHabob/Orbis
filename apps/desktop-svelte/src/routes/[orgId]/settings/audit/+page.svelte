<!--
  @page AuditLogsPage
  Lists all audit logs in the system, showing user actions, entity changes, and timestamps.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  History,
  Search,
  Eye,
  Info,
  Calendar,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatDate } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let auditLogs = $state<any[]>([]);
let users = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Details Dialog
let selectedLog = $state<any | null>(null);
let isDetailsOpen = $state(false);

// Filters
let searchQuery = $state("");
let actionFilter = $state("all");

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [logsList, usersList] = await Promise.all([
      commands.auditLogs.getAll(data.orgId),
      commands.users.getUsers(data.orgId),
    ]);
    // Sort logs by date descending
    auditLogs = logsList.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
    users = usersList;
  } catch (e) {
    console.error("Error loading audit logs:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadData();
  }
});

function getUserName(userId: string | null): string {
  if (!userId) return "System";
  const user = users.find((u) => u.id === userId);
  if (!user) return userId.substring(0, 8);
  return user.full_name || user.username || userId.substring(0, 8);
}

function getActionColor(action: string): string {
  const act = String(action).toLowerCase();
  if (act.includes("create") || act.includes("insert") || act.includes("add")) {
    return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
  }
  if (act.includes("update") || act.includes("edit") || act.includes("modify")) {
    return "bg-blue-500/10 text-blue-500 border-blue-500/20";
  }
  if (act.includes("delete") || act.includes("remove") || act.includes("unlink") || act.includes("withdraw")) {
    return "bg-rose-500/10 text-rose-500 border-rose-500/20";
  }
  return "bg-purple-500/10 text-purple-500 border-purple-500/20";
}

const filteredLogs = $derived.by(() => {
  return auditLogs.filter((log) => {
    const userLabel = getUserName(log.user_id).toLowerCase();
    const textMatch =
      searchQuery.trim() === "" ||
      userLabel.includes(searchQuery.toLowerCase()) ||
      String(log.entity_type).toLowerCase().includes(searchQuery.toLowerCase()) ||
      String(log.action).toLowerCase().includes(searchQuery.toLowerCase()) ||
      String(log.metadata || "").toLowerCase().includes(searchQuery.toLowerCase());

    const actionMatch = actionFilter === "all" || String(log.action).toLowerCase() === actionFilter;

    return textMatch && actionMatch;
  });
});

function openDetails(log: any) {
  selectedLog = log;
  isDetailsOpen = true;
}

function closeDetails() {
  selectedLog = null;
  isDetailsOpen = false;
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <!-- Top header section -->
  <div class="flex items-center justify-between pb-6 select-none border-b border-border">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <History class="text-primary" size={20} />
        <span>{m.audit_title ? m.audit_title() : "سجل التدقيق"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.audit_desc ? m.audit_desc() : "سجل جميع العمليات والعمليات والتعديلات التي حدثت في النظام"}
      </p>
    </div>
  </div>

  <!-- Search & Filters -->
  <div class="flex items-center justify-between gap-4 py-4 select-none">
    <div class="relative flex-1 max-w-sm">
      <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
      <Input
        type="text"
        placeholder={m.search_placeholder ? m.search_placeholder() : "البحث بالمستخدم أو الكيان..."}
        bind:value={searchQuery}
        class="pl-9 h-9 text-xs"
      />
    </div>

    <div class="flex items-center gap-2">
      <select
        bind:value={actionFilter}
        class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none h-9 cursor-pointer"
      >
        <option value="all">{m.attendance_status_unknown() === "غير مححدد" ? "كل العمليات" : "All Actions"}</option>
        <option value="create">Create</option>
        <option value="update">Update</option>
        <option value="delete">Delete</option>
      </select>

      <Button variant="outline" size="sm" onclick={loadData} class="h-9 text-xs flex items-center gap-1 cursor-pointer">
        <span>{m.attendance_status_unknown() === "غير محدد" ? "تحديث" : "Refresh"}</span>
      </Button>
    </div>
  </div>

  <!-- Content Table -->
  <div class="flex-1 overflow-y-auto py-2">
    {#if isLoading}
      <div class="flex flex-col gap-3">
        {#each Array(5) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
    {:else if error}
      <div
        in:fade={{ duration: 150 }}
        class="border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start"
      >
        <ShieldAlert size={18} />
        <div>
          <span class="font-bold">Error:</span> {error}
        </div>
      </div>
    {:else if filteredLogs.length === 0}
      <div
        in:fade={{ duration: 150 }}
        class="border border-dashed border-border/80 rounded-2xl p-12 flex flex-col items-center justify-center text-center gap-3 select-none"
      >
        <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <History size={24} />
        </div>
        <h3 class="text-sm font-bold text-foreground">
          {m.audit_empty ? m.audit_empty() : "لا توجد سجلات تدقيق"}
        </h3>
        <p class="text-xs text-muted-foreground max-w-sm">
          {m.audit_empty_desc ? m.audit_empty_desc() : "لم يتم تسجيل أي عمليات بعد"}
        </p>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/30">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{m.audit_col_time ? m.audit_col_time() : "الوقت"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.audit_col_user ? m.audit_col_user() : "المستخدم"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.audit_col_action ? m.audit_col_action() : "الإجراء"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.audit_col_entity ? m.audit_col_entity() : "الكيان"}</TableHead>
              <TableHead class="text-start font-bold text-xs">{m.audit_col_details ? m.audit_col_details() : "التفاصيل"}</TableHead>
              <TableHead class="text-end font-bold text-xs w-[80px]"></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each filteredLogs as log (log.id)}
              <TableRow class="hover:bg-muted/20 transition-all duration-150">
                <TableCell class="text-start text-xs text-muted-foreground whitespace-nowrap">
                  {formatDate(log.created_at)}
                </TableCell>
                <TableCell class="text-start text-xs font-bold text-foreground">
                  {getUserName(log.user_id)}
                </TableCell>
                <TableCell class="text-start text-xs">
                  <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold border {getActionColor(log.action)}">
                    {log.action}
                  </span>
                </TableCell>
                <TableCell class="text-start text-xs">
                  <div class="flex flex-col text-start">
                    <span class="font-semibold text-foreground">{log.entity_type}</span>
                    <span class="text-[10px] text-muted-foreground font-mono">{log.entity_id.substring(0, 8)}...</span>
                  </div>
                </TableCell>
                <TableCell class="text-start text-xs text-muted-foreground line-clamp-1 max-w-[250px]">
                  {log.metadata || '-'}
                </TableCell>
                <TableCell class="text-end text-xs">
                  <Button variant="ghost" size="icon" class="h-8 w-8 cursor-pointer" onclick={() => openDetails(log)}>
                    <Eye size={13} />
                  </Button>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if}
  </div>
</div>

<!-- Details Modal -->
{#if isDetailsOpen && selectedLog}
  <div 
    class="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 select-none"
    onclick={closeDetails}
    aria-modal="true"
    role="dialog"
  >
    <div 
      class="bg-card border border-border rounded-xl shadow-lg w-full max-w-2xl flex flex-col max-h-[85vh]"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="p-6 border-b border-border text-start">
        <h3 class="text-lg font-bold text-foreground flex items-center gap-2">
          <Info size={18} class="text-primary" />
          <span>{m.audit_col_details ? m.audit_col_details() : "تفاصيل السجل"}</span>
        </h3>
        <p class="text-xs text-muted-foreground mt-1">
          {selectedLog.id}
        </p>
      </div>

      <div class="p-6 overflow-y-auto flex flex-col gap-4 text-start select-text">
        <!-- Grid fields -->
        <div class="grid grid-cols-2 gap-4 text-xs">
          <div>
            <span class="font-bold text-muted-foreground block mb-0.5">{m.audit_col_time ? m.audit_col_time() : "الوقت"}</span>
            <span class="text-foreground">{formatDate(selectedLog.created_at)}</span>
          </div>
          <div>
            <span class="font-bold text-muted-foreground block mb-0.5">{m.audit_col_user ? m.audit_col_user() : "المستخدم"}</span>
            <span class="text-foreground font-bold">{getUserName(selectedLog.user_id)}</span>
          </div>
          <div>
            <span class="font-bold text-muted-foreground block mb-0.5">{m.audit_col_action ? m.audit_col_action() : "الإجراء"}</span>
            <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold border {getActionColor(selectedLog.action)}">
              {selectedLog.action}
            </span>
          </div>
          <div>
            <span class="font-bold text-muted-foreground block mb-0.5">{m.audit_col_entity ? m.audit_col_entity() : "الكيان"}</span>
            <span class="text-foreground font-mono">{selectedLog.entity_type} ({selectedLog.entity_id})</span>
          </div>
          {#if selectedLog.ip_address}
            <div>
              <span class="font-bold text-muted-foreground block mb-0.5">IP Address</span>
              <span class="text-foreground font-mono">{selectedLog.ip_address}</span>
            </div>
          {/if}
          {#if selectedLog.user_agent}
            <div class="col-span-2">
              <span class="font-bold text-muted-foreground block mb-0.5">User Agent</span>
              <span class="text-foreground text-[10px] font-mono break-all">{selectedLog.user_agent}</span>
            </div>
          {/if}
        </div>

        <!-- Data changes -->
        {#if selectedLog.old_data || selectedLog.new_data}
          <div class="border-t border-border pt-4 flex flex-col gap-3">
            <span class="font-bold text-xs text-foreground block">
              {m.attendance_status_unknown() === "غير محدد" ? "تغييرات البيانات" : "Data Changes"}
            </span>

            <div class="grid grid-cols-2 gap-4">
              <!-- Old data -->
              <div class="flex flex-col gap-1">
                <span class="text-[10px] font-bold text-muted-foreground">
                  {m.attendance_status_unknown() === "غير محدد" ? "البيانات السابقة" : "Old Data"}
                </span>
                <pre class="bg-muted p-3 rounded-lg text-[10px] font-mono overflow-auto max-h-[200px] border border-border/40 select-all scrollbar-hide">
                  {selectedLog.old_data ? JSON.stringify(JSON.parse(selectedLog.old_data), null, 2) : "Null"}
                </pre>
              </div>

              <!-- New data -->
              <div class="flex flex-col gap-1">
                <span class="text-[10px] font-bold text-muted-foreground">
                  {m.attendance_status_unknown() === "غير محدد" ? "البيانات الجديدة" : "New Data"}
                </span>
                <pre class="bg-muted p-3 rounded-lg text-[10px] font-mono overflow-auto max-h-[200px] border border-border/40 select-all scrollbar-hide">
                  {selectedLog.new_data ? JSON.stringify(JSON.parse(selectedLog.new_data), null, 2) : "Null"}
                </pre>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <div class="p-4 border-t border-border flex justify-end">
        <Button variant="outline" size="sm" onclick={closeDetails} class="text-xs font-bold cursor-pointer">
          {m.ft_form_cancel ? m.ft_form_cancel() : "إغلاق"}
        </Button>
      </div>
    </div>
  </div>
{/if}
