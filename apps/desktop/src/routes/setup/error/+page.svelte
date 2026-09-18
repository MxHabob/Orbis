<!--
  Error Recovery Screen — shown when startup fails.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { AlertTriangle, RefreshCw, Settings } from "@lucide/svelte";

function retry() {
  goto("/", { replaceState: true });
}

function reconfigure() {
  goto("/setup/connection", { replaceState: true });
}
</script>

<svelte:head>
	<title>Orbis — Error</title>
</svelte:head>

<div class="setup-card">
	<div class="error-icon">
		<AlertTriangle size={40} strokeWidth={1.5} />
	</div>
	<h1>Something went wrong</h1>
	<p class="error-message">
		Orbis could not start properly. This may be due to a database
		connection issue or a configuration problem.
	</p>

	<div class="btn-group">
		<button class="primary-btn" onclick={retry}>
			<RefreshCw size={16} />
			Try Again
		</button>
		<button class="secondary-btn" onclick={reconfigure}>
			<Settings size={16} />
			Reconfigure Connection
		</button>
	</div>
</div>

<style>
	.setup-card {
		width: 100%;
		max-width: 420px;
		background: hsl(222 47% 11% / 0.9);
		border: 1px solid hsl(215 20% 22%);
		border-radius: 16px;
		padding: 2.5rem;
		text-align: center;
		backdrop-filter: blur(20px);
	}

	.error-icon {
		color: hsl(35 90% 55%);
		margin-bottom: 1.25rem;
	}

	h1 {
		font-size: 1.3rem;
		font-weight: 600;
		color: hsl(210 40% 96%);
		margin: 0 0 0.75rem;
	}

	.error-message {
		font-size: 0.85rem;
		color: hsl(215 20% 55%);
		line-height: 1.6;
		margin: 0 0 2rem;
	}

	.btn-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.primary-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: hsl(210 100% 50%);
		border: none;
		border-radius: 10px;
		color: white;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.primary-btn:hover {
		background: hsl(210 100% 55%);
	}

	.secondary-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.7rem 1rem;
		background: hsl(215 20% 18%);
		border: 1px solid hsl(215 20% 28%);
		border-radius: 10px;
		color: hsl(210 40% 80%);
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.secondary-btn:hover {
		background: hsl(215 20% 22%);
	}
</style>
