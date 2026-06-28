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
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui-svelte/select";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  History,
  Search,
  Eye,
  Info,
  Calendar,
  ShieldAlert,
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
    auditLogs = logsList.sort(
      (a, b) =>
        new Date(b.created_at as string).getTime() -
        new Date(a.created_at as string).getTime(),
    );
    users = usersList;
  } catch (e) {
    console.error("Error loading audit logs:", e);
    error = (e as any)?.message || "Unknown error";
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

function getActionVariant(action: string): "default" | "lime" | "destructive" | "secondary" | "outline" | "brand" | "ghost" {
  const act = String(action).toLowerCase();
  if (act.includes("create") || act.includes("insert") || act.includes("add")) {
    return "lime";
  }
  if (
    act.includes("delete") ||
    act.includes("remove") ||
    act.includes("unlink") ||
    act.includes("withdraw")
  ) {
    return "destructive";
  }
  return "secondary";
}

const filteredLogs = $derived.by(() => {
  return auditLogs.filter((log) => {
    const userLabel = getUserName(log.user_id).toLowerCase();
    const textMatch =
      searchQuery.trim() === "" ||
      userLabel.includes(searchQuery.toLowerCase()) ||
      String(log.entity_type)
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) ||
      String(log.action).toLowerCase().includes(searchQuery.toLowerCase()) ||
      String(log.metadata || "")
        .toLowerCase()
        .includes(searchQuery.toLowerCase());

    const actionMatch =
      actionFilter === "all" ||
      String(log.action).toLowerCase() === actionFilter;

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
      <div class="w-36 text-start">
        <Select type="single" value={actionFilter} onValueChange={(v) => (actionFilter = v ?? "all")}>
          <SelectTrigger class="h-9 w-full text-xs">
            {#if actionFilter === "all"}
              {m.attendance_status_unknown() === "غير محدد" ? "كل العمليات" : "All Actions"}
            {:else}
              {actionFilter.toUpperCase()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.attendance_status_unknown() === "غير محدد" ? "كل العمليات" : "All Actions"}</SelectItem>
            <SelectItem value="create">Create</SelectItem>
            <SelectItem value="update">Update</SelectItem>
            <SelectItem value="delete">Delete</SelectItem>
          </SelectContent>
        </Select>
      </div>

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
      <div in:fade={{ duration: 150 }} class="overflow-hidden">
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
                  <Badge variant={getActionVariant(log.action)} class="text-[10px] font-semibold py-0">
                    {log.action}
                  </Badge>
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
                <TableCell class="text-end">
                  <Button variant="ghost" size="icon-sm" onclick={() => openDetails(log)}>
                    <Eye size={14} />
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

<!-- Details Dialog -->
{#if selectedLog}
  <Dialog open={isDetailsOpen} onOpenChange={(open) => { if (!open) closeDetails(); }}>
    <DialogContent class="max-w-2xl text-start font-sans">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Info size={18} class="text-primary" />
          <span>{m.audit_details_title ? m.audit_details_title() : "تفاصيل سجل التدقيق"}</span>
        </DialogTitle>
        <DialogDescription>
          {formatDate(selectedLog.created_at)}
        </DialogDescription>
      </DialogHeader>

      <div class="grid grid-cols-2 gap-4 py-2 text-xs">
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">User:</span>
          <span class="text-foreground">{getUserName(selectedLog.user_id)}</span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">Action:</span>
          <div>
            <Badge variant={getActionVariant(selectedLog.action)} class="text-[9px] font-bold">
              {selectedLog.action}
            </Badge>
          </div>
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
        {#if selectedLog.user_agent}
          <div class="flex flex-col gap-1 col-span-2">
            <span class="text-muted-foreground font-semibold">User Agent:</span>
            <span class="text-foreground text-[10px] leading-relaxed">{selectedLog.user_agent}</span>
          </div>
        {/if}
      </div>

      {#if selectedLog.metadata}
        <div class="flex flex-col gap-1.5 text-xs text-start border-t border-border pt-4">
          <span class="text-muted-foreground font-semibold">Metadata / Changes:</span>
          <pre class="bg-muted p-3 rounded-lg text-[10px] font-mono overflow-x-auto text-foreground whitespace-pre-wrap leading-normal">
            {selectedLog.metadata}
          </pre>
        </div>
      {/if}

      <DialogFooter class="border-t border-border pt-4">
        <Button variant="ghost" size="sm" onclick={closeDetails}>
          {m.attendance_status_unknown() === "غير محدد" ? "إغلاق" : "Close"}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
{/if}
