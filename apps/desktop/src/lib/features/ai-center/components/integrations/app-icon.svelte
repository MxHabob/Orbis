<!--
@component AppIcon
Renders a Lucide icon by name string or SVG logo HTML without forced external borders or containers.
Scales both square and wide landscape logos proportionally and consistently within bounded limits.
-->
<script lang="ts">
import * as icons from "@lucide/svelte";

let {
  icon,
  logo = null,
  size = 24,
  class: className = "",
}: {
  icon: string;
  logo?: any;
  size?: number;
  class?: string;
} = $props();

// Dynamically resolve the Lucide icon component by name
const IconComponent = $derived((icons as Record<string, any>)[icon] ?? icons.Package);

const logoHtml = $derived(
  logo ? (typeof logo === "function" ? logo() : logo) : null
);
</script>

<div class="flex items-center justify-start shrink-0 [&>svg]:max-w-full [&>svg]:max-h-full [&>svg]:w-auto [&>svg]:h-auto [&>svg]:object-contain {className}">
  {#if logoHtml}
    {@html logoHtml}
  {:else}
    <IconComponent {size} strokeWidth={1.6} />
  {/if}
</div>
