<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";
import type { Snippet } from "svelte";

let {
  title,
  description = "",
  badge = undefined,
  isComplete = false,
  children,
}: {
  title: string;
  description?: string;
  badge?: string;
  isComplete?: boolean;
  children: Snippet;
} = $props();
</script>

<Card>
  <CardHeader class="gap-0 px-5 pt-5 sm:px-6 sm:pt-6">
    <div class="flex items-start justify-between gap-4">
      <div class="min-w-0 space-y-1.5">
        <CardTitle class="text-sm font-semibold tracking-tight text-foreground sm:text-base">
          {title}
        </CardTitle>
        {#if description}
          <CardDescription class="sr-only">
            {description}
          </CardDescription>
        {/if}
        {#if isComplete}
          <span class="sr-only">{m.onboarding_step8_status_ready()}</span>
        {/if}
      </div>

      {#if badge}
        <Badge variant="secondary" class="shrink-0 text-xs">
          {badge}
        </Badge>
      {/if}
    </div>
  </CardHeader>
  <CardContent class="space-y-4 px-5 pb-5 pt-4 sm:px-6 sm:pb-6">
    {@render children()}
  </CardContent>
</Card>
