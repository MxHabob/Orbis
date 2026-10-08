<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Badge } from "@orbis/ui/badge";
  import { ShieldCheck } from "@orbis/icons";

</script>

<div class="mx-auto w-full max-w-3xl space-y-6 py-2 text-start">
  <div class="space-y-2">
    <Badge variant="outline" class="border-primary/30 text-primary">
      {m.ui_onboarding_mfa_optional_security()}
    </Badge>
  </div>

  <section class="flex items-start gap-4 rounded-2xl border border-border/70 bg-card p-5 shadow-sm sm:p-6">
    <div class="flex size-11 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
      <ShieldCheck class="size-5" aria-hidden="true" />
    </div>
    <div class="space-y-2">
      <h3 class="text-sm font-semibold text-foreground">
        {m.ui_onboarding_mfa_elevated_authority_guard()}
      </h3>
      <p class="text-sm leading-relaxed text-muted-foreground">
        {m.ui_onboarding_mfa_you_can_safely_skip_this_step_and_configure_2fa_later_in_settings()}
      </p>
    </div>
  </section>
</div>
