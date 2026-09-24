<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { ConnectionSetup } from "$lib/features/auth";
</script>

<svelte:head>
  <title>{m.setup_connection_page_title()}</title>
</svelte:head>

<ConnectionSetup />

