<!--
@component AppIcon
Renders a Lucide icon by name string. Used in app-card and app-detail.
-->
<script lang="ts">
import * as icons from "lucide-svelte";

let {
  icon,
  size = 24,
  class: className = "",
}: {
  icon: string;
  size?: number;
  class?: string;
} = $props();

// Dynamically resolve the Lucide icon component by name
const IconComponent = $derived((icons as Record<string, any>)[icon] ?? icons.Package);
</script>

<div class="flex items-center justify-center shrink-0 {className}">
  <IconComponent {size} strokeWidth={1.6} />
</div>
