<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  UserPlus,
  Wallet,
  BookOpen,
  BarChart3,
  ShieldCheck,
  CalendarClock,
  ClipboardCheck,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { academicContext, workflowInbox, authStore } from "$lib/stores";

let { orgId }: { orgId: string } = $props();

// Phase-aware actions — show different actions based on the current academic phase
const ACTIONS = $derived.by(() => {
  const phase = academicContext.periodPhase;
  const ro = academicContext.isReadOnly;
  const role = authStore.user?.role ?? "viewer";
  const isAdmin = ["admin", "super_admin", "principal"].includes(role);

  const actions: Array<{
    label: string;
    icon: typeof UserPlus;
    href: string;
    show: boolean;
    badge?: string;
  }> = [
    // Always visible — core navigation
    {
      label: m.action_enroll_student(),
      icon: UserPlus,
      href: `/${orgId}/students/new`,
      show: !ro && (phase === "active" || phase === "pre_start"),
    },
    {
      label: m.action_record_payment(),
      icon: Wallet,
      href: `/${orgId}/finance/payments`,
      show: !ro,
    },
    {
      label: m.action_manage_classes(),
      icon: BookOpen,
      href: `/${orgId}/academics`,
      show: true,
    },
    // Phase-specific actions
    {
      label: m.academics_control_label
        ? m.academics_control_label()
        : "Control",
      icon: ShieldCheck,
      href: `/${orgId}/academics/control`,
      show: phase === "grade_entry" || phase === "exams",
    },
    {
      label: m.calendar_title ? m.calendar_title() : "Calendar",
      icon: CalendarClock,
      href: `/${orgId}/academics/calendar`,
      show: isAdmin,
    },
    {
      label: m.action_view_reports(),
      icon: BarChart3,
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
      show: workflowInbox.hasActions && isAdmin,
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
