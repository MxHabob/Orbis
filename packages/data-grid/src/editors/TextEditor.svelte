<!--
@component TextEditor
Custom RevoGrid cell editor for text input using Svelte 5 Runes.
Uses @orbis/ui-svelte/input for visual alignment with the design system.
-->
<script lang="ts">
import { onMount } from "svelte";
import { Input } from "@orbis/ui-svelte/input";

interface Props {
  val?: unknown;
  column?: { prop: string | number; meta?: Record<string, unknown> };
  save?: (value: unknown, preventFocus?: boolean) => void;
  close?: (focusNext?: boolean) => void;
}

let { val, save, close }: Props = $props();

let inputRef: HTMLInputElement | null = $state(null);
let inputValue = $state("");

function handleInput(e: Event) {
  const target = e.target as HTMLInputElement;
  inputValue = target.value;
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
  save?.(inputValue, false);
}

function handleBlur() {
  commit();
}

onMount(() => {
  inputValue = val != null ? String(val) : "";
  if (inputRef) {
    inputRef.focus();
    inputRef.select();
  }
});

// Expose getValue method so RevoGrid's SvelteEditorAdapter can read the value
export function getValue() {
  return inputValue;
}
</script>

<Input
  bind:ref={inputRef}
  type="text"
  value={inputValue}
  oninput={handleInput}
  onkeydown={handleKeyDown}
  onblur={handleBlur}
  class="h-full w-full text-start outline-none ring-0 border-0"
/>
