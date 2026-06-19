<!--
  Login page — (auth)/login/+page.svelte
  Standalone auth page outside the AppChrome shell.
  Uses UI components from @orbis/ui-svelte and Tailwind CSS.
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores';
	import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@orbis/ui-svelte/card';
	import { Input } from '@orbis/ui-svelte/input';
	import { Button } from '@orbis/ui-svelte/button';
	import { Label } from '@orbis/ui-svelte/label';
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

<div class="flex h-screen w-screen items-center justify-center bg-muted p-4">
	<Card class="w-full max-w-[380px] p-6 shadow-lg border border-border bg-card animate-scale-in">
		<CardHeader class="flex flex-col items-center gap-2 pb-6 text-center">
			<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary text-primary-foreground">
				<School size={20} strokeWidth={1.8} />
			</div>
			<CardTitle class="text-xl font-bold tracking-tight text-foreground">Welcome back</CardTitle>
			<CardDescription class="text-xs text-muted-foreground">Sign in to continue to your school</CardDescription>
		</CardHeader>

		<CardContent class="p-0">
			<form class="flex flex-col gap-4" onsubmit={handleSubmit}>
				{#if error}
					<div class="rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs text-destructive animate-fly-in">
						<span>{error}</span>
					</div>
				{/if}

				<div class="flex flex-col gap-1.5">
					<Label for="email" class="text-xs font-semibold text-foreground">Email address</Label>
					<Input
						id="email"
						type="email"
						placeholder="you@school.edu"
						bind:value={email}
						required
						autocomplete="email"
						disabled={isLoading}
						class="input-sunken"
					/>
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="password" class="text-xs font-semibold text-foreground">Password</Label>
					<div class="relative w-full">
						<Input
							id="password"
							type={showPassword ? 'text' : 'password'}
							placeholder="••••••••"
							bind:value={password}
							required
							autocomplete="current-password"
							disabled={isLoading}
							class="input-sunken pr-10"
						/>
						<button
							type="button"
							class="absolute right-2 top-1/2 -translate-y-1/2 flex h-8 w-8 items-center justify-center text-muted-foreground hover:text-foreground rounded transition-colors"
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

				<Button
					type="submit"
					class="btn-tactile-primary w-full mt-2"
					disabled={isLoading}
				>
					{#if isLoading}
						<Loader2 size={14} strokeWidth={2} class="animate-spin mr-1" />
						Signing in...
					{:else}
						Sign in
					{/if}
				</Button>
			</form>

			<p class="mt-6 text-center text-xs text-muted-foreground">
				Don't have an account? <a href="/signup" class="font-medium text-primary hover:underline">Get started</a>
			</p>
		</CardContent>
	</Card>
</div>
