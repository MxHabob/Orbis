<!--
@component CollapsibleSummary
A wrapper component that collapses along with the header on scroll, matching the top position.
-->
<script lang="ts">
import { type Snippet } from "svelte";
import { cn } from "@orbis/ui-svelte/utils";

let {
  children,
  class: className,
}: {
  children: Snippet;
  class?: string;
} = $props();
</script>

<div
  class={cn("transition-transform will-change-transform", className)}
  style="transform: translateY(calc(var(--header-offset, 0px) * -1)); transition-duration: var(--header-transition, 200ms);"
>
  {@render children()}
</div>
