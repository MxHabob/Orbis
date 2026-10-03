<!--
@component SettingsRow
Reusable settings row matching the exact Organization Settings design system.
-->
<script lang="ts">
import type { Snippet } from "svelte";
import { Label } from "@orbis/ui/label";

interface Props {
  title?: string;
  description?: string;
  labelFor?: string;
  required?: boolean;
  headerSlot?: Snippet;
  children?: Snippet;
  last?: boolean;
  class?: string;
  controlClass?: string;
}

let {
  title,
  description,
  labelFor,
  required = false,
  headerSlot,
  children,
  last = false,
  class: className = "",
  controlClass = "",
}: Props = $props();
</script>

<div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border {last ? 'last:border-b-0' : ''} gap-4 {className}">
  <div class="flex flex-col gap-1 text-start">
    {#if headerSlot}
      {@render headerSlot()}
    {:else if title}
      <Label for={labelFor} class="text-xs font-semibold text-foreground">
        {title}{#if required}<span class="text-destructive" aria-hidden="true"> *</span>{/if}
      </Label>
    {/if}
    {#if description}
      <span class="text-[11px] text-muted-foreground leading-relaxed">
        {description}
      </span>
    {/if}
  </div>

  {#if children}
    <div class="w-full md:w-120 max-w-full shrink-0 flex items-center justify-end {controlClass}">
      {@render children()}
    </div>
  {/if}
</div>
