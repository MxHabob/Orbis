<!--
  Login page — (auth)/login/+page.svelte
  Redesigned to match the old desktop React application layout and design.
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores';
	import { Input } from '@orbis/ui-svelte/input';
	import { Button } from '@orbis/ui-svelte/button';
	import { Zap, Eye, EyeOff, Loader2 } from 'lucide-svelte';

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

<div class="min-h-screen bg-background flex relative w-screen">
	<!-- Logo - Fixed position matching website header exactly -->
	<nav class="fixed top-0 left-0 right-0 z-50 w-full pointer-events-none">
		<div class="relative py-3 xl:py-4 px-4 sm:px-4 md:px-4 lg:px-4 xl:px-6 2xl:px-8 flex items-center">
			<a
				href="/"
				class="flex items-center gap-2 hover:opacity-80 active:opacity-80 transition-opacity duration-200 pointer-events-auto"
			>
				<div class="w-6 h-6">
					<Zap class="w-full h-full text-foreground" />
				</div>
			</a>
		</div>
	</nav>

	<!-- Login Form -->
	<div class="w-full flex-1 flex flex-col items-center justify-center p-8 lg:p-12 pb-6">
		<div class="w-full max-w-md flex flex-col flex-1 justify-center min-h-[500px]">
			<div class="space-y-8 flex-1 flex flex-col justify-center">
				<!-- Header -->
				<div class="text-center space-y-2">
					<h1 class="text-lg mb-4 font-serif">
						Welcome to Orbis School
					</h1>
					<p class="font-sans text-sm text-muted-foreground">
						Sign in or create an account
					</p>
				</div>

				<!-- Form wrapper -->
				<div class="space-y-3 flex flex-col items-center justify-center w-full">
					<form class="w-full space-y-4" onsubmit={handleSubmit}>
						{#if error}
							<div class="rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs text-destructive">
								<span>{error}</span>
							</div>
						{/if}

						<div class="flex flex-col gap-1.5">
							<Input
								id="email"
								type="email"
								placeholder="Email address"
								bind:value={email}
								required
								autocomplete="email"
								disabled={isLoading}
							/>
						</div>

						<div class="flex flex-col gap-1.5">
							<div class="relative w-full">
								<Input
									id="password"
									type={showPassword ? 'text' : 'password'}
									placeholder="Password"
									bind:value={password}
									required
									autocomplete="current-password"
									disabled={isLoading}
									class="pr-10"
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
							class="w-full font-medium h-[40px] mt-2"
							disabled={isLoading}
						>
							{#if isLoading}
								<Loader2 size={14} strokeWidth={2} class="mr-1" />
								Signing in...
							{:else}
								Sign In
							{/if}
						</Button>
					</form>
				</div>
			</div>

			<!-- Terms and Privacy Policy - Bottom aligned -->
			<div class="text-center mt-auto pt-8">
				<p class="font-sans text-xs text-muted-foreground">
					By signing in you agree to our{' '}
					<a
						href="https://orbis.ai/terms"
						class="text-muted-foreground hover:text-primary transition-colors underline"
						target="_blank"
						rel="noreferrer"
					>
						Terms of service
					</a>{' '}
					&{' '}
					<a
						href="https://orbis.ai/policy"
						class="text-muted-foreground hover:text-primary transition-colors underline"
						target="_blank"
						rel="noreferrer"
					>
						Privacy policy
					</a>
				</p>
			</div>
		</div>
	</div>
</div>
