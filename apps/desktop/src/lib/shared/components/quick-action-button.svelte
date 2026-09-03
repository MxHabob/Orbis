<!--
@component QuickActionButton
Generic action button with icon, label, and optional action indicator.
-->
<script lang="ts">
  import type { ComponentType } from "svelte";
  import type { Icon } from "@lucide/svelte";
  import { Button } from "@orbis/ui";

  let {
    label,
    icon: IconComponent,
    onclick,
    href,
    variant = "secondary",
    class: className = "",
  }: {
    label: string;
    icon?: ComponentType<Icon> | any;
    onclick?: () => void;
    href?: string;
    variant?: "default" | "secondary" | "outline" | "ghost" | "link";
    class?: string;
  } = $props();
</script>

<Button
  {variant}
  size="sm"
  class="flex items-center gap-2.5 px-3 py-2 font-bold text-xs font-sans transition-colors cursor-pointer {className}"
  {onclick}
  {href}
>
  {#if IconComponent}
    <IconComponent size={14} />
  {/if}
  <span>{label}</span>
  <span class="opacity-50 text-[10px] font-black ms-0.5">+</span>
</Button>
