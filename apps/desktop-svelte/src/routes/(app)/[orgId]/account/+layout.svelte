<script lang="ts">
import { page } from "$app/state";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import type { Snippet } from "svelte";
import * as common from "$lib/paraglide/messages/common";
import { getLocale } from "$lib/paraglide/runtime";

let { children }: { children: Snippet } = $props();

const orgId = $derived(page.params.orgId || "");
const basePath = $derived(`/${orgId}/account`);

const menuItems = $derived([
  {
    path: "",
    label: common.account_menu_information(),
  },
  {
    path: "security",
    label: common.account_menu_security(),
  },
]);
</script>

<div
  class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide"
  dir={getLocale() === "ar" ? "rtl" : "ltr"}
>
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    <!-- Secondary Navigation -->
    <SecondaryMenu items={menuItems} {basePath} />

    <!-- Sub-route content -->
    <div class="w-full flex-1">
      {@render children()}
    </div>
  </div>
</div>
