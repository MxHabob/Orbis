<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { UserForm } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.common_user_form_title_create()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <UserForm {orgId} />
</div>
