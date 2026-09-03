<!--
@component OverrideRequestDialog
Shared dialog for creating override/approval requests.

Used when a user attempts an action that is currently blocked
(e.g., editing grades after grade_entry_close, late enrollment).
Calls `commands.enrollment_workflow.getJob()` and shows success/error feedback.

Usage:
  <OverrideRequestDialog
    bind:open={showOverride}
    orgId={data.orgId}
    overrideType="grade_modification"
    entityType="grade"
    entityId={gradeId}
    riskLevel="medium"
    onSuccess={() => toast.success('Override submitted')}
  />
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { commands, extractErrorMessage } from "$lib/core/ipc";
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
import { Loader2, AlertTriangle, Shield } from "@lucide/svelte";
import type { OverrideType, RiskLevel } from "@orbis/sdk-core";

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
      return m.common_override_risk_low ? m.common_override_risk_low() : "Low";
    case "medium":
      return m.common_override_risk_medium
        ? m.common_override_risk_medium()
        : "Medium";
    case "high":
      return m.common_override_risk_high
        ? m.common_override_risk_high()
        : "High";
    case "critical":
      return m.common_override_risk_critical
        ? m.common_override_risk_critical()
        : "Critical";
    default:
      return riskLevel;
  }
});

const typeLabel = $derived.by(() => {
  switch (overrideType) {
    case "grade_modification":
      return m.common_override_type_grade
        ? m.common_override_type_grade()
        : "Grade Modification";
    case "late_enrollment":
      return m.common_override_type_enrollment
        ? m.common_override_type_enrollment()
        : "Late Enrollment";
    case "attendance_correction":
      return m.common_override_type_attendance
        ? m.common_override_type_attendance()
        : "Attendance Correction";
    case "fee_waiver":
      return m.common_override_type_fee
        ? m.common_override_type_fee()
        : "Fee Waiver";
    case "period_unlock":
      return m.common_override_type_unlock
        ? m.common_override_type_unlock()
        : "Period Unlock";
    case "deadline_extension":
      return m.common_override_type_deadline
        ? m.common_override_type_deadline()
        : "Deadline Extension";
    case "promotion_override":
      return m.common_override_type_promotion
        ? m.common_override_type_promotion()
        : "Promotion Override";
    default:
      return overrideType;
  }
});

const approvalChainPreview = $derived.by(() => {
  switch (riskLevel) {
    case "low":
      return m.common_override_chain_auto
        ? m.common_override_chain_auto()
        : "Auto-approved";
    case "medium":
      return m.common_override_chain_manager
        ? m.common_override_chain_manager()
        : "Manager approval required";
    case "high":
      return m.common_override_chain_admin
        ? m.common_override_chain_admin()
        : "Manager + Admin approval required";
    case "critical":
      return m.common_override_chain_full
        ? m.common_override_chain_full()
        : "Full approval chain required";
    default:
      return "";
  }
});

async function submit() {
  if (!reason.trim()) {
    toast.error(
      m.common_override_reason_required
        ? m.common_override_reason_required()
        : "Please provide a reason",
    );
    return;
  }
  isSubmitting = true;
  try {
    await commands.enrollment_workflow.getJob(entityId);
    toast.success(
      m.common_override_submitted
        ? m.common_override_submitted()
        : "Override request submitted",
    );
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
        <Shield class="size-5 text-primary" />
        {m.common_override_request_title ? m.common_override_request_title() : "Override Request"}
      </DialogTitle>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <!-- Override Type + Risk -->
      <div class="flex items-center justify-between bg-muted/50 rounded-lg p-3">
        <div class="flex flex-col gap-1">
          <span class="text-xs text-muted-foreground">
            {m.common_word_type ? m.common_word_type() : "Type"}
          </span>
          <span class="text-sm font-medium">{typeLabel}</span>
        </div>
        <Badge variant={riskBadgeVariant} class="text-xs">
          {#if riskLevel === "high" || riskLevel === "critical"}
            <AlertTriangle class="size-3 me-1" />
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
          {m.common_promo_reason ? m.common_promo_reason() : "Reason"} *
        </Label>
        <Textarea
          id="override-reason"
          bind:value={reason}
          placeholder={m.common_override_reason_placeholder ? m.common_override_reason_placeholder() : "Explain why this override is needed..."}
          rows={3}
        />
      </div>

      <!-- Approval Chain Preview -->
      <div class="text-xs text-muted-foreground bg-muted/30 rounded-lg p-3 flex items-center gap-2">
        <Shield class="size-3.5 shrink-0" />
        {approvalChainPreview}
      </div>
    </div>

    <DialogFooter class="flex-nowrap shrink-0">
      <Button variant="outline" onclick={() => { open = false; }} disabled={isSubmitting}>
        {m.common_action_cancel ? m.common_action_cancel() : "Cancel"}
      </Button>
      <Button onclick={submit} disabled={isSubmitting || !reason.trim()}>
        {#if isSubmitting}
          <Loader2 class="size-4 animate-spin me-2" />
        {/if}
        {m.common_override_submit ? m.common_override_submit() : "Submit Request"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
