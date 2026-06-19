<script lang="ts">
	import { cn } from '@orbis/ui-svelte/utils';
	import { Zap } from 'lucide-svelte';
	import { page } from '$app/state';

	let isExpanded = $state(false);

	const navItems = [
		{ href: '/', icon: 'layout-dashboard', label: 'Overview' },
		{ href: '/grades', icon: 'graduation-cap', label: 'Grades' },
		{ href: '/reports', icon: 'file-bar-chart', label: 'Reports' },
		{ href: '/vault', icon: 'folder-lock', label: 'Vault' },
		{ href: '/settings', icon: 'settings', label: 'Settings' },
		{ href: '/account', icon: 'user', label: 'Account' },
	];

	function isActive(href: string) {
		if (href === '/') return page.url.pathname === '/';
		return page.url.pathname.startsWith(href);
	}
</script>

<aside
	class={cn(
		'h-screen shrink-0 flex-col overflow-hidden rounded-tl-[10px] rounded-bl-[10px] justify-between fixed top-0 pb-4 items-center flex z-50 transition-all duration-200 ease-[cubic-bezier(0.4,0,0.2,1)]',
		'bg-background border-e border-border inset-s-0',
		isExpanded ? 'w-60' : 'w-17.5'
	)}
	role="navigation"
	onmouseenter={() => (isExpanded = true)}
	onmouseleave={() => (isExpanded = false)}
>
	<!-- Logo -->
	<div
		class={cn(
			'absolute top-0 inset-s-0 h-17.5 flex items-center justify-center bg-background border-b border-border transition-all duration-200 ease-[cubic-bezier(0.4,0,0.2,1)]',
			isExpanded ? 'w-full' : 'w-17.25'
		)}
	>
		<a href="/" class="absolute inset-s-5.5 transition-none">
			<Zap class="h-6 w-6" />
		</a>
	</div>

	<!-- Nav Items -->
	<div class="flex flex-col w-full pt-17.5 flex-1 border-b border-border mb-3">
		<nav class="flex flex-col gap-1 p-2">
			{#each navItems as item (item.href)}
				<a
					href={item.href}
					class={cn(
						'flex items-center gap-3 rounded-md px-3 py-2.5 text-sm font-medium transition-colors duration-150',
						isActive(item.href)
							? 'bg-primary/10 text-primary'
							: 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'
					)}
				>
					<span class="h-5 w-5 shrink-0 flex items-center justify-center">
						<Zap class="h-4 w-4" />
					</span>
					{#if isExpanded}
						<span class="truncate">{item.label}</span>
					{/if}
				</a>
			{/each}
		</nav>
	</div>
</aside>
