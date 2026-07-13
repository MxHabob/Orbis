<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@orbis/ui-svelte";
import { Zap } from "lucide-svelte";
import { uiStore } from "$lib/stores";

</script>

<Tooltip>
	<TooltipTrigger>
		{#snippet child({ props }: { props: any })}
					<Button 
						{...props}
						variant="outline" 
						size="icon-sm" 
						class="relative cursor-pointer rounded-[10px]"
						onclick={() => uiStore.setUpgradeDialog(true)}
						aria-label={m.common_upgrade_title()}
					>
						<Zap size={16} strokeWidth={1.8} class="text-foreground" />
						<span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-primary border border-background"></span>
					</Button>
		{/snippet}
	</TooltipTrigger>
	<TooltipContent side="right" sideOffset={12}>
		{m.common_upgrade_title()}
	</TooltipContent>
</Tooltip>
