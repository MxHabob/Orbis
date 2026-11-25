<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { EllipsisHorizontal } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import type { SplitActionItem } from "./split-action-menu.svelte";

let {
  actions,
  label = m.operations_ui_actions(),
}: {
  actions: SplitActionItem[];
  label?: string;
} = $props();
</script>

<DropdownMenu>
  <DropdownMenuTrigger class="outline-none">
    {#snippet child({ props })}
      <Button
        {...props}
        variant="outline"
        size="sm"
        class="h-9 gap-1.5 px-3 text-xs"
      >
        <EllipsisHorizontal class="size-4" />
        <span>{label}</span>
      </Button>
    {/snippet}
  </DropdownMenuTrigger>
  <DropdownMenuContent align="end" class="min-w-48">
    {#each actions as action (action.id)}
      <DropdownMenuItem
        disabled={action.disabled}
        class="gap-2 text-sm {action.destructive ? 'text-destructive focus:text-destructive' : ''}"
        onclick={action.onSelect}
      >
        {#if action.icon}
          {@const Icon = action.icon}
          <Icon class="size-4 shrink-0 text-muted-foreground" />
        {/if}
        <span>{action.label}</span>
        {#if action.badge}
          <span class="ms-auto rounded-full bg-muted px-1.5 py-0.5 text-[10px] text-muted-foreground">
            {action.badge}
          </span>
        {/if}
      </DropdownMenuItem>
    {/each}
  </DropdownMenuContent>
</DropdownMenu>
