<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();

const orgId = $derived(page.params.orgId || "");
const basePath = $derived(`/${orgId}/account`);

const menuItems = $derived([
  {
    path: "",
    label: m.common_account_menu_information(),
  },
  {
    path: "security",
    label: m.common_account_menu_security(),
  },
  {
    path: "appearance",
    label: m.appearance_nav_label(),
  },
]);
</script>

<div class="flex h-full w-full flex-col overflow-hidden">
  <!-- Secondary Navigation -->
  <div class="shrink-0 px-5 pt-2 md:px-8">
    <SecondaryMenu items={menuItems} {basePath} />
  </div>

  <!-- Sub-route content -->
  <div class="min-h-0 flex-1 overflow-y-auto scrollbar-hide px-5 py-6 pb-10 md:px-8 md:py-7">
    <div class="w-full">
      {@render children()}
    </div>
  </div>
</div>
