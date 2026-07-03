<!--
@component NumberEditor
Custom RevoGrid cell editor for numeric grade input.
Uses S.C.R.A.P.S. design tokens for validation feedback.

Features:
- Min/max validation with visual feedback
- Step support (0.5 increments)
- Auto-select on focus
- Enter/Tab/Arrow key navigation
- Error state uses S.C.R.A.P.S. destructive token
-->
<script lang="ts">
  import { onMount } from "svelte";

  // RevoGrid editor interface
  interface EditorProps {
    val?: unknown;
    column?: { prop: string; meta?: Record<string, unknown> };
    save?: (value: unknown, preventFocus?: boolean) => void;
    close?: (focusNext?: boolean) => void;
  }

  let { val, column, save, close }: EditorProps = $props();

  let inputEl: HTMLInputElement | undefined = $state();
  let inputValue = $state(val != null ? String(val) : "");
  let hasError = $state(false);

  // Get constraints from column meta
  const minScore = $derived(Number(column?.meta?.min ?? 0));
  const maxScore = $derived(Number(column?.meta?.max ?? 100));
  const step = $derived(Number(column?.meta?.step ?? 0.5));

  function validate(v: string): boolean {
    if (v === "" || v === null || v === undefined) return true;
    const num = parseFloat(v);
    if (isNaN(num)) return false;
    if (num < minScore || num > maxScore) return false;
    return true;
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    inputValue = target.value;
    hasError = !validate(inputValue);
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === "Tab") {
      e.preventDefault();
      e.stopPropagation();
      commit();
    } else if (e.key === "Escape") {
      e.preventDefault();
      close?.(false);
    }
  }

  function commit() {
    if (hasError) return;
    const numValue = inputValue === "" ? null : parseFloat(inputValue);
    save?.(numValue, false);
  }

  function handleBlur() {
    commit();
  }

  onMount(() => {
    // Auto-focus and select content on mount
    if (inputEl) {
      inputEl.focus();
      inputEl.select();
    }
  });
</script>

<input
  bind:this={inputEl}
  type="number"
  min={minScore}
  max={maxScore}
  {step}
  value={inputValue}
  oninput={handleInput}
  onkeydown={handleKeyDown}
  onblur={handleBlur}
  class="orbis-editor-input"
  class:orbis-editor-error={hasError}
/>
