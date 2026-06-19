<!--
@component AppSidebar
Vertical icon-only sidebar navigation with active page indicator.
Follows GitButler's AppSidebar pattern: icons-only, top nav + bottom actions,
and an animated active indicator bar.
-->
<script lang="ts">
	import { page } from '$app/state';
	import { Avatar, AvatarFallback, AvatarImage } from '@orbis/ui-svelte';
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

<div class="sidebar">
	<div class="sidebar-top">
		{#each navItems as item (item.href)}
			<div class="sidebar-item-wrap">
				{#if isActive(item.href, item.match)}
					<div class="active-indicator" in:slide={{ axis: 'x', duration: 150 }}></div>
				{/if}
				<a
					href={item.href}
					class={cn(
						'sidebar-btn',
						isActive(item.href, item.match) && 'sidebar-btn-active'
					)}
					title={item.label}
					aria-label={item.label}
					aria-current={isActive(item.href, item.match) ? 'page' : undefined}
					class:disabled
				>
					<item.icon size={18} strokeWidth={1.8} />
				</a>
			</div>
		{/each}
	</div>

	<div class="sidebar-bottom">
		{#each bottomItems as item (item.href)}
			<div class="sidebar-item-wrap">
				{#if isActive(item.href, item.match)}
					<div class="active-indicator" in:slide={{ axis: 'x', duration: 150 }}></div>
				{/if}
				<a
					href={item.href}
					class={cn(
						'sidebar-btn',
						isActive(item.href, item.match) && 'sidebar-btn-active'
					)}
					title={item.label}
					aria-label={item.label}
					class:disabled
				>
					<item.icon size={18} strokeWidth={1.8} />
				</a>
			</div>
		{/each}

		<div class="sidebar-item-wrap">
			<button
				class="sidebar-btn sidebar-btn-ghost"
				title="Feedback"
				aria-label="Feedback"
			>
				<MessageSquare size={16} strokeWidth={1.8} />
			</button>
		</div>

		<div class="sidebar-item-wrap sidebar-user-wrap">
			<button
				class="sidebar-user-btn"
				title={authStore.user?.full_name ?? 'Account'}
				aria-label="Account"
			>
				<Avatar class="sidebar-avatar">
					<AvatarImage
						src={authStore.user?.avatar ?? undefined}
						alt={authStore.user?.full_name ?? 'User'}
					/>
					<AvatarFallback>{userInitials}</AvatarFallback>
				</Avatar>
			</button>
		</div>
	</div>
</div>

<style>
	.sidebar {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
		width: var(--sidebar-width);
		height: 100%;
		padding: 0 12px 12px 12px;
		flex-shrink: 0;
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
	}

	.active-indicator {
		position: absolute;
		left: -4px;
		top: 50%;
		width: 10px;
		height: 16px;
		transform: translate(-50%, -50%);
		border-radius: var(--radius-m);
		background-color: var(--active-indicator);
	}

	.sidebar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
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
		padding: 0;
		background: transparent;
		border: none;
	}

	.sidebar-avatar {
		width: 36px;
		height: 36px;
	}

	.disabled {
		pointer-events: none;
		opacity: 0.5;
	}
</style>
