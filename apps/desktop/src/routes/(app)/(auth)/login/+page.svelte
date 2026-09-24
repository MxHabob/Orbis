<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { LoginWorkspace } from "$lib/features/auth";
</script>

<svelte:head>
	<title>{m.common_login_title()}</title>
</svelte:head>

<LoginWorkspace />

