<script lang="ts">
import { Dialog, DialogContent } from "@orbis/ui-svelte/dialog";
import { Button } from "@orbis/ui-svelte/button";
import { Sparkles, Award } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { isDesktopApp } from "$lib/services/ipc";
import illustration from "$lib/assets/image.png";

let { open = $bindable(false) }: { open: boolean } = $props();

const handleLearnMore = async () => {
  const url = "https://mulverse.co";
  if (isDesktopApp()) {
    try {
      const { open: openUrl } = await import("@tauri-apps/plugin-opener");
      await openUrl(url);
    } catch (e) {
      window.open(url, "_blank");
    }
  } else {
    window.open(url, "_blank");
  }
};
</script>

<Dialog bind:open>
  <DialogContent
    showCloseButton={true}
    class="sm:max-w-[380px] p-0 gap-0 overflow-hidden bg-zinc-950 text-white rounded-xl border border-zinc-800 shadow-2xl"
  >
    <!-- Top Illustration region (Light theme section) -->
    <div class="bg-zinc-100 h-[190px] flex items-center justify-center relative p-6">
      <img
        src={illustration}
        alt="Orbis Illustration"
        class="h-full w-auto object-contain max-h-[150px]"
        draggable="false"
      />
    </div>

    <div
      class="p-6 flex flex-col select-text"
      dir={getLocale() === "ar" ? "rtl" : "ltr"}
    >
      <h2 class="text-xl font-bold  mb-5 text-center">
        {m.whats_new_header_title()}
      </h2>

      <!-- Features List -->
      <div class="flex flex-col gap-4 mb-6 text-start">
        <!-- Feature item 1 -->
        <div class="flex items-start gap-3">
          <div class="flex flex-col gap-0.5 min-w-0">
            <h3 class="text-xs font-semibold ">
              {m.whats_new_feature1_title()}
            </h3>
            <p class="text-[11px]  leading-normal font-light">
              {m.whats_new_feature1_desc()}
            </p>
          </div>
        </div>

        <!-- Feature item 2 -->
        <div class="flex items-start gap-3">
          <div class="flex flex-col gap-0.5 min-w-0">
            <h3 class="text-xs font-semibold ">
              {m.whats_new_feature2_title()}
            </h3>
            <p class="text-[11px]  leading-normal font-light">
              {m.whats_new_feature2_desc()}
            </p>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="text-[9px] mt-5 pt-3 w-full">
        <Button
          class="w-full  font-semibold h-9"
          onclick={handleLearnMore}
        >
          {m.whats_new_footer()}
      </Button>
      </div>
    </div>
  </DialogContent>
</Dialog>
