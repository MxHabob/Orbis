<!--
  @component SettingsBottomBar
  Floating, glassmorphic bottom action bar that appears smoothly when settings are modified.
  Reusable across AI Center Settings, General Settings, Academic Policies, and other configuration hubs.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { fly } from "svelte/transition";
import { quintOut } from "svelte/easing";
import { Button } from "@orbis/ui-svelte/button";
import { Loader2, Check, RotateCcw, AlertCircle } from "@lucide/svelte";
import { isAr } from "$lib/core/i18n";

interface Props {
  visible?: boolean;
  isSaving?: boolean;
  isDirty?: boolean;
  saveLabel?: string;
  savingLabel?: string;
  resetLabel?: string;
  message?: string;
  showReset?: boolean;
  onsave?: () => void;
  onreset?: () => void;
}

let {
  visible = true,
  isSaving = false,
  isDirty = true,
  saveLabel,
  savingLabel,
  resetLabel,
  message,
  showReset = true,
  onsave,
  onreset,
}: Props = $props();

const displaySaveLabel = $derived(
  saveLabel || (m.settings_action_save_changes ? m.settings_action_save_changes() : (isAr() ? "حفظ التغييرات" : "Save Changes"))
);

const displaySavingLabel = $derived(
  savingLabel || (m.common_state_saving ? m.common_state_saving() : (isAr() ? "جاري الحفظ..." : "Saving..."))
);

const displayResetLabel = $derived(
  resetLabel || (isAr() ? "تراجع" : "Discard")
);

const displayMessage = $derived(
  message || (isAr() ? "لديك تعديلات غير محفوظة" : "You have unsaved changes")
);
</script>

{#if visible && isDirty}
  <div
    class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-[92%] max-w-xl pointer-events-auto select-none"
    transition:fly={{ y: 35, duration: 250, easing: quintOut }}
  >
    <div class="bg-card/95 backdrop-blur-xl border border-border/80 shadow-2xl rounded-2xl px-5 py-3 flex items-center justify-between gap-4 ring-1 ring-black/5 dark:ring-white/5">
      
      <!-- Unsaved Changes Status Indicator -->
      <div class="flex items-center gap-2.5 text-start overflow-hidden">
        <span class="relative flex h-2.5 w-2.5 shrink-0">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-amber-500"></span>
        </span>
        <span class="text-xs font-medium text-foreground truncate">
          {displayMessage}
        </span>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-2 shrink-0">
        {#if showReset && onreset}
          <Button
            type="button"
            variant="ghost"
            size="sm"
            class="h-8 text-xs font-medium text-muted-foreground hover:text-foreground px-3 cursor-pointer"
            disabled={isSaving}
            onclick={onreset}
          >
            <RotateCcw size={12} class="me-1" />
            {displayResetLabel}
          </Button>
        {/if}

        <Button
          type="button"
          variant="default"
          size="sm"
          class="h-8 px-5 text-xs font-semibold shadow-xs cursor-pointer"
          disabled={isSaving}
          onclick={onsave}
        >
          {#if isSaving}
            <Loader2 size={13} class="animate-spin me-1.5" />
            {displaySavingLabel}
          {:else}
            <Check size={13} class="me-1.5" />
            {displaySaveLabel}
          {/if}
        </Button>
      </div>

    </div>
  </div>
{/if}
