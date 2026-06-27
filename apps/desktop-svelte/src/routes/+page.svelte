<!--
  Root page — redirects to the org dashboard or login.
  Restores auth from localStorage on first load.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { onMount } from "svelte";
import { Loader2 } from "lucide-svelte";

onMount(async () => {
  authStore.restore();

  if (authStore.isAuthenticated && authStore.orgId) {
    await authStore.checkSetupStatus();
    goto(`/${authStore.orgId}`, { replaceState: true });
  } else {
    goto("/login", { replaceState: true });
  }
});
</script>

<svelte:head>
	<title>Orbis</title>
</svelte:head>

<div class="loading-root">
	<Loader2 size={20} strokeWidth={2} />
</div>

<style>
	.loading-root {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100vw;
		height: 100vh;
		color: hsl(var(--muted-foreground));
		background-color: hsl(var(--muted));
	}
</style>
