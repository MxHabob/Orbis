<!--
@component PromotionLayout
Horizontal SecondaryMenu tab layout for Academic Promotion.
Wraps wizard and history sub-routes.
-->
<script lang="ts">
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import { TrendingUp, History } from "lucide-svelte";
import type { Snippet } from "svelte";
import * as common from "$lib/paraglide/messages/common";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/academics/promotion`);

const menuItems = $derived([
  {
    path: "",
    label: common.promo_wizard_title ? common.promo_wizard_title() : "معالج الترفيع",
    icon: TrendingUp,
  },
  {
    path: "history",
    label: common.promo_history ? common.promo_history() : "سجل الترفيع",
    icon: History,
  },
]);
</script>

<div class="h-full w-full bg-background flex flex-col overflow-hidden">
  <div class="px-6 border-b border-border/20 bg-background/30 shrink-0">
    <SecondaryMenu items={menuItems} {basePath} />
  </div>
  <div class="grow overflow-hidden">
    {@render children()}
  </div>
</div>
