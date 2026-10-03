<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AccountSecurity } from "$lib/features/auth";
</script>

<svelte:head>
  <title>{m.common_account_security_title()} | Orbis</title>
</svelte:head>

<AccountSecurity />

