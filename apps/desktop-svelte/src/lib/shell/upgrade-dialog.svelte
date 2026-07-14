<!--
@component UpgradeDialog
A premium modal dialog for upgrading to the Business Plan.
Triggered by the Zap icon in the sidebar. Matches Sentry's upgrade layout.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Dialog, DialogContent } from "@orbis/ui-svelte/dialog";
import { Button } from "@orbis/ui-svelte";

import { Check } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";

let { open = $bindable(false) }: { open: boolean } = $props();

function handleClose() {
  open = false;
}
</script>

<Dialog bind:open>
  <DialogContent
    showCloseButton={true}
    dir={getLocale() === "ar" ? "rtl" : "ltr"}
    class="sm:max-w-[800px] w-full p-0 gap-0 overflow-hidden bg-card border border-border rounded-2xl flex select-none text-card-foreground shadow-[0_24px_50px_rgba(0,0,0,0.25)] dark:shadow-[0_24px_50px_rgba(0,0,0,0.65)]"
  >
    <!-- Top-Right Geometric Slanted Corner Graphic (using brand primary with opacity, flips in RTL layout) -->
    <svg
      class="absolute top-0 inset-e-0 h-32 w-64 text-primary/15 dark:text-primary/10 opacity-90 overflow-visible z-0 pointer-events-none"
      style={getLocale() === "ar" ? "transform: scaleX(-1);" : ""}
      viewBox="0 0 250 110"
      fill="currentColor"
    >
      <path d="M70 0 L250 0 L250 110 L220 110 L220 85 L190 85 L190 55 L160 55 L160 25 L70 25 Z" />
    </svg>

    <!-- Bottom-Left Geometric Slanted Corner Graphic (using brand primary with opacity, flips in RTL layout) -->
    <svg
      class="absolute bottom-0 inset-s-0 h-32 w-64 text-primary/15 dark:text-primary/10 opacity-90 overflow-visible z-0 pointer-events-none"
      style={getLocale() === "ar" ? "transform: scaleX(-1);" : ""}
      viewBox="0 0 250 110"
      fill="currentColor"
    >
      <path d="M0 110 L0 0 L20 0 L20 30 L50 30 L50 65 L85 65 L85 90 L250 90 L250 110 Z" />
    </svg>

    <!-- Left Column (Pitch & Action) -->
    <div class="w-[60%] flex flex-col justify-between p-10 pe-6 z-10 text-start">
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
        <div class="flex flex-col gap-3.5 text-muted-foreground text-[11.5px] leading-relaxed pr-2">
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
      <div class="flex items-center gap-3 mt-6">
        <Button
          type="button"
          class="px-5 py-2.5 text-[11.5px] font-bold rounded-lg bg-primary hover:bg-primary/90 text-primary-foreground cursor-pointer transition-all duration-150 shadow-md border-0"
        >
          {m.common_upgrade_btn_now()}
        </Button>
        <Button
          variant="outline"
          onclick={handleClose}
          class="px-5 py-2.5 text-[11.5px] font-bold rounded-lg bg-muted border border-border hover:bg-muted/80 text-muted-foreground cursor-pointer transition-all duration-150"
        >
          {m.common_upgrade_btn_later()}
        </Button>
      </div>
    </div>

    <!-- Right Column (Features & Current Plan) -->
    <div class="w-[40%] flex flex-col justify-between p-10 ps-8 border-s border-border z-10 text-start bg-muted/10">
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
      <div class="mt-8 flex flex-col gap-0.5 select-none">
        <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-widest">
          {m.common_dt_type_system()}
        </span>
        <span class="text-[15px] font-extrabold text-foreground">
          {m.common_upgrade_plan_dev()}
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
