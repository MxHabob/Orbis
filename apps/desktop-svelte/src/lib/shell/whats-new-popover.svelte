<script lang="ts">
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@orbis/ui-svelte/popover";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Megaphone } from "lucide-svelte";
import { isDesktopApp } from "$lib/core/ipc";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import orbisEcosystem from "$lib/assets/orbis-ecosystem.png";
import mulverseOrbis from "$lib/assets/mulverse-orbis.png";

const handleLearnMore = async () => {
  const url = "https://mulverse.com";
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

<Popover>
  <PopoverTrigger>
    <Button
      variant="outline"
      size="icon-sm"
      class="rounded-[8px] shrink-0 cursor-pointer"
      title="What's New"
      aria-label="What's new"
    >
      <Megaphone size={14} class="text-foreground stroke-[2px]" />
    </Button>
  </PopoverTrigger>
  
  <PopoverContent
    side="bottom"
    align="end"
    sideOffset={8}
    class="w-[360px] bg-card border border-border shadow-2xl rounded-xl p-0 overflow-hidden z-50 flex flex-col font-sans text-start text-card-foreground"
  >
    <!-- Announcement Header -->
    <div class="px-4 py-3 border-b border-border bg-card/50 flex items-center justify-between select-none">
      <h4 class="font-bold text-xs text-foreground">
        {m.whats_new_header_title()}
      </h4>
    </div>

    <!-- Announcement Feed List (Scrollable) -->
    <div class="whats-new-list flex-1 overflow-y-auto max-h-[440px] p-4 flex flex-col gap-5 bg-card">
      
      <!-- Announcement Item 1 -->
      <div class="flex flex-col gap-2.5 text-start" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
        <div class="flex items-start justify-between gap-2">
          <h5 class="text-[12px] font-bold text-foreground leading-snug">
            {m.whats_new_title_1()}
          </h5>
          <Badge variant="brand" class="text-[9px] px-1.5 py-0 shrink-0 font-semibold bg-primary/10 text-primary border-none">
            {m.whats_new_badge_feature()}
          </Badge>
        </div>
        
        <p class="text-[11px] text-muted-foreground leading-relaxed font-light">
          {m.whats_new_desc_1()}
          <button 
            type="button" 
            onclick={handleLearnMore}
            class="text-primary hover:underline font-medium inline-block p-0 bg-transparent border-0 cursor-pointer text-[11px] ms-1"
          >
            {m.whats_new_read_more()}
          </button>
        </p>

        <!-- Illustration Image 1 -->
        <div class="w-full rounded-lg overflow-hidden border border-border bg-muted/20 flex items-center justify-center">
          <img
            src={orbisEcosystem}
            alt="Orbis Ecosystem Illustration"
            class="w-full h-auto object-cover"
            draggable="false"
          />
        </div>
      </div>

      <!-- Divider -->
      <div class="h-px bg-border/60"></div>

      <!-- Announcement Item 2 -->
      <div class="flex flex-col gap-2.5 text-start pb-2" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
        <div class="flex items-start justify-between gap-2">
          <h5 class="text-[12px] font-bold text-foreground leading-snug">
            {m.whats_new_title_2()}
          </h5>
          <Badge variant="brand" class="text-[9px] px-1.5 py-0 shrink-0 font-semibold bg-primary/10 text-primary border-none">
            {m.whats_new_badge_integration()}
          </Badge>
        </div>
        
        <p class="text-[11px] text-muted-foreground leading-relaxed font-light">
          {m.whats_new_desc_2()}
        </p>

        <!-- Illustration Image 2 -->
        <div class="w-full rounded-lg overflow-hidden border border-border bg-muted/20 flex items-center justify-center">
          <img
            src={mulverseOrbis}
            alt="Mulverse Integration Illustration"
            class="w-full h-auto object-cover"
            draggable="false"
          />
        </div>
      </div>

    </div>
  </PopoverContent>
</Popover>

<style>
  :global(.whats-new-list) {
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }
  :global(.whats-new-list)::-webkit-scrollbar {
    width: 3px !important;
    display: block !important;
  }
  :global(.whats-new-list)::-webkit-scrollbar-track {
    background: transparent !important;
  }
  :global(.whats-new-list)::-webkit-scrollbar-thumb {
    background-color: var(--border) !important;
    border-radius: 9999px !important;
  }
</style>
