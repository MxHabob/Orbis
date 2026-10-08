<!--
  @component StepAdmin
  Step 4: Initial Primary Administrator Account.
  Spacious 2-column layout with a real user avatar preview and real-time password strength,
  and elevated design tokens.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { OnboardingFormModel } from "../types";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { Card, CardContent } from "@orbis/ui/card";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

let {
  form = $bindable<OnboardingFormModel>(),
  errors = {},
}: {
  form: OnboardingFormModel;
  errors?: Record<string, string>;
} = $props();

const passwordStrength = $derived.by(() => {
  const pwd = form.adminPassword || "";
  let score = 0;
  if (pwd.length >= 8) score++;
  if (pwd.length >= 12) score++;
  if (/[A-Z]/.test(pwd)) score++;
  if (/[0-9]/.test(pwd)) score++;
  if (/[^A-Za-z0-9]/.test(pwd)) score++;
  return score;
});

const strengthLabel = $derived.by(() => {
  if (!form.adminPassword) return { text: "", color: "bg-muted", width: "0%" };
  if (passwordStrength <= 2) return { text: m.ui_onboarding_admin_weak(), color: "bg-destructive", width: "30%" };
  if (passwordStrength <= 3) return { text: m.ui_onboarding_admin_fair(), color: "bg-amber-500", width: "60%" };
  if (passwordStrength <= 4) return { text: m.ui_onboarding_admin_good(), color: "bg-blue-500", width: "80%" };
  return { text: m.ui_onboarding_admin_strong(), color: "bg-emerald-500", width: "100%" };
});
</script>

<div class="space-y-6 max-w-5xl mx-auto py-2">
  <div class="space-y-2 text-start">
    <p class="text-sm text-muted-foreground max-w-2xl leading-relaxed">
      {m.operations_ui_admin_account_description()}
    </p>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 text-start">
    <!-- Left Column: Avatar & Role Identity Preview -->
    <Card class="lg:col-span-4">
      <CardContent class="flex flex-col items-center justify-between p-6 text-center">
      <div class="space-y-4 w-full flex flex-col items-center">
        <UserAvatar
          name={`${form.adminFirstNameAr || form.adminFirstName} ${form.adminLastNameAr || form.adminLastName}`.trim() || m.ui_onboarding_admin_administrator()}
          size="lg"
          class="size-20 border-2 border-primary/30 shadow-sm ring-4 ring-primary/10"
        />

        <div class="space-y-1 w-full">
          <div class="text-sm font-bold text-foreground truncate">
            {form.adminFirstNameAr || form.adminFirstName || (m.ui_onboarding_admin_administrator())} {form.adminLastNameAr || form.adminLastName || ""}
          </div>
          <div class="text-xs font-mono text-muted-foreground truncate">
            {form.adminEmail || "admin@institution.edu"}
          </div>
        </div>
      </div>

      <div class="w-full mt-6 pt-4 border-t border-border/40 text-xs text-muted-foreground leading-relaxed">
        {m.operations_ui_admin_password_security_notice()}
      </div>
      </CardContent>
    </Card>

    <!-- Right Column: Form Inputs -->
    <Card class="lg:col-span-8">
      <CardContent class="space-y-5 p-6">
      <div class="border-b border-border/40 pb-3">
        <h3 class="text-sm font-bold text-foreground uppercase tracking-wider font-mono">
          {m.ui_onboarding_admin_administrator_credentials()}
        </h3>
      </div>

      <div class="space-y-4">
        <BilingualNameFields
          arId="admin-fn-ar"
          enId="admin-fn-en"
          arLabel={m.ui_onboarding_admin_first_name_arabic()}
          enLabel={m.ui_onboarding_admin_first_name_english()}
          enRequired
          bind:arValue={form.adminFirstNameAr}
          bind:enValue={form.adminFirstName}
          arPlaceholder={m.operations_ui_admin_first_name_arabic_example()}
          enPlaceholder="e.g. Tariq"
          enError={errors.adminFirstName}
          kind="person-name"
          requestSuggestion={suggestLocalTranslation}
          replaceUneditedInitialValues
          loadingMessage={m.bilingual_entry_loading()}
          failureMessage={m.bilingual_entry_failure()}
        />
        <BilingualNameFields
          arId="admin-ln-ar"
          enId="admin-ln-en"
          arLabel={m.ui_onboarding_admin_last_name_arabic()}
          enLabel={m.ui_onboarding_admin_last_name_english()}
          enRequired
          bind:arValue={form.adminLastNameAr}
          bind:enValue={form.adminLastName}
          arPlaceholder={m.operations_ui_admin_last_name_arabic_example()}
          enPlaceholder="e.g. Al-Mansoor"
          enError={errors.adminLastName}
          kind="person-name"
          requestSuggestion={suggestLocalTranslation}
          replaceUneditedInitialValues
          loadingMessage={m.bilingual_entry_loading()}
          failureMessage={m.bilingual_entry_failure()}
        />
      </div>

      <!-- Email Address -->
      <div class="space-y-1.5">
        <Label for="admin-email" class="text-xs font-medium text-foreground">
          {m.ui_onboarding_admin_primary_email_address()}
          <span class="text-destructive" aria-hidden="true"> *</span>
        </Label>
        <Input
          id="admin-email"
          type="email"
          placeholder="admin@institution.edu"
          bind:value={form.adminEmail}
          required
          aria-invalid={errors.adminEmail ? "true" : undefined}
          aria-describedby={errors.adminEmail ? "admin-email-error" : undefined}
          class="h-9 text-xs font-mono"
        />
        {#if errors.adminEmail}
          <p id="admin-email-error" class="text-[11px] text-destructive" role="alert">
            {errors.adminEmail}
          </p>
        {/if}
      </div>

      <div class="space-y-1.5">
        <Label for="admin-phone" class="text-xs font-medium text-foreground">
          {m.setup_bootstrap_admin_phone_optional()}
        </Label>
        <Input
          id="admin-phone"
          type="tel"
          autocomplete="tel"
          bind:value={form.adminPhone}
          class="h-9 text-xs"
        />
      </div>

      <!-- Password and Confirmation -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="admin-pwd" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_password()}
            <span class="text-destructive" aria-hidden="true"> *</span>
          </Label>
          <Input
            id="admin-pwd"
            type="password"
            placeholder="••••••••••••"
            bind:value={form.adminPassword}
            required
            aria-invalid={errors.adminPassword ? "true" : undefined}
            aria-describedby={errors.adminPassword ? "admin-pwd-error" : undefined}
            class="h-9 text-xs"
          />
          {#if errors.adminPassword}
            <p id="admin-pwd-error" class="text-[11px] text-destructive" role="alert">
              {errors.adminPassword}
            </p>
          {/if}

          <!-- Real-Time Password Strength Meter -->
          {#if form.adminPassword}
            <div class="space-y-1 pt-1">
              <div class="h-1.5 w-full bg-muted rounded-full overflow-hidden">
                <div
                  class="h-full {strengthLabel.color} transition-all duration-300"
                  style="width: {strengthLabel.width};"
                ></div>
              </div>
              <div class="flex justify-between items-center text-[10px] text-muted-foreground font-mono">
                <span>{m.ui_onboarding_admin_strength()}</span>
                <span class="font-bold">{strengthLabel.text}</span>
              </div>
            </div>
          {/if}
        </div>

        <div class="space-y-1.5">
          <Label for="admin-pwd-confirm" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_confirm_password()}
            <span class="text-destructive" aria-hidden="true"> *</span>
          </Label>
          <Input
            id="admin-pwd-confirm"
            type="password"
            placeholder="••••••••••••"
            bind:value={form.adminConfirmPassword}
            required
            aria-invalid={errors.adminConfirmPassword ? "true" : undefined}
            aria-describedby={errors.adminConfirmPassword ? "admin-pwd-confirm-error" : undefined}
            class="h-9 text-xs"
          />
          {#if errors.adminConfirmPassword}
            <p id="admin-pwd-confirm-error" class="text-[11px] text-destructive" role="alert">
              {errors.adminConfirmPassword}
            </p>
          {/if}
        </div>
      </div>
      </CardContent>
      </Card>
    </div>
</div>
