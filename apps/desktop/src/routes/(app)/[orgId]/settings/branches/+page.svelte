<!--
  @page CurrenciesSettingsPage
  Allows administrators to manage multi-currency exchange rates, define base currencies,
  add new currencies, edit currency configurations, and delete unused currencies.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { Button } from "@orbis/ui/button";

let orgId = $derived((page.params.orgId as string) || "");
</script>

<svelte:head>
  <title>{m.settings_nav_currencies()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto flex flex-col gap-6 scrollbar-hide">
<div class="h-full w-full flex justify-center items-center flex-col gap-2">
  <h1 class="text-3xl font-bold">{m.settings_branches_coming_soon()}</h1>
  <p>{orgId}</p>
  <div class="flex gap-2 mt-2">
    <Button>{m.common_send_feedback()}</Button>
  </div>
</div>
</div>
