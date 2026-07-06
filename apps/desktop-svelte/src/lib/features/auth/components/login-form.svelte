<script lang="ts">
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Eye, EyeOff, Loader2 } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { toast } from "svelte-sonner";

import { commands } from "$lib/core/ipc";

// Prop indicators if any
let { selectedBranch = "", selectedYear = "" } = $props<{ selectedBranch?: string; selectedYear?: string }>();

let email = $state("");
let password = $state("");
let showPassword = $state(false);
let error = $state<string | null>(null);
let isLoading = $state(false);

async function handleSubmit(e: SubmitEvent) {
  if (!email || !password) {
    error = m.login_fill_all_fields();
    return;
  }

  isLoading = true;
  error = null;

  try {
    // Authenticate with authStore
    await authStore.login(email, password);
    
    // Switch to the selected academic year if specified
    if (selectedYear && authStore.orgId) {
      try {
        const years = await commands.academicYears.list(authStore.orgId);
        const normalized = selectedYear.replace("_", "-");
        const targetYear = years.find(
          (y) => y.year.name.includes(normalized) || y.year.name === normalized
        );
        if (targetYear) {
          await commands.academicYears.switch(targetYear.year.id);
          console.log("Switched academic year on login to:", targetYear.year.name);
        }
      } catch (switchErr) {
        console.warn("Failed to switch academic year on login:", switchErr);
      }
    }

    console.log("Logged in with branch:", selectedBranch, "year:", selectedYear);

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
</script>

<form class="space-y-4" onsubmit={handleSubmit}>
	{#if error}
		<div class="rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs text-destructive flex items-start gap-2 animate-in fade-in slide-in-from-top-1">
			<span>{error}</span>
		</div>
	{/if}

	<!-- Account Field -->
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

	<!-- Actions Bar -->
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
