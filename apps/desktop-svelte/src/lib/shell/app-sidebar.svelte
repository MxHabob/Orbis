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
  FileText,
} from "lucide-svelte";
import { authStore } from "$lib/stores";
import OrgSidebarSwitcher from "./org-sidebar-switcher.svelte";
import UserSidebarMenu from "./user-sidebar-menu.svelte";
import OnboardingPopover from "./onboarding-popover.svelte";
import UpgradeButton from "./upgrade-button.svelte";
import GitIntegrationPopover from "./git-integration-popover.svelte";
import SystemStatusPopover from "./system-status-popover.svelte";
import QuickActionsDropdown from "./quick-actions-dropdown.svelte";
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
  const unlocalizedPathname = decodeURIComponent(deLocalizeUrl(page.url).pathname);
  if (match === "exact") return unlocalizedPathname === href;
  return unlocalizedPathname.startsWith(href);
}
</script>

<TooltipProvider delayDuration={150}>
	<div class="flex h-full w-12 shrink-0 flex-col items-center justify-between py-3 select-none">
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
				<GitIntegrationPopover />
				<UpgradeButton />
				<SystemStatusPopover {orgId} />
				<QuickActionsDropdown {orgId} />
			</div>

			<UserSidebarMenu {orgId} />
		</div>
	</div>
</TooltipProvider>
