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
  Radio,
  FileText,
} from "lucide-svelte";
import { authStore, uiStore } from "$lib/stores";
import OrgSidebarSwitcher from "./OrgSidebarSwitcher.svelte";
import UserSidebarMenu from "./UserSidebarMenu.svelte";
import OnboardingPopover from "./OnboardingPopover.svelte";
import * as m from "$lib/paraglide/messages";
import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";
import { onboardingState } from "$lib/components/onboarding/onboarding-state.svelte";
import { onMount } from "svelte";

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
</script>

<TooltipProvider delayDuration={150}>
	<div class="flex h-full w-15 shrink-0 flex-col items-center justify-between py-3 select-none">
		<div class="flex flex-col items-center gap-3">
			<OrgSidebarSwitcher {orgId} orgName={orgNameDisplay} />

			<!-- Divider -->
			<div class="w-8 h-px bg-slate-200/80 dark:bg-zinc-700  self-center"></div>

			{#each navItems as item (item.href)}
				<div class="flex">
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

				<Tooltip>
					<TooltipTrigger>
						{#snippet child({ props })}
							<Button 
								{...props}
								variant="outline" 
								size="icon-sm" 
								class="relative cursor-pointer rounded-[10px]"
							>
								<Radio size={16} strokeWidth={1.8} class="text-foreground" />
								<span class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full bg-primary border border-background"></span>
							</Button>
						{/snippet}
					</TooltipTrigger>
					<TooltipContent side="right" sideOffset={12}>
						{m.nav_configure_seer_tooltip()}
					</TooltipContent>
				</Tooltip>

				<Tooltip>
					<TooltipTrigger>
						{#snippet child({ props })}
							<Button 
								{...props}
								variant="outline" 
								size="icon-sm" 
								class="relative cursor-pointer rounded-[10px]"
							>
								<MoreHorizontal size={16} strokeWidth={1.8} class="text-foreground" />
							</Button>
						{/snippet}
					</TooltipTrigger>
					<TooltipContent side="right" sideOffset={12}>
						{m.nav_more_tooltip()}
					</TooltipContent>
				</Tooltip>
			</div>

			<UserSidebarMenu {orgId} />
		</div>
	</div>
</TooltipProvider>
