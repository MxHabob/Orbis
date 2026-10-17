<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Checkbox } from "@orbis/ui/checkbox";
  import { Label } from "@orbis/ui/label";
  import { Check, ShieldCheck } from "@orbis/icons";
  import { Card, CardContent } from "@orbis/ui/card";
  import type { OnboardingFormModel } from "../types";

  let { form = $bindable<OnboardingFormModel>() }: {
    form: OnboardingFormModel;
  } = $props();
</script>

<div class="mx-auto w-full max-w-3xl space-y-6 py-2 text-start">
  <Card>
    <CardContent class="space-y-6 p-5 sm:p-7">
      <div class="flex items-start gap-3">
        <ShieldCheck class="mt-0.5 size-5 shrink-0 text-primary" aria-hidden="true" />
        <div class="min-w-0 flex-1 space-y-2">
          <h2 class="text-lg font-semibold tracking-tight text-foreground">
            {m.ui_onboarding_mfa_two_factor_authentication_setup()}
          </h2>
          <p class="text-xs font-medium text-muted-foreground">
            {m.ui_onboarding_mfa_optional_security()}
          </p>
          <p class="text-sm leading-relaxed text-muted-foreground">
            {m.ui_onboarding_mfa_activation_after_bootstrap_desc()}
          </p>
        </div>
      </div>

      <div class="flex items-start gap-3">
        <Checkbox id="bootstrap-enable-mfa" bind:checked={form.enableMfa} class="mt-0.5" />
        <div class="min-w-0 flex-1 space-y-1">
          <Label for="bootstrap-enable-mfa" class="cursor-pointer text-sm font-medium text-foreground">
            {m.common_account_mfa_enable_btn()}
          </Label>
          <p class="text-xs leading-relaxed text-muted-foreground">
            {m.ui_onboarding_mfa_you_can_safely_skip_this_step_and_configure_2fa_later_in_settings()}
          </p>
        </div>
        {#if form.enableMfa}
          <Check class="mt-0.5 size-4 shrink-0 text-primary" aria-hidden="true" />
        {/if}
      </div>

      <div class="flex items-start gap-3 text-muted-foreground">
        <ShieldCheck class="mt-0.5 size-4 shrink-0 text-primary" aria-hidden="true" />
        <p class="text-xs leading-relaxed text-muted-foreground">
          {m.ui_onboarding_mfa_enrollment_timing_hint()}
        </p>
      </div>
    </CardContent>
  </Card>
</div>
