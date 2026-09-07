<!--
@component SettingsCard
Reusable settings card matching the exact Organization Settings design system.
-->
<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
  title?: string;
  headerSlot?: Snippet;
  headerAction?: Snippet;
  children: Snippet;
  footer?: Snippet;
  footerClass?: string;
  onsubmit?: (e: SubmitEvent | Event) => void;
  class?: string;
}

let {
  title,
  headerSlot,
  headerAction,
  children,
  footer,
  footerClass = "",
  onsubmit,
  class: className = "",
}: Props = $props();
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col {className}">
  <!-- Card Header -->
  {#if headerSlot || title || headerAction}
    <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none flex items-center justify-between">
      {#if headerSlot}
        {@render headerSlot()}
      {:else if title}
        <span>{title}</span>
      {:else}
        <div></div>
      {/if}

      {#if headerAction}
        <div class="flex items-center gap-2">
          {@render headerAction()}
        </div>
      {/if}
    </div>
  {/if}

  {#if onsubmit}
    <form {onsubmit} class="flex flex-col flex-1">
      <!-- Card Body -->
      <div class="flex flex-col">
        {@render children()}
      </div>

      <!-- Optional Card Footer -->
      {#if footer}
        <div class="bg-muted/10 px-6 py-3 border-t border-border flex justify-start {footerClass}">
          {@render footer()}
        </div>
      {/if}
    </form>
  {:else}
    <!-- Card Body -->
    <div class="flex flex-col">
      {@render children()}
    </div>

    <!-- Optional Card Footer -->
    {#if footer}
      <div class="bg-muted/10 px-6 py-3 border-t border-border flex justify-start {footerClass}">
        {@render footer()}
      </div>
    {/if}
  {/if}
</div>
