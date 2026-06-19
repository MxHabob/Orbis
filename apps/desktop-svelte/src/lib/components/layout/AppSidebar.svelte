<!--
@component AppSidebar
Vertical icon-only sidebar navigation with active page indicator.
Follows GitButler's AppSidebar pattern: icons-only, top nav + bottom actions,
and an animated active indicator bar.
-->
<script lang="ts">
	import { page } from '$app/state';
	import {
		Avatar,
		AvatarFallback,
		AvatarImage,
		Tooltip,
		TooltipContent,
		TooltipProvider,
		TooltipTrigger,
	} from '@orbis/ui-svelte';
	import { cn } from '@orbis/ui-svelte/utils';
	import {
		LayoutDashboard,
		Users,
		BookOpen,
		Wallet,
		BarChart3,
		Settings,
		MessageSquare,
	} from 'lucide-svelte';
	import { slide } from 'svelte/transition';
	import { authStore } from '$lib/stores';

	let { orgId, disabled = false }: { orgId: string; disabled?: boolean } = $props();

	const navItems = $derived([
		{ href: `/${orgId}`, icon: LayoutDashboard, label: 'Overview', match: 'exact' },
		{ href: `/${orgId}/students`, icon: Users, label: 'Students', match: 'prefix' },
		{ href: `/${orgId}/academics`, icon: BookOpen, label: 'Academics', match: 'prefix' },
		{ href: `/${orgId}/finance`, icon: Wallet, label: 'Finance', match: 'prefix' },
		{ href: `/${orgId}/reports`, icon: BarChart3, label: 'Reports', match: 'prefix' },
	] as const);

	const bottomItems = $derived([
		{ href: `/${orgId}/settings`, icon: Settings, label: 'Settings', match: 'prefix' },
	] as const);

	const userInitials = $derived(
		(authStore.user?.full_name ?? 'User')
			.split(/\s+/)
			.filter(Boolean)
			.slice(0, 2)
			.map((part) => part[0]?.toUpperCase() ?? '')
			.join('') || 'U'
	);

	function isActive(href: string, match: 'exact' | 'prefix'): boolean {
		const pathname = page.url.pathname;
		if (match === 'exact') return pathname === href;
		return pathname.startsWith(href);
	}
</script>

<TooltipProvider delayDuration={150}>
	<div class="sidebar">
		<div class="sidebar-top">
			{#each navItems as item (item.href)}
				<div class="sidebar-item-wrap">
					{#if isActive(item.href, item.match)}
						<div class="active-indicator" in:slide={{ axis: 'x', duration: 150 }}></div>
					{/if}
					<Tooltip>
						<TooltipTrigger asChild>
							<a
								href={item.href}
								class={cn(
									'sidebar-btn',
									isActive(item.href, item.match) && 'sidebar-btn-active'
								)}
								aria-label={item.label}
								aria-current={isActive(item.href, item.match) ? 'page' : undefined}
								class:disabled
							>
								<item.icon size={18} strokeWidth={1.8} />
							</a>
						</TooltipTrigger>
						<TooltipContent side="right" sideOffset={10}>{item.label}</TooltipContent>
					</Tooltip>
				</div>
			{/each}
		</div>

		<div class="sidebar-bottom">
			{#each bottomItems as item (item.href)}
				<div class="sidebar-item-wrap">
					{#if isActive(item.href, item.match)}
						<div class="active-indicator" in:slide={{ axis: 'x', duration: 150 }}></div>
					{/if}
					<Tooltip>
						<TooltipTrigger asChild>
							<a
								href={item.href}
								class={cn(
									'sidebar-btn',
									isActive(item.href, item.match) && 'sidebar-btn-active'
								)}
								aria-label={item.label}
								class:disabled
							>
								<item.icon size={18} strokeWidth={1.8} />
							</a>
						</TooltipTrigger>
						<TooltipContent side="right" sideOffset={10}>{item.label}</TooltipContent>
					</Tooltip>
				</div>
			{/each}

			<div class="sidebar-item-wrap">
				<Tooltip>
					<TooltipTrigger asChild>
						<button class="sidebar-btn sidebar-btn-ghost" aria-label="Feedback">
							<MessageSquare size={16} strokeWidth={1.8} />
						</button>
					</TooltipTrigger>
					<TooltipContent side="right" sideOffset={10}>Feedback</TooltipContent>
				</Tooltip>
			</div>

			<div class="sidebar-item-wrap sidebar-user-wrap">
				<Tooltip>
					<TooltipTrigger asChild>
						<button class="sidebar-user-btn" aria-label="Account">
							<Avatar>
								<AvatarImage
									src={authStore.user?.avatar ?? undefined}
									alt={authStore.user?.full_name ?? 'User'}
								/>
								<AvatarFallback>{userInitials}</AvatarFallback>
							</Avatar>
						</button>
					</TooltipTrigger>
					<TooltipContent side="right" sideOffset={10}>
						{authStore.user?.full_name ?? 'Account'}
					</TooltipContent>
				</Tooltip>
			</div>
		</div>
	</div>
</TooltipProvider>

<style>
	.sidebar {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
		width: var(--sidebar-width);
		height: 100%;
		padding: 12px 0;
		flex-shrink: 0;
		box-sizing: border-box;
	}

	.sidebar-top,
	.sidebar-bottom {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.sidebar-bottom {
		gap: 8px;
	}

	.sidebar-item-wrap {
		position: relative;
		display: flex;
		justify-content: center;
		width: 100%;
	}

	.active-indicator {
		position: absolute;
		inset-inline-start: 0;
		top: 50%;
		width: 3px;
		height: 28px;
		transform: translateY(-50%);
		border-radius: 999px;
		background-color: var(--active-indicator);
		pointer-events: none;
		z-index: 1;
	}

	.sidebar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: var(--radius-ml);
		color: hsl(var(--muted-foreground));
		border: 1px solid transparent;
		transition:
			background-color var(--transition-fast),
			color var(--transition-fast),
			border-color var(--transition-fast);
	}

	.sidebar-btn:hover:not(.disabled) {
		background-color: hsl(var(--accent));
		color: hsl(var(--accent-foreground));
		border-color: var(--panel-border);
	}

	.sidebar-btn-active {
		background-color: var(--panel-bg);
		color: hsl(var(--foreground));
		border-color: var(--panel-border);
	}

	.sidebar-btn-ghost {
		opacity: 0.4;
	}

	.sidebar-btn-ghost:hover {
		opacity: 0.7;
	}

	.sidebar-user-wrap {
		margin-top: 2px;
	}

	.sidebar-user-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: var(--radius-ml);
		cursor: pointer;
	}

	.sidebar-user-btn:hover {
		background-color: hsl(var(--accent));
	}

	.sidebar-avatar {
		width: 36px;
		height: 36px;
		flex-shrink: 0;
	}

	.disabled {
		pointer-events: none;
		opacity: 0.5;
	}
</style>
