<script lang="ts">
import type { Component } from "svelte";
import { Sparkles } from "@orbis/icons";
import { Button } from "@orbis/ui/button";

let {
  icon: Icon = Sparkles,
  title,
  description = "",
  actionLabel = "",
  onaction = undefined,
  class: className = "",
}: {
  icon?: Component<any>;
  title: string;
  description?: string;
  actionLabel?: string;
  onaction?: () => void;
  class?: string;
} = $props();
</script>

<div class="flex flex-col items-center justify-center p-8 text-center h-full min-h-50 gap-4 {className}">
  <div class="w-14 h-14 rounded-2xl bg-muted/30 border border-border/40 flex items-center justify-center text-muted-foreground shadow-2xs">
    <Icon size={28} strokeWidth={1.5} />
  </div>
  <div class="flex flex-col gap-1.5 max-w-sm">
    <h3 class="text-sm sm:text-base font-bold text-foreground tracking-tight">{title}</h3>
    {#if description}
      <p class="text-xs text-muted-foreground leading-relaxed">
        {description}
      </p>
    {/if}
  </div>
  {#if actionLabel && onaction}
    <Button variant="outline" size="sm" class="mt-1 cursor-pointer text-xs" onclick={onaction}>
      {actionLabel}
    </Button>
  {/if}
</div>
