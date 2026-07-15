<!--
@component AppCard
Integration app card — matches midday.ai/integrations card design.
Shows: icon, name, short description, category tag, status badge.
-->
<script lang="ts">
import type { OrbisApp } from "@orbis/app-store";
import { getCategoryName } from "@orbis/app-store";
import AppIcon from "./app-icon.svelte";

let {
  app,
  href,
}: {
  app: OrbisApp;
  href: string;
} = $props();
</script>

<a
  {href}
  class="group border border-border/60 p-5 hover:border-foreground/20
         transition-all duration-200 flex flex-col rounded-lg bg-background"
>
  <!-- Top: Icon + Badges -->
  <div class="flex items-start justify-between mb-4">
    <div
      class="h-10 w-10 flex items-center justify-center overflow-hidden rounded-lg
             bg-muted/50 text-foreground"
    >
      <AppIcon icon={app.icon} size={22} />
    </div>
    <div class="flex gap-1.5">
      {#if app.status === "connected"}
        <span
          class="text-[10px] font-medium text-emerald-600 bg-emerald-50
                 dark:bg-emerald-950/30 dark:text-emerald-400 px-2 py-0.5 rounded-md"
        >
          Connected
        </span>
      {/if}
      {#if app.status === "error"}
        <span
          class="text-[10px] font-medium text-red-600 bg-red-50
                 dark:bg-red-950/30 dark:text-red-400 px-2 py-0.5 rounded-md"
        >
          Error
        </span>
      {/if}
      {#if app.beta}
        <span class="text-[10px] font-medium text-primary bg-primary/10 px-2 py-0.5 rounded-md">
          Beta
        </span>
      {/if}
      {#if !app.active}
        <span class="text-[10px] font-medium text-muted-foreground bg-muted px-2 py-0.5 rounded-md">
          Coming soon
        </span>
      {/if}
    </div>
  </div>

  <!-- Name -->
  <h3 class="text-sm font-medium text-foreground mb-1.5 group-hover:text-primary transition-colors">
    {app.name}
  </h3>

  <!-- Description -->
  <p class="text-xs text-muted-foreground leading-relaxed flex-1 line-clamp-2">
    {app.short_description}
  </p>

  <!-- Footer: Category -->
  <div class="mt-4 pt-3 border-t border-border/40">
    <span class="text-[10px] font-medium text-muted-foreground/70 uppercase tracking-wider">
      {getCategoryName(app.category)}
    </span>
  </div>
</a>
