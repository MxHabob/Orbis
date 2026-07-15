<!--
@component AppCard
Integration app card — uses official UI Card and Badge components with localized messages.
Shows: icon, name, short description, category tag, status badge.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { OrbisApp } from "@orbis/app-store";
import { getCategoryName } from "@orbis/app-store";
import AppIcon from "./app-icon.svelte";
import { Badge } from "@orbis/ui-svelte/badge";
import { Card } from "@orbis/ui-svelte/card";

let {
  app,
  href,
}: {
  app: OrbisApp;
  href: string;
} = $props();
</script>

<a {href} class="block h-full">
  <Card class="p-5 h-full flex flex-col justify-between bg-card border-border/60">
    <div>
      <!-- Top: Icon + UI Badges -->
      <div class="flex items-start justify-between mb-4">
        <div class="h-10 w-10 flex items-center justify-center overflow-hidden rounded-lg bg-muted/60 text-foreground">
          <AppIcon icon={app.icon} size={22} />
        </div>
        <div class="flex gap-1.5">
          {#if app.status === "connected"}
            <Badge variant="outline" class="text-[10px] font-semibold text-primary border-primary/20 bg-primary/10 px-2 py-0.5">
              {m.ai_center_integrations_status_connected ? m.ai_center_integrations_status_connected() : "Connected"}
            </Badge>
          {/if}
          {#if app.status === "error"}
            <Badge variant="destructive" class="text-[10px] font-semibold px-2 py-0.5">
              {m.ai_center_integrations_status_error ? m.ai_center_integrations_status_error() : "Error"}
            </Badge>
          {/if}
          {#if app.beta}
            <Badge variant="secondary" class="text-[10px] font-semibold px-2 py-0.5">
              {m.ai_center_integrations_status_beta ? m.ai_center_integrations_status_beta() : "Beta"}
            </Badge>
          {/if}
          {#if !app.active}
            <Badge variant="outline" class="text-[10px] font-medium text-muted-foreground bg-muted px-2 py-0.5">
              {m.ai_center_integrations_status_coming_soon ? m.ai_center_integrations_status_coming_soon() : "Coming Soon"}
            </Badge>
          {/if}
        </div>
      </div>

      <!-- Name -->
      <h3 class="text-sm font-semibold text-foreground mb-1.5">
        {app.name}
      </h3>

      <!-- Description -->
      <p class="text-xs text-muted-foreground leading-relaxed flex-1 line-clamp-2">
        {app.short_description}
      </p>
    </div>

    <!-- Footer: Category Tag -->
    <div class="mt-4 pt-3 border-t border-border/40">
      <Badge variant="outline" class="text-[10px] font-mono font-medium text-muted-foreground border-border/60 bg-muted/20">
        {getCategoryName(app.category)}
      </Badge>
    </div>
  </Card>
</a>
