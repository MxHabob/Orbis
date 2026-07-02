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
    label: m.settings_nav_templates ? m.settings_nav_templates() : "قوالب المستندات",
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
			<div class="w-8 h-px bg-slate-200/80 dark:bg-zinc-700 my-1 self-center"></div>

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
			<div class="flex flex-col items-center w-8 gap-1 overflow-hidden">
				<OnboardingPopover />
				<Button
					variant="outline"
					size="icon"
					class="w-full h-8 flex items-center justify-center hover:bg-slate-50 dark:hover:bg-zinc-800/60 relative border-b border-[#f1f5f9] dark:border-zinc-800 cursor-pointer"
					onclick={() => uiStore.setUpgradeDialog(true)}
				>
					<div class="relative">
						<div class="w-[20px] h-[20px] rounded-full border border-slate-800 dark:border-zinc-400 text-slate-800 dark:text-zinc-400 flex items-center justify-center">
							<Zap size={10} strokeWidth={2} />
						</div>
						<span class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full bg-[#6366f1] border border-white dark:border-zinc-900"></span>
					</div>
				</Button>
				<Button variant="outline" size="icon"  class="w-full h-8 flex items-center justify-center hover:bg-slate-50 dark:hover:bg-zinc-800/60 relative border-b border-[#f1f5f9] dark:border-zinc-800 cursor-pointer">
					<div class="relative">
						<div class="w-[20px] h-[20px] rounded-full border border-slate-800 dark:border-zinc-400 text-slate-800 dark:text-zinc-400 flex items-center justify-center">
							<Radio size={10} strokeWidth={2} />
						</div>
						<span class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full bg-[#6366f1] border border-white dark:border-zinc-900"></span>
					</div>
				</Button>
				<Button variant="outline" size="icon"  class="w-full h-8 flex items-center justify-center hover:bg-slate-50 dark:hover:bg-zinc-800/60 cursor-pointer">
					<MoreHorizontal size={14} class="text-slate-600 dark:text-zinc-400" />
				</Button>
			</div>

			<UserSidebarMenu {orgId} />
		</div>
	</div>
</TooltipProvider>
