<!--
  @component SettingsBottomBar
  Floating bottom action bar that appears smoothly when settings are modified.
  Reusable across AI Center Settings, General Settings, Academic Policies, and other configuration hubs.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { fly } from "svelte/transition";
import { quintOut } from "svelte/easing";
import { Button } from "@orbis/ui/button";
import { AdjustmentsHorizontal, ArrowPath, InfoCircle } from "@orbis/icons";

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
  saveLabel || m.settings_action_save_changes(),
);

const displaySavingLabel = $derived(savingLabel || m.common_state_saving());

const displayResetLabel = $derived(
  resetLabel || m.settings_action_discard_changes(),
);

const displayMessage = $derived(message || m.settings_unsaved_changes());
</script>

{#if visible && isDirty}
  <div
    class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 w-[calc(100%-2rem)] max-w-md pointer-events-auto select-none"
    transition:fly={{ y: 35, duration: 250, easing: quintOut }}
  >
    <div class="bg-card border border-border shadow-lg rounded-lg px-3 py-2 flex items-center justify-between gap-3">
      <!-- Unsaved Changes Status Indicator -->
      <div class="flex min-w-0 items-center gap-2 text-start">
        <span class="flex size-7 shrink-0 items-center justify-center rounded-full bg-amber-500/10 text-amber-500">
          <InfoCircle size={15} strokeWidth={2} />
        </span>
        <span class="truncate text-sm font-medium text-foreground">
          {displayMessage}
        </span>
      </div>

      <!-- Action Buttons -->
      <div class="flex shrink-0 items-center gap-1.5">
        {#if showReset && onreset}
          <Button
            type="button"
            variant="ghost"
            size="sm"
            class="h-8 px-2.5 text-xs font-medium text-muted-foreground hover:text-foreground cursor-pointer"
            disabled={isSaving}
            onclick={onreset}
          >
            {displayResetLabel}
          </Button>
        {/if}

        <Button
          type="button"
          variant="default"
          size="sm"
          class="h-8 px-3 text-xs font-medium shadow-xs cursor-pointer"
          disabled={isSaving}
          onclick={onsave}
        >
          {#if isSaving}
            <ArrowPath size={13} class="animate-spin me-1.5" />
          {:else}
            {displaySaveLabel}
          {/if}
        </Button>
      </div>

    </div>
  </div>
{/if}
