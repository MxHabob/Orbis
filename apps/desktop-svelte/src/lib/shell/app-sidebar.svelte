<!--
@component AppSidebar
Vertical icon-only sidebar navigation with active page indicator.
Follows GitButler's AppSidebar pattern: icons-only, top nav + bottom actions,
and an animated active indicator bar.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
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
  Settings,
  FileText,
  Brain,
} from "@lucide/svelte";
import OrgSidebarSwitcher from "./org-sidebar-switcher.svelte";
import UserSidebarMenu from "./user-sidebar-menu.svelte";
import OnboardingPopover from "./onboarding-popover.svelte";
import UpgradeButton from "./upgrade-button.svelte";
import GitIntegrationPopover from "./mulverse-integration-popover.svelte";
import AutomationStatusPopover from "./orbis-autopilot-popover.svelte";
import MoreActionsDropdown from "./more-actions-dropdown.svelte";

import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";
import { onboardingState } from "$lib/features/onboarding/state.svelte";
import { createOrganizationQuery } from "$lib/features/settings/queries";
import { onMount } from "svelte";
import { prefetchRouteData } from "$lib/core/ipc";

let {
  orgId,
  orgName: initialOrgName = "Orbis School",
}: {
  orgId: string;
  orgName?: string;
} = $props();

const orgQuery = createOrganizationQuery(() => orgId);

onMount(() => {
  onboardingState.load();
});

const fallbackName = $derived.by(() => {
  if (initialOrgName && initialOrgName !== "Orbis School")
    return initialOrgName;
  if (orgId) {
    return orgId
      .split("-")
      .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
      .join(" ");
  }
  return "Orbis School";
});

const orgNameDisplay = $derived(orgQuery.data?.name || fallbackName);

const navItems = $derived([
  {
    href: `/${orgId}`,
    icon: LayoutDashboard,
    label: m.common_nav_overview(),
    match: "exact",
  },
  {
    href: `/${orgId}/students`,
    icon: Users,
    label: m.common_word_students(),
    match: "prefix",
  },
  {
    href: `/${orgId}/academics`,
    icon: BookOpen,
    label: m.common_nav_academics(),
    match: "prefix",
  },
  {
    href: `/${orgId}/finance`,
    icon: Wallet,
    label: m.common_nav_finance(),
    match: "prefix",
  },
  {
    href: `/${orgId}/templates`,
    icon: FileText,
    label: m.settings_nav_templates(),
    match: "prefix",
  },
  {
    href: `/${orgId}/ai-center`,
    icon: Brain,
    label: "AI Center",
    match: "prefix",
  },
  {
    href: `/${orgId}/settings`,
    icon: Settings,
    label: m.common_word_settings(),
    match: "prefix",
  },
] as const);

function isActive(href: string, match: "exact" | "prefix"): boolean {
  const unlocalizedPathname = decodeURIComponent(
    deLocalizeUrl(page.url).pathname,
  );
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
				<div class="flex">
					<Tooltip>
						<TooltipTrigger>
							<Button
								href={localizeHref(item.href)}
								variant={isActive(item.href, item.match) ? 'default' : 'ghost'}
								class="rounded-[10px]"
								aria-label={item.label}
								aria-current={isActive(item.href, item.match) ? 'page' : undefined}
								onmouseenter={() => {
									// Prefetch data for the destination page on hover
									const segment = item.href.split('/').pop() || 'dashboard';
									prefetchRouteData(segment, orgId);
								}}
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
				<AutomationStatusPopover {orgId} />
				<MoreActionsDropdown />
			</div>

			<UserSidebarMenu {orgId} />
		</div>
	</div>
</TooltipProvider>
