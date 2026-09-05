<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@orbis/ui/popover";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Megaphone } from "@lucide/svelte";
import { isDesktopApp } from "$lib/core/ipc";

import orbisEcosystem from "$lib/assets/orbis-ecosystem.png";
import mulverseOrbis from "$lib/assets/mulverse-orbis.png";

const handleLearnMore = async () => {
  const url = "https://mulverse.com";
  if (isDesktopApp) {
    try {
      const { openUrl } = await import("@tauri-apps/plugin-opener");
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
      class="cursor-pointer"
      title={m.common_whats_new_header_title()}
      aria-label={m.common_whats_new_header_title()}
    >
      <Megaphone size={14} class="text-foreground stroke-[2px]" />
    </Button>
  </PopoverTrigger>
  
  <PopoverContent
    dir="auto"
    side="bottom"
    align="end"
    sideOffset={8}
    class="w-[min(420px,calc(100vw-2rem))] max-w-[calc(100vw-2rem)] rounded-xl p-0 overflow-hidden font-sans text-start"
  >
    <!-- Announcement Header -->
    <div class="sticky top-0 z-10 px-4 py-3 border-b border-border flex items-center justify-between select-none">
      <h4 class="font-bold text-sm text-foreground">
        {m.common_whats_new_header_title()}
      </h4>
    </div>

    <!-- Announcement Feed List (Scrollable) -->
    <div class="whats-new-list flex-1 overflow-y-auto max-h-[min(440px,calc(100vh-7rem))] px-4 py-4 flex flex-col gap-5 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
      
      <!-- Announcement Item 1 -->
      <div class="flex flex-col gap-3 text-start">
        <div class="flex items-start justify-between gap-2">
          <h5 class="text-[12px] font-bold text-foreground leading-snug">
            {m.common_whats_new_title_1()}
          </h5>
          <Badge variant="brand" class="shrink-0">
            {m.common_whats_new_badge_feature()}
          </Badge>
        </div>
        
        <p class="text-[11px] text-muted-foreground leading-relaxed font-light">
          {m.common_whats_new_desc_1()}
          <button 
            type="button" 
            onclick={handleLearnMore}
            class="text-primary hover:underline font-medium inline-block p-0 bg-transparent border-0 cursor-pointer text-[11px] ms-1"
          >
            {m.common_whats_new_read_more()}
          </button>
        </p>

        <!-- Illustration Image 1 -->
        <div class="w-full aspect-[16/7] rounded-lg overflow-hidden border border-border bg-muted/20 flex items-center justify-center">
          <img
            src={orbisEcosystem}
            alt=""
            class="w-full h-full object-cover"
            draggable="false"
          />
        </div>
      </div>

      <!-- Divider -->
      <div class="h-px bg-border/60"></div>

      <!-- Announcement Item 2 -->
      <div class="flex flex-col gap-3 text-start pb-2">
        <div class="flex items-start justify-between gap-2">
          <h5 class="text-[12px] font-bold text-foreground leading-snug">
            {m.common_whats_new_title_2()}
          </h5>
          <Badge variant="brand" class="shrink-0">
            {m.common_whats_new_badge_integration()}
          </Badge>
        </div>
        
        <p class="text-[11px] text-muted-foreground leading-relaxed font-light">
          {m.common_whats_new_desc_2()}
        </p>

        <!-- Illustration Image 2 -->
        <div class="w-full aspect-[16/7] rounded-lg overflow-hidden border border-border bg-muted/20 flex items-center justify-center">
          <img
            src={mulverseOrbis}
            alt=""
            class="w-full h-full object-cover"
            draggable="false"
          />
        </div>
      </div>

    </div>
  </PopoverContent>
</Popover>
