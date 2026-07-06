<script lang="ts">
import { page } from "$app/state";
import { Button } from "@orbis/ui-svelte/button";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@orbis/ui-svelte";
import {
  MoreHorizontal,
  UserPlus,
  CreditCard,
  CalendarClock,
  ShieldCheck,
  CheckSquare,
} from "lucide-svelte";
import { academicContext, workflowInbox } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

const currentSegment = $derived.by(() => {
  const parts = deLocalizeUrl(page.url).pathname.split("/").filter(Boolean);
  return parts[1] ?? "dashboard";
});

const quickActions = $derived.by(() => {
  const seg = currentSegment;
  const actions: Array<{ label: string; icon: typeof UserPlus; href: string }> = [];

  if (seg === "students" || seg === "dashboard") {
    if (academicContext.canEnroll) {
      actions.push({ label: m.action_enroll_student(), icon: UserPlus, href: `/${orgId}/students/new` });
    }
  }
  if (seg === "finance" || seg === "dashboard") {
    actions.push({ label: m.action_record_payment(), icon: CreditCard, href: `/${orgId}/finance/payments` });
  }
  if (seg === "academics" || seg === "dashboard") {
    actions.push({ label: m.calendar_title ? m.calendar_title() : "Calendar", icon: CalendarClock, href: `/${orgId}/academics/calendar` });
    actions.push({ label: m.academics_control_label ? m.academics_control_label() : "Control", icon: ShieldCheck, href: `/${orgId}/academics/control` });
  }
  if (workflowInbox.hasActions) {
    actions.push({ label: m.settings_nav_approvals ? m.settings_nav_approvals() : "Approvals", icon: CheckSquare, href: `/${orgId}/settings/approvals` });
  }
  return actions;
});
</script>

<DropdownMenu>
	<DropdownMenuTrigger>
		{#snippet child({ props })}
			<Tooltip>
				<TooltipTrigger>
					<Button 
						{...props}
						variant="outline" 
						size="icon-sm" 
						class="relative cursor-pointer rounded-[10px]"
						aria-label={m.sidebar_quick_actions ? m.sidebar_quick_actions() : "Quick Actions"}
					>
						<MoreHorizontal size={16} strokeWidth={1.8} class="text-foreground" />
						{#if quickActions.length > 0}
							<span class="absolute -top-0.5 -end-0.5 size-3.5 bg-primary text-primary-foreground text-[8px] rounded-full flex items-center justify-center font-bold border border-background">
								{quickActions.length}
							</span>
						{/if}
					</Button>
				</TooltipTrigger>
				<TooltipContent side="right" sideOffset={12}>
					{m.sidebar_quick_actions ? m.sidebar_quick_actions() : "Quick Actions"}
				</TooltipContent>
			</Tooltip>
		{/snippet}
	</DropdownMenuTrigger>
	<DropdownMenuContent side="right" sideOffset={12} class="w-52">
		<div class="px-2.5 py-1.5 text-[10px] font-semibold text-muted-foreground border-b border-border/40 mb-1">
			{m.sidebar_quick_actions ? m.sidebar_quick_actions() : "Quick Actions"}
		</div>
		{#if quickActions.length > 0}
			{#each quickActions as action (action.href)}
				{@const Icon = action.icon}
				<DropdownMenuItem class="text-xs cursor-pointer">
					<a href={localizeHref(action.href)} class="flex items-center gap-2 w-full">
						<Icon class="size-3.5 shrink-0" />
						<span>{action.label}</span>
					</a>
				</DropdownMenuItem>
			{/each}
		{:else}
			<div class="px-2.5 py-3 text-xs text-muted-foreground text-center">
				{m.sidebar_no_actions ? m.sidebar_no_actions() : "No actions available"}
			</div>
		{/if}
	</DropdownMenuContent>
</DropdownMenu>
