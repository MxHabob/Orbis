<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  BookOpen,
  ChartBar,
  ClipboardCheck,
  CreditCard,
  UserBadgeCheck,
  UserPlus,
} from "@orbis/icons";

import { localizeHref } from "$lib/paraglide/runtime";
import { academicContext, workflowInbox, effectiveContext } from "$lib/stores";

let { orgId }: { orgId: string } = $props();

// Phase-aware actions — show different actions based on the current academic phase
const ACTIONS = $derived.by(() => {
  const phase = academicContext.periodPhase;
  const ro = academicContext.isReadOnly;
  const canManageApprovals =
    effectiveContext.hasPermission("workflow:manage") ||
    effectiveContext.hasPermission("approvals:read") ||
    effectiveContext.hasPermission("*");

  const actions: Array<{
    label: string;
    icon: any;
    href: string;
    show: boolean;
    badge?: string;
  }> = [
    // Always visible — core navigation
    {
      label: m.common_action_enroll_student(),
      icon: UserPlus,
      href: `/${orgId}/students/new`,
      show: !ro && (phase === "active" || phase === "pre_start"),
    },
    {
      label: m.common_action_record_payment(),
      icon: CreditCard,
      href: `/${orgId}/finance/payments`,
      show: !ro,
    },
    {
      label: m.common_action_manage_classes(),
      icon: BookOpen,
      href: `/${orgId}/academics`,
      show: true,
    },
    // Phase-specific actions
    {
      label: m.common_control_label ? m.common_control_label() : "Control",
      icon: UserBadgeCheck,
      href: `/${orgId}/academics/exams/control`,
      show: phase === "grade_entry" || phase === "exams",
    },
    {
      label: m.common_action_view_reports(),
      icon: ChartBar,
      href: `/${orgId}/reports`,
      show: true,
    },
    // Pending approvals shortcut (only when there are pending items)
    {
      label: m.settings_nav_approvals
        ? m.settings_nav_approvals()
        : "Approvals",
      icon: ClipboardCheck,
      href: `/${orgId}/settings/approvals`,
      show: workflowInbox.hasActions && canManageApprovals,
      badge:
        workflowInbox.totalCount > 0
          ? String(workflowInbox.totalCount)
          : undefined,
    },
  ];

  return actions.filter((a) => a.show);
});
</script>

<div class="flex items-center justify-center gap-3 pb-12 w-full flex-wrap">
	{#each ACTIONS as action (action.href)}
		{@const Icon = action.icon}
		<Button
			href={localizeHref(action.href)}
		>
			<Icon size={13} />
			<span>{action.label}</span>
			{#if action.badge}
				<Badge variant="destructive" class="text-[10px] px-1.5 h-4 ms-1">{action.badge}</Badge>
			{/if}
		</Button>
	{/each}
</div>
