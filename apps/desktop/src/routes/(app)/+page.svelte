<!--
  Root page — startup orchestrator.
  Resolves the startup phase and routes to the correct screen:
  - connection_setup → /connection
  - bootstrap → /onboarding
  - login / ready → /login or /[orgId]
  - error → /connection
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { onMount } from "svelte";
import { resolveStartupPhase, type StartupState } from "$lib/core/startup";
import { localizeHref } from "$lib/paraglide/runtime";
import { ArrowPath } from "@orbis/icons";

let startupState: StartupState | null = $state(null);
let statusText = $state("Initializing...");

onMount(async () => {
  statusText = "Checking connection...";
  const state = await resolveStartupPhase();
  startupState = state;

  switch (state.phase) {
    case "connection_setup":
      statusText = "Setup required";
      goto(localizeHref("/connection"), { replaceState: true });
      break;

    case "bootstrap":
      statusText = "Organization setup required";
      goto(localizeHref("/onboarding"), { replaceState: true });
      break;

    case "login":
    case "ready":
      statusText = "Restoring session...";
      await authStore.restore();
      if (authStore.isAuthenticated && authStore.orgId) {
        if (authStore.needsOnboarding) {
          goto(localizeHref("/onboarding"), { replaceState: true });
        } else {
          goto(localizeHref(`/${authStore.orgId}`), { replaceState: true });
        }
      } else {
        goto(localizeHref("/login"), { replaceState: true });
      }
      break;

    case "error":
      statusText = state.error ?? "An error occurred";
      goto(localizeHref("/connection"), { replaceState: true });
      break;

    default:
      // Unknown phase — fallback to login
      goto(localizeHref("/login"), { replaceState: true });
  }
});
</script>

<svelte:head>
	<title>Orbis</title>
</svelte:head>

<div class="loading-root">
	<div class="loading-content">
		<span class="loading-icon" aria-hidden="true">
			<ArrowPath size={20} strokeWidth={2} class="animate-spin text-primary" />
		</span>
		<span class="loading-text">{statusText}</span>
	</div>
</div>

<style>
	.loading-root {
		position: fixed;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		color: hsl(var(--muted-foreground));
		background-color: hsl(var(--muted));
	}

	.loading-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
	}

	.loading-icon {
		animation: loading-spin 0.9s linear infinite;
	}

	.loading-text {
		font-size: 0.8rem;
		opacity: 0.7;
	}

	@keyframes loading-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
