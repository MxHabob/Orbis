<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import RegistrationSettings from "$lib/features/settings/registration-settings.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_reg_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <RegistrationSettings {orgId} />
</div>
