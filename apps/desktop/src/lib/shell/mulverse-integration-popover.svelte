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
	  class="flex max-h-[min(40rem,calc(100vh-4rem))] w-[min(320px,calc(100vw-1rem))] flex-col gap-0 overflow-hidden rounded-md border border-border bg-card p-0 text-start font-sans shadow-md"
	>
	  <div class="border-b border-border px-4 py-3">
	    <h4 class="text-sm font-semibold text-foreground">
	      {m.common_sidebar_mulverse_connect_title()}
	    </h4>
	  </div>

	  <p class="px-4 py-3 text-xs leading-relaxed text-muted-foreground [overflow-wrap:anywhere]">
	    {m.common_sidebar_mulverse_connect_desc()}
	  </p>

	  <div class="border-t border-border px-4 py-3">
	    <Button
	      href={localizeHref(`/${orgId}/settings/mulverse`)}
	      size="sm"
	      class="w-full text-xs font-semibold"
	    >
	      {m.common_sidebar_mulverse_configure()}
	    </Button>
	  </div>
	</PopoverContent>
</Popover>
