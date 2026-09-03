<!--
@component InstitutionInfo
A tooltip wrapper that shows informational text based on the financial institution API provider.
-->
<script lang="ts">
import { type Snippet } from "svelte";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@orbis/ui";

let {
  provider,
  side = "right",
  children,
}: {
  provider: string;
  side?: "top" | "right" | "bottom" | "left";
  children: Snippet;
} = $props();

const description = $derived.by(() => {
  switch (provider?.toLowerCase()) {
    case "gocardless":
      return "With GoCardless we can connect to more than 2,500 banks in 31 countries across the UK and Europe.";
    case "plaid":
      return "With Plaid we can connect to 12,000+ financial institutions across the US, Canada, UK, and Europe.";
    case "teller":
      return "With Teller we can connect instantly to more than 5,000 financial institutions in the US.";
    case "enablebanking":
      return "With Enable Banking we can connect to more than 2,500 banks in Europe.";
    default:
      return `Information about connection provider: ${provider}`;
  }
});
</script>

<TooltipProvider delayDuration={100}>
  <Tooltip>
    <TooltipTrigger>
      {@render children()}
    </TooltipTrigger>
    <TooltipContent {side} class="w-[300px] text-xs p-3">
      {description}
    </TooltipContent>
  </Tooltip>
</TooltipProvider>
