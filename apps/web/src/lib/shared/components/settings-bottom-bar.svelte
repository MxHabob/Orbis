<!--
  @component SettingsBottomBar
  Compact floating bottom action bar for dirty state & bulk selection actions.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button, type ButtonVariant } from "@orbis/ui/button";

interface Props {
  visible?: boolean;
  isSaving?: boolean;
  isDirty?: boolean;
  saveLabel?: string;
  savingLabel?: string;
  resetLabel?: string;
  message?: string;
  showReset?: boolean;
  saveVariant?: ButtonVariant;
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
  maxWidthClass = "",
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

const dotClass = $derived(
  statusVariant === "rose"
    ? "bg-destructive"
    : statusVariant === "emerald"
      ? "bg-emerald-500"
      : statusVariant === "primary"
        ? "bg-primary"
        : "bg-amber-500",
);
</script>

{#if visible && isDirty}
  <div
    class="fixed bottom-9 left-1/2 -translate-x-1/2 z-50 w-fit max-w-[calc(100%-2rem)] {maxWidthClass} pointer-events-auto select-none"
  >
    <div class="bg-card/95 backdrop-blur-md border border-border/70 shadow-lg rounded-lg px-2.5 py-1.5 flex flex-wrap items-center justify-between gap-2.5">
      <!-- Status Dot & Concise Message -->
      <div class="flex min-w-0 items-center gap-2 ps-1 text-start">
        <span class="size-2 shrink-0 rounded-full {dotClass}"></span>
        <span class="whitespace-nowrap text-xs font-medium text-foreground">
          {displayMessage}
        </span>
      </div>

      <div class="hidden sm:block h-4 w-px bg-border/60 shrink-0"></div>

      <!-- Action Buttons -->
      <div class="flex shrink-0 items-center gap-1.5">
        {#if actions}
          {@render actions()}
        {:else}
          {#if showReset && onreset}
            <Button
              type="button"
              variant="ghost"
              size="xs"
              class="h-7 px-2.5 text-xs font-medium text-muted-foreground hover:text-foreground"
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
              size="xs"
              class="h-7 px-3 text-xs font-semibold shadow-2xs"
              disabled={isSaving}
              onclick={onsave}
            >
              {#if isSaving}
                <LoadingIndicator class="me-1.5" size={12} />
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
