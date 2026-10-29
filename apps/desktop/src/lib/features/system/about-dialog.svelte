<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api/client";
import type { AppInfo } from "$lib/api/types";
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { GlobeAlt, Sparkles } from "@orbis/icons";
import mulverseOrbisImg from "$lib/assets/mulverse-orbis.png";

let { open = $bindable(false) }: { open?: boolean } = $props();

let info = $state<AppInfo | null>(null);
let isLoading = $state(false);

const OFFICIAL_WEBSITE = "https://www.mulverse.com/";

async function loadAppInfo() {
  isLoading = true;
  try {
    info = await api.systemGetAppInfo();
  } catch (error) {
    console.error("Failed to load application information:", error);
  } finally {
    isLoading = false;
  }
}

async function openChangelog() {
  try {
    await api.openChangelogWindow();
  } catch (error) {
    console.error("Failed to open the changelog window:", error);
    window.open("/changelog", "_blank", "width=860,height=680");
  }
}

function openWebsite() {
  window.open(OFFICIAL_WEBSITE, "_blank", "noopener,noreferrer");
}

$effect(() => {
  if (open && !info && !isLoading) void loadAppInfo();
});
</script>

<Dialog bind:open>
  <DialogContent
    showCloseButton={false}
    class=" w-full max-w-lg sm:max-w-[560px] gap-0 overflow-hidden font-sans"
  >
    <DialogTitle class="sr-only">About {info?.name ?? "Orbis"}</DialogTitle>
    <DialogDescription class="sr-only">Orbis and Mulverse Information</DialogDescription>

    <!-- Top Visual Banner -->
    <div class="relative w-full h-48 sm:h-52 overflow-hidden flex items-center justify-center">
      <img
        src={mulverseOrbisImg}
        alt="Mulverse Orbis"
        class="w-full h-full object-cover select-none"
      />
    </div>

    <!-- Bottom Content Area -->
    <div class="pt-4 px-1 pb-1 flex flex-col gap-1.5 text-start">
      <span class=" text-[11px] font-semibold tracking-wider">
        {m.ui_system_about_company_name()}
      </span>

      <div class="flex items-center gap-2">
        <h2 class="text-xl sm:text-2xl font-black tracking-tight capitalize">
          {info?.name ?? "Orbis"}
        </h2>
        {#if info}
          <span class="text-[11px] font-mono font-semibold px-2 py-0.5 rounded-full border">
            v{info.version}
          </span>
        {/if}
      </div>

      <p class="text-xs sm:text-[13px] leading-relaxed max-w-lg pt-0.5">
        {m.ui_system_about_company_desc()}
      </p>

      {#if info}
        <div class="flex flex-wrap items-center gap-x-2 gap-y-0.5 text-[10.5px] font-mono pt-1">
          <span class="capitalize">{info.build_profile}</span>
          <span>•</span>
          <span>{info.target_os}{info.target_arch ? `/${info.target_arch}` : ""}</span>
          <span>•</span>
          <span class="font-sans">{m.ui_system_about_hq_location()}</span>
        </div>
      {/if}

      <div class="flex items-center justify-between gap-2 pt-3 mt-1">
        <Button
          variant="ghost"
          size="sm"
          onclick={openWebsite}
          class="inline-flex items-center gap-1.5 text-xs font-semibold cursor-pointer select-none"
        >
          <GlobeAlt size={13} />
          <span>{m.ui_system_about_website()}</span>
        </Button>

        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onclick={openChangelog}
            class=" inline-flex items-center gap-1.5 text-xs font-semibold cursor-pointer select-none"
          >
            <Sparkles size={13} />
            <span>{m.ui_system_about_changelog()}</span>
          </Button>

          <Button
            variant="default"
            size="sm"
            onclick={() => (open = false)}
            class="text-xs h-8 px-5 font-semibold cursor-pointer"
          >
            {m.ui_system_about_close()}
          </Button>
        </div>
      </div>
    </div>
  </DialogContent>
</Dialog>
