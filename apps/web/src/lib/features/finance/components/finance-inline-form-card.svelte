<script lang="ts">
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import { XMark } from "@orbis/icons";

type Accent = "primary" | "emerald" | "rose" | "indigo";

let {
  title,
  accent = "primary",
  contentClass = "gap-4",
  busy = false,
  errorMessage,
  metadata,
  onclose,
  onsave,
  actions,
  children,
}: {
  title: string;
  accent?: Accent;
  contentClass?: string;
  busy?: boolean;
  errorMessage?: string;
  metadata?: Snippet;
  onclose: () => void;
  onsave: () => void | Promise<void>;
  actions: Snippet;
  children: Snippet;
} = $props();

const accentClasses: Record<Accent, { border: string; indicator: string }> = {
  primary: { border: "border-primary/20", indicator: "bg-primary" },
  emerald: { border: "border-emerald-500/20", indicator: "bg-emerald-500" },
  rose: { border: "border-rose-500/20", indicator: "bg-rose-500" },
  indigo: { border: "border-indigo-500/20", indicator: "bg-indigo-500" },
};
</script>

<form
  aria-label={title}
  aria-busy={busy}
  onsubmit={(event) => {
    event.preventDefault();
    if (!busy) void onsave();
  }}
>
  <Card class="border {accentClasses[accent].border} flex max-h-[min(72dvh,44rem)] flex-col gap-0 overflow-hidden bg-card py-0 text-start">
    <CardHeader class="grid shrink-0 grid-cols-[minmax(0,1fr)_auto] items-center gap-3 border-b border-border/40 px-4 py-3">
      <div class="flex min-w-0 flex-wrap items-center gap-x-5 gap-y-3">
        <CardTitle class="flex items-center gap-2 text-sm font-semibold text-foreground">
          <span class="size-2 shrink-0 rounded-full {accentClasses[accent].indicator}"></span>
          {title}
        </CardTitle>
        {#if metadata}
          <fieldset disabled={busy} class="min-w-0">
            {@render metadata()}
          </fieldset>
        {/if}
      </div>
      <Button
        type="button"
        variant="ghost"
        size="icon-xs"
        class="text-muted-foreground"
        aria-label={m.finance_ui_cancel()}
        disabled={busy}
        onclick={onclose}
      >
        <XMark size={14} />
      </Button>
    </CardHeader>

    <CardContent class="min-h-0 overflow-y-auto overscroll-contain p-4">
      <fieldset disabled={busy} class="flex min-w-0 flex-col {contentClass}">
        {@render children()}
      </fieldset>
    </CardContent>
    <div class="shrink-0 border-t border-border bg-card px-4 py-3">
      {#if errorMessage}
        <p class="mb-3 rounded-md border border-destructive/30 bg-destructive/5 px-3 py-2 text-sm text-destructive" role="alert">{errorMessage}</p>
      {/if}
      {@render actions()}
    </div>
  </Card>
</form>
