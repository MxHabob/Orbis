<!--
  Login page — (auth)/login/+page.svelte
  Redesigned to match the old desktop React application layout and design.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Card } from "@orbis/ui-svelte/card";
import { Input } from "@orbis/ui-svelte/input";
import { Zap, Eye, EyeOff, Loader2 } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";

let email = $state("");
let password = $state("");
let showPassword = $state(false);
let error = $state<string | null>(null);
let isLoading = $state(false);

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!email || !password) {
    error = m.login_fill_all_fields();
    return;
  }

  isLoading = true;
  error = null;

  try {
    await authStore.login(email, password);
    const orgId = authStore.orgId;
    goto(localizeHref(`/${orgId}`));
  } catch (err) {
    error = err instanceof Error ? err.message : m.login_failed();
  } finally {
    isLoading = false;
  }
}
</script>

<svelte:head>
	<title>{m.login_title()}</title>
</svelte:head>

<div class="min-h-screen bg-background flex relative w-screen">
	<!-- Logo - Fixed position matching website header exactly -->
	<nav class="fixed top-0 left-0 right-0 z-50 w-full pointer-events-none">
		<div class="relative py-3 xl:py-4 px-4 sm:px-4 md:px-4 lg:px-4 xl:px-6 2xl:px-8 flex items-center">
			<a
				href={localizeHref("/")}
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
		<Card class="w-full max-w-md flex min-h-[500px] flex-col justify-center">
			<div class="space-y-8 flex-1 flex flex-col justify-center p-6 sm:p-8">
				<!-- Header -->
				<div class="text-center space-y-2">
					<h1 class="text-lg mb-4 font-serif">
						{m.login_welcome()}
					</h1>
					<p class="font-sans text-sm text-muted-foreground">
						{m.login_subtitle()}
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
								placeholder={m.login_email_placeholder()}
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
									placeholder={m.login_password_placeholder()}
									bind:value={password}
									required
									autocomplete="current-password"
									disabled={isLoading}
									class="pr-10"
								/>
								<Button
									type="button"
									variant="ghost"
									size="icon"
									class="absolute right-2 top-1/2 -translate-y-1/2"
									onclick={() => (showPassword = !showPassword)}
									aria-label={showPassword ? m.login_hide_password() : m.login_show_password()}
								>
									{#if showPassword}
										<EyeOff size={14} strokeWidth={2} />
									{:else}
										<Eye size={14} strokeWidth={2} />
									{/if}
								</Button>
							</div>
						</div>

						<Button
							type="submit"
							class="w-full font-medium h-[40px] mt-2"
							disabled={isLoading}
						>
							{#if isLoading}
								<Loader2 size={14} strokeWidth={2} class="mr-1" />
								{m.login_submitting()}
							{:else}
								{m.login_submit()}
							{/if}
						</Button>
					</form>
				</div>
			</div>

			<!-- Terms and Privacy Policy - Bottom aligned -->
			<div class="text-center mt-auto pt-8">
				<p class="font-sans text-xs text-muted-foreground">
					{m.login_terms_prefix()}{' '}
					<a
						href="https://orbis.ai/terms"
						class="text-muted-foreground hover:text-primary transition-colors underline"
						target="_blank"
						rel="noreferrer"
					>
						{m.login_terms_of_service()}
					</a>{' '}
					{m.login_and()}{' '}
					<a
						href="https://orbis.ai/policy"
						class="text-muted-foreground hover:text-primary transition-colors underline"
						target="_blank"
						rel="noreferrer"
					>
						{m.login_privacy_policy()}
					</a>
				</p>
			</div>
		</Card>
	</div>
</div>

