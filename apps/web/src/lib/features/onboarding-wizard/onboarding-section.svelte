<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
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
  <CardHeader class="gap-0 border-b border-border/60 px-4 py-4 sm:px-6">
    <div class="flex items-start justify-between gap-4">
      <div class="min-w-0 space-y-1.5">
        <CardTitle class="text-sm font-semibold tracking-tight text-foreground sm:text-base">
          {title}
        </CardTitle>
        {#if description}
          <CardDescription class="max-w-2xl text-sm leading-relaxed">
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
  <CardContent class="space-y-4 p-4 sm:p-6">
    {@render children()}
  </CardContent>
</Card>
