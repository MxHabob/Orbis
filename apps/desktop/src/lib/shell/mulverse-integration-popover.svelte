<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import { Tooltip, TooltipContent, TooltipTrigger } from "@orbis/ui";
import { Merge } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();
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
						class="relative cursor-pointer"
						aria-label={m.common_sidebar_mulverse_connect_title()}
					>
						<Merge size={16} strokeWidth={1.8} class="text-foreground" />
						<span class="absolute -top-0.5 -inset-e-0.5 w-1.5 h-1.5 rounded-full bg-indigo-500"></span>
					</Button>
				</TooltipTrigger>
				<TooltipContent side="inline-end" sideOffset={12}>
					{m.common_sidebar_mulverse_connect_title()}
				</TooltipContent>
			</Tooltip>
		{/snippet}
	</PopoverTrigger>
	<PopoverContent
    side="inline-end"
    align="end"
    sideOffset={12}
	 class="w-64 p-4 space-y-3 backdrop-blur-md">
		<div class="space-y-1">
			<h4 class="text-xs font-bold text-foreground">
				{m.common_sidebar_mulverse_connect_title()}
			</h4>
			<p class="text-[10px] text-muted-foreground leading-normal">
				{m.common_sidebar_mulverse_connect_desc()}
			</p>
		</div>

		<Button
			href={localizeHref(`/${orgId}/settings/mulverse`)}
			size="sm"
			class="w-full text-[10px] h-8 rounded-lg font-semibold"
		>
			{m.common_sidebar_mulverse_configure()}
		</Button>
	</PopoverContent>
</Popover>
