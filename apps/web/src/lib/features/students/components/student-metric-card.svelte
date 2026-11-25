<script lang="ts">
import type { Component, Snippet } from "svelte";
import { Card } from "@orbis/ui/card";
import { Skeleton } from "@orbis/ui/skeleton";

type MetricTone = "primary" | "success" | "warning" | "danger" | "neutral";

const toneStyles: Record<MetricTone, { icon: string; value: string }> = {
  primary: {
    icon: "bg-primary/10 text-primary",
    value: "text-foreground",
  },
  success: {
    icon: "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400",
    value: "text-emerald-600 dark:text-emerald-400",
  },
  warning: {
    icon: "bg-amber-500/10 text-amber-600 dark:text-amber-400",
    value: "text-amber-600 dark:text-amber-400",
  },
  danger: {
    icon: "bg-destructive/10 text-destructive",
    value: "text-destructive",
  },
  neutral: {
    icon: "bg-muted text-muted-foreground",
    value: "text-foreground",
  },
};

let {
  title,
  value,
  icon,
  tone = "primary",
  loading = false,
  children,
}: {
  title: string;
  value: string;
  icon: Component<{ class?: string; size?: number }>;
  tone?: MetricTone;
  loading?: boolean;
  children?: Snippet;
} = $props();

const Icon = $derived(icon);
</script>

<Card class="flex min-h-28 min-w-0 flex-col justify-between gap-3 p-4">
  <div class="flex items-start justify-between gap-3">
    <div class="min-w-0">
      <p class="truncate text-xs font-medium text-muted-foreground">{title}</p>
      {#if loading}
        <Skeleton class="mt-2 h-6 w-28" />
      {:else}
        <p class="mt-1 truncate text-lg font-semibold tabular-nums {toneStyles[tone].value}">
          {value}
        </p>
      {/if}
    </div>
    <div class="flex size-9 shrink-0 items-center justify-center rounded-lg {toneStyles[tone].icon}">
      <Icon size={17} />
    </div>
  </div>

  {#if children && !loading}
    <div class="min-h-4 text-[11px] text-muted-foreground">
      {@render children()}
    </div>
  {/if}
</Card>
