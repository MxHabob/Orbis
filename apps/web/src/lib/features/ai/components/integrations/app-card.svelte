<!--
@component AppCard
Integration app card — uses official UI Card and Badge components with localized messages.
Shows: icon, name, short description, category tag, status badge.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { OrbisApp } from "@orbis/app-store";
import { getCategoryName, getCategoryNameAr } from "@orbis/app-store";
import AppIcon from "./app-icon.svelte";
import { Badge } from "@orbis/ui/badge";
import { Card, CardContent, CardFooter } from "@orbis/ui/card";
import { l } from "$lib/core/i18n";

let {
  app,
  connected = false,
  actionLabel,
  onselect,
}: {
  app: OrbisApp;
  connected?: boolean;
  actionLabel?: string;
  onselect: () => void;
} = $props();
</script>

<button
  type="button"
  class="group block h-full w-full cursor-pointer appearance-none border-0 bg-transparent p-0 text-start text-inherit"
  onclick={onselect}
>
  <Card class="h-full flex flex-col justify-between hover:border-foreground/30 transition-colors">
    <CardContent class="p-5 flex flex-col flex-1 text-start">
      <!-- Top: Icon + UI Badges -->
      <div class="flex items-center justify-between gap-3 mb-4">
        <div class="w-12 h-7 flex items-center justify-start text-foreground shrink-0">
          <AppIcon icon={app.icon} logo={app.logo} size={24} class="w-full h-full [&>svg]:max-w-[48px] [&>svg]:max-h-[26px]" />
        </div>
        <div class="flex gap-1.5 flex-wrap justify-end">
          {#if connected || app.status === "connected"}
            <Badge variant="outline" class="text-[10px] font-semibold text-primary border-primary/20 bg-primary/10 px-2 py-0.5">
              {m.ai_center_integrations_status_connected()}
            </Badge>
          {:else if app.status === "error"}
            <Badge variant="destructive" class="text-[10px] font-semibold px-2 py-0.5">
              {m.ai_center_integrations_status_error()}
            </Badge>
          {:else if app.beta}
            <Badge variant="secondary" class="text-[10px] font-semibold px-2 py-0.5">
              {m.ai_center_integrations_status_beta()}
            </Badge>
          {:else if !app.active}
            <Badge variant="outline" class="text-[10px] font-medium text-muted-foreground bg-muted px-2 py-0.5">
              {m.ai_center_integrations_status_coming_soon()}
            </Badge>
          {/if}
        </div>
      </div>

      <!-- Name -->
      <h3 class="text-sm font-semibold text-foreground mb-1.5">
        {l(app.name_ar, app.name)}
      </h3>

      <!-- Description -->
      <p class="text-xs text-muted-foreground leading-relaxed flex-1 line-clamp-2">
        {l(app.short_description_ar, app.short_description)}
      </p>
    </CardContent>

    <!-- Footer: Category Tag and optional action -->
    <CardFooter class="py-2.5 px-5 flex items-center justify-between gap-2">
      <Badge variant="outline" class="text-[10px] font-mono font-medium text-muted-foreground border-border bg-muted/40">
        {l(getCategoryNameAr(app.category), getCategoryName(app.category))}
      </Badge>
      {#if actionLabel}
        <span class="text-xs font-semibold text-primary group-hover:underline">
          {actionLabel}
        </span>
      {/if}
    </CardFooter>
  </Card>
</button>
