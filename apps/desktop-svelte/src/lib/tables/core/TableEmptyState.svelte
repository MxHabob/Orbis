<!--
  @component TableEmptyState
  Generic empty/no-results state for data tables.
  Displays an icon, title, and optional description.
-->
<script lang="ts">
  import { TableProperties } from "lucide-svelte";
  import { Button } from "@orbis/ui-svelte/button";
  import type { Component } from "svelte";

  interface Props {
    /** Icon component to display */
    icon?: Component;
    /** Main title text */
    title?: string;
    /** Description / hint text */
    description?: string;
    /** Show a retry/action button */
    actionLabel?: string;
    onAction?: () => void;
  }

  let {
    icon,
    title = "No data available",
    description = "",
    actionLabel,
    onAction,
  }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center py-16 gap-4 text-muted-foreground select-none">
  {#if icon}
    <svelte:component this={icon} size={40} class="opacity-25" />
  {:else}
    <TableProperties size={40} class="opacity-25" />
  {/if}

  <div class="flex flex-col gap-1 text-center max-w-xs">
    <p class="text-sm font-medium text-foreground/60">{title}</p>
    {#if description}
      <p class="text-xs text-muted-foreground/60">{description}</p>
    {/if}
  </div>

  {#if actionLabel && onAction}
    <Button variant="outline" size="sm" class="text-xs mt-2" onclick={onAction}>
      {actionLabel}
    </Button>
  {/if}
</div>
