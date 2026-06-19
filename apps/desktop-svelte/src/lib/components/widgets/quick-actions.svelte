<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { UserPlus, Wallet, BookOpen, BarChart3 } from "lucide-svelte";

let { orgId }: { orgId: string } = $props();

const ACTIONS = $derived([
  {
    label: "Enroll Student",
    icon: UserPlus,
    href: `/${orgId}/students/new`,
  },
  {
    label: "Record Payment",
    icon: Wallet,
    href: `/${orgId}/finance/payments`,
  },
  {
    label: "Manage Classes",
    icon: BookOpen,
    href: `/${orgId}/academics`,
  },
  {
    label: "View Reports",
    icon: BarChart3,
    href: `/${orgId}/reports`,
  },
] as const);
</script>

<div class="flex items-center justify-center gap-3 pb-12 w-full flex-wrap">
	{#each ACTIONS as action (action.label)}
		{@const Icon = action.icon}
		<Button
			href={action.href}
		>
			<Icon size={13}  />
			<span>{action.label}</span>
		</Button>
	{/each}
</div>

