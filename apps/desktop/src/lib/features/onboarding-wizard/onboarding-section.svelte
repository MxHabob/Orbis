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
  <CardHeader class="gap-3 px-5 pt-5 sm:px-6 sm:pt-6">
    <div class="flex items-start justify-between gap-4">
      <div class="min-w-0 space-y-1.5">
        <CardTitle class="flex items-center gap-2 text-base font-semibold tracking-tight text-foreground sm:text-lg">
          {title}
          {#if isComplete}
            <span
              class="inline-flex size-5 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary text-xs font-bold"
              aria-label={m.onboarding_step8_status_ready()}
            >
              ✓
            </span>
          {/if}
        </CardTitle>
        {#if description}
          <CardDescription class="max-w-3xl text-sm leading-relaxed text-muted-foreground">
            {description}
          </CardDescription>
        {/if}
      </div>

      {#if badge}
        <Badge variant="secondary" class="shrink-0 text-xs">
          {badge}
        </Badge>
      {/if}
    </div>
  </CardHeader>
  <CardContent class="space-y-4 px-5 pb-5 pt-0 sm:px-6 sm:pb-6">
    {@render children()}
  </CardContent>
</Card>
