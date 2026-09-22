<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { Button } from "@orbis/ui/button";
import { Sparkles } from "@orbis/icons";

import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui/tooltip";

let {
  query,
  label,
  variant = "outline",
  size = "sm",
  class: className = "",
}: {
  query: string;
  label: string;
  variant?:
    | "default"
    | "lime"
    | "outline"
    | "ghost"
    | "destructive"
    | "secondary"
    | "link";
  size?:
    | "default"
    | "sm"
    | "xs"
    | "lg"
    | "icon"
    | "icon-sm"
    | "icon-xs"
    | "icon-lg";
  class?: string;
} = $props();

function askOrbis() {
  const newUrl = new URL(page.url.href);
  newUrl.searchParams.set("ask_orbis", query);
  goto(newUrl.pathname + newUrl.search, {
    replaceState: true,
    keepFocus: true,
    noScroll: true,
  });
}
</script>

<TooltipProvider delayDuration={1000}>
  <Tooltip>
    <TooltipTrigger>
      {#snippet child({ props })}
        <Button
          {...props}
          {variant}
          {size}
          class="font-semibold gap-1.5 border-primary/20 hover:bg-primary/5 hover:text-primary transition-all duration-300 shadow-sm {className}"
          onclick={askOrbis}
        >
          <Sparkles size={12} class="text-primary animate-pulse" />
          <span>{m.academics_student_evaluation_ask_orbis_btn_short()}</span>
        </Button>
      {/snippet}
    </TooltipTrigger>
    <TooltipContent side="top" sideOffset={6} class="text-[9px] font-light text-muted-foreground max-w-[200px] text-center leading-normal">
      {label}
    </TooltipContent>
  </Tooltip>
</TooltipProvider>
