<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Badge } from "@orbis/ui/badge";
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
  <Card class="overflow-hidden border-border/60 shadow-sm">
    <CardContent class="space-y-6 p-5 sm:p-7">
      <div class="flex items-start gap-4">
        <div class="flex size-12 shrink-0 items-center justify-center rounded-2xl bg-primary/10 text-primary ring-1 ring-primary/15">
          <ShieldCheck class="size-6" aria-hidden="true" />
        </div>
        <div class="min-w-0 flex-1 space-y-2">
          <Badge variant="outline" class="border-primary/30 text-primary">
            {m.ui_onboarding_mfa_optional_security()}
          </Badge>
          <h2 class="text-lg font-semibold tracking-tight text-foreground">
            {m.ui_onboarding_mfa_two_factor_authentication_setup()}
          </h2>
          <p class="text-sm leading-relaxed text-muted-foreground">
            {m.ui_onboarding_mfa_activation_after_bootstrap_desc()}
          </p>
        </div>
      </div>

      <div class="flex items-start gap-3 rounded-xl border border-border/60 bg-muted/20 p-4 transition-colors hover:bg-muted/40">
        <Checkbox id="bootstrap-enable-mfa" bind:checked={form.enableMfa} class="mt-0.5" />
        <div class="min-w-0 flex-1 space-y-1">
          <Label for="bootstrap-enable-mfa" class="cursor-pointer text-sm font-semibold text-foreground">
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

      <div class="flex items-start gap-3 rounded-lg border border-primary/15 bg-primary/5 px-4 py-3">
        <ShieldCheck class="mt-0.5 size-4 shrink-0 text-primary" aria-hidden="true" />
        <p class="text-xs leading-relaxed text-muted-foreground">
          {m.ui_onboarding_mfa_enrollment_timing_hint()}
        </p>
      </div>
    </CardContent>
  </Card>
</div>
