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
  onclose,
  children,
}: {
  title: string;
  accent?: Accent;
  contentClass?: string;
  onclose: () => void;
  children: Snippet;
} = $props();

const accentClasses: Record<Accent, { border: string; indicator: string }> = {
  primary: { border: "border-primary/20", indicator: "bg-primary" },
  emerald: { border: "border-emerald-500/20", indicator: "bg-emerald-500" },
  rose: { border: "border-rose-500/20", indicator: "bg-rose-500" },
  indigo: { border: "border-indigo-500/20", indicator: "bg-indigo-500" },
};
</script>

<div class="min-h-0 max-h-[min(70vh,44rem)] shrink-0 overflow-x-hidden overflow-y-auto overscroll-contain">
  <Card class="border {accentClasses[accent].border} bg-card text-start shadow-sm">
    <CardHeader class="flex flex-row items-center justify-between border-b border-border/40 px-4 py-3">
      <CardTitle class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-foreground">
        <span class="h-2 w-2 rounded-full {accentClasses[accent].indicator}"></span>
        {title}
      </CardTitle>
      <Button
        type="button"
        variant="ghost"
        size="icon-xs"
        class="text-muted-foreground"
        aria-label={m.finance_ui_cancel()}
        onclick={onclose}
      >
        <XMark size={14} />
      </Button>
    </CardHeader>

    <CardContent class="flex flex-col p-4 {contentClass}">
      {@render children()}
    </CardContent>
  </Card>
</div>
