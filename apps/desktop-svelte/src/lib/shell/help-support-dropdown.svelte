<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@orbis/ui-svelte";
import { HelpCircle } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
</script>

<DropdownMenu>
	<DropdownMenuTrigger>
		{#snippet child({ props })}
			<Tooltip>
				<TooltipTrigger>
					<Button 
						{...props}
						variant="outline" 
						size="icon-sm" 
						class="relative cursor-pointer rounded-[10px]"
						aria-label={m.sidebar_help_resources()}
					>
						<HelpCircle size={16} strokeWidth={1.8} class="text-foreground" />
					</Button>
				</TooltipTrigger>
				<TooltipContent side="right" sideOffset={12}>
					{m.sidebar_help_resources()}
				</TooltipContent>
			</Tooltip>
		{/snippet}
	</DropdownMenuTrigger>
	<DropdownMenuContent side="right" sideOffset={12} class="w-52">
		<!-- Resources -->
		<DropdownMenuItem class="text-xs cursor-pointer flex justify-between items-center" onclick={() => window.open('https://docs.orbis.edu', '_blank')}>
			<span>{m.sidebar_help_resources()}</span>
			<span class="text-[10px] text-muted-foreground">→</span>
		</DropdownMenuItem>

		<!-- Community -->
		<DropdownMenuItem class="text-xs cursor-pointer flex justify-between items-center" onclick={() => window.open('https://community.orbis.edu', '_blank')}>
			<span>{m.sidebar_help_community()}</span>
			<span class="text-[10px] text-muted-foreground">→</span>
		</DropdownMenuItem>

		<!-- Legal -->
		<DropdownMenuItem class="text-xs cursor-pointer flex justify-between items-center" onclick={() => window.open('https://orbis.edu/legal', '_blank')}>
			<span>{m.sidebar_help_legal()}</span>
			<span class="text-[10px] text-muted-foreground">→</span>
		</DropdownMenuItem>

		<DropdownMenuSeparator />

		<!-- Give Feedback -->
		<DropdownMenuItem class="text-xs cursor-pointer" onclick={() => window.open('mailto:feedback@orbis.edu')}>
			<span>{m.sidebar_help_feedback()}</span>
		</DropdownMenuItem>

		<!-- Tour Navigation -->
		<DropdownMenuItem class="text-xs cursor-pointer" onclick={() => alert('Tour is coming soon!')}>
			<span>{m.sidebar_help_tour()}</span>
		</DropdownMenuItem>
	</DropdownMenuContent>
</DropdownMenu>
