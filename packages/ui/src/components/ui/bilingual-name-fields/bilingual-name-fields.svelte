<script lang="ts">
import { onDestroy } from "svelte";
import type { Snippet } from "svelte";
import type {
  SuggestionRequest,
  SuggestionResult,
  TranslationLanguage,
} from "@orbis/translation";
import { Input } from "../input/index.js";

type FieldControlArgs = {
  id: string;
  placeholder: string;
  required: boolean;
  disabled: boolean;
  ariaInvalid?: "true";
  ariaDescribedBy?: string;
  onInput: (value: string) => void;
  onBlur?: () => void;
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
  arDescription?: string;
  enDescription?: string;
  arControl?: Snippet<[FieldControlArgs]>;
  arError?: string;
  enError?: string;
  onArBlur?: () => void;
  onEnBlur?: () => void;
  disabled?: boolean;
  arRequired?: boolean;
  enRequired?: boolean;
  debounceMs?: number;
  loadingMessage: string;
  failureMessage: string;
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
  arDescription = "",
  enDescription = "",
  arControl,
  arError,
  enError,
  onArBlur,
  onEnBlur,
  disabled = false,
  arRequired = false,
  enRequired = false,
  debounceMs = 350,
  loadingMessage,
  failureMessage,
}: Props = $props();

let isLoading = $state(false);
let requestFailed = $state(false);
let generatedLanguage = $state<TranslationLanguage | null>(null);
let generatedValue = "";
let requestSequence = 0;
let debounceTimer: ReturnType<typeof setTimeout> | undefined;

function valueFor(language: TranslationLanguage): string {
  return language === "ar" ? arValue : enValue;
}

function isGenerated(language: TranslationLanguage): boolean {
  return generatedLanguage === language && valueFor(language) === generatedValue;
}

function invalidatePendingRequest() {
  requestSequence += 1;
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = undefined;
  isLoading = false;
  requestFailed = false;
}

function handleInput(text: string, sourceLanguage: TranslationLanguage) {
  if (generatedLanguage === sourceLanguage) {
    generatedLanguage = null;
    generatedValue = "";
  }
  scheduleSuggestion(text, sourceLanguage);
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
      if (sequence !== requestSequence || result.status !== "suggestion") return;

      const targetLanguage = sourceLanguage === "ar" ? "en" : "ar";
      const targetValue = valueFor(targetLanguage);
      if (targetValue.trim() && !isGenerated(targetLanguage)) return;

      if (targetLanguage === "ar") arValue = result.text;
      else enValue = result.text;
      generatedLanguage = targetLanguage;
      generatedValue = result.text;
    } catch {
      if (sequence === requestSequence) requestFailed = true;
    } finally {
      if (sequence === requestSequence) isLoading = false;
    }
  }, debounceMs);
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
    {#if arDescription}
      <p id={`${arId}-description`} class="text-[11px] leading-5 text-muted-foreground">
        {arDescription}
      </p>
    {/if}
    <div class="relative">
      {#if arControl}
        {@render arControl({
          id: arId,
          placeholder: arPlaceholder,
          required: arRequired,
          disabled,
          ariaInvalid: arError ? "true" : undefined,
          ariaDescribedBy: [
            arDescription ? `${arId}-description` : "",
            arError ? `${arId}-error` : "",
          ].filter(Boolean).join(" ") || undefined,
          onInput: (value) => handleInput(value, "ar"),
          onBlur: onArBlur,
        })}
      {:else}
        <Input
          id={arId}
          type="text"
          dir="rtl"
          bind:value={arValue}
          oninput={(event) => handleInput(event.currentTarget.value, "ar")}
          onblur={onArBlur}
          placeholder={arPlaceholder}
          aria-invalid={arError ? "true" : undefined}
          aria-describedby={[
            arDescription ? `${arId}-description` : "",
            arError ? `${arId}-error` : "",
          ].filter(Boolean).join(" ") || undefined}
          required={arRequired}
          {disabled}
          autocomplete="off"
        />
      {/if}
    </div>
    {#if arError}
      <p id={`${arId}-error`} class="text-xs text-destructive">{arError}</p>
    {/if}
  </div>

  <div class="flex flex-col gap-2">
    <label for={enId} class="text-xs font-semibold text-muted-foreground">
      {enLabel}{#if enRequired}<span class="text-destructive" aria-hidden="true"> *</span>{/if}
    </label>
    {#if enDescription}
      <p id={`${enId}-description`} class="text-[11px] leading-5 text-muted-foreground">
        {enDescription}
      </p>
    {/if}
    <div class="relative">
      <Input
        id={enId}
        type="text"
        dir="ltr"
        bind:value={enValue}
        oninput={(event) => handleInput(event.currentTarget.value, "en")}
        onblur={onEnBlur}
        placeholder={enPlaceholder}
        aria-invalid={enError ? "true" : undefined}
        aria-describedby={[
          enDescription ? `${enId}-description` : "",
          enError ? `${enId}-error` : "",
        ].filter(Boolean).join(" ") || undefined}
        required={enRequired}
        {disabled}
        autocomplete="off"
      />
    </div>
    {#if enError}
      <p id={`${enId}-error`} class="text-xs text-destructive">{enError}</p>
    {/if}
  </div>
</div>

<div class="min-h-5" role="status" aria-live="polite" aria-atomic="true">
  {#if isLoading}
    <span class="sr-only">{loadingMessage}</span>
  {:else if requestFailed}
    <p class="text-xs text-muted-foreground">{failureMessage}</p>
  {/if}
</div>
