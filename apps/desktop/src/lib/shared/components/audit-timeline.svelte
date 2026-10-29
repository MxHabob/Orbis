<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { api } from "$lib/api/client";
import type { AuditLogDto, UserDto } from "$lib/api/types";
import { formatDate, l } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import { orgContext } from "$lib/stores";

let {
  entityType,
  entityId,
  maxHeight = "260px",
}: {
  entityType: string;
  entityId: string;
  maxHeight?: string;
} = $props();

let expandedAccordion = $state<string | undefined>("audit-history");

const auditQuery = createQuery(() => ({
  queryKey: ["entity-audit", orgContext.orgId, entityType, entityId],
  queryFn: async (): Promise<AuditLogDto[]> => {
    const logs = await api.getEntityAudit(entityType, entityId, 50, 0);
    return [...logs].sort(
      (a, b) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
    );
  },
  enabled: Boolean(orgContext.orgId && entityId),
}));

const usersQuery = createQuery(() => ({
  queryKey: ["users", orgContext.orgId],
  queryFn: () => api.listUsers(),
  enabled: Boolean(orgContext.orgId),
  staleTime: 5 * 60 * 1000,
}));

const logs = $derived(auditQuery.data ?? []);
const users = $derived<UserDto[]>(usersQuery.data ?? []);
const usersById = $derived(new Map(users.map((user) => [String(user.id), user])));
const cardUsers = $derived(
  logs
    .map((log) => getActorName(log.user_id))
    .filter((name) => !!name && name !== m.shared_ui_system_actor())
    .slice(0, 3),
);
const isLoading = $derived(Boolean(orgContext.orgId && entityId && auditQuery.isLoading));
const errorMessage = $derived(
  auditQuery.error instanceof Error
    ? auditQuery.error.message
    : m.shared_ui_audit_load_failed(),
);

function getActorName(userId: string | null | undefined): string {
  if (!userId) return m.shared_ui_system_actor();
  const user = usersById.get(String(userId));
  if (!user) return userId;
  return l(user.full_name_ar, user.full_name) || userId;
}

function formatActionLabel(action: string): string {
  const map: Record<string, string> = {
    "student.created": m.shared_ui_audit_student_created(),
    "student.updated": m.shared_ui_audit_student_updated(),
    "student.status_changed": m.shared_ui_audit_student_status_changed(),
    "employee.created": m.shared_ui_audit_employee_created(),
    "employee.updated": m.shared_ui_audit_employee_updated(),
    "employee.status_changed": m.shared_ui_audit_employee_status_changed(),
    "journal_entry.posted": m.shared_ui_audit_journal_posted(),
    "payment.received": m.shared_ui_audit_payment_received(),
    "grade.recorded": m.shared_ui_audit_grade_recorded(),
    "attendance.marked": m.shared_ui_audit_attendance_marked(),
    "group.created": m.shared_ui_audit_group_created(),
    "group.updated": m.shared_ui_audit_group_updated(),
    "user_group.assigned": m.shared_ui_audit_group_assigned(),
  };
  return map[action] || action;
}

function getDetailsContent(log: AuditLogDto): string | null {
  if (log.details != null) {
    if (typeof log.details === "string") return log.details;
    return JSON.stringify(log.details, null, 2) ?? null;
  }
  return log.metadata ?? null;
}
</script>

<Card aria-label={m.shared_ui_audit_trail_title()} class="text-card-foreground">
  <Accordion type="single" bind:value={expandedAccordion} collapsible class="w-full">
    <AccordionItem value="audit-history" class="border-0">
      <CardHeader class="px-4 py-3 normal-case tracking-normal text-foreground">
        <AccordionTrigger class="flex min-w-0 flex-1 items-center justify-between gap-3 rounded-none px-0 py-0 text-left text-sm text-foreground hover:no-underline">
          <div class="flex min-w-0 items-center gap-3">
            <div class="flex shrink-0 items-center -space-x-2">
              {#each cardUsers as userName, index (userName + '-' + index)}
                <span class="flex size-7 items-center justify-center rounded-full border border-background bg-muted text-[10px] font-semibold text-muted-foreground">
                  {userName.slice(0, 1).toUpperCase()}
                </span>
              {/each}
            </div>
            <CardTitle class="text-sm font-semibold text-foreground normal-case tracking-normal">
              {m.shared_ui_audit_trail_title()}
            </CardTitle>
          </div>
          {#if auditQuery.isSuccess}
            <span class="rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium text-muted-foreground">
              {logs.length}
            </span>
          {/if}
        </AccordionTrigger>
      </CardHeader>

      <AccordionContent>
        <CardContent class="overflow-y-auto px-0 py-0" style="max-height: {maxHeight};">
          {#if isLoading}
            <div class="space-y-3 px-6 py-4">
              {#each [1, 2, 3] as item (item)}
                <div class="space-y-2 py-2">
                  <div class="h-4 w-2/5 rounded bg-muted/40"></div>
                  <div class="h-3 w-1/4 rounded bg-muted/30"></div>
                </div>
              {/each}
            </div>
          {:else if auditQuery.isError}
            <div class="flex items-center justify-between gap-3 px-6 py-4">
              <p class="text-sm text-destructive">{errorMessage}</p>
              <Button variant="outline" size="sm" onclick={() => auditQuery.refetch()} disabled={auditQuery.isFetching}>
                {m.common_retry()}
              </Button>
            </div>
          {:else if logs.length === 0}
            <div class="flex min-h-[120px] items-center justify-center py-5 text-center text-sm text-muted-foreground">
              {m.shared_ui_no_audit_history()}
            </div>
          {:else}
            <div class="divide-y divide-border">
              {#each logs as log (log.id)}
                {@const detailsContent = getDetailsContent(log)}
                <div class="px-6 py-4">
                  <div class="flex items-start justify-between gap-3">
                    <p class="truncate text-sm font-semibold text-foreground">
                      {formatActionLabel(log.action)}
                    </p>
                    <time datetime={log.created_at} class="shrink-0 text-xs text-muted-foreground">
                      {formatDate(log.created_at)}
                    </time>
                  </div>

                  <div class="mt-1 text-xs text-muted-foreground">
                    <span class="truncate">{m.shared_ui_by()} {getActorName(log.user_id)}</span>
                  </div>

                  {#if detailsContent}
                    <div class="mt-2 border-t border-border pt-2">
                      <pre class="max-h-28 overflow-auto text-[11px] leading-relaxed whitespace-pre-wrap break-words text-muted-foreground"><code>{detailsContent}</code></pre>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </CardContent>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</Card>
