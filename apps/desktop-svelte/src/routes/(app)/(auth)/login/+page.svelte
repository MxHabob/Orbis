<!--
  Login page — (auth)/login/+page.svelte
  Redesigned to match Sentry's exact layout and design system (S.C.R.A.P.S.) with credentials on the left and environment selection on the right.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Card } from "@orbis/ui-svelte/card";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui-svelte/select";
import { Zap, Eye, EyeOff, Loader2, Megaphone } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { toast } from "svelte-sonner";

let email = $state("");
let password = $state("");
let showPassword = $state(false);
let error = $state<string | null>(null);
let isLoading = $state(false);

let selectedBranch = $state("main_branch");
let selectedYear = $state("2026_2027");

const branches = $derived([
  { value: "main_branch", label: m.login_default_branch() },
  { value: "secondary_branch", label: m.login_other_branch() }
]);

const academicYears = $derived([
  { value: "2026_2027", label: m.login_current_year() },
  { value: "2027_2028", label: m.login_next_year() }
]);

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
    goto(localizeHref(`/${authStore.orgId}`));
  } catch (err) {
    error = err instanceof Error ? err.message : m.login_failed();
  } finally {
    isLoading = false;
  }
}

function handleForgotPassword() {
  toast.info(m.login_forgot_password_toast());
}

function handleRequestToJoin() {
  toast.success(m.login_request_join_toast());
}
</script>

<svelte:head>
	<title>{m.login_title()}</title>
</svelte:head>

<div class="h-full w-full bg-background flex flex-col items-center justify-center relative overflow-hidden" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
	<!-- Top Announcement Banner (Sentry style) -->
	<div class="w-full bg-primary text-primary-foreground py-2 px-4 text-center text-md font-semibold flex items-center justify-center gap-2 transition-all duration-200 shrink-0 select-none z-10">
		<span>{m.login_banner_text()}</span>
		<a
			href="https://orbis.ai/blog"
			target="_blank"
			rel="noreferrer"
			class="underline hover:opacity-90 transition-opacity flex items-center gap-0.5"
		>
			{m.login_banner_link()}
			<span class="rtl:rotate-180 inline-block font-sans">→</span>
		</a>
	</div>

	<!-- Ambient Grid and SVG repeating background icons (Matches Sentry look exactly) -->
	<svg class="absolute inset-0 w-full h-full text-foreground/5 pointer-events-none select-none z-0" xmlns="http://www.w3.org/2000/svg">
		<defs>
			<pattern id="sentry-pattern" width="160" height="160" patternUnits="userSpaceOnUse" patternTransform="rotate(15)">
				<!-- Grid lines -->
				<line x1="0" y1="0" x2="160" y2="0" stroke="currentColor" stroke-width="0.5" stroke-dasharray="4 4" />
				<line x1="0" y1="0" x2="0" y2="160" stroke="currentColor" stroke-width="0.5" stroke-dasharray="4 4" />
				
				<!-- Triangle Warning at (20, 20) -->
				<path d="M20 15 L35 40 L5 40 Z" fill="none" stroke="currentColor" stroke-width="1" />
				<line x1="20" y1="24" x2="20" y2="32" stroke="currentColor" stroke-width="1.5" />
				<circle cx="20" cy="36" r="1" fill="currentColor" />
				
				<!-- Zap at (100, 30) -->
				<path d="M110 15 L95 32 L105 32 L98 48 L115 28 L105 28 Z" fill="none" stroke="currentColor" stroke-width="1" />
				
				<!-- Circle / 404 at (30, 100) -->
				<circle cx="35" cy="105" r="15" fill="none" stroke="currentColor" stroke-width="1" />
				<text x="35" y="109" font-size="9" font-family="sans-serif" font-weight="bold" fill="currentColor" text-anchor="middle">404</text>
				
				<!-- Shield at (110, 100) -->
				<path d="M100 95 C110 90 120 90 130 95 L130 110 C130 120 115 125 115 125 C115 125 100 120 100 110 Z" fill="none" stroke="currentColor" stroke-width="1" />
			</pattern>
		</defs>
		<rect width="100%" height="100%" fill="url(#sentry-pattern)" />
	</svg>
	<div class="absolute inset-0 bg-linear-to-t from-background via-background/40 to-background pointer-events-none z-0"></div>
	<div class="flex-1 w-full flex flex-col items-center justify-center p-4 sm:p-8 md:p-12 relative z-10">
		<Card class="w-full max-w-4xl overflow-hidden flex flex-col md:flex-row min-h-[480px]">
			<div class="w-full md:w-20 bg-brand-purple-dark flex md:flex-col items-center justify-between md:justify-start p-4 md:py-8 gap-4 text-primary-foreground shrink-0 select-none">
				<div class="flex items-center justify-center text-primary-foreground">
					<Zap class="size-6 fill-current" />
				</div>
			</div>

			<!-- Main Content Panel -->
			<div class="flex-1 p-6 sm:p-8 md:p-10 flex flex-col justify-between">
				<div>
					<div class="flex flex-col text-start mb-4 select-none">
						<h2 class="text-xl md:text-2xl font-bold tracking-tight text-foreground">
							{m.login_sign_in_to_continue()}
						</h2>
					</div>

					<div class="flex flex-col md:flex-row gap-8 md:gap-10">
						<div class="flex-1 flex flex-col gap-4">
							<form class="space-y-4" onsubmit={handleSubmit}>
								{#if error}
									<div class="rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs text-destructive flex items-start gap-2 animate-in fade-in slide-in-from-top-1">
										<span>{error}</span>
									</div>
								{/if}

								<div class="flex flex-col gap-2 text-start">
									<Label for="email" class="text-xs font-semibold text-foreground select-none flex items-center gap-1.5 uppercase tracking-wider">
										<span>{m.login_account_label()}</span>
										<span class="text-[9px] text-muted-foreground/60 font-semibold">{m.login_required_tag()}</span>
									</Label>
									<Input
										id="email"
										type="email"
										placeholder={m.login_account_placeholder()}
										bind:value={email}
										required
										autocomplete="email"
										disabled={isLoading}
										class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input"
									/>
								</div>

								<!-- Password Field -->
								<div class="flex flex-col gap-2 text-start">
									<Label for="password" class="text-xs font-semibold text-foreground select-none flex items-center gap-1.5 uppercase tracking-wider">
										<span>{m.login_password_label()}</span>
										<span class="text-[9px] text-muted-foreground/60 font-semibold">{m.login_required_tag()}</span>
									</Label>
									<div class="flex items-center gap-2">
										<Input
											id="password"
											type={showPassword ? 'text' : 'password'}
											placeholder={m.login_password_placeholder_sentry()}
											bind:value={password}
											required
											autocomplete="current-password"
											disabled={isLoading}
											class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input flex-1"
										/>
										<Button
											type="button"
											variant="outline"
											size="icon"
											class="h-10 w-10 shrink-0 text-muted-foreground hover:text-foreground cursor-pointer rounded-md bg-background/50 border-input"
											onclick={() => (showPassword = !showPassword)}
											aria-label={showPassword ? m.login_hide_password() : m.login_show_password()}
										>
											{#if showPassword}
												<EyeOff size={15} strokeWidth={2} />
											{:else}
												<Eye size={15} strokeWidth={2} />
											{/if}
										</Button>
									</div>
								</div>
								<div class="flex items-center gap-4 pt-2">
									<Button
										type="submit"
										variant="default"
										class="font-semibold h-10 text-xs px-6 cursor-pointer bg-primary text-primary-foreground hover:bg-primary/95"
										disabled={isLoading}
									>
										{#if isLoading}
											<Loader2 size={13} strokeWidth={2} class="mr-1.5 animate-spin" />
											{m.login_submitting()}
										{:else}
											{m.login_continue()}
										{/if}
									</Button>

									<Button
										type="button"
										variant="link"
										class="text-xs text-muted-foreground hover:text-foreground px-0 h-auto cursor-pointer font-normal underline underline-offset-4 hover:no-underline"
										onclick={handleForgotPassword}
									>
										{m.login_forgot_password()}
									</Button>
								</div>
							</form>
						</div>
						<div class="hidden md:block w-px bg-border/60 self-stretch"></div>
						<div class="flex-1 flex flex-col gap-4 border-t md:border-t-0 border-border/60 pt-6 md:pt-0">
							<div class="flex flex-col gap-1 text-start mb-2">
								<h3 class="text-sm font-semibold text-foreground select-none">
									{m.login_env_title()}
								</h3>
							</div>
							<div class="flex flex-col gap-2 text-start">
								<Label class="text-xs font-semibold text-foreground select-none uppercase tracking-wider">
									{m.login_select_org()}
								</Label>
								<Select
									type="single"
									value={selectedBranch}
									onValueChange={(v: string | undefined) => (selectedBranch = v ?? "")}
								>
									<SelectTrigger class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input w-full flex justify-between items-center px-3">
										{branches.find(b => b.value === selectedBranch)?.label || selectedBranch}
									</SelectTrigger>
									<SelectContent>
										{#each branches as b}
											<SelectItem value={b.value}>{b.label}</SelectItem>
										{/each}
									</SelectContent>
								</Select>
							</div>
							<div class="flex flex-col gap-2 text-start">
								<Label class="text-xs font-semibold text-foreground select-none uppercase tracking-wider">
									{m.login_select_year()}
								</Label>
								<Select
									type="single"
									value={selectedYear}
									onValueChange={(v: string | undefined) => (selectedYear = v ?? "")}
								>
									<SelectTrigger class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input w-full flex justify-between items-center px-3">
										{academicYears.find(y => y.value === selectedYear)?.label || selectedYear}
									</SelectTrigger>
									<SelectContent>
										{#each academicYears as y}
											<SelectItem value={y.value}>{y.label}</SelectItem>
										{/each}
									</SelectContent>
								</Select>
							</div>
						</div>
					</div>
				</div>

				<div class="text-center pt-8 border-t border-border/40 mt-8 select-none">
					<p class="font-sans text-[10px] text-muted-foreground/80 leading-normal">
						{m.login_terms_prefix()}{' '}
						<a
							href="https://mulverse.com/?route=terms"
							class="text-muted-foreground hover:text-primary transition-colors underline font-medium"
							target="_blank"
							rel="noreferrer"
						>
							{m.login_terms_of_service()}
						</a>{' '}
						{m.login_and()}{' '}
						<a
							href="https://mulverse.com/?route=policy"
							class="text-muted-foreground hover:text-primary transition-colors underline font-medium"
							target="_blank"
							rel="noreferrer"
						>
							{m.login_privacy_policy()}
						</a>
					</p>
				</div>
			</div>
		</Card>
	</div>
</div>





