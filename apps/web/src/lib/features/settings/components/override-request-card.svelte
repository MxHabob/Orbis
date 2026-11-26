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

<Card class="overflow-hidden rounded-md border-border/60 bg-card shadow-2xs transition-colors hover:border-border">
  <div class="flex flex-col gap-3 p-4">
    <div class="flex flex-wrap items-center justify-between gap-2">
      <div class="flex flex-wrap items-center gap-1.5">
        <Badge
          variant={typeInfo.variant}
          class="h-5 gap-1 rounded-full px-2 text-[11px] font-semibold"
        >
          <TypeIcon size={12} />
          {typeInfo.label}
        </Badge>
        <Badge
          variant={statusBadgeVariant(request.status)}
          class="h-5 rounded-full px-2 text-[10px] font-semibold"
        >
          {statusLabel(request.status)}
        </Badge>
      </div>

      <div class="flex items-center gap-3 text-[11px] text-muted-foreground">
        {#if showRequester}
          <span class="inline-flex items-center gap-1">
            <User size={12} class="shrink-0" />
            <span class="font-medium text-foreground">
              {request.requested_by_name || request.requested_by}
            </span>
          </span>
        {/if}
        <time datetime={request.created_at} class="font-mono">
          {formatDate(request.created_at)}
        </time>
      </div>
    </div>

    <p class="text-xs font-medium leading-relaxed text-foreground sm:text-sm">
      {request.reason}
    </p>

    {#if request.field_name || request.old_value || request.new_value}
      <div class="flex flex-wrap items-center gap-2 rounded-md border border-border/50 bg-muted/30 px-3 py-1.5 font-mono text-xs">
        {#if request.field_name}
          <span class="font-sans font-medium text-muted-foreground">
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
            class="h-5 rounded-full px-2 text-[10px] font-medium"
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
      <div class="rounded-md border border-border/50 bg-muted/30 px-3 py-2 text-xs">
        <span class="font-semibold text-foreground">
          {m.ui_copy_resolution_371c8da4()}
        </span>
        <span class="ms-1 text-muted-foreground">
          {request.resolution_notes}
        </span>
      </div>
    {/if}
  </div>

  {#if canReview}
    <div class="flex flex-col gap-2.5 border-t border-border/50 bg-muted/20 px-4 py-3 sm:flex-row sm:items-end">
      <Textarea
        bind:value={() => actionNotes ?? "", (v) => (actionNotes = v)}
        placeholder={m.ui_copy_approval_notes_or_decision_reason_optional_a41d972b()}
        rows={1}
        disabled={processing}
        class="min-h-9 flex-1 bg-background text-xs"
      />
      <div class="flex shrink-0 items-center gap-2">
        <Button
          variant="lime"
          size="sm"
          class="h-8 gap-1.5 px-3 text-xs font-semibold"
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
        <Button
          variant="destructive"
          size="sm"
          class="h-8 gap-1.5 px-3 text-xs font-semibold"
          disabled={processing}
          onclick={onreject}
        >
          <XMark size={13} />
          {m.common_approval_reject()}
        </Button>
      </div>
    </div>
  {/if}
</Card>
