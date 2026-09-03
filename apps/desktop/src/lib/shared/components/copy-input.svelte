<!--
@component CopyInput
A styled click-to-copy input bar with smooth checkmark state transition.
-->
<script lang="ts">
import { cn } from "@orbis/ui/utils";
import { Copy, Check } from "@lucide/svelte";

let {
  value,
  class: className,
  oncopy,
}: {
  value: string;
  class?: string;
  oncopy?: () => void;
} = $props();

let isCopied = $state(false);
let timeoutId: ReturnType<typeof setTimeout> | undefined;

function handleCopy() {
  if (!value) return;

  navigator.clipboard
    .writeText(value)
    .then(() => {
      isCopied = true;
      oncopy?.();

      if (timeoutId) clearTimeout(timeoutId);
      timeoutId = setTimeout(() => {
        isCopied = false;
      }, 2000);
    })
    .catch((err) => {
      console.error("Failed to copy text: ", err);
    });
}
</script>

<button
  type="button"
  onclick={handleCopy}
  class={cn(
    "flex items-center justify-between w-full border border-border bg-card hover:bg-muted/30 text-start rounded-lg py-2 px-4 cursor-pointer focus:outline-hidden focus:ring-1 focus:ring-primary/45 transition-colors select-none",
    className
  )}
>
  <span class="text-muted-foreground text-xs truncate mr-4 select-all font-mono">
    {value}
  </span>

  <div class="relative w-4 h-4 shrink-0 flex items-center justify-center">
    <!-- Copy Icon -->
    <span
      class="absolute transition-all duration-200 flex items-center justify-center"
      class:opacity-0={isCopied}
      class:scale-75={isCopied}
      class:rotate-12={isCopied}
    >
      <Copy class="h-3.5 w-3.5 text-muted-foreground hover:text-foreground" />
    </span>

    <!-- Check Icon -->
    <span
      class="absolute transition-all duration-200 flex items-center justify-center"
      class:opacity-0={!isCopied}
      class:scale-75={!isCopied}
      class:rotate-0={!isCopied}
      class:scale-100={isCopied}
    >
      <Check class="h-3.5 w-3.5 text-emerald-500" />
    </span>
  </div>
</button>
