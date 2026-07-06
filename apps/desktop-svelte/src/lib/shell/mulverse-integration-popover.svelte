<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@orbis/ui-svelte/popover";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@orbis/ui-svelte";
import { GitBranch } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
</script>

<Popover>
	<PopoverTrigger>
		{#snippet child({ props }: { props: any })}
			<Tooltip>
				<TooltipTrigger>
					<Button 
						{...props}
						variant="outline" 
						size="icon-sm" 
						class="relative cursor-pointer rounded-[10px]"
						aria-label={m.sidebar_mulverse_connect_title()}
					>
						<GitBranch size={16} strokeWidth={1.8} class="text-foreground" />
						<span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-indigo-500"></span>
					</Button>
				</TooltipTrigger>
				<TooltipContent side="right" sideOffset={12}>
					{m.sidebar_mulverse_connect_title()}
				</TooltipContent>
			</Tooltip>
		{/snippet}
	</PopoverTrigger>
	<PopoverContent side="right" sideOffset={12} class="w-64 p-4 rounded-xl space-y-3">
		<div class="space-y-1">
			<h4 class="text-xs font-bold text-foreground">
				{m.sidebar_mulverse_connect_title()}
			</h4>
			<p class="text-[10px] text-muted-foreground leading-normal">
				{m.sidebar_mulverse_connect_desc()}
			</p>
		</div>

		<Button 
			size="sm"
			class="w-full text-[10px] h-8 rounded-lg font-semibold"
			onclick={() => window.open('https://github.com/apps/orbis-seer', '_blank')}
		>
			{m.sidebar_mulverse_configure()}
		</Button>
	</PopoverContent>
</Popover>
