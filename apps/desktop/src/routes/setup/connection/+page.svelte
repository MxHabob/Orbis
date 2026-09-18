<!--
  Connection Setup Screen — first-time setup for database connection.
  Supports Local (SQLite) and Remote (Orbis Server) modes.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import type { ConnectionMode, RemoteHealthResult } from "$lib/api/types";
import { Loader2, Database, Globe, CheckCircle2, XCircle, ArrowRight } from "@lucide/svelte";

let selectedMode: ConnectionMode = $state("local");
let serverUrl = $state("");
let dbFilename = $state("orbis.db");
let isConnecting = $state(false);
let error = $state("");
let healthResult: RemoteHealthResult | null = $state(null);
let isCheckingHealth = $state(false);

async function checkHealth() {
  if (!serverUrl.trim()) return;
  isCheckingHealth = true;
  healthResult = null;
  error = "";
  try {
    healthResult = await api.connectionCheckRemoteHealth(serverUrl.trim());
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isCheckingHealth = false;
  }
}

async function connect() {
  isConnecting = true;
  error = "";
  try {
    await api.connectionSetConfig(
      selectedMode,
      selectedMode === "remote" ? serverUrl.trim() : undefined,
      selectedMode === "local" ? dbFilename.trim() : undefined,
    );
    const result = await api.connectionConnect();
    if (result.connected) {
      // Check if organization exists
      const hasOrg = await api.systemHasOrganization();
      if (hasOrg) {
        goto("/login", { replaceState: true });
      } else {
        goto("/setup/bootstrap", { replaceState: true });
      }
    }
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isConnecting = false;
  }
}
</script>

<svelte:head>
	<title>Orbis — Connection Setup</title>
</svelte:head>

<div class="setup-card">
	<div class="setup-header">
		<div class="logo-mark">
			<svg viewBox="0 0 32 32" fill="none" width="40" height="40">
				<circle cx="16" cy="16" r="14" stroke="hsl(210 100% 60%)" stroke-width="2.5" opacity="0.3" />
				<circle cx="16" cy="16" r="8" fill="hsl(210 100% 60%)" opacity="0.8" />
				<circle cx="16" cy="16" r="3" fill="white" />
			</svg>
		</div>
		<h1>Welcome to Orbis</h1>
		<p class="subtitle">Configure your database connection to get started</p>
	</div>

	<div class="mode-selector">
		<button
			class="mode-card"
			class:active={selectedMode === "local"}
			onclick={() => { selectedMode = "local"; error = ""; }}
		>
			<Database size={24} />
			<div class="mode-info">
				<span class="mode-title">Local Database</span>
				<span class="mode-desc">SQLite — data stored on this device</span>
			</div>
		</button>

		<button
			class="mode-card"
			class:active={selectedMode === "remote"}
			onclick={() => { selectedMode = "remote"; error = ""; }}
		>
			<Globe size={24} />
			<div class="mode-info">
				<span class="mode-title">Remote Server</span>
				<span class="mode-desc">Connect to an Orbis Server instance</span>
			</div>
		</button>
	</div>

	{#if selectedMode === "local"}
		<div class="field-group">
			<label for="db-filename">Database File</label>
			<input
				id="db-filename"
				type="text"
				bind:value={dbFilename}
				placeholder="orbis.db"
			/>
			<span class="field-hint">Stored in your application data directory</span>
		</div>
	{:else}
		<div class="field-group">
			<label for="server-url">Server URL</label>
			<div class="input-with-action">
				<input
					id="server-url"
					type="url"
					bind:value={serverUrl}
					placeholder="https://api.orbis.school"
				/>
				<button
					class="check-btn"
					onclick={checkHealth}
					disabled={isCheckingHealth || !serverUrl.trim()}
				>
					{#if isCheckingHealth}
						<Loader2 size={14} class="spin" />
					{:else}
						Test
					{/if}
				</button>
			</div>

			{#if healthResult}
				<div class="health-result" class:healthy={healthResult.reachable} class:unhealthy={!healthResult.reachable}>
					{#if healthResult.reachable}
						<CheckCircle2 size={16} />
						<span>Connected — {healthResult.version ?? "OK"} ({healthResult.latency_ms}ms)</span>
					{:else}
						<XCircle size={16} />
						<span>{healthResult.error ?? "Connection failed"}</span>
					{/if}
				</div>
			{/if}
		</div>
	{/if}

	{#if error}
		<div class="error-banner">{error}</div>
	{/if}

	<button
		class="connect-btn"
		onclick={connect}
		disabled={isConnecting || (selectedMode === "remote" && !serverUrl.trim())}
	>
		{#if isConnecting}
			<Loader2 size={16} class="spin" />
			Connecting...
		{:else}
			Connect
			<ArrowRight size={16} />
		{/if}
	</button>
</div>

<style>
	.setup-card {
		width: 100%;
		max-width: 480px;
		background: hsl(222 47% 11% / 0.9);
		border: 1px solid hsl(215 20% 22%);
		border-radius: 16px;
		padding: 2.5rem;
		backdrop-filter: blur(20px);
	}

	.setup-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.logo-mark {
		display: flex;
		justify-content: center;
		margin-bottom: 1rem;
	}

	h1 {
		font-size: 1.5rem;
		font-weight: 600;
		color: hsl(210 40% 96%);
		margin: 0 0 0.5rem;
	}

	.subtitle {
		font-size: 0.875rem;
		color: hsl(215 20% 55%);
		margin: 0;
	}

	.mode-selector {
		display: flex;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.mode-card {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 1rem;
		border: 1.5px solid hsl(215 20% 22%);
		border-radius: 12px;
		background: transparent;
		color: hsl(215 20% 55%);
		cursor: pointer;
		transition: all 0.2s ease;
		text-align: center;
	}

	.mode-card:hover {
		border-color: hsl(210 100% 50% / 0.4);
		color: hsl(210 40% 80%);
	}

	.mode-card.active {
		border-color: hsl(210 100% 50%);
		background: hsl(210 100% 50% / 0.08);
		color: hsl(210 100% 70%);
	}

	.mode-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.mode-title {
		font-size: 0.875rem;
		font-weight: 600;
	}

	.mode-desc {
		font-size: 0.7rem;
		opacity: 0.7;
	}

	.field-group {
		margin-bottom: 1.5rem;
	}

	.field-group label {
		display: block;
		font-size: 0.8rem;
		font-weight: 500;
		color: hsl(215 20% 65%);
		margin-bottom: 0.5rem;
	}

	.field-group input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: hsl(222 47% 8%);
		border: 1px solid hsl(215 20% 22%);
		border-radius: 8px;
		color: hsl(210 40% 96%);
		font-size: 0.875rem;
		outline: none;
		transition: border-color 0.15s;
	}

	.field-group input:focus {
		border-color: hsl(210 100% 50%);
	}

	.field-hint {
		display: block;
		font-size: 0.7rem;
		color: hsl(215 20% 45%);
		margin-top: 0.375rem;
	}

	.input-with-action {
		display: flex;
		gap: 0.5rem;
	}

	.input-with-action input {
		flex: 1;
	}

	.check-btn {
		padding: 0.625rem 1rem;
		background: hsl(215 20% 18%);
		border: 1px solid hsl(215 20% 28%);
		border-radius: 8px;
		color: hsl(210 40% 80%);
		font-size: 0.8rem;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.15s;
	}

	.check-btn:hover:not(:disabled) {
		background: hsl(215 20% 22%);
	}

	.check-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.health-result {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		margin-top: 0.5rem;
		padding: 0.5rem 0.75rem;
		border-radius: 8px;
	}

	.health-result.healthy {
		color: hsl(142 70% 60%);
		background: hsl(142 70% 40% / 0.1);
	}

	.health-result.unhealthy {
		color: hsl(0 70% 60%);
		background: hsl(0 70% 40% / 0.1);
	}

	.error-banner {
		padding: 0.75rem 1rem;
		background: hsl(0 70% 40% / 0.15);
		border: 1px solid hsl(0 70% 40% / 0.3);
		border-radius: 8px;
		color: hsl(0 70% 70%);
		font-size: 0.8rem;
		margin-bottom: 1.5rem;
	}

	.connect-btn {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: hsl(210 100% 50%);
		border: none;
		border-radius: 10px;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.connect-btn:hover:not(:disabled) {
		background: hsl(210 100% 55%);
		transform: translateY(-1px);
		box-shadow: 0 4px 16px hsl(210 100% 50% / 0.3);
	}

	.connect-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.spin) {
		animation: spin 0.9s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
