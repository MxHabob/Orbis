<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
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

let accordionValue = $state<string | undefined>("activity");

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
const usersById = $derived(
  new Map(users.map((user) => [String(user.id), user])),
);
const isLoading = $derived(
  Boolean(orgContext.orgId && entityId && auditQuery.isLoading),
);
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
</script>

<Accordion type="single" bind:value={accordionValue} collapsible class="w-full">
  <AccordionItem value="activity" class="border-0">
    <AccordionTrigger class="py-2 text-sm font-semibold text-foreground hover:no-underline">
      {m.shared_ui_audit_trail_title()}
    </AccordionTrigger>
    <AccordionContent>
      <section class="mx-3" aria-label={m.shared_ui_audit_trail_title()}>
        <div class="overflow-y-auto" style="max-height: {maxHeight};">
          {#if isLoading}
            <p class="py-2 text-sm text-muted-foreground">{m.shared_ui_refreshing()}</p>
          {:else if auditQuery.isError}
            <div class="flex items-center gap-2 py-2">
              <p class="text-sm text-destructive">{errorMessage}</p>
              <Button
                variant="ghost"
                size="sm"
                class="h-auto px-1 py-0 text-sm"
                onclick={() => auditQuery.refetch()}
                disabled={auditQuery.isFetching}
              >
                {m.common_retry()}
              </Button>
            </div>
          {:else if logs.length === 0}
            <p class="py-2 text-sm text-muted-foreground">{m.shared_ui_no_audit_history()}</p>
          {:else}
            <div class="relative ms-1 border-s border-border py-1">
              {#each logs as log (log.id)}
                <div class="relative pb-4 ps-5 last:pb-0">
                  <span class="absolute -start-[4.5px] top-1.5 size-2 rounded-full bg-muted-foreground"></span>
                  <p class="text-sm leading-relaxed text-foreground">
                    <span class="font-medium">{formatActionLabel(log.action)}</span>
                    <span class="text-muted-foreground"> · {m.shared_ui_by()} {getActorName(log.user_id)}</span>
                    <time datetime={log.created_at} class="text-muted-foreground">
                      · {formatDate(log.created_at, "relative")}
                    </time>
                  </p>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </section>
    </AccordionContent>
  </AccordionItem>
</Accordion>
