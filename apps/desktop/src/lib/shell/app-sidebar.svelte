<!--
@component AppSidebar
Vertical icon-only sidebar navigation with active page indicator.
Follows GitButler's AppSidebar pattern: icons-only, top nav + bottom actions,
and an animated active indicator bar.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { Button } from "@orbis/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@orbis/ui";
import {
  AiCenter,
  BookOpen,
  EmployeeAffairs,
  FinanceSystem,
  Grid,
  Settings,
  StudentAffairs,
  TemplateLibrary,
} from "@orbis/icons";
import OrgSidebarSwitcher from "./org-sidebar-switcher.svelte";
import UserSidebarMenu from "./user-sidebar-menu.svelte";
import UpgradeButton from "./upgrade-button.svelte";
import GitIntegrationPopover from "./mulverse-integration-popover.svelte";
import AutomationStatusPopover from "./orbis-autopilot-popover.svelte";
import MoreActionsDropdown from "./more-actions-dropdown.svelte";
import OnboardingPopover from "$lib/features/onboarding-wizard/onboarding-popover.svelte";
import SystemClock from "./system-clock.svelte";

import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";
import { createOrganizationQuery } from "$lib/features/settings/queries";
import { l } from "$lib/core/i18n/locale";
import { onMount } from "svelte";
import { prefetchRouteData } from "$lib/api";

let {
  orgId,
  orgName: initialOrgName,
}: {
  orgId: string;
  orgName?: string | null;
} = $props();

const orgQuery = createOrganizationQuery(() => orgId);
const orgNameDisplay = $derived(
  l(orgQuery.data?.name_ar, orgQuery.data?.name || initialOrgName || ""),
);
const orgNameArabic = $derived(orgQuery.data?.name_ar ?? null);
const orgPrimaryColor = $derived(orgQuery.data?.primary_color ?? "var(--primary)");

const navItems = $derived([
  {
    href: `/${orgId}`,
    icon: Grid,
    label: m.common_nav_overview(),
    match: "exact",
  },
  {
    href: `/${orgId}/students`,
    icon: StudentAffairs,
    label: m.common_word_students(),
    match: "prefix",
  },
  {
    href: `/${orgId}/employees`,
    icon: EmployeeAffairs,
    label: m.employees_nav_title(),
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
    icon: FinanceSystem,
    label: m.common_nav_finance(),
    match: "prefix",
  },
  {
    href: `/${orgId}/templates`,
    icon: TemplateLibrary,
    label: m.settings_nav_templates(),
    match: "prefix",
  },
  {
    href: `/${orgId}/ai-center`,
    icon: AiCenter,
    label: m.common_autopilot_popover_ai_center(),
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
			<OrgSidebarSwitcher
        {orgId}
        orgName={orgNameDisplay}
        orgNameAr={orgNameArabic}
        primaryColor={orgPrimaryColor}
      />

			<!-- Divider -->
			<div class="w-8 h-px bg-slate-200/80 dark:bg-zinc-700 self-center"></div>

			{#each navItems as item (item.href)}
				<div class="flex">
					<Tooltip>
						<TooltipTrigger>
							<Button
								href={localizeHref(item.href)}
								variant={isActive(item.href, item.match) ? 'default' : 'ghost'}
								size="icon"
								class="rounded-[10px]"
								aria-label={item.label}
								aria-current={isActive(item.href, item.match) ? 'page' : undefined}
								onmouseenter={() => {
									// Prefetch data for the destination page on hover
									const segment = item.href.split('/').pop() || 'dashboard';
									prefetchRouteData(segment, orgId);
								}}
							>
								<item.icon size={20} />
							</Button>
						</TooltipTrigger>
						<TooltipContent side="inline-end" sideOffset={12}>{item.label}</TooltipContent>
					</Tooltip>
				</div>
			{/each}
		</div>

		<div class="flex flex-col gap-2 items-center w-full relative">
			<div class="flex flex-col items-center w-8 gap-1.5">
				<OnboardingPopover {orgId} />
				<GitIntegrationPopover {orgId} />
				<UpgradeButton />
				<AutomationStatusPopover {orgId} />
        <MoreActionsDropdown orgName={orgNameDisplay} />
			</div>
      <SystemClock />
			<UserSidebarMenu {orgId} />
		</div>
	</div>
</TooltipProvider>
