<!--
@component AppSidebar
Vertical icon-only sidebar navigation with active page indicator.
Follows GitButler's AppSidebar pattern: icons-only, top nav + bottom actions,
and an animated active indicator bar.
-->
<script lang="ts">
import { page } from "$app/state";
import { Button } from "@orbis/ui-svelte/button";
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@orbis/ui-svelte";
import {
  LayoutDashboard,
  Users,
  BookOpen,
  Wallet,
  BarChart3,
  Settings,
  MoreHorizontal,
  Zap,
  Activity,
  FileText,
  UserPlus,
  CalendarClock,
  CheckSquare,
  ShieldCheck,
  CreditCard,
} from "lucide-svelte";
import { authStore, uiStore, academicContext, workflowInbox } from "$lib/stores";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@orbis/ui-svelte/popover";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
} from "@orbis/ui-svelte/dropdown-menu";
import OrgSidebarSwitcher from "./org-sidebar-switcher.svelte";
import UserSidebarMenu from "./user-sidebar-menu.svelte";
import OnboardingPopover from "./onboarding-popover.svelte";
import * as m from "$lib/paraglide/messages";
import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";
import { onboardingState } from "$lib/features/onboarding/state.svelte";
import { onMount } from "svelte";
import { prefetchRouteData } from "$lib/core/ipc";

let {
  orgId,
  orgName: initialOrgName = "Orbis School",
  disabled = false,
}: {
  orgId: string;
  orgName?: string;
  disabled?: boolean;
} = $props();

let localOrgName = $state("");

onMount(() => {
  onboardingState.load();
});

$effect(() => {
  if (initialOrgName && initialOrgName !== "Orbis School") {
    localOrgName = initialOrgName;
    return;
  }
  const storedCustom = localStorage.getItem("orbis_custom_orgs");
  if (storedCustom) {
    try {
      const customProjects = JSON.parse(storedCustom);
      const match = customProjects.find((p: any) => p.id === orgId);
      if (match) {
        localOrgName = match.label;
        return;
      }
    } catch {}
  }
  if (orgId) {
    localOrgName = orgId
      .split("-")
      .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
      .join(" ");
  } else {
    localOrgName = "Orbis School";
  }
});

const orgNameDisplay = $derived(
  localOrgName || initialOrgName || "Orbis School",
);

const navItems = $derived([
  {
    href: `/${orgId}`,
    icon: LayoutDashboard,
    label: m.nav_overview(),
    match: "exact",
  },
  {
    href: `/${orgId}/reports`,
    icon: BarChart3,
    label: m.nav_reports(),
    match: "exact",
  },
  {
    href: `/${orgId}/students`,
    icon: Users,
    label: m.nav_students(),
    match: "prefix",
  },
  {
    href: `/${orgId}/academics`,
    icon: BookOpen,
    label: m.nav_academics(),
    match: "prefix",
  },
  {
    href: `/${orgId}/finance`,
    icon: Wallet,
    label: m.nav_finance(),
    match: "prefix",
  },
  {
    href: `/${orgId}/templates`,
    icon: FileText,
    label: m.settings_nav_templates
      ? m.settings_nav_templates()
      : "قوالب المستندات",
    match: "prefix",
  },
  {
    href: `/${orgId}/settings`,
    icon: Settings,
    label: m.nav_settings(),
    match: "prefix",
  },
] as const);

const userInitials = $derived(
  (authStore.user?.full_name ?? "User").trim().charAt(0).toUpperCase() || "U",
);

const orgInitials = $derived(
  orgNameDisplay.trim().charAt(0).toUpperCase() || "O",
);

function isActive(href: string, match: "exact" | "prefix"): boolean {
  const unlocalizedPathname = deLocalizeUrl(page.url).pathname;
  if (match === "exact") return unlocalizedPathname === href;
  return unlocalizedPathname.startsWith(href);
}

// Current page segment for context-aware quick actions
const currentSegment = $derived.by(() => {
  const parts = deLocalizeUrl(page.url).pathname.split("/").filter(Boolean);
  return parts[1] ?? "dashboard";
});

// Phase label for the activity status
const phaseLabel = $derived.by(() => {
  const phase = academicContext.cyclePhase;
  switch (phase) {
    case "pre_start": return m.phase_pre_start ? m.phase_pre_start() : "Pre-Start";
    case "active": return m.phase_active ? m.phase_active() : "Active";
    case "exams": return m.phase_exams ? m.phase_exams() : "Exams";
    case "grade_entry": return m.phase_grade_entry ? m.phase_grade_entry() : "Grade Entry";
    case "closed": return m.phase_closed ? m.phase_closed() : "Closed";
    default: return "";
  }
});

// Context-aware quick actions based on current page
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

<TooltipProvider delayDuration={150}>
	<div class="flex h-full w-15 shrink-0 flex-col items-center justify-between py-3 select-none">
		<div class="flex flex-col items-center gap-3">
			<OrgSidebarSwitcher {orgId} orgName={orgNameDisplay} />

			<!-- Divider -->
			<div class="w-8 h-px bg-slate-200/80 dark:bg-zinc-700  self-center"></div>

			{#each navItems as item (item.href)}
				<div
					class="flex"
					onmouseenter={() => {
						// Prefetch data for the destination page on hover
						const segment = item.href.split('/').pop() || 'dashboard';
						prefetchRouteData(segment, orgId);
					}}
				>
					<Tooltip>
						<TooltipTrigger>
							<Button
								href={localizeHref(item.href)}
								variant={isActive(item.href, item.match) ? 'default' : 'ghost'}
								class="rounded-[10px]"
								aria-label={item.label}
								aria-current={isActive(item.href, item.match) ? 'page' : undefined}
							>
								<item.icon size={20} strokeWidth={1.8} />
							</Button>
						</TooltipTrigger>
						<TooltipContent side="right" sideOffset={12}>{item.label}</TooltipContent>
					</Tooltip>
				</div>
			{/each}
		</div>

		<div class="flex flex-col gap-2 items-center w-full relative">
			<div class="flex flex-col items-center w-8 gap-1.5">
				<OnboardingPopover />
				
				<Tooltip>
					<TooltipTrigger>
						{#snippet child({ props })}
							<Button
								{...props}
								variant="outline"
								size="icon-sm"
								class="relative cursor-pointer rounded-[10px]"
								onclick={() => uiStore.setUpgradeDialog(true)}
							>
								<Zap size={16} strokeWidth={1.8} class="text-foreground" />
								<span class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full bg-primary border border-background"></span>
							</Button>
						{/snippet}
					</TooltipTrigger>
					<TooltipContent side="right" sideOffset={12}>
						{m.upgrade_title()}
					</TooltipContent>
				</Tooltip>

				<!-- Activity Status Popover -->
				<Popover>
					<PopoverTrigger>
						{#snippet child({ props })}
							<Button 
								{...props}
								variant="outline" 
								size="icon-sm" 
								class="relative cursor-pointer rounded-[10px]"
							>
								<Activity size={16} strokeWidth={1.8} class="text-foreground" />
								{#if workflowInbox.hasActions}
									<span class="absolute -top-0.5 -end-0.5 w-1.5 h-1.5 rounded-full bg-destructive border border-background animate-pulse"></span>
								{:else}
									<span class="absolute -top-0.5 -end-0.5 w-1.5 h-1.5 rounded-full bg-primary border border-background"></span>
								{/if}
							</Button>
						{/snippet}
					</PopoverTrigger>
					<PopoverContent side="right" sideOffset={12} class="w-56 p-0">
						<div class="p-3 border-b border-border/40">
							<p class="text-xs font-semibold text-foreground">{m.sidebar_status_title ? m.sidebar_status_title() : "System Status"}</p>
						</div>
						<div class="p-3 flex flex-col gap-2.5">
							<!-- Academic Phase -->
							<div class="flex items-center justify-between text-xs">
								<span class="text-muted-foreground">{m.sidebar_status_phase ? m.sidebar_status_phase() : "Phase"}</span>
								{#if phaseLabel}
									<Badge variant="secondary" class="text-[10px] h-4 px-1.5">{phaseLabel}</Badge>
								{:else}
									<span class="text-muted-foreground">—</span>
								{/if}
							</div>
							<!-- Year Status -->
							<div class="flex items-center justify-between text-xs">
								<span class="text-muted-foreground">{m.sidebar_status_year ? m.sidebar_status_year() : "Year"}</span>
								<span class="font-medium text-foreground">{academicContext.yearLabel || "—"}</span>
							</div>
							<!-- Pending Approvals -->
							<div class="flex items-center justify-between text-xs">
								<span class="text-muted-foreground">{m.approvals_tab_pending ? m.approvals_tab_pending() : "Pending"}</span>
								{#if workflowInbox.hasActions}
									<Badge variant="destructive" class="text-[10px] h-4 px-1.5">{workflowInbox.totalCount}</Badge>
								{:else}
									<span class="text-primary text-[10px] font-medium">✓</span>
								{/if}
							</div>
							<!-- Read-Only Mode -->
							{#if academicContext.isReadOnly}
								<div class="flex items-center gap-1.5 text-[10px] text-muted-foreground bg-muted/50 rounded px-2 py-1">
									<span>🔒</span>
									<span>{m.year_archived_banner ? m.year_archived_banner() : "Read-only mode"}</span>
								</div>
							{/if}
						</div>
					</PopoverContent>
				</Popover>

				<!-- Quick Actions Dropdown -->
				<DropdownMenu>
					<DropdownMenuTrigger>
						{#snippet child({ props })}
							<Button 
								{...props}
								variant="outline" 
								size="icon-sm" 
								class="relative cursor-pointer rounded-[10px]"
							>
								<MoreHorizontal size={16} strokeWidth={1.8} class="text-foreground" />
								{#if quickActions.length > 0}
									<span class="absolute -top-0.5 -end-0.5 size-3.5 bg-primary text-primary-foreground text-[8px] rounded-full flex items-center justify-center font-bold border border-background">
										{quickActions.length}
									</span>
								{/if}
							</Button>
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
			</div>

			<UserSidebarMenu {orgId} />
		</div>
	</div>
</TooltipProvider>
