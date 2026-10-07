<script lang="ts">
import { onDestroy } from "svelte";
import { Button } from "../button/index.js";
import { Input } from "../input/index.js";

type TranslationLanguage = "ar" | "en";
type SuggestionRequest = {
  text: string;
  sourceLanguage: TranslationLanguage;
  kind: "label" | "person-name";
};
type SuggestionResult =
  | {
      status: "suggestion";
      text: string;
      approximate: boolean;
    }
  | {
      status: "unavailable";
      reason: "empty" | "unknown-term" | "unsupported-script";
    };

type Props = {
  arId: string;
  enId: string;
  arLabel: string;
  enLabel: string;
  requestSuggestion: (
    request: SuggestionRequest,
  ) => SuggestionResult | Promise<SuggestionResult>;
  kind: SuggestionRequest["kind"];
  arValue?: string;
  enValue?: string;
  arPlaceholder?: string;
  enPlaceholder?: string;
  arError?: string;
  enError?: string;
  disabled?: boolean;
  arRequired?: boolean;
  enRequired?: boolean;
  debounceMs?: number;
  loadingMessage: string;
  unavailableMessage: string;
  failureMessage: string;
  approximateMessage: string;
  acceptMessage: string;
  replaceMessage: string;
  dismissMessage: string;
};

let {
  arId,
  enId,
  arLabel,
  enLabel,
  requestSuggestion,
  kind,
  arValue = $bindable(""),
  enValue = $bindable(""),
  arPlaceholder = "",
  enPlaceholder = "",
  arError,
  enError,
  disabled = false,
  arRequired = false,
  enRequired = false,
  debounceMs = 350,
  loadingMessage,
  unavailableMessage,
  failureMessage,
  approximateMessage,
  acceptMessage,
  replaceMessage,
  dismissMessage,
}: Props = $props();

let isLoading = $state(false);
let candidate = $state<SuggestionResult | null>(null);
let requestSource = $state<TranslationLanguage | null>(null);
let requestFailed = $state(false);
let requestSequence = 0;
let debounceTimer: ReturnType<typeof setTimeout> | undefined;

function invalidatePendingRequest() {
  requestSequence += 1;
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = undefined;
  isLoading = false;
  candidate = null;
  requestSource = null;
  requestFailed = false;
}

function scheduleSuggestion(text: string, sourceLanguage: TranslationLanguage) {
  invalidatePendingRequest();
  if (!text.trim() || disabled) return;

  const sequence = requestSequence;
  isLoading = true;
  debounceTimer = setTimeout(async () => {
    try {
      const result = await requestSuggestion({
        text,
        sourceLanguage,
        kind,
      });
      if (sequence !== requestSequence) return;
      candidate = result;
      requestSource = sourceLanguage;
    } catch {
      if (sequence !== requestSequence) return;
      requestFailed = true;
    } finally {
      if (sequence === requestSequence) isLoading = false;
    }
  }, debounceMs);
}

function acceptSuggestion() {
  if (candidate?.status !== "suggestion" || !requestSource) return;
  if (requestSource === "ar") enValue = candidate.text;
  else arValue = candidate.text;
  candidate = null;
  requestSource = null;
}

function dismissSuggestion() {
  candidate = null;
  requestSource = null;
}

onDestroy(() => {
  invalidatePendingRequest();
});
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
  <div class="flex flex-col gap-2">
    <label for={arId} class="text-xs font-semibold text-muted-foreground">
      {arLabel}{#if arRequired}<span class="text-destructive" aria-hidden="true"> *</span>{/if}
    </label>
    <Input
      id={arId}
      type="text"
      dir="rtl"
      bind:value={arValue}
      oninput={(event) => scheduleSuggestion(event.currentTarget.value, "ar")}
      placeholder={arPlaceholder}
      aria-invalid={arError ? "true" : undefined}
      aria-describedby={arError ? `${arId}-error` : undefined}
      required={arRequired}
      {disabled}
      autocomplete="off"
    />
    {#if arError}
      <p id={`${arId}-error`} class="text-xs text-destructive">{arError}</p>
    {/if}
  </div>

  <div class="flex flex-col gap-2">
    <label for={enId} class="text-xs font-semibold text-muted-foreground">
      {enLabel}{#if enRequired}<span class="text-destructive" aria-hidden="true"> *</span>{/if}
    </label>
    <Input
      id={enId}
      type="text"
      dir="ltr"
      bind:value={enValue}
      oninput={(event) => scheduleSuggestion(event.currentTarget.value, "en")}
      placeholder={enPlaceholder}
      aria-invalid={enError ? "true" : undefined}
      aria-describedby={enError ? `${enId}-error` : undefined}
      required={enRequired}
      {disabled}
      autocomplete="off"
    />
    {#if enError}
      <p id={`${enId}-error`} class="text-xs text-destructive">{enError}</p>
    {/if}
  </div>
</div>

<div class="min-h-7" role="status" aria-live="polite" aria-atomic="true">
  {#if isLoading}
    <p class="text-xs text-muted-foreground">{loadingMessage}</p>
  {:else if requestFailed}
    <p class="text-xs text-muted-foreground">{failureMessage}</p>
  {:else if candidate?.status === "unavailable"}
    <p class="text-xs text-muted-foreground">{unavailableMessage}</p>
  {:else if candidate?.status === "suggestion" && requestSource}
    <div class="flex flex-wrap items-center gap-2 text-xs">
      <span class="text-muted-foreground">
        {requestSource === "ar" ? enLabel : arLabel}:
      </span>
      <span class="font-medium text-foreground" dir="auto">{candidate.text}</span>
      {#if candidate.approximate}
        <span class="text-muted-foreground">{approximateMessage}</span>
      {/if}
      <Button
        type="button"
        variant="outline"
        size="sm"
        disabled={disabled}
        onclick={acceptSuggestion}
      >
        {(requestSource === "ar" ? enValue : arValue).trim()
          ? replaceMessage
          : acceptMessage}
      </Button>
      <Button
        type="button"
        variant="ghost"
        size="sm"
        disabled={disabled}
        onclick={dismissSuggestion}
      >
        {dismissMessage}
      </Button>
    </div>
  {/if}
</div>
