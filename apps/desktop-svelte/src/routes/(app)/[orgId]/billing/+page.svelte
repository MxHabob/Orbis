<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { page } from "$app/state";
  let orgId = $derived((page.params.orgId as string) || "");
</script>

<svelte:head>
  <title>Billing | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto flex flex-col gap-6 scrollbar-hide">
  <div class="h-full w-full flex justify-center items-center flex-col gap-2">
    <h1 class="text-3xl font-bold">{m.billing_coming_soon()}</h1>
    <p>{orgId}</p>
  </div>
</div>
