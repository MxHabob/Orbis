<!--
  Login page — (auth)/login/+page.svelte
  Redesigned to match Sentry's exact layout and design system (S.C.R.A.P.S.) with credentials on the left and environment selection on the right.
  Refactored into clean sub-components for modularity and maintainability.
-->
<script lang="ts">
import { Card } from "@orbis/ui-svelte/card";
import { Logo } from "@orbis/ui-svelte/logo";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

import AnnouncementBanner from "$lib/features/auth/components/announcement-banner.svelte";
import BackgroundPattern from "$lib/features/auth/components/background-pattern.svelte";
import LoginForm from "$lib/features/auth/components/login-form.svelte";
import EnvironmentSelect from "$lib/features/auth/components/environment-select.svelte";

let selectedBranch = $state("");
let selectedYear = $state("");
</script>

<svelte:head>
	<title>{m.login_title()}</title>
</svelte:head>

<div class="h-full w-full bg-background flex flex-col items-center justify-center relative overflow-hidden" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
	<AnnouncementBanner />

	<BackgroundPattern />

	<!-- Centered Login Box -->
	<div class="flex-1 w-full flex flex-col items-center justify-center p-4 sm:p-8 md:p-12 relative z-10">
		<Card class="w-full max-w-4xl bg-card border border-border shadow-2xl rounded-2xl overflow-hidden flex flex-col md:flex-row min-h-[480px]">
			<!-- Accent Left Sidebar -->
			<div class="w-full md:w-20 bg-brand-purple-dark flex md:flex-col items-center justify-between md:justify-start p-4 md:py-8 gap-4 text-primary-foreground shrink-0 select-none">
				<div class="flex items-center justify-center text-primary-foreground">
					<Logo class="size-8 text-primary-foreground" dotClass="fill-white" />
				</div>
			</div>

			<!-- Main Content Panel -->
			<div class="flex-1 p-6 sm:p-8 md:p-10 flex flex-col justify-between">
				<div>
					<!-- Sentry-like Header: Sign in to continue -->
					<div class="flex flex-col text-start mb-4 select-none">
						<h2 class="text-xl md:text-2xl font-bold tracking-tight text-foreground">
							{m.login_sign_in_to_continue()}
						</h2>
					</div>

					<!-- Navigation Tabs Placeholder Style (Only "Sign In" is available, no SSO) -->
					<div class="flex items-center justify-between border-b border-border/60 pb-1 mb-6 flex-wrap gap-2">
						<div class="flex items-center gap-4">
							<span 
								class="border-b-2 border-primary text-foreground rounded-none px-1 py-1.5 font-semibold text-xs bg-transparent uppercase tracking-wider select-none"
							>
								{m.login_submit()}
							</span>
						</div>
					</div>

					<!-- Split Layout for Credentials Form & Environment Inputs -->
					<div class="flex flex-col md:flex-row gap-8 md:gap-10">
						<!-- Left Side: Credentials Form -->
						<div class="flex-1 flex flex-col gap-4">
							<LoginForm  />
						</div>

						<!-- Vertical Divider Line (Only on md devices) -->
						<div class="hidden md:block w-px bg-border/60 self-stretch"></div>

						<!-- Right Side: Environment Selection -->
						<div class="flex-1 flex flex-col gap-4 border-t md:border-t-0 border-border/60 pt-6 md:pt-0">
							<EnvironmentSelect />
						</div>
					</div>
				</div>

				<!-- Footer Policy Terms -->
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
