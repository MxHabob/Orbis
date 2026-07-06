<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { Button } from "@orbis/ui-svelte/button";
import { Sparkles } from "lucide-svelte";

let {
  query,
  label,
  variant = "outline",
  size = "sm",
  class: className = ""
}: {
  query: string;
  label: string;
  variant?: "default" | "lime" | "outline" | "ghost" | "destructive" | "secondary" | "link";
  size?: "default" | "sm" | "xs" | "lg" | "icon" | "icon-sm" | "icon-xs" | "icon-lg";
  class?: string;
} = $props();

function askOrbis() {
  const newUrl = new URL(page.url.href);
  newUrl.searchParams.set("ask_orbis", query);
  goto(newUrl.pathname + newUrl.search, {
    replaceState: true,
    keepFocus: true,
    noScroll: true
  });
}
</script>

<Button
  {variant}
  {size}
  class="font-semibold gap-1.5 border-primary/20 hover:bg-primary/5 hover:text-primary transition-all duration-300 shadow-sm {className}"
  onclick={askOrbis}
>
  <Sparkles size={12} class="text-primary animate-pulse" />
  <span>{label}</span>
</Button>
