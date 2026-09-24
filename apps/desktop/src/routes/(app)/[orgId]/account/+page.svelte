<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AccountProfile } from "$lib/features/auth";
</script>

<svelte:head>
  <title>{m.common_account_settings_title()} | Orbis</title>
</svelte:head>

<AccountProfile />

