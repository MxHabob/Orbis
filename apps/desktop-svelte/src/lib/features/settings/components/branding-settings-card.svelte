<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";

interface Props {
  primaryColor: string;
  isSaving?: boolean;
}

let { primaryColor = $bindable(), isSaving = false }: Props = $props();
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col">
  <!-- Card Header -->
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    {m.settings_general_section_branding()}
  </div>

  <div class="flex flex-col">
    <!-- Primary Color -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="org_color" class="text-xs font-semibold text-foreground">
          {m.settings_general_color()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_color_desc()}</span>
      </div>
      <div class="w-full md:w-[480px] shrink-0 flex items-center justify-start gap-2">
        <input
          id="org_color"
          type="color"
          bind:value={primaryColor}
          class="w-9 h-9 rounded-lg border border-border cursor-pointer bg-transparent shrink-0"
          disabled={isSaving}
        />
        <Input
          type="text"
          bind:value={primaryColor}
          placeholder="#84cc16"
          class="font-mono uppercase h-9 w-28 text-center shrink-0 bg-background"
          disabled={isSaving}
        />
      </div>
    </div>
  </div>
</div>
