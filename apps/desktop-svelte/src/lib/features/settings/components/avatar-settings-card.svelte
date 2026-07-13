<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";

interface Props {
  logo: string;
  name: string;
  isSaving?: boolean;
  onsave: (e: Event) => void;
}

let { logo = $bindable(), name, isSaving = false, onsave }: Props = $props();
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col">
  <!-- Card Header -->
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    {m.settings_general_section_avatar()}
  </div>

  <!-- Card Body -->
  <div class="px-6 py-5 flex items-center justify-start text-start select-none">
    <AvatarUpload
      bind:avatarUrl={logo}
      size={64}
      fallbackText={name}
    />
  </div>

  <!-- Card Footer -->
  <div class="bg-muted/10 px-6 py-3 border-t border-border flex justify-start">
    <Button type="button" class="font-semibold text-xs h-8 px-4" onclick={onsave} disabled={isSaving}>
      {isSaving ? m.common_state_saving() : m.settings_general_avatar_save()}
    </Button>
  </div>
</div>
