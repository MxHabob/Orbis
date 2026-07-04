<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { UserPlus, Wallet, BookOpen, BarChart3 } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

const ACTIONS = $derived([
  {
    label: m.action_enroll_student(),
    icon: UserPlus,
    href: `/${orgId}/students/new`,
  },
  {
    label: m.action_record_payment(),
    icon: Wallet,
    href: `/${orgId}/finance/payments`,
  },
  {
    label: m.action_manage_classes(),
    icon: BookOpen,
    href: `/${orgId}/academics`,
  },
  {
    label: m.action_view_reports(),
    icon: BarChart3,
    href: `/${orgId}/reports`,
  },
] as const);
</script>

<div class="flex items-center justify-center gap-3 pb-12 w-full flex-wrap">
	{#each ACTIONS as action (action.href)}
		{@const Icon = action.icon}
		<Button
			href={localizeHref(action.href)}
		>
			<Icon size={13}  />
			<span>{action.label}</span>
		</Button>
	{/each}
</div>


