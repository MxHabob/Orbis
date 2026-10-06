<script lang="ts">
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

<Card class="border-border/60 bg-card/80 shadow-xs backdrop-blur-xs transition-colors hover:border-border">
  <CardHeader class="pb-4">
    <div class="flex items-start justify-between gap-4">
      <div class="space-y-1">
        <CardTitle class="text-lg font-semibold tracking-tight text-foreground flex items-center gap-2">
          {title}
          {#if isComplete}
            <span class="inline-flex size-5 items-center justify-center rounded-full bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-xs font-bold">
              ✓
            </span>
          {/if}
        </CardTitle>
        {#if description}
          <CardDescription class="text-xs text-muted-foreground sm:text-sm">
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
  <CardContent class="pt-0">
    {@render children()}
  </CardContent>
</Card>
