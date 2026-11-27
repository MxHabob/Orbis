<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { formatDate } from "$lib/core/i18n";
import { Card } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Textarea } from "@orbis/ui/textarea";
import { LoadingIndicator } from "@orbis/ui";
import {
  AdjustmentsHorizontal,
  CheckCircle,
  CheckSquare,
  ChevronRight,
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
  actionNotes = $bindable(),
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

const UUID_RE =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

export function formatDisplayValue(val?: string | null): string {
  if (!val) return "";
  return UUID_RE.test(val.trim()) ? `${val.trim().slice(0, 8)}…` : val;
}

export function getOverrideTypeInfo(type: string) {
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

export function statusBadgeVariant(status: string) {
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

export function statusLabel(status: string): string {
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
const requesterDisplay = $derived(
  request.requested_by_name || formatDisplayValue(request.requested_by),
);
</script>

<Card class="flex flex-col overflow-hidden rounded-md border-border/60 bg-card shadow-2xs">
  <div class="flex flex-col gap-4 p-5">
    <div class="flex flex-wrap items-start justify-between gap-3 border-b border-border/50 pb-4">
      <div class="flex flex-wrap items-center gap-2">
        <Badge
          variant={typeInfo.variant}
          class="h-6 gap-1.5 rounded-full px-2.5 text-xs font-semibold"
        >
          <TypeIcon size={13} />
          {typeInfo.label}
        </Badge>
        <Badge
          variant={statusBadgeVariant(request.status)}
          class="h-6 rounded-full px-2.5 text-[11px] font-semibold"
        >
          {statusLabel(request.status)}
        </Badge>
      </div>

      <div class="flex flex-wrap items-center gap-3 text-xs text-muted-foreground">
        {#if showRequester}
          <span class="inline-flex items-center gap-1.5" title={request.requested_by}>
            <User size={13} class="shrink-0" />
            <span class="font-medium text-foreground">{requesterDisplay}</span>
          </span>
        {/if}
        <time datetime={request.created_at} class="font-mono text-[11px]">
          {formatDate(request.created_at)}
        </time>
      </div>
    </div>

    <div class="space-y-1.5">
      <p class="text-sm font-medium leading-relaxed text-foreground">
        {request.reason}
      </p>
    </div>

    {#if request.field_name || request.old_value || request.new_value}
      <div class="flex flex-wrap items-center gap-2 rounded-md border border-border/60 bg-muted/40 px-3.5 py-2.5 font-mono text-xs">
        {#if request.field_name}
          <span class="font-sans font-semibold text-muted-foreground">
            {request.field_name}
          </span>
        {/if}
        {#if request.old_value}
          <span class="break-all text-muted-foreground line-through" title={request.old_value}>
            {formatDisplayValue(request.old_value)}
          </span>
        {/if}
        {#if request.old_value && request.new_value}
          <span aria-hidden="true" class="text-muted-foreground">→</span>
        {/if}
        {#if request.new_value}
          <span class="break-all font-semibold text-foreground" title={request.new_value}>
            {formatDisplayValue(request.new_value)}
          </span>
        {/if}
      </div>
    {/if}

    {#if request.approval_chain.length > 0}
      <div class="flex flex-wrap items-center gap-1.5 pt-1">
        {#each request.approval_chain as step, index (step.step_order)}
          <Badge
            variant={statusBadgeVariant(step.status)}
            class="h-5 rounded-full px-2.5 text-[11px] font-medium"
          >
            {step.approver_name || step.approver_role}
          </Badge>
          {#if index < request.approval_chain.length - 1}
            <ChevronRight size={12} class="text-muted-foreground rtl:rotate-180" />
          {/if}
        {/each}
      </div>
    {/if}

    {#if request.resolution_notes}
      <div class="rounded-md border border-border/60 bg-muted/40 px-3.5 py-2.5 text-xs">
        <span class="font-semibold text-foreground">
          {m.ui_copy_resolution_371c8da4()}
        </span>
        <span class="ms-1.5 text-muted-foreground">
          {request.resolution_notes}
        </span>
      </div>
    {/if}
  </div>

  {#if canReview}
    <div class="flex flex-col gap-3 border-t border-border/60 bg-muted/30 p-4">
      <Textarea
        bind:value={() => actionNotes ?? "", (v) => (actionNotes = v)}
        placeholder={m.ui_copy_approval_notes_or_decision_reason_optional_a41d972b()}
        rows={2}
        disabled={processing}
        class="min-h-16 bg-background text-xs"
      />
      <div class="flex flex-wrap items-center justify-end gap-2">
        <Button
          variant="destructive"
          size="sm"
          class="h-8 gap-1.5 px-3.5 text-xs font-semibold"
          disabled={processing}
          onclick={onreject}
        >
          <XMark size={13} />
          {m.common_approval_reject()}
        </Button>
        <Button
          variant="lime"
          size="sm"
          class="h-8 gap-1.5 px-4 text-xs font-semibold"
          disabled={processing}
          onclick={onapprove}
        >
          {#if processing}
            <LoadingIndicator size={13} />
          {:else}
            <CheckCircle size={13} />
          {/if}
          {m.ui_copy_approve_7da54ae0()}
        </Button>
      </div>
    </div>
  {/if}
</Card>
