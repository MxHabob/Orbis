<!--
  Login page — (auth)/login/+page.svelte
  Redesigned to match Sentry's exact layout and design system (S.C.R.A.P.S.) with credentials on the left and environment selection on the right.
  Refactored into clean sub-components for modularity and maintainability.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card } from "@orbis/ui-svelte/card";
import { Logo } from "@orbis/ui-svelte/logo";

import { getLocale } from "$lib/paraglide/runtime";

import AnnouncementBanner from "$lib/features/auth/components/announcement-banner.svelte";
import BackgroundPattern from "$lib/features/auth/components/background-pattern.svelte";
import LoginForm from "$lib/features/auth/components/login-form.svelte";
import EnvironmentSelect from "$lib/features/auth/components/environment-select.svelte";

let selectedBranch = $state("");
let selectedYear = $state("");
</script>

<svelte:head>
	<title>{m.common_login_title()}</title>
</svelte:head>

<div class="login-page h-full min-h-0 w-full bg-background flex flex-col items-center relative overflow-hidden" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
	<AnnouncementBanner />

	<BackgroundPattern />

	<!-- Centered Login Box -->
	<div class="login-stage flex-1 min-h-0 w-full flex flex-col items-center justify-center p-3 sm:p-6 lg:p-10 relative z-10">
		<Card class="login-card w-full max-w-4xl max-h-full bg-card border border-border shadow-2xl rounded-2xl overflow-hidden flex flex-col lg:flex-row">
			<!-- Accent Left Sidebar -->
			<div class="w-full lg:w-20 bg-brand-purple-dark flex lg:flex-col items-center justify-between lg:justify-start p-4 lg:py-8 gap-4 text-primary-foreground shrink-0 select-none">
				<div class="flex items-center justify-center text-primary-foreground">
					<Logo class="size-8 text-primary-foreground" dotClass="fill-white" />
				</div>
			</div>

			<!-- Main Content Panel -->
			<div class="login-content flex-1 p-5 sm:p-8 lg:p-10 flex flex-col justify-between min-w-0 overflow-hidden">
				<div>
					<!-- Sentry-like Header: Sign in to continue -->
					<div class="flex flex-col text-start mb-4 select-none">
						<h2 class="text-xl sm:text-2xl font-bold tracking-tight text-foreground">
							{m.auth_login_sign_in_to_continue()}
						</h2>
					</div>

					<!-- Navigation Tabs Placeholder Style (Only "Sign In" is available, no SSO) -->
					<div class="flex items-center justify-between border-b border-border/60 pb-1 mb-6 flex-wrap gap-2">
						<div class="flex items-center gap-4">
							<span 
								class="border-b-2 border-primary text-foreground rounded-none px-1 py-1.5 font-semibold text-xs bg-transparent uppercase tracking-wider select-none"
							>
								{m.auth_login_submit()}
							</span>
						</div>
					</div>

					<!-- Split Layout for Credentials Form & Environment Inputs -->
					<div class="flex flex-col lg:flex-row gap-6 sm:gap-8 lg:gap-10">
						<!-- Left Side: Credentials Form -->
						<div class="flex-1 flex flex-col gap-4">
							<LoginForm  />
						</div>

						<!-- Vertical Divider Line (Only on md devices) -->
						<div class="hidden lg:block w-px bg-border/60 self-stretch"></div>

						<!-- Right Side: Environment Selection -->
						<div class="flex-1 flex flex-col gap-4 border-t lg:border-t-0 border-border/60 pt-6 lg:pt-0 min-w-0">
							<EnvironmentSelect />
						</div>
					</div>
				</div>

				<!-- Footer Policy Terms -->
				<div class="text-center pt-6 sm:pt-8 border-t border-border/40 mt-6 sm:mt-8 select-none">
					<p class="font-sans text-[10px] text-muted-foreground/80 leading-normal">
						{m.auth_login_terms_prefix()}{' '}
						<a
							href="https://mulverse.com/?route=terms"
							class="text-muted-foreground hover:text-primary transition-colors underline font-medium"
							target="_blank"
							rel="noreferrer"
						>
							{m.common_sidebar_help_terms()}
						</a>{' '}
						{m.auth_login_and()}{' '}
						<a
							href="https://mulverse.com/?route=policy"
							class="text-muted-foreground hover:text-primary transition-colors underline font-medium"
							target="_blank"
							rel="noreferrer"
						>
							{m.common_login_privacy_policy()}
						</a>
					</p>
				</div>
			</div>
		</Card>
	</div>
</div>

<style>
	@media (max-height: 680px) {
		.login-stage {
			padding: 0.5rem 1rem;
		}

		.login-content {
			padding: 1rem 1.25rem;
		}

		.login-content :global(.mb-6) {
			margin-bottom: 0.75rem;
		}

		.login-content :global(.pt-6) {
			padding-top: 0.75rem;
		}

		.login-content :global(.mt-6) {
			margin-top: 0.75rem;
		}
	}

	@media (max-height: 560px) {
		.login-content {
			padding-top: 0.625rem;
			padding-bottom: 0.625rem;
		}

		.login-content :global(.space-y-4) > :global(:not([hidden]) ~ :not([hidden])) {
			margin-top: 0.5rem;
		}
	}
</style>
