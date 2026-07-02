<!--
@component SearchField
A reusable search input field with query binding/runes, search icon prefix,
and Escape key shortcut to clear the search query.
-->
<script lang="ts">
import { Input } from "@orbis/ui-svelte/input";
import { cn } from "@orbis/ui-svelte/utils";
import { Search } from "lucide-svelte";

let {
  placeholder = "Search...",
  value = $bindable(""),
  class: className,
  oninput,
  onclear,
}: {
  placeholder?: string;
  value?: string;
  class?: string;
  oninput?: (value: string) => void;
  onclear?: () => void;
} = $props();

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  value = target.value;
  oninput?.(value);
}

function handleKeyDown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    value = "";
    oninput?.("");
    onclear?.();
    event.preventDefault();
  }
}
</script>

<div class={cn("relative w-full md:max-w-[380px] flex items-center", className)}>
  <Search class="absolute pointer-events-none start-3 text-muted-foreground/60 shrink-0" size={15} />
  
  <Input
    type="text"
    placeholder={placeholder}
    {value}
    oninput={handleInput}
    onkeydown={handleKeyDown}
    class="ps-9 w-full h-9 text-xs"
    autocomplete="off"
    autocapitalize="none"
    autocorrect="off"
    spellcheck="false"
  />
</div>
