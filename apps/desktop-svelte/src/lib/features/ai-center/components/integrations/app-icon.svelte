<!--
@component AppIcon
Renders a Lucide icon by name string. Used in app-card and app-detail.
-->
<script lang="ts">
import * as icons from "lucide-svelte";

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

<div class="flex items-center justify-center shrink-0 {className}">
  {#if logoHtml}
    {@html logoHtml}
  {:else}
    <IconComponent {size} strokeWidth={1.6} />
  {/if}
</div>
