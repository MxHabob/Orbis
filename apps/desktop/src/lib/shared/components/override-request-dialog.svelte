<!--
@component OverrideRequestDialog
Shared dialog for creating override/approval requests.

Used when a user attempts an action that is currently blocked
(e.g., editing grades after grade_entry_close, late enrollment).
Calls `api.submitOverride()` and shows success/error feedback.

Usage:
  <OverrideRequestDialog
    bind:open={showOverride}
    orgId={data.orgId}
    overrideType="grade_modification"
    entityType="grade"
    entityId={gradeId}
    riskLevel="medium"
    onSuccess={() => toast.success(m.shared_ui_override_submitted())}
  />
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";

import { api, extractErrorMessage } from "$lib/api";
import { authStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Textarea } from "@orbis/ui/textarea";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import {
  AdjustmentsHorizontal,
  ArrowPath,
  InfoCircle,
  UserBadgeCheck,
} from "@orbis/icons";
import type { OverrideType, RiskLevel } from "@orbis/sdk";

interface Props {
  open: boolean;
  orgId: string;
  overrideType: OverrideType;
  entityType: string;
  entityId: string;
  riskLevel?: RiskLevel;
  fieldName?: string;
  oldValue?: string;
  newValue?: string;
  onSuccess?: () => void;
}

let {
  open = $bindable(false),
  orgId,
  overrideType,
  entityType,
  entityId,
  riskLevel = "medium",
  fieldName,
  oldValue,
  newValue,
  onSuccess,
}: Props = $props();

let reason = $state("");
let isSubmitting = $state(false);

// Derived labels
const riskBadgeVariant = $derived.by(() => {
  switch (riskLevel) {
    case "low":
      return "secondary" as const;
    case "medium":
      return "default" as const;
    case "high":
      return "destructive" as const;
    case "critical":
      return "destructive" as const;
    default:
      return "default" as const;
  }
});

const riskLabel = $derived.by(() => {
  switch (riskLevel) {
    case "low":
      return m.common_override_risk_low();
    case "medium":
      return m.common_override_risk_medium();
    case "high":
      return m.common_override_risk_high();
    case "critical":
      return m.common_override_risk_critical();
    default:
      return riskLevel;
  }
});

const typeLabel = $derived.by(() => {
  switch (overrideType) {
    case "grade_modification":
      return m.common_override_type_grade();
    case "late_enrollment":
      return m.common_override_type_enrollment();
    case "attendance_correction":
      return m.common_override_type_attendance();
    case "fee_waiver":
      return m.common_override_type_fee();
    case "period_unlock":
      return m.common_override_type_unlock();
    case "deadline_extension":
      return m.common_override_type_deadline();
    case "promotion_override":
      return m.common_override_type_promotion();
    default:
      return overrideType;
  }
});

const approvalChainPreview = $derived.by(() => {
  switch (riskLevel) {
    case "low":
      return m.common_override_chain_auto();
    case "medium":
      return m.common_override_chain_manager();
    case "high":
      return m.common_override_chain_admin();
    case "critical":
      return m.common_override_chain_full();
    default:
      return "";
  }
});

async function submit() {
  if (!reason.trim()) {
    toast.error(m.common_override_reason_required());
    return;
  }
  isSubmitting = true;
  try {
    await api.submitOverride({
      overrideType: overrideType as any,
      targetEntityId: entityId,
      reason,
      payload: {
        fieldName,
        oldValue,
        newValue,
        entityType,
      },
    });
    toast.success(m.common_override_submitted());
    reason = "";
    open = false;
    onSuccess?.();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open>
  <DialogContent class="sm:max-w-md">
    <DialogHeader>
      <DialogTitle class="flex items-center gap-2">
        <UserBadgeCheck class="size-5 text-primary" />
        {m.common_override_request_title()}
      </DialogTitle>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <!-- Override Type + Risk -->
      <div class="flex items-center justify-between bg-muted/50 rounded-lg p-3">
        <div class="flex flex-col gap-1">
          <span class="text-xs text-muted-foreground">
            {m.common_word_type()}
          </span>
          <span class="text-sm font-medium">{typeLabel}</span>
        </div>
        <Badge variant={riskBadgeVariant} class="text-xs">
          {#if riskLevel === "high" || riskLevel === "critical"}
            <InfoCircle class="size-3 me-1" />
          {/if}
          {riskLabel}
        </Badge>
      </div>

      <!-- Value Change (if applicable) -->
      {#if oldValue || newValue}
        <div class="flex items-center gap-2 text-xs bg-muted/30 rounded-lg p-3">
          {#if fieldName}
            <span class="text-muted-foreground">{fieldName}:</span>
          {/if}
          {#if oldValue}
            <span class="line-through text-muted-foreground">{oldValue}</span>
            <span class="text-muted-foreground">→</span>
          {/if}
          {#if newValue}
            <span class="font-medium text-foreground">{newValue}</span>
          {/if}
        </div>
      {/if}

      <!-- Reason -->
      <div class="flex flex-col gap-2">
        <Label for="override-reason">
          {m.common_promo_reason()} *
        </Label>
        <Textarea
          id="override-reason"
          bind:value={reason}
          placeholder={m.common_override_reason_placeholder()}
          rows={3}
        />
      </div>

      <!-- Approval Chain Preview -->
      <div class="text-xs text-muted-foreground bg-muted/30 rounded-lg p-3 flex items-center gap-2">
        <UserBadgeCheck class="size-3.5 shrink-0" />
        {approvalChainPreview}
      </div>
    </div>

    <DialogFooter class="flex-nowrap shrink-0">
      <Button variant="outline" onclick={() => { open = false; }} disabled={isSubmitting}>
        {m.common_action_cancel()}
      </Button>
      <Button onclick={submit} disabled={isSubmitting || !reason.trim()}>
        {#if isSubmitting}
          <LoadingIndicator class="size-4 me-2"  size={16} />
        {/if}
        {m.common_override_submit()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
