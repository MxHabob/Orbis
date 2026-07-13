<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import * as settings from "$lib/paraglide/messages/settings";

interface Props {
  onremove: () => void;
}

let { onremove }: Props = $props();
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col">
  <!-- Card Header -->
  <div class="bg-muted/50 px-6 py-3 border border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    {settings.general_section_remove()}
  </div>

  <!-- Card Body -->
  <div class="px-6 py-5 flex flex-col md:flex-row md:items-center justify-between gap-4 text-start">
    <div class="flex flex-col gap-1">
      <span class="text-xs font-semibold text-foreground">{settings.general_remove_title()}</span>
      <span class="text-[11px] text-muted-foreground leading-relaxed">
        {settings.general_remove_desc()}
      </span>
    </div>
    <div class="w-full md:w-auto flex items-center justify-end shrink-0">
      <Button type="button" variant="destructive" onclick={onremove} class="font-semibold text-xs h-9 px-6 select-none">
        {settings.general_remove_btn()}
      </Button>
    </div>
  </div>
</div>
