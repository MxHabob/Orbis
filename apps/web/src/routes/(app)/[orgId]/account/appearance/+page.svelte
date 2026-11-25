<script lang="ts">
import * as m from "$lib/paraglide/messages";
import AppearanceSettings from "$lib/features/settings/appearance-settings.svelte";
</script>

<svelte:head>
  <title>{m.appearance_title()} | Orbis</title>
</svelte:head>

<AppearanceSettings />
