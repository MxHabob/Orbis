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
  MessageSquare,
} from "lucide-svelte";
import { authStore } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";

let { orgId, disabled = false }: { orgId: string; disabled?: boolean } =
  $props();

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
    href: `/${orgId}/settings`,
    icon: Settings,
    label: m.nav_settings(),
    match: "prefix",
  },
] as const);

const bottomItems = $derived([
  {
    href: `/${orgId}/settings`,
    icon: Settings,
    label: m.nav_settings(),
    match: "prefix",
  },
] as const);

const userInitials = $derived(
  (authStore.user?.full_name ?? "User")
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() ?? "")
    .join("") || "U",
);

function isActive(href: string, match: "exact" | "prefix"): boolean {
  const unlocalizedPathname = deLocalizeUrl(page.url).pathname;
  if (match === "exact") return unlocalizedPathname === href;
  return unlocalizedPathname.startsWith(href);
}
</script>

<TooltipProvider delayDuration={150}>
	<div class="flex h-full w-15 shrink-0 flex-col items-center justify-between py-3">
		<div class="flex flex-col gap-3">
			{#each navItems as item (item.href)}
				<div class="flex">
					<Tooltip>
						<TooltipTrigger asChild>
							<Button
								href={localizeHref(item.href)}
								variant={isActive(item.href, item.match) ? 'default' : 'ghost'}
								aria-label={item.label}
								aria-current={isActive(item.href, item.match) ? 'page' : undefined}
							>
								<item.icon size={18} strokeWidth={1.8} />
							</Button>
						</TooltipTrigger>
						<TooltipContent side="right" sideOffset={12}>{item.label}</TooltipContent>
					</Tooltip>
				</div>
			{/each}
		</div>

		<div class="flex flex-col gap-3">
			{#each bottomItems as item (item.href)}
				<div class="flex">
					<Tooltip>
						<TooltipTrigger asChild>
							<Button
								href={localizeHref(item.href)}
								variant="lime"
								aria-label={item.label}
							>
								<item.icon size={18} strokeWidth={1.8} />
							</Button>
						</TooltipTrigger>
						<TooltipContent side="right" sideOffset={10}>{item.label}</TooltipContent>
					</Tooltip>
				</div>
			{/each}

			<div class="flex">
				<Tooltip>
					<TooltipTrigger asChild>
						<Button variant="ghost" size="icon" class="sidebar-btn sidebar-btn-ghost" aria-label={m.nav_feedback()}>
							<MessageSquare size={16} strokeWidth={1.8} />
						</Button>
					</TooltipTrigger>
					<TooltipContent side="right" sideOffset={10}>{m.nav_feedback()}</TooltipContent>
				</Tooltip>
			</div>

			<Tooltip>
			  <TooltipTrigger asChild>
				<Button variant="ghost" aria-label={m.nav_account()}>
				 <Avatar>
					<AvatarImage
						src={authStore.user?.avatar ?? undefined}
						alt={authStore.user?.full_name ?? 'User'}
					/>
					<AvatarFallback>{userInitials}</AvatarFallback>
				 </Avatar>
				</Button>
			   </TooltipTrigger>
			  <TooltipContent side="right" sideOffset={10}>
					{authStore.user?.full_name ?? m.nav_account()}
				</TooltipContent>
			</Tooltip>
		</div>
	</div>
</TooltipProvider>
