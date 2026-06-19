<!--
@component AppHeader
Three-zone header bar following GitButler's AppHeader pattern.
Left zone (actions) | Center zone (selector) | Right zone (actions).
Both left and right use flex-basis: 0 so center stays perfectly centered.
-->
<script lang="ts">
	import { cn } from '../../../../../../packages/ui-svelte/utils';
	import { Search, Bell, Plus, School } from 'lucide-svelte';
	import { authStore } from '$lib/stores';
	import { isDesktopApp } from '$lib/services/ipc';

	let {
		orgId,
		orgName = 'Orbis School',
	}: {
		orgId: string;
		orgName?: string;
	} = $props();

	const isDesktop = isDesktopApp();
</script>

<div
	class="chrome-header"
	class:mac-offset={isDesktop}
	data-tauri-drag-region={isDesktop || undefined}
>
	<!-- Left Zone -->
	<div class="chrome-left" data-tauri-drag-region={isDesktop || undefined}>
		<button
			class="header-action-btn"
			title="Search (⌘K)"
			aria-label="Search"
		>
			<Search size={15} strokeWidth={2} />
			<span class="header-action-label">Search...</span>
			<kbd class="header-kbd">⌘K</kbd>
		</button>
	</div>

	<!-- Center Zone -->
	<div class="chrome-center" data-tauri-drag-region={isDesktop || undefined}>
		<button class="org-selector" title="Switch organization">
			<School size={15} strokeWidth={2} class="text-muted-foreground" />
			<span class="org-selector-label">{orgName}</span>
			<svg width="10" height="6" viewBox="0 0 10 6" fill="none" class="org-chevron">
				<path d="M1 1L5 5L9 1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
			</svg>
		</button>
	</div>

	<!-- Right Zone -->
	<div class="chrome-right" data-tauri-drag-region={isDesktop || undefined}>
		<button
			class="header-icon-btn"
			title="Notifications"
			aria-label="Notifications"
		>
			<Bell size={15} strokeWidth={2} />
		</button>

		<button
			class="header-icon-btn avatar-btn"
			title="Account"
			aria-label="Account"
		>
			<span class="avatar-initial">
				{authStore.user?.full_name?.charAt(0) ?? 'U'}
			</span>
		</button>
	</div>
</div>

<style>
	.chrome-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: var(--header-height);
		padding: 0 14px;
		gap: 12px;
		flex-shrink: 0;
	}

	.mac-offset {
		padding-top: 4px;
	}

	/* Left and Right flex equally so center stays centered */
	.chrome-left,
	.chrome-right {
		display: flex;
		align-items: center;
		flex-grow: 1;
		flex-basis: 0;
		min-width: max-content;
		gap: 8px;
	}

	.chrome-right {
		justify-content: flex-end;
	}

	.chrome-center {
		display: flex;
		flex-shrink: 1;
		overflow: hidden;
	}

	/* ── Buttons ── */
	.header-action-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		height: var(--size-button);
		padding: 0 10px;
		border-radius: var(--radius-button);
		border: 1px solid var(--panel-border);
		background: var(--panel-bg);
		color: hsl(var(--muted-foreground));
		font-size: 12px;
		transition:
			border-color var(--transition-fast),
			background-color var(--transition-fast);
	}

	.header-action-btn:hover {
		border-color: hsl(var(--border));
		background-color: hsl(var(--accent));
	}

	.header-action-label {
		white-space: nowrap;
	}

	.header-kbd {
		display: inline-flex;
		align-items: center;
		margin-left: 8px;
		padding: 0 4px;
		height: 18px;
		border-radius: 4px;
		background: hsl(var(--muted));
		border: 1px solid var(--panel-border);
		font-family: ui-monospace, monospace;
		font-size: 10px;
		color: hsl(var(--muted-foreground));
	}

	.header-icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: var(--size-button);
		height: var(--size-button);
		border-radius: var(--radius-button);
		color: hsl(var(--muted-foreground));
		transition:
			background-color var(--transition-fast),
			color var(--transition-fast);
	}

	.header-icon-btn:hover {
		background-color: hsl(var(--accent));
		color: hsl(var(--accent-foreground));
	}

	/* ── Org Selector ── */
	.org-selector {
		display: flex;
		align-items: center;
		gap: 6px;
		height: var(--size-button);
		padding: 0 10px;
		border-radius: var(--radius-button);
		border: 1px solid var(--panel-border);
		background: transparent;
		color: hsl(var(--foreground));
		font-size: 12px;
		font-weight: 600;
		white-space: nowrap;
		transition:
			border-color var(--transition-fast),
			background-color var(--transition-fast);
	}

	.org-selector:hover {
		border-color: hsl(var(--border));
		background-color: hsl(var(--accent));
	}

	.org-selector-label {
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.org-chevron {
		flex-shrink: 0;
		color: hsl(var(--muted-foreground));
		opacity: 0.6;
	}

	/* ── Avatar ── */
	.avatar-btn {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		background-color: hsl(var(--primary));
		color: hsl(var(--primary-foreground));
		font-size: 11px;
		font-weight: 600;
	}

	.avatar-btn:hover {
		opacity: 0.9;
	}

	.avatar-initial {
		text-transform: uppercase;
	}
</style>
