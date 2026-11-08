<!--
  @component SettingsBottomBar
  Floating bottom action bar that appears smoothly when settings are modified.
  Reusable across AI Center Settings, General Settings, Academic Policies, and other configuration hubs.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
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
  saveVariant?:
    | "default"
    | "destructive"
    | "primary"
    | "secondary"
    | "lime"
    | "outline";
  statusVariant?: "amber" | "rose" | "primary" | "emerald";
  maxWidthClass?: string;
  actions?: import("svelte").Snippet;
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
  saveVariant = "default",
  statusVariant = "amber",
  maxWidthClass = "max-w-lg",
  actions,
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
    class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 w-[calc(100%-2rem)] {maxWidthClass} pointer-events-auto select-none"
    transition:fly={{ y: 35, duration: 250, easing: quintOut }}
  >
    <div class="bg-card/95 backdrop-blur-md border border-border/80 shadow-xl rounded-xl px-3.5 py-2.5 flex items-center justify-between gap-3">
      <!-- Status Indicator & Message -->
      <div class="flex min-w-0 items-center gap-2.5 text-start">
        {#if statusVariant === "rose"}
          <span class="flex size-7 shrink-0 items-center justify-center rounded-full bg-rose-500/15 text-rose-600 dark:text-rose-400">
            <InfoCircle size={15} strokeWidth={2} />
          </span>
        {:else if statusVariant === "emerald"}
          <span class="flex size-7 shrink-0 items-center justify-center rounded-full bg-emerald-500/15 text-emerald-600 dark:text-emerald-400">
            <InfoCircle size={15} strokeWidth={2} />
          </span>
        {:else if statusVariant === "primary"}
          <span class="flex size-7 shrink-0 items-center justify-center rounded-full bg-primary/15 text-primary">
            <InfoCircle size={15} strokeWidth={2} />
          </span>
        {:else}
          <span class="flex size-7 shrink-0 items-center justify-center rounded-full bg-amber-500/15 text-amber-500">
            <InfoCircle size={15} strokeWidth={2} />
          </span>
        {/if}
        <span class="truncate text-xs sm:text-sm font-medium text-foreground">
          {displayMessage}
        </span>
      </div>

      <!-- Action Buttons -->
      <div class="flex shrink-0 items-center gap-1.5">
        {#if actions}
          {@render actions()}
        {:else}
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

          {#if onsave}
            <Button
              type="button"
              variant={saveVariant}
              size="sm"
              class="h-8 px-3 text-xs font-semibold shadow-xs cursor-pointer"
              disabled={isSaving}
              onclick={onsave}
            >
              {#if isSaving}
                <LoadingIndicator class="me-1.5"  size={13} />
                <span>{displaySavingLabel}</span>
              {:else}
                <span>{displaySaveLabel}</span>
              {/if}
            </Button>
          {/if}
        {/if}
      </div>

    </div>
  </div>
{/if}
