<script lang="ts">
	import { Search, Bell, User } from 'lucide-svelte';
	import { authStore } from '$lib/stores';
</script>

<header
	class="md:m-0 z-50 px-6 md:border-b h-[70px] flex justify-between items-center top-0 backdrop-blur-xl bg-background bg-opacity-70 rounded-t-[10px] transition-transform"
>
	<!-- Search -->
	<button
		class="flex items-center gap-2 rounded-md border border-border bg-background px-3 py-1.5 text-sm text-muted-foreground hover:bg-accent transition-colors"
	>
		<Search class="h-4 w-4" />
		<span>Search...</span>
		<kbd class="ml-4 pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border border-border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground">
			⌘K
		</kbd>
	</button>

	<!-- Right side -->
	<div class="flex gap-2 ms-auto items-center">
		<button class="inline-flex items-center justify-center h-9 w-9 rounded-md hover:bg-accent transition-colors">
			<Bell class="h-4 w-4 text-muted-foreground" />
		</button>
		<button class="inline-flex items-center justify-center h-8 w-8 rounded-full bg-primary text-primary-foreground text-xs font-medium">
			{authStore.user?.full_name?.charAt(0) ?? 'U'}
		</button>
	</div>
</header>
