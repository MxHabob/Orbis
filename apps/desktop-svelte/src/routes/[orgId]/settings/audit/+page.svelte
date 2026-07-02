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
} from "@orbis/ui-svelte/table";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui-svelte/avatar";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui-svelte/dialog";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  History,
  Info,
  ShieldAlert,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
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
let actionFilter = $state("all");

// Pagination
let currentPage = $state(1);
const itemsPerPage = 8;

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

function getUserEmail(userId: string | null): string {
  if (!userId) return "system@orbis.local";
  const user = users.find((u) => u.id === userId);
  if (!user) return "system@orbis.local";
  return user.email || user.username || "system@orbis.local";
}

function getUserName(userId: string | null): string {
  if (!userId) return m.audit_user_system ? m.audit_user_system() : "System";
  const user = users.find((u) => u.id === userId);
  if (!user) return m.audit_user_system ? m.audit_user_system() : "System";
  return getLocale() === "ar" && user.name_ar ? user.name_ar : (user.full_name || user.username || "System");
}

function getUserAvatar(userId: string | null): string | undefined {
  if (!userId) return undefined;
  const user = users.find((u) => u.id === userId);
  return user?.avatar ? String(user.avatar) : undefined;
}

function getUserInitials(userId: string | null): string {
  if (!userId) return "S";
  const user = users.find((u) => u.id === userId);
  if (!user) return "U";
  const name = user.full_name || user.username || "U";
  return name.charAt(0).toUpperCase();
}

function formatAuditDate(dateStr: string): string {
  const d = new Date(dateStr);
  if (getLocale() === "ar") {
    return d.toLocaleDateString("ar-EG", { month: "short", day: "numeric" });
  }
  return d.toLocaleDateString("en-US", { month: "short", day: "numeric" });
}

function formatAuditTime(dateStr: string): string {
  const d = new Date(dateStr);
  const timeStr = d.toLocaleTimeString(getLocale() === "ar" ? "ar-EG" : "en-US", {
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  });
  return `${timeStr} UTC`;
}

// Reset page on filter change
$effect(() => {
  if (actionFilter) {
    currentPage = 1;
  }
});

const filteredLogs = $derived.by(() => {
  return auditLogs.filter((log) => {
    return actionFilter === "all" || String(log.action).toLowerCase() === actionFilter.toLowerCase();
  });
});

const uniqueActions = $derived.by(() => {
  const actions = auditLogs.map(log => log.action);
  return [...new Set(actions)].filter(Boolean);
});

const totalPages = $derived(Math.ceil(filteredLogs.length / itemsPerPage));
const paginatedLogs = $derived(
  filteredLogs.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
);

function openDetails(log: any) {
  selectedLog = log;
  isDetailsOpen = true;
}

function closeDetails() {
  selectedLog = null;
  isDetailsOpen = false;
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide font-sans">
  <!-- Search and Filtering Bar -->
  <div class="flex items-center justify-end gap-3 w-full select-none">
    <!-- Time Filter (Mock for UI visual symmetry) -->
    <div class="w-full sm:w-36 text-start">
      <Select type="single" value="all-time">
        <SelectTrigger class="h-9 w-full text-xs bg-background border-border rounded-[6px] border shadow-[0_2px_0_0_var(--color-border),_0_1px_3px_0_oklch(0_0_0_/_0.08)] transition-all hover:bg-muted/10 font-semibold text-foreground flex items-center justify-between px-3 cursor-pointer select-none outline-none">
          {m.audit_filter_time ? m.audit_filter_time() : "All time"}
        </SelectTrigger>
        <SelectContent class="bg-background border-border">
          <SelectItem value="all-time">{m.audit_filter_time ? m.audit_filter_time() : "All time"}</SelectItem>
        </SelectContent>
      </Select>
    </div>

    <!-- Action Filter Dropdown -->
    <div class="w-full sm:w-56 text-start">
      <Select type="single" value={actionFilter} onValueChange={(v) => (actionFilter = v ?? "all")}>
        <SelectTrigger class="h-9 w-full text-xs bg-background border-primary rounded-[6px] border-[1.5px] shadow-[0_2px_0_0_oklch(0.58_0.1_285_/_0.15),_0_1px_3px_0_oklch(0_0_0_/_0.08)] transition-all hover:bg-muted/10 font-semibold text-foreground flex items-center justify-between px-3 cursor-pointer select-none outline-none">
          {actionFilter === "all"
            ? (m.audit_filter_action ? m.audit_filter_action() : "Select Action:")
            : actionFilter}
        </SelectTrigger>
        <SelectContent class="bg-background border-border max-h-[300px] overflow-y-auto">
          <SelectItem value="all">{m.audit_filter_action ? m.audit_filter_action() : "Select Action:"}</SelectItem>
          {#each uniqueActions as act}
            <SelectItem value={act}>{act}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>
  </div>

  <!-- Main Grid/Table Content -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      <Skeleton class="h-10 w-full" />
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-12 w-full" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
      <p class="text-xs text-destructive font-semibold">Failed to load audit logs</p>
      <p class="text-xs text-muted-foreground">{error}</p>
      <Button variant="outline" class="text-xs" onclick={loadData}>Retry</Button>
    </div>
  {:else if filteredLogs.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10">
      <History size={48} class="text-muted-foreground/30" />
      <div class="flex flex-col gap-1 text-center">
        <p class="text-xs font-semibold">{m.audit_empty ? m.audit_empty() : "No audit logs found"}</p>
        <p class="text-[10px] text-muted-foreground/80">{m.audit_empty_desc ? m.audit_empty_desc() : "No operations have been recorded yet."}</p>
      </div>
    </div>
  {:else}
    <div class="border border-border rounded-sm overflow-hidden" in:fade={{ duration: 150 }}>
      <div class="overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow class="bg-muted border-b border-border/60">
              <TableHead class="text-start text-xs font-bold text-muted-foreground py-3 w-[40%]">
                {m.audit_col_member ? m.audit_col_member() : "MEMBER"}
              </TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground py-3 w-[30%]">
                {m.audit_col_action ? m.audit_col_action() : "ACTION"}
              </TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground py-3 w-[15%]">
                {m.audit_col_ip ? m.audit_col_ip() : "IP"}
              </TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground py-3 w-[15%]">
                {m.audit_col_time ? m.audit_col_time() : "TIME"}
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each paginatedLogs as log (log.id)}
              <TableRow
                class="hover:bg-accent/5 transition-colors duration-150 cursor-pointer"
                onclick={() => openDetails(log)}
              >
                <!-- MEMBER Column -->
                <TableCell class="py-3 text-start">
                  <div class="flex items-center gap-3">
                    <Avatar class="w-8 h-8">
                      <AvatarImage src={getUserAvatar(log.user_id)} alt="User" class="object-cover" />
                      <AvatarFallback class="text-xs">
                        {getUserInitials(log.user_id)}
                      </AvatarFallback>
                    </Avatar>
                    <div class="flex flex-col text-start">
                      <span class="text-xs font-bold text-foreground">
                        {getUserName(log.user_id)}
                      </span>
                      <span class="text-[9px] text-muted-foreground leading-none mt-0.5 truncate max-w-[320px]">
                        {getUserEmail(log.user_id)}
                      </span>
                    </div>
                  </div>
                </TableCell>

                <!-- ACTION Column -->
                <TableCell class="py-3 text-start">
                  <span class="font-mono text-xs text-foreground/80">
                    {log.action}
                  </span>
                </TableCell>

                <!-- IP Column -->
                <TableCell class="py-3 text-start">
                  <span class="font-mono text-xs text-muted-foreground">
                    {log.ip_address || "127.0.0.1"}
                  </span>
                </TableCell>

                <!-- TIME Column -->
                <TableCell class="py-3 text-start">
                  <div class="flex flex-col text-start">
                    <span class="text-xs font-semibold text-foreground">
                      {formatAuditDate(log.created_at)}
                    </span>
                    <span class="text-[9px] text-muted-foreground font-mono mt-0.5">
                      {formatAuditTime(log.created_at)}
                    </span>
                  </div>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>

    </div>

    <!-- Pagination -->
    {#if totalPages > 1}
      <div class="flex justify-end select-none mt-2">
        <div class="flex items-center bg-background border border-border rounded-[6px] p-0.5 shadow-[0_2px_0_0_var(--color-border),_0_1px_3px_0_oklch(0_0_0_/_0.05)]">
          <button
            disabled={currentPage === 1}
            onclick={() => currentPage--}
            class="h-7 w-7 rounded-[4px] hover:bg-muted/20 flex items-center justify-center cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed text-foreground transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" class="size-3.5 rtl:rotate-180"><polyline points="15 18 9 12 15 6"/></svg>
          </button>
          <div class="h-4 w-px bg-border/60 mx-0.5"></div>
          <button
            disabled={currentPage === totalPages}
            onclick={() => currentPage++}
            class="h-7 w-7 rounded-[4px] hover:bg-muted/20 flex items-center justify-center cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed text-foreground transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" class="size-3.5 rtl:rotate-180"><polyline points="9 18 15 12 9 6"/></svg>
          </button>
        </div>
      </div>
    {/if}
  {/if}
</div>

<!-- Details Dialog -->
{#if selectedLog}
  <Dialog open={isDetailsOpen} onOpenChange={(open: boolean) => { if (!open) closeDetails(); }}>
    <DialogContent class="max-w-2xl text-start font-sans">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Info size={18} class="text-primary" />
          <span>{m.audit_details_title ? m.audit_details_title() : "تفاصيل سجل التدقيق"}</span>
        </DialogTitle>
        <DialogDescription>
          {formatAuditDate(selectedLog.created_at)} {formatAuditTime(selectedLog.created_at)}
        </DialogDescription>
      </DialogHeader>

      <div class="grid grid-cols-2 gap-4 py-2 text-xs">
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">User:</span>
          <span class="text-foreground">{getUserEmail(selectedLog.user_id)}</span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground font-semibold">Action:</span>
          <div>
            <span class="text-xs text-foreground font-mono bg-muted/40 px-2 py-0.5 rounded-md border border-border/50">
              {selectedLog.action}
            </span>
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
    </DialogContent>
  </Dialog>
{/if}
