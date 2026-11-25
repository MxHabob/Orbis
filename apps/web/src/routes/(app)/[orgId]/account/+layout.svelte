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

<div class="h-full w-full overflow-y-auto scrollbar-hide px-2">
  <div class="w-full  flex flex-col gap-6">
    <!-- Secondary Navigation -->
    <SecondaryMenu items={menuItems} {basePath} />

    <!-- Sub-route content -->
    <div class="w-full flex-1">
      {@render children()}
    </div>
  </div>
</div>
