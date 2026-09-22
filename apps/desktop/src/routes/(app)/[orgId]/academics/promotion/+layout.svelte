<!--
@component PromotionLayout
Horizontal SecondaryMenu tab layout for Academic Promotion.
Wraps wizard and history sub-routes.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import { ChartBar, ClockRotateRight } from "@orbis/icons";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/academics/promotion`);

const menuItems = $derived([
  {
    path: "",
    label: m.common_promo_wizard_title
      ? m.common_promo_wizard_title()
      : "معالج الترفيع",
    icon: ChartBar,
  },
  {
    path: "history",
    label: m.common_promo_history ? m.common_promo_history() : "سجل الترفيع",
    icon: ClockRotateRight,
  },
]);
</script>

<div class="h-full w-full bg-background flex flex-col overflow-hidden">
  <div class="p-2 border-b border-border/20 bg-background/30 shrink-0">
    <SecondaryMenu items={menuItems} {basePath} />
  </div>
  <div class="grow overflow-hidden">
    {@render children()}
  </div>
</div>
