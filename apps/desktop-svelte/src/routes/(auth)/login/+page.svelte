<!--
  Login page — (auth)/login/+page.svelte
  Standalone auth page outside the AppChrome shell.
  Clean centered card with email/password form.
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores';
	import { School, Eye, EyeOff, Loader2 } from 'lucide-svelte';

	let email = $state('');
	let password = $state('');
	let showPassword = $state(false);
	let error = $state<string | null>(null);
	let isLoading = $state(false);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!email || !password) {
			error = 'Please fill in all fields';
			return;
		}

		isLoading = true;
		error = null;

		try {
			await authStore.login(email, password);
			const orgId = authStore.orgId;
			goto(`/${orgId}`);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Login failed. Please try again.';
		} finally {
			isLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Login — Orbis</title>
</svelte:head>

<div class="login-root">
	<div class="login-container animate-scale-in">
		<!-- Logo -->
		<div class="login-logo">
			<div class="logo-icon">
				<School size={24} strokeWidth={1.8} />
			</div>
			<span class="logo-text">Orbis</span>
		</div>

		<!-- Heading -->
		<div class="login-heading">
			<h1 class="login-title">Welcome back</h1>
			<p class="login-subtitle">Sign in to continue to your school</p>
		</div>

		<!-- Form -->
		<form class="login-form" onsubmit={handleSubmit}>
			{#if error}
				<div class="login-error animate-fly-in">
					<span>{error}</span>
				</div>
			{/if}

			<div class="field">
				<label class="field-label" for="email">Email address</label>
				<input
					id="email"
					type="email"
					class="field-input"
					placeholder="you@school.edu"
					bind:value={email}
					required
					autocomplete="email"
					disabled={isLoading}
				/>
			</div>

			<div class="field">
				<label class="field-label" for="password">Password</label>
				<div class="field-input-wrap">
					<input
						id="password"
						type={showPassword ? 'text' : 'password'}
						class="field-input"
						placeholder="••••••••"
						bind:value={password}
						required
						autocomplete="current-password"
						disabled={isLoading}
					/>
					<button
						type="button"
						class="field-toggle"
						onclick={() => (showPassword = !showPassword)}
						aria-label={showPassword ? 'Hide password' : 'Show password'}
					>
						{#if showPassword}
							<EyeOff size={14} strokeWidth={2} />
						{:else}
							<Eye size={14} strokeWidth={2} />
						{/if}
					</button>
				</div>
			</div>

			<button
				type="submit"
				class="login-btn"
				disabled={isLoading}
			>
				{#if isLoading}
					<Loader2 size={14} strokeWidth={2} class="spin" />
					Signing in...
				{:else}
					Sign in
				{/if}
			</button>
		</form>

		<p class="login-footer">
			Don't have an account? <a href="/signup" class="login-link">Get started</a>
		</p>
	</div>
</div>

<style>
	.login-root {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100vw;
		height: 100vh;
		background-color: hsl(var(--muted));
	}

	.login-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		width: 100%;
		max-width: 380px;
		padding: 36px 32px 28px;
		border-radius: var(--radius-l);
		border: 1px solid var(--panel-border);
		background: var(--panel-bg);
		box-shadow:
			0 8px 30px -4px hsl(var(--foreground) / 0.06),
			0 2px 8px -2px hsl(var(--foreground) / 0.04);
	}

	/* ── Logo ── */
	.login-logo {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 24px;
	}

	.logo-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: var(--radius-ml);
		background: hsl(var(--primary));
		color: hsl(var(--primary-foreground));
	}

	.logo-text {
		font-size: 20px;
		font-weight: 700;
		color: hsl(var(--foreground));
		letter-spacing: -0.02em;
	}

	/* ── Heading ── */
	.login-heading {
		text-align: center;
		margin-bottom: 24px;
	}

	.login-title {
		font-size: 18px;
		font-weight: 700;
		color: hsl(var(--foreground));
		margin-bottom: 4px;
	}

	.login-subtitle {
		font-size: 13px;
		color: hsl(var(--muted-foreground));
	}

	/* ── Form ── */
	.login-form {
		display: flex;
		flex-direction: column;
		gap: 14px;
		width: 100%;
	}

	.login-error {
		padding: 10px 12px;
		border-radius: var(--radius-m);
		background: hsl(0 84% 60% / 0.08);
		border: 1px solid hsl(0 84% 60% / 0.2);
		color: hsl(0 84% 40%);
		font-size: 12px;
		line-height: 1.4;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	.field-label {
		font-size: 12px;
		font-weight: 500;
		color: hsl(var(--foreground));
	}

	.field-input {
		height: 36px;
		padding: 0 10px;
		border-radius: var(--radius-m);
		border: 1px solid var(--panel-border);
		background: var(--panel-bg);
		color: hsl(var(--foreground));
		font-size: 13px;
		transition:
			border-color var(--transition-fast),
			box-shadow var(--transition-fast);
	}

	.field-input::placeholder {
		color: hsl(var(--muted-foreground) / 0.5);
	}

	.field-input:focus {
		border-color: hsl(var(--primary));
		box-shadow: 0 0 0 2px hsl(var(--primary) / 0.15);
	}

	.field-input:disabled {
		opacity: 0.6;
	}

	.field-input-wrap {
		position: relative;
	}

	.field-input-wrap .field-input {
		width: 100%;
		padding-right: 36px;
	}

	.field-toggle {
		position: absolute;
		right: 2px;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		color: hsl(var(--muted-foreground));
		border-radius: var(--radius-s);
		transition: color var(--transition-fast);
	}

	.field-toggle:hover {
		color: hsl(var(--foreground));
	}

	.login-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		height: 36px;
		margin-top: 4px;
		border-radius: var(--radius-m);
		background: hsl(var(--primary));
		color: hsl(var(--primary-foreground));
		font-size: 13px;
		font-weight: 600;
		transition:
			opacity var(--transition-fast),
			transform var(--transition-fast);
	}

	.login-btn:hover:not(:disabled) {
		opacity: 0.92;
	}

	.login-btn:active:not(:disabled) {
		transform: scale(0.98);
	}

	.login-btn:disabled {
		opacity: 0.7;
	}

	/* ── Footer ── */
	.login-footer {
		margin-top: 20px;
		font-size: 12px;
		color: hsl(var(--muted-foreground));
	}

	.login-link {
		color: hsl(var(--primary));
		font-weight: 500;
		text-decoration: none;
	}

	.login-link:hover {
		text-decoration: underline;
	}

	/* ── Spin animation ── */
	:global(.spin) {
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>
