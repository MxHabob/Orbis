<!--
  @component StepAdmin
  Step 4: Create the initial primary administrator account.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { OnboardingFormModel } from "../types";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { Card, CardContent } from "@orbis/ui/card";
import { ShieldCheck, UserBadgeCheck } from "@orbis/icons";
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

<section class="mx-auto w-full max-w-3xl space-y-6 py-2 text-start">
  <header class="space-y-2">
    <p class="max-w-2xl text-sm leading-relaxed text-muted-foreground">
      {m.operations_ui_admin_account_description()}
    </p>
  </header>

  <Card>
    <CardContent class="space-y-6 p-5 sm:p-7">
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div class="flex min-w-0 items-center gap-3">
          <UserBadgeCheck class="size-5 shrink-0 text-primary" aria-hidden="true" />
          <div class="min-w-0">
            <h2 class="text-base font-semibold tracking-tight text-foreground">
              {m.ui_onboarding_admin_administrator_credentials()}
            </h2>
          </div>
        </div>

        <div class="flex min-w-0 items-center gap-3">
          <UserAvatar
            name={`${form.adminFirstNameAr || form.adminFirstName} ${form.adminLastNameAr || form.adminLastName}`.trim() || m.ui_onboarding_admin_administrator()}
            size="sm"
            class="size-10 shrink-0"
          />
          <div class="min-w-0">
            <p class="truncate text-sm font-semibold text-foreground">
              {`${form.adminFirstNameAr || form.adminFirstName} ${form.adminLastNameAr || form.adminLastName}`.trim() || m.ui_onboarding_admin_administrator()}
            </p>
            <p class="truncate text-xs text-muted-foreground">
              {form.adminEmail || "admin@institution.edu"}
            </p>
          </div>
        </div>
      </div>

      <div class="space-y-5">
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

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div class="space-y-1.5 sm:col-span-2">
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
            class="h-10"
          />
          {#if errors.adminEmail}
            <p id="admin-email-error" class="text-xs text-destructive" role="alert">
              {errors.adminEmail}
            </p>
          {/if}
        </div>

        <div class="space-y-1.5 sm:col-span-2">
          <Label for="admin-phone" class="text-xs font-medium text-foreground">
            {m.setup_bootstrap_admin_phone_optional()}
          </Label>
          <Input
            id="admin-phone"
            type="tel"
            autocomplete="tel"
            bind:value={form.adminPhone}
            class="h-10"
          />
        </div>
      </div>

      <div class="space-y-4">
        <div class="flex items-start gap-3 text-muted-foreground">
          <ShieldCheck class="mt-0.5 size-4 shrink-0 text-primary" aria-hidden="true" />
          <p class="text-xs leading-relaxed text-muted-foreground">
            {m.operations_ui_admin_password_security_notice()}
          </p>
        </div>

        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
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
              autocomplete="new-password"
              class="h-10"
            />
            {#if errors.adminPassword}
              <p id="admin-pwd-error" class="text-xs text-destructive" role="alert">
                {errors.adminPassword}
              </p>
            {/if}

            {#if form.adminPassword}
              <div class="space-y-1.5 pt-1">
                <div
                  class="h-1.5 w-full overflow-hidden rounded-full bg-muted"
                  role="meter"
                  aria-label={m.ui_onboarding_admin_strength()}
                  aria-valuemin={0}
                  aria-valuemax={5}
                  aria-valuenow={passwordStrength}
                >
                  <div
                    class="h-full {strengthLabel.color} transition-all duration-300"
                    style="width: {strengthLabel.width};"
                  ></div>
                </div>
                <div class="flex items-center justify-between text-xs text-muted-foreground">
                  <span>{m.ui_onboarding_admin_strength()}</span>
                  <span class="font-semibold text-foreground">{strengthLabel.text}</span>
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
              autocomplete="new-password"
              class="h-10"
            />
            {#if errors.adminConfirmPassword}
              <p id="admin-pwd-confirm-error" class="text-xs text-destructive" role="alert">
                {errors.adminConfirmPassword}
              </p>
            {/if}
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</section>
