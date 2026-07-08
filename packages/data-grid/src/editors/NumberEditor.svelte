<!--
@component NumberEditor
Custom RevoGrid cell editor for numeric input using Svelte 5 Runes.
Uses @orbis/ui-svelte/input for visual alignment with the design system.
-->
<script lang="ts">
import { onMount } from "svelte";
import { Input } from "@orbis/ui-svelte/input";

// RevoGrid editor interface props
interface Props {
  val?: unknown;
  column?: { prop: string | number; meta?: Record<string, unknown> };
  save?: (value: unknown, preventFocus?: boolean) => void;
  close?: (focusNext?: boolean) => void;
}

let { val, column, save, close }: Props = $props();

let inputRef: HTMLInputElement | null = $state(null);
let inputValue = $state("");
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
  inputValue = val != null ? String(val) : "";
  // Focus and select input content on mount
  if (inputRef) {
    inputRef.focus();
    inputRef.select();
  }
});

// Expose getValue method so RevoGrid's SvelteEditorAdapter can read the value
export function getValue() {
  if (hasError) return val;
  return inputValue === "" ? null : parseFloat(inputValue);
}
</script>

<Input
  bind:ref={inputRef}
  type="number"
  min={minScore}
  max={maxScore}
  {step}
  value={inputValue}
  oninput={handleInput}
  onkeydown={handleKeyDown}
  onblur={handleBlur}
  class="h-full w-full text-center font-mono outline-none ring-0 border-0 {!validate(inputValue) ? 'text-destructive bg-destructive/10 border border-destructive' : ''}"
/>
