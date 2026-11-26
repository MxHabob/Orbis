<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { formatDate } from "$lib/core/i18n";
import { Card, CardContent } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Textarea } from "@orbis/ui/textarea";
import { Separator } from "@orbis/ui/separator";
import {
  AdjustmentsHorizontal,
  ArrowPath,
  CheckCircle,
  CheckSquare,
  ClockRotateRight,
  Lock,
  Tag,
  User,
  UserBadgeCheck,
  XMark,
} from "@orbis/icons";
import type { OverrideRequestDto } from "$lib/api/types";

let {
  request,
  showRequester,
  canReview,
  processing,
  actionNotes = $bindable(""),
  onapprove,
  onreject,
}: {
  request: OverrideRequestDto;
  showRequester: boolean;
  canReview: boolean;
  processing: boolean;
  actionNotes?: string;
  onapprove: () => void;
  onreject: () => void;
} = $props();

function getOverrideTypeInfo(type: string) {
  switch (type) {
    case "admission_approval":
      return {
        label: m.settings_ui_approval_admission(),
        icon: UserBadgeCheck,
        variant: "lime" as const,
      };
    case "capacity_override":
      return {
        label: m.settings_ui_approval_capacity_override(),
        icon: User,
        variant: "warning" as const,
      };
    case "grade_change":
      return {
        label: m.settings_ui_approval_locked_grade_change(),
        icon: AdjustmentsHorizontal,
        variant: "default" as const,
      };
    case "prerequisite_bypass":
      return {
        label: m.settings_ui_approval_prerequisite_bypass(),
        icon: Lock,
        variant: "destructive" as const,
      };
    case "late_enrollment":
      return {
        label: m.settings_ui_approval_late_enrollment(),
        icon: ClockRotateRight,
        variant: "outline" as const,
      };
    case "discount_override":
      return {
        label: m.settings_ui_approval_discount_override(),
        icon: Tag,
        variant: "secondary" as const,
      };
    default:
      return {
        label: m.settings_ui_approval_unknown_type({
          type: type.replace(/_/g, " "),
        }),
        icon: CheckSquare,
        variant: "outline" as const,
      };
  }
}

function statusBadgeVariant(status: string) {
  switch (status) {
    case "approved":
    case "executed":
      return "lime" as const;
    case "rejected":
      return "destructive" as const;
    case "pending":
      return "warning" as const;
    case "expired":
    case "cancelled":
      return "secondary" as const;
    default:
      return "outline" as const;
  }
}

function statusLabel(status: string): string {
  switch (status) {
    case "pending":
      return m.notifications_popover_request_status_pending();
    case "approved":
      return m.notifications_popover_request_status_approved();
    case "rejected":
      return m.notifications_popover_request_status_rejected();
    case "expired":
      return m.notifications_popover_request_status_expired();
    case "executed":
      return m.notifications_popover_request_status_executed();
    case "cancelled":
      return m.notifications_popover_request_status_cancelled();
    default:
      return status;
  }
}

const typeInfo = $derived(getOverrideTypeInfo(request.override_type));
const TypeIcon = $derived(typeInfo.icon);
</script>

<Card class="border-border/60 bg-card shadow-xs">
  <CardContent class="flex flex-col gap-3 p-4 sm:p-5">
    <div class="flex flex-wrap items-start justify-between gap-3">
      <div class="flex flex-wrap items-center gap-2">
        <Badge
          variant={typeInfo.variant}
          class="gap-1.5 text-xs font-semibold"
        >
          <TypeIcon size={14} />
          {typeInfo.label}
        </Badge>
        <Badge
          variant={statusBadgeVariant(request.status)}
          class="text-[10px] font-semibold"
        >
          {statusLabel(request.status)}
        </Badge>
      </div>
      <time
        datetime={request.created_at}
        class="text-[11px] text-muted-foreground"
      >
        {formatDate(request.created_at)}
      </time>
    </div>

    <div class="space-y-2">
      <p class="text-sm font-medium leading-relaxed text-foreground">
        {request.reason}
      </p>
      {#if showRequester}
        <p class="text-xs text-muted-foreground">
          {m.ui_copy_requested_by_a2679cef()}
          <span class="ms-1 font-medium text-foreground">
            {request.requested_by_name || request.requested_by}
          </span>
        </p>
      {/if}
    </div>

    {#if request.field_name || request.old_value || request.new_value}
      <div class="flex flex-wrap items-center gap-2 rounded-md border border-border/60 bg-muted/30 px-3 py-2 text-xs">
        {#if request.field_name}
          <span class="font-medium text-muted-foreground">
            {request.field_name}
          </span>
        {/if}
        {#if request.old_value}
          <span class="break-all text-muted-foreground line-through">
            {request.old_value}
          </span>
        {/if}
        {#if request.old_value && request.new_value}
          <span aria-hidden="true" class="text-muted-foreground">→</span>
        {/if}
        {#if request.new_value}
          <span class="break-all font-semibold text-foreground">
            {request.new_value}
          </span>
        {/if}
      </div>
    {/if}

    {#if request.approval_chain.length > 0}
      <div class="flex flex-wrap items-center gap-1.5">
        {#each request.approval_chain as step, index (step.step_order)}
          <Badge
            variant={statusBadgeVariant(step.status)}
            class="text-[10px] font-medium"
          >
            {step.approver_name || step.approver_role}
          </Badge>
          {#if index < request.approval_chain.length - 1}
            <span aria-hidden="true" class="text-xs text-muted-foreground">›</span>
          {/if}
        {/each}
      </div>
    {/if}

    {#if request.resolution_notes}
      <div class="rounded-md border border-border/60 bg-muted/30 px-3 py-2 text-xs">
        <span class="font-semibold text-foreground">
          {m.ui_copy_resolution_371c8da4()}
        </span>
        <span class="text-muted-foreground">
          {request.resolution_notes}
        </span>
      </div>
    {/if}

    {#if canReview}
      <Separator />
      <div class="flex flex-col gap-3 sm:flex-row sm:items-end">
        <Textarea
          bind:value={actionNotes}
          placeholder={m.ui_copy_approval_notes_or_decision_reason_optional_a41d972b()}
          rows={2}
          disabled={processing}
          class="min-h-16 flex-1 text-xs"
        />
        <div class="flex shrink-0 gap-2">
          <Button
            variant="lime"
            size="sm"
            class="gap-1.5"
            disabled={processing}
            onclick={onapprove}
          >
            {#if processing}
              <ArrowPath size={14} class="animate-spin" />
            {:else}
              <CheckCircle size={14} />
            {/if}
            {m.ui_copy_approve_7da54ae0()}
          </Button>
          <Button
            variant="destructive"
            size="sm"
            class="gap-1.5"
            disabled={processing}
            onclick={onreject}
          >
            <XMark size={14} />
            {m.common_approval_reject()}
          </Button>
        </div>
      </div>
    {/if}
  </CardContent>
</Card>
