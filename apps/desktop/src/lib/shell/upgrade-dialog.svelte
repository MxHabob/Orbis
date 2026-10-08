<!--
@component UpgradeDialog
A premium modal dialog for upgrading to the Business Plan.
Triggered by the Bolt icon in the sidebar. Matches Sentry's upgrade layout.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Dialog, DialogContent } from "@orbis/ui/dialog";
import { Button } from "@orbis/ui";
import { Check } from "@orbis/icons";
import { licenseStore, uiStore } from "$lib/stores";

let { open = $bindable(false) }: { open: boolean } = $props();

function handleClose() {
  open = false;
}

function handleActivate() {
  open = false;
  uiStore.setActivationDialog(true);
}

const currentPlanDisplay = $derived.by(() => {
  if (licenseStore.isTrial) {
    return `${licenseStore.plan ?? "Trial"} (${m.license_trial_days_remaining({ count: String(licenseStore.trialDaysRemaining) })})`;
  }
  if (licenseStore.plan) {
    return licenseStore.edition ? `${licenseStore.plan} — ${licenseStore.edition}` : licenseStore.plan;
  }
  return m.common_upgrade_plan_dev();
});
</script>

<Dialog bind:open>
  <DialogContent
    showCloseButton={true}
    class="sm:max-w-[60rem] min-h-[32rem] w-full p-0 gap-0 overflow-hidden grid grid-cols-[3fr_2fr] select-none"
  >
    <!-- Geometric corner accents follow the inherited text direction. -->
    <div class="absolute top-0 inset-e-0 h-20 w-[26rem] origin-center rtl:-scale-x-100 bg-primary/15 dark:bg-[#d9d1e8] opacity-90 z-0 pointer-events-none [clip-path:polygon(0_0,100%_0,100%_100%,76%_100%,76%_70%,57%_70%,57%_42%,8%_42%)]">
    </div>

    <div class="absolute bottom-0 inset-s-0 h-24 w-56 origin-center rtl:-scale-x-100 bg-primary/15 dark:bg-[#d9d1e8] opacity-90 z-0 pointer-events-none [clip-path:polygon(0_0,24%_0,24%_28%,45%_28%,45%_56%,66%_56%,66%_82%,100%_82%,100%_100%,0_100%)]">
    </div>

    <!-- Left Column (Pitch & Action) -->
    <div class="flex flex-col justify-center p-10 pe-6 z-10 text-start">
      <div class="flex flex-col gap-4">
        <!-- Badge -->
        <span class="text-[9.5px] font-bold tracking-widest text-primary uppercase">
          {m.common_upgrade_next_stop()}
        </span>

        <!-- Headline -->
        <h2 class="text-[23px] font-extrabold text-foreground leading-tight">
          {m.common_upgrade_title()}
        </h2>

        <!-- Description Paragraphs -->
        <div class="flex flex-col gap-3.5 text-muted-foreground text-[11.5px] leading-relaxed pe-2">
          <p>
            {m.common_upgrade_desc_1()}
          </p>
          <p>
            {m.common_upgrade_desc_2()}
          </p>
          <p>
            {m.common_upgrade_desc_3()}
          </p>
          <p class="font-semibold text-foreground/90">
            {m.common_upgrade_desc_4()}
          </p>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-3 mt-8">
        <Button
          type="button"
          onclick={handleActivate}
          class="px-5 py-2.5 text-[11.5px] font-bold"
        >
          {m.common_upgrade_btn_now()}
        </Button>
        <Button
          variant="secondary"
          onclick={handleClose}
          class="px-5 py-2.5 text-[11.5px] font-bold"
        >
          {m.common_upgrade_btn_later()}
        </Button>
      </div>
    </div>

    <!-- Right Column (Features & Current Plan) -->
    <div class="flex flex-col justify-center p-10 ps-8 border-s border-border z-10 text-start bg-muted/10">
      <!-- Features List -->
      <div class="flex flex-col">
        <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-4.5">
          {m.common_upgrade_features_title()}
        </h3>
        
        <ul class="flex flex-col gap-3">
          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-foreground">
            <div class="w-4 h-4 rounded-full border border-border flex items-center justify-center text-primary shrink-0 bg-card">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>{m.common_upgrade_feature_1()}</span>
          </li>
          
          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-foreground">
            <div class="w-4 h-4 rounded-full border border-border flex items-center justify-center text-primary shrink-0 bg-card">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>{m.common_upgrade_feature_2()}</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-foreground">
            <div class="w-4 h-4 rounded-full border border-border flex items-center justify-center text-primary shrink-0 bg-card">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>{m.common_upgrade_feature_3()}</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-foreground">
            <div class="w-4 h-4 rounded-full border border-border flex items-center justify-center text-primary shrink-0 bg-card">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>{m.common_upgrade_feature_4()}</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-foreground">
            <div class="w-4 h-4 rounded-full border border-border flex items-center justify-center text-primary shrink-0 bg-card">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>{m.common_upgrade_feature_5()}</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11.5px] text-muted-foreground italic font-medium">
            <div class="w-4 h-4 flex items-center justify-center text-muted-foreground shrink-0">
              <Check size={11} strokeWidth={3} />
            </div>
            <span>{m.common_upgrade_feature_more()}</span>
          </li>
        </ul>
      </div>

      <!-- Current Plan Status -->
      <div class="mt-10 flex flex-col gap-0.5 select-none">
        <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-widest">
          {m.license_current_plan()}
        </span>
        <span class="text-[15px] font-extrabold text-foreground">
          {currentPlanDisplay}
        </span>
        <a
          href="https://www.mulverse.com"
          target="_blank"
          rel="noopener noreferrer"
          class="text-[11px] font-semibold text-primary hover:underline mt-1.5 transition-colors"
        >
          {m.common_upgrade_compare_plans()}
        </a>
      </div>
    </div>
  </DialogContent>
</Dialog>
