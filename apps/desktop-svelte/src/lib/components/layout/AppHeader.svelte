<!--
@component AppHeader
Three-zone header bar following GitButler's AppHeader pattern.
Left zone (actions) | Center zone (selector) | Right zone (actions).
Both left and right use flex-basis: 0 so center stays perfectly centered.
-->
<script lang="ts">
	import {
		DropdownMenu,
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuSeparator,
		DropdownMenuTrigger,
	} from '@orbis/ui-svelte/dropdown-menu';
	import {
		Search,
		Bell,
		School,
		ChevronDown,
		UserCircle,
		LogOut,
	} from 'lucide-svelte';
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
	const userInitials = $derived(
		(authStore.user?.full_name ?? 'User')
			.split(/\s+/)
			.filter(Boolean)
			.slice(0, 2)
			.map((part) => part[0]?.toUpperCase() ?? '')
			.join('') || 'U'
	);
</script>

<div
	class="flex h-[var(--header-height)] shrink-0 items-center justify-between gap-3 px-[14px]"
	class:pt-1={isDesktop}
	data-tauri-drag-region={isDesktop || undefined}
>
	<!-- Left Zone -->
	<div
		class="flex min-w-max flex-1 basis-0 items-center gap-2"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<button
			class="inline-flex h-8 items-center gap-1.5 rounded-[var(--radius-button)] border border-border bg-background px-2.5 text-[12px] text-muted-foreground transition-colors hover:border-border hover:bg-accent"
			title="Search (⌘K)"
			aria-label="Search"
		>
			<Search size={15} strokeWidth={2} />
			<span class="whitespace-nowrap">Search...</span>
			<kbd class="ml-2 inline-flex h-[18px] items-center rounded-[4px] border border-border bg-muted px-1 font-mono text-[10px] text-muted-foreground">⌘K</kbd>
		</button>
	</div>

	<!-- Center Zone -->
	<div
		class="flex min-w-0 shrink overflow-hidden"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<DropdownMenu>
			<DropdownMenuTrigger
				class="inline-flex h-8 items-center gap-1.5 whitespace-nowrap rounded-[var(--radius-button)] border border-transparent px-2.5 text-[12px] font-semibold text-foreground transition-colors hover:border-border hover:bg-accent"
				title="Switch organization"
			>
				<School size={15} strokeWidth={2} class="text-muted-foreground" />
				<span class="truncate">{orgName}</span>
				<ChevronDown size={12} strokeWidth={2} class="shrink-0 text-muted-foreground opacity-60" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="center" sideOffset={8} class="w-56">
				<DropdownMenuItem class="cursor-pointer">
					<UserCircle size={14} strokeWidth={1.8} />
					<span>{orgName}</span>
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem class="cursor-pointer">
					<School size={14} strokeWidth={1.8} />
					<span>Switch organization</span>
				</DropdownMenuItem>
				<DropdownMenuItem class="cursor-pointer text-destructive focus:text-destructive">
					<LogOut size={14} strokeWidth={1.8} />
					<span>Logout</span>
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	</div>

	<!-- Right Zone -->
	<div
		class="flex min-w-max flex-1 basis-0 items-center justify-end gap-2"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<button
			class="inline-flex h-8 w-8 items-center justify-center  text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
			title="Notifications"
			aria-label="Notifications"
		>
			<Bell size={15} strokeWidth={2} />
		</button>
	</div>
</div>
