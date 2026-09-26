<script lang="ts">
import { onMount } from "svelte";
import { api } from "$lib/api/client";
import type { AuditLogDto } from "$lib/api/types";
import { orgContext } from "$lib/stores";
import { Skeleton } from "@orbis/ui/skeleton";
import { Button } from "@orbis/ui/button";
import * as m from "$lib/paraglide/messages";
    import { formatDate } from "$lib/core/i18n";

let {
  entityType,
  entityId,
  maxHeight = "400px",
}: {
  entityType: string;
  entityId: string;
  maxHeight?: string;
} = $props();

let loading = $state(true);
let error = $state<string | null>(null);
let logs = $state<AuditLogDto[]>([]);

async function loadAuditHistory() {
  if (!entityId) {
    loading = false;
    logs = [];
    return;
  }
  loading = true;
  error = null;
  try {
    logs = await api.getEntityAudit(entityType, entityId, 50, 0);
  } catch (err: any) {
    error = err?.message || "فشل تحميل سجل التدقيق / Failed to load audit logs";
  } finally {
    loading = false;
  }
}

onMount(() => {
  loadAuditHistory();
});

$effect(() => {
  if (entityId) {
    loadAuditHistory();
  }
});

function formatActionLabel(action: string): string {
  const map: Record<string, string> = {
    "student.created": "إنشاء طالب / Student Created",
    "student.updated": "تعديل طالب / Student Updated",
    "student.status_changed": "تغيير حالة الطالب / Status Changed",
    "employee.created": "إنشاء موظف / Employee Created",
    "employee.updated": "تعديل موظف / Employee Updated",
    "employee.status_changed": "تغيير حالة الموظف / Status Changed",
    "journal_entry.posted": "ترحيل قيد يومية / Journal Posted",
    "payment.received": "استلام دفعة مالية / Payment Received",
    "grade.recorded": "رصد درجة / Grade Recorded",
    "attendance.marked": "تسجيل حضور / Attendance Marked",
    "group.created": "إنشاء مجموعة / Group Created",
    "group.updated": "تعديل مجموعة / Group Updated",
    "user_group.assigned": "إسناد لمجموعة / Group Assigned",
  };
  return map[action] || action;
}

function getActionColor(action: string): string {
  if (action.includes("created") || action.includes("assigned") || action.includes("received")) {
    return "bg-emerald-500/10 text-emerald-600 border-emerald-500/20";
  }
  if (action.includes("deleted") || action.includes("reversed") || action.includes("removed")) {
    return "bg-destructive/10 text-destructive border-destructive/20";
  }
  if (action.includes("posted") || action.includes("status")) {
    return "bg-blue-500/10 text-blue-600 border-blue-500/20";
  }
  return "bg-muted text-muted-foreground border-border";
}
</script>

<div class="flex flex-col gap-3 p-3 bg-card rounded-lg border border-border/60 text-card-foreground">
  <div class="flex items-center justify-between border-b border-border/40 pb-2">
    <div class="flex items-center gap-2">
      <span class="text-sm font-semibold flex items-center gap-1.5">
        <span class="text-primary">📋</span>
        <span>سجل التدقيق والتعديلات (Audit Trail)</span>
      </span>
      <span class="text-xs px-2 py-0.5 rounded-full bg-muted text-muted-foreground font-mono">
        {logs.length}
      </span>
    </div>
    <Button
      variant="ghost"
      size="sm"
      class="h-7 text-xs px-2 text-muted-foreground hover:text-foreground"
      onclick={loadAuditHistory}
      disabled={loading}
    >
      {loading ? "جاري التحديث..." : "تحديث"}
    </Button>
  </div>

  <div class="overflow-y-auto pr-1 flex flex-col gap-3" style="max-height: {maxHeight};">
    {#if loading}
      <div class="flex flex-col gap-2.5 py-2">
        <Skeleton class="h-10 w-full rounded" />
        <Skeleton class="h-10 w-full rounded" />
        <Skeleton class="h-10 w-full rounded" />
      </div>
    {:else if error}
      <div class="p-3 rounded-md bg-destructive/10 text-destructive text-xs flex items-center justify-between">
        <span>{error}</span>
        <Button variant="outline" size="sm" class="h-6 text-[10px]" onclick={loadAuditHistory}>
          إعادة المحاولة
        </Button>
      </div>
    {:else if logs.length === 0}
      <div class="py-8 text-center text-xs text-muted-foreground flex flex-col items-center gap-1.5">
        <span class="text-xl opacity-60">📑</span>
        <span>لا يوجد سجل عمليات مسجل لهذا العنصر حتى الآن</span>
      </div>
    {:else}
      <div class="relative border-s-2 border-border/80 ms-2.5 flex flex-col gap-4 py-1">
        {#each logs as log (log.id)}
          <div class="relative ms-4 flex flex-col gap-1 text-xs">
            <!-- Bullet dot -->
            <div class="absolute -inset-s-5.75 top-1 h-3 w-3 rounded-full border-2 border-background bg-primary"></div>

            <div class="flex items-center gap-2 flex-wrap">
              <span class="px-2 py-0.5 rounded text-[10px] font-semibold border {getActionColor(log.action)}">
                {formatActionLabel(log.action)}
              </span>
              <span class="text-[11px] text-muted-foreground font-mono">
                {formatDate(log.created_at)}
              </span>
            </div>

            <div class="flex items-center gap-2 text-[11px] text-muted-foreground">
              <span>بواسطة:</span>
              <span class="font-medium text-foreground">
                {log.user_id ? log.user_id : "النظام (System / Autopilot)"}
              </span>
              {#if log.ip_address}
                <span class="text-[10px] opacity-70">({log.ip_address})</span>
              {/if}
            </div>

            {#if log.details && Object.keys(log.details).length > 0}
              <div class="mt-1 p-2 rounded bg-muted/50 border border-border/40 font-mono text-[10px] text-muted-foreground overflow-x-auto">
                <pre class="whitespace-pre-wrap">{JSON.stringify(log.details, null, 2)}</pre>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
