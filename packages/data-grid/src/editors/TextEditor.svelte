<!--
@component TextEditor
Custom RevoGrid cell editor for text inputs.
Integrates with S.C.R.A.P.S. design tokens for light/dark mode compatibility.
-->
<script lang="ts">
import { onMount } from "svelte";

interface EditorProps {
  val?: unknown;
  column?: { prop: string; meta?: Record<string, unknown> };
  save?: (value: unknown, preventFocus?: boolean) => void;
  close?: (focusNext?: boolean) => void;
}

let { val, save, close }: EditorProps = $props();

let inputEl: HTMLInputElement | undefined = $state();
let inputValue = $state(val != null ? String(val) : "");

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
  if (inputEl) {
    inputEl.focus();
    inputEl.select();
  }
});
</script>

<input
  bind:this={inputEl}
  type="text"
  value={inputValue}
  oninput={handleInput}
  onkeydown={handleKeyDown}
  onblur={handleBlur}
  class="orbis-editor-input text-start"
/>

<style>
  .orbis-editor-input {
    width: 100%;
    height: 100%;
    border: none;
    outline: none;
    background: transparent !important;
    color: var(--foreground) !important;
    font-family: inherit;
    font-size: 12.5px;
    padding: 0 12px;
    box-sizing: border-box;
  }
</style>
