<script lang="ts">
import type { Component, Snippet } from "svelte";
import { ChevronDown } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";

export interface SplitActionItem {
  id: string;
  label: string;
  icon?: Component<{ class?: string }>;
  badge?: string;
  destructive?: boolean;
  disabled?: boolean;
  onSelect: () => void;
}

let {
  label,
  menuLabel,
  icon,
  actions,
  onprimary,
  primary,
  disabled = false,
}: {
  label: string;
  menuLabel: string;
  icon?: Component<{ class?: string }>;
  actions: SplitActionItem[];
  onprimary: () => void;
  primary?: Snippet;
  disabled?: boolean;
} = $props();
</script>

<div class="inline-flex items-center">
  {#if primary}
    {@render primary()}
  {:else}
    <Button
      size="sm"
      class="rounded-e-none gap-1.5"
      {disabled}
      onclick={onprimary}
    >
      {#if icon}
        {@const Icon = icon}
        <Icon class="size-4" />
      {/if}
      <span>{label}</span>
    </Button>
  {/if}

  <DropdownMenu>
    <DropdownMenuTrigger class="outline-none">
      {#snippet child({ props })}
        <Button
          {...props}
          size="sm"
          class="rounded-s-none border-s border-primary-foreground/20 px-2"
          {disabled}
          aria-label={menuLabel}
        >
          <ChevronDown class="size-4" />
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
            {@const ActionIcon = action.icon}
            <ActionIcon class="size-4 shrink-0 text-muted-foreground" />
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
</div>
