<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import LoginRules from "$lib/features/settings/login-rules.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_login_rules()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <LoginRules {orgId} />
</div>
