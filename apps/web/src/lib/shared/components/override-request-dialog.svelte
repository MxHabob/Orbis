<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { LoadingIndicator, toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@orbis/ui/dialog";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import { api, extractErrorMessage, queryClient, queryKeys } from "$lib/api";
import { l } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import { periodOverrides } from "$lib/shared/utils/period-override.svelte";
import { academicContext } from "$lib/stores";

let { orgId }: { orgId: string } = $props();
const request = $derived(periodOverrides.request?.orgId === orgId ? periodOverrides.request : null);
const target = $derived(request?.targets[0]);
const period = $derived(academicContext.allPeriods.find((item) => item.id === target?.period_id));
let submitting = $state(false);

const targetLabel = createQuery(() => {
  const current = target;
  return {
    queryKey: ["period-override-label", orgId, current?.entity_type, current?.entity_id],
    enabled: current?.entity_type === "section" || current?.entity_type === "section_subject",
    staleTime: 60_000,
    retry: false,
    queryFn: async () => {
      if (current?.entity_type === "section_subject") {
        const assignment = await api.getSectionSubject(current.entity_id);
        const [section, subject] = await Promise.all([
          api.getSection(assignment.section_id), api.getSubject(assignment.subject_id),
        ]);
        return [l(section.name_ar, section.name), l(subject.name_ar, subject.name)].filter(Boolean).join(" · ");
      }
      if (current?.entity_type === "section") {
        const section = await api.getSection(current.entity_id);
        return l(section.name_ar, section.name);
      }
      return "";
    },
  };
});

function dismiss() {
  if (submitting) return;
  periodOverrides.dismiss();
}

async function submit(event: SubmitEvent) {
  event.preventDefault();
  if (!request || !target || submitting || !request.reason.trim()) return;
  const current = request;
  submitting = true;
  try {
    await api.submitOverride({ ...target, reason: current.reason.trim() });
    void queryClient.invalidateQueries({ queryKey: queryKeys.workflow.all });
    toast.success(m.common_override_submitted());
    if (periodOverrides.request === current) {
      const remaining = current.targets.slice(1);
      if (remaining.length) {
        current.targets = remaining;
      } else {
        periodOverrides.dismiss();
      }
    }
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    submitting = false;
  }
}
</script>

<Dialog open={Boolean(target)} onOpenChange={(open) => { if (!open) dismiss(); }}>
  <DialogContent class="sm:max-w-md">
    {#if target && request}
      <DialogHeader>
        <DialogTitle>
          {target.override_type === "grade_change" ? m.common_override_type_grade() : m.common_override_type_enrollment()}
        </DialogTitle>
        <DialogDescription>
          {target.override_type === "grade_change" ? m.common_override_grade_closed() : m.common_override_enrollment_closed()}
        </DialogDescription>
      </DialogHeader>

      <form class="flex flex-col gap-4" onsubmit={submit}>
        {#if period || targetLabel.data}
          <div class="rounded-md bg-muted px-3 py-2 text-sm">
            {#if targetLabel.data}<p class="font-medium">{targetLabel.data}</p>{/if}
            {#if period}<p class="text-muted-foreground">{l(period.name_ar, period.name)}</p>{/if}
          </div>
        {/if}
        {#if request.targets.length > 1}
          <p class="text-sm text-muted-foreground">{m.common_override_targets_remaining({ count: request.targets.length })}</p>
        {/if}
        <div class="flex flex-col gap-2">
          <Label for="override-reason">{m.common_promo_reason()}</Label>
          <Textarea id="override-reason" bind:value={request.reason} rows={3} required
            placeholder={m.common_override_reason_placeholder()} disabled={submitting} />
        </div>
        <DialogFooter>
          <Button type="button" variant="outline" onclick={dismiss} disabled={submitting}>
            {m.common_action_cancel()}
          </Button>
          <Button type="submit" disabled={submitting || !request.reason.trim()}>
            {#if submitting}<LoadingIndicator class="me-2 size-4" size={16} />{/if}
            {m.common_override_submit()}
          </Button>
        </DialogFooter>
      </form>
    {/if}
  </DialogContent>
</Dialog>
