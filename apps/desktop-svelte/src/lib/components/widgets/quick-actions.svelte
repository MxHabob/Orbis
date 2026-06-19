<script lang="ts">
	import { UserPlus, Wallet, BookOpen, BarChart3 } from "lucide-svelte";

	let { orgId }: { orgId: string } = $props();

	const ACTIONS = $derived([
		{
			label: "Enroll Student",
			icon: UserPlus,
			href: `/${orgId}/students/new`
		},
		{
			label: "Record Payment",
			icon: Wallet,
			href: `/${orgId}/finance/payments`
		},
		{
			label: "Manage Classes",
			icon: BookOpen,
			href: `/${orgId}/academics`
		},
		{
			label: "View Reports",
			icon: BarChart3,
			href: `/${orgId}/reports`
		}
	] as const);

	const buttonClassName =
		"flex items-center gap-1.5 border bg-white border-[#e6e6e6] hover:bg-[#f7f7f7] hover:border-[#d0d0d0] dark:border-[#1d1d1d] dark:bg-[#0c0c0c] dark:hover:bg-[#0f0f0f] dark:hover:border-[#222222] px-3 py-1.5 text-xs text-muted-foreground/60 hover:text-foreground transition-all duration-300 rounded-md cursor-pointer group";

	const iconClassName =
		"text-muted-foreground/40 group-hover:text-foreground transition-colors duration-300";
</script>

<div class="flex items-center justify-center gap-3 pt-2 pb-12 w-full flex-wrap">
	{#each ACTIONS as action (action.label)}
		{@const Icon = action.icon}
		<a
			href={action.href}
			class={buttonClassName}
		>
			<Icon size={13} class={iconClassName} />
			<span>{action.label}</span>
		</a>
	{/each}
</div>
