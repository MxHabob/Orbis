<!--
@component PeriodGateAction
Centralized smart action button that adapts based on Period Gate verdict.

Replaces manual button logic across enrollment, grades, attendance pages.
Calls `commands.academic.checkPeriodGate()` (or enrollment-specific) on mount,
then shows appropriate UI based on the verdict.

Usage:
  <PeriodGateAction
    gateType="enrollment"
    periodId={activePeriod.id}
    classId={selectedClass?.id}
    onAction={handleDirectAction}
    onDraft={handleSaveDraft}
    onOverride={openOverrideDialog}
  >
    {#snippet allowed()}
      <Button>{m.shared_enroll_student_btn()}</Button>
    {/snippet}
  </PeriodGateAction>
-->
<script lang="ts">
import { commands } from "$lib/core/ipc";
import { createQuery } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Loader2, Lock, Clock, AlertTriangle, Send } from "@lucide/svelte";
import type { GateType, GateVerdict, GateDecision } from "@orbis/sdk-core";
import type { Snippet } from "svelte";

interface Props {
  gateType: GateType;
  periodId: string;
  classId?: string;
  /** Called when the action is directly allowed. */
  onAction?: () => void;
  /** Called when the user chooses "save as draft". */
  onDraft?: () => void;
  /** Called when the user wants to request an override. */
  onOverride?: (verdict: GateVerdict) => void;
  /** Custom label for the primary action button. */
  label?: string;
  /** Custom label (Arabic) */
  labelAr?: string;
  /** Size variant */
  size?: "sm" | "default" | "lg";
  /** Allowed slot */
  allowed?: Snippet;
  /** Late slot */
  late?: Snippet;
  /** Override slot */
  override?: Snippet;
  /** Blocked slot */
  blocked?: Snippet<[string]>;
}

let {
  gateType,
  periodId,
  classId,
  onAction,
  onDraft,
  onOverride,
  label = "إضافة",
  size = "default",
  allowed,
  late,
  override: overrideSlot,
  blocked,
}: Props = $props();

const gateQuery = createQuery({
  queryKey: ["period-gate", gateType, periodId, classId ?? ""],
  queryFn: async () => {
    if (gateType === "enrollment" && classId) {
      return commands.academic.checkEnrollmentEligibility(periodId, classId);
    }
    return commands.academic.checkPeriodGate(gateType, periodId);
  },
  staleTime: 30_000,
  refetchOnWindowFocus: true,
  enabled: !!periodId,
});

let verdict = $derived($gateQuery.data as GateVerdict | undefined);
let decision = $derived(verdict?.decision ?? "allowed");
let reason = $derived(verdict?.reason ?? "");

const decisionColors: Record<GateDecision, string> = {
  allowed: "bg-emerald-500/10 text-emerald-600 border-emerald-500/20",
  late: "bg-amber-500/10 text-amber-600 border-amber-500/20",
  requires_override: "bg-orange-500/10 text-orange-600 border-orange-500/20",
  blocked: "bg-red-500/10 text-red-600 border-red-500/20",
};
</script>

{#if $gateQuery.isLoading}
  <Button {size} disabled>
    <Loader2 class="mr-2 h-4 w-4 animate-spin" />
    جارِ التحقق...
  </Button>
{:else if verdict}
  {#if decision === "allowed"}
    {#if allowed}
      {@render allowed()}
    {:else}
      <Button {size} onclick={onAction}>
        {label}
      </Button>
    {/if}

  {:else if decision === "late"}
    {#if late}
      {@render late()}
    {:else}
      <div class="flex items-center gap-2">
        <Button {size} variant="outline" class="border-amber-500/30 text-amber-600" onclick={onAction}>
          <Clock class="mr-2 h-4 w-4" />
          {label} (متأخر)
        </Button>
        {#if verdict.allowed_actions.includes("save_draft")}
          <Button {size} variant="ghost" onclick={onDraft}>
            حفظ كمسودة
          </Button>
        {/if}
      </div>
    {/if}

  {:else if decision === "requires_override"}
    {#if overrideSlot}
      {@render overrideSlot()}
    {:else}
      <div class="flex items-center gap-2">
        {#if verdict.allowed_actions.includes("save_draft")}
          <Button {size} variant="outline" onclick={onDraft}>
            📋 حفظ كمسودة
          </Button>
        {/if}
        <Button {size} variant="secondary" onclick={() => onOverride?.(verdict!)}>
          <Send class="mr-2 h-4 w-4" />
          طلب تجاوز
        </Button>
        <Badge variant="outline" class={decisionColors.requires_override}>
          {reason}
        </Badge>
      </div>
    {/if}

  {:else if decision === "blocked"}
    {#if blocked}
      {@render blocked(reason)}
    {:else}
      <div class="flex items-center gap-2">
        <Button {size} disabled>
          <Lock class="mr-2 h-4 w-4" />
          {label}
        </Button>
        <Badge variant="outline" class={decisionColors.blocked}>
          <AlertTriangle class="mr-1 h-3 w-3" />
          {reason}
        </Badge>
      </div>
    {/if}
  {/if}
{:else}
  <!-- Fallback: show direct action if gate check fails -->
  <Button {size} onclick={onAction}>
    {label}
  </Button>
{/if}
