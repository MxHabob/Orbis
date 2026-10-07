<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { AdjustmentsHorizontal, ArrowPath, Eye, EyeSlash } from "@orbis/icons";

import { localizeHref } from "$lib/paraglide/runtime";
import { toast } from "@orbis/ui";

import { loginInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

// No props needed

let showPassword = $state(false);
const form = new FormController({
  initialValues: { identifier: "", password: "" },
  schema: loginInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (payload) => {
    await authStore.login(payload.identifier, payload.password);
    const targetOrgId = authStore.orgId || "default";
    await goto(localizeHref(`/${targetOrgId}`), {
      invalidateAll: true,
      replaceState: true,
    });
  });
  if (!submitted && !form.formError && Object.keys(form.errors).length === 0) {
    form.formError = m.auth_login_failed();
  }
}

function handleForgotPassword() {
  toast.info(m.auth_login_forgot_password_toast());
}
</script>

<form class="space-y-4" onsubmit={handleSubmit}>
	{#if form.formError}
		<div class="rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs text-destructive flex items-start gap-2 animate-in fade-in slide-in-from-top-1">
			<span>{form.formError}</span>
		</div>
	{/if}

	<!-- Account Field -->
	<FormField id="email" label={m.common_nav_account()} error={fieldError("identifier")} required>
		{#snippet children()}
			<Input
				id="email"
				type="email"
				placeholder={m.auth_login_account_placeholder()}
				bind:value={form.values.identifier}
				onblur={() => form.markTouched("identifier")}
				required
				aria-invalid={Boolean(fieldError("identifier"))}
				autocomplete="email"
				disabled={form.isSubmitting}
				class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input"
			/>
		{/snippet}
	</FormField>

	<!-- Password Field -->
	<FormField id="password" label={m.common_user_form_password()} error={fieldError("password")} required>
		{#snippet children()}
			<div class="flex items-center gap-2">
			<Input
				id="password"
				type={showPassword ? 'text' : 'password'}
				placeholder={m.common_user_form_password()}
				bind:value={form.values.password}
				onblur={() => form.markTouched("password")}
				required
				aria-invalid={Boolean(fieldError("password"))}
				autocomplete="current-password"
				disabled={form.isSubmitting}
				class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input flex-1"
			/>
			<Button
				type="button"
				variant="outline"
				size="icon"
				class="h-10 w-10 shrink-0 text-muted-foreground hover:text-foreground cursor-pointer rounded-md bg-background/50 border-input"
				onclick={() => (showPassword = !showPassword)}
				aria-label={showPassword ? m.auth_login_hide_password() : m.auth_login_show_password()}
			>
				{#if showPassword}
					<EyeSlash size={15} strokeWidth={2} />
				{:else}
					<Eye size={15} strokeWidth={2} />
				{/if}
			</Button>
			</div>
		{/snippet}
	</FormField>

	<!-- Actions Bar -->
	<div class="flex flex-wrap items-center gap-3 sm:gap-4 pt-2">
		<Button
			type="submit"
			variant="default"
			class="font-semibold h-10 text-xs px-6 cursor-pointer bg-primary text-primary-foreground hover:bg-primary/95"
			disabled={form.isSubmitting}
		>
			{#if form.isSubmitting}
				<LoadingIndicator class="mr-1.5"  size={13} />
				{m.auth_login_submitting()}
			{:else}
				{m.auth_login_continue()}
			{/if}
		</Button>

		<Button
			type="button"
			variant="link"
			class="text-xs text-muted-foreground hover:text-foreground px-0 h-auto cursor-pointer font-normal underline underline-offset-4 hover:no-underline"
			onclick={handleForgotPassword}
		>
			{m.auth_login_forgot_password()}
		</Button>
	</div>
</form>
