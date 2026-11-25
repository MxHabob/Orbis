<script lang="ts">
import { untrack } from "svelte";
import * as m from "$lib/paraglide/messages";
/**
 * ColorPicker — Advanced color picker with spectrum, HEX/RGB inputs,
 * opacity slider, preset swatches, and recent colors.
 *
 * Usage:
 *   <ColorPicker value="#ff0000" onchange={(c) => ...} />
 */
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import { AdjustmentsHorizontal } from "@orbis/icons";

let {
  value = "#000000",
  opacity = 1,
  onchange,
  onopacitychange,
  showOpacity = false,
  allowNone = false,
  size = "default",
}: {
  value: string;
  opacity?: number;
  onchange: (color: string) => void;
  onopacitychange?: (opacity: number) => void;
  showOpacity?: boolean;
  allowNone?: boolean;
  size?: "default" | "sm";
} = $props();

// ── State ────────────────────────────────────────────────────
let hexInput = $state(untrack(() => value));
let isOpen = $state(false);

// Recent colors (persisted in memory)
let recentColors = $state<string[]>([]);
const MAX_RECENT = 12;
const pickerValue = $derived(
  value.toLowerCase() === "transparent" ? "#000000" : value,
);

// Preset swatches — curated palette
const presetColors = [
  // Row 1: Neutrals
  "#000000",
  "#1a1a1a",
  "#333333",
  "#555555",
  "#777777",
  "#999999",
  "#cccccc",
  "#ffffff",
  // Row 2: Primary colors
  "#ef4444",
  "#f97316",
  "#eab308",
  "#22c55e",
  "#06b6d4",
  "#3b82f6",
  "#8b5cf6",
  "#ec4899",
  // Row 3: Light variants
  "#fecaca",
  "#fed7aa",
  "#fef08a",
  "#bbf7d0",
  "#a5f3fc",
  "#bfdbfe",
  "#ddd6fe",
  "#fbcfe8",
  // Row 4: Dark variants
  "#991b1b",
  "#9a3412",
  "#854d0e",
  "#166534",
  "#155e75",
  "#1e40af",
  "#5b21b6",
  "#9d174d",
];

// ── Handlers ─────────────────────────────────────────────────

function handleColorInput(e: Event) {
  const target = e.target as HTMLInputElement;
  const newColor = target.value;
  hexInput = newColor;
  onchange(newColor);
  addToRecent(newColor);
}

function handleHexInput(e: Event) {
  const target = e.target as HTMLInputElement;
  let hex = target.value.trim();
  if (!hex.startsWith("#")) hex = "#" + hex;
  hexInput = hex;
  if (/^#[0-9a-fA-F]{6}$/.test(hex)) {
    onchange(hex);
    addToRecent(hex);
  }
}

function selectPreset(color: string) {
  hexInput = color;
  onchange(color);
  addToRecent(color);
}

function selectNone() {
  onchange("transparent");
}

function handleOpacityInput(e: Event) {
  const val = parseInt((e.target as HTMLInputElement).value) || 100;
  const clamped = Math.max(0, Math.min(100, val));
  onopacitychange?.(clamped / 100);
}

function addToRecent(color: string) {
  const normalized = color.toLowerCase();
  recentColors = [
    normalized,
    ...recentColors.filter((c) => c !== normalized),
  ].slice(0, MAX_RECENT);
}

// Keep hexInput in sync when value prop changes externally
$effect(() => {
  hexInput = pickerValue;
});

const swatchSize = $derived(size === "sm" ? 20 : 24);
</script>

<Popover bind:open={isOpen}>
  <PopoverTrigger>
    <button
      type="button"
      class="color-trigger"
      style="width: {swatchSize}px; height: {swatchSize}px;"
      aria-label={m.template_color_picker_open()}
    >
      <span
        class="color-preview"
        style="background: {value}; opacity: {opacity};"
      ></span>
    </button>
  </PopoverTrigger>
  <PopoverContent class="w-[240px] p-0" align="start" side="bottom">
    <div class="picker-container">
      <!-- Native color input (hidden, provides the spectrum) -->
      <div class="spectrum-row">
        <input
          type="color"
          value={pickerValue}
          oninput={handleColorInput}
          class="spectrum-input"
        />
        <div class="hex-row">
          <span class="hex-label">#</span>
          <Input
            type="text"
            value={hexInput.replace("#", "")}
            oninput={handleHexInput}
            class="h-7 text-[10px] font-mono flex-grow"
            maxlength={7}
            placeholder="000000"
          />
        </div>
      </div>

      {#if allowNone}
        <button
          type="button"
          class="none-option"
          class:active={value.toLowerCase() === "transparent"}
          aria-pressed={value.toLowerCase() === "transparent"}
          onclick={selectNone}
        >
          <span class="none-swatch"></span>
          {m.template_color_none()}
        </button>
      {/if}

      {#if showOpacity}
        <div class="opacity-row">
          <span class="opacity-label">{m.template_color_opacity()}</span>
          <input
            type="range"
            min="0"
            max="100"
            value={Math.round(opacity * 100).toString()}
            oninput={handleOpacityInput}
            class="opacity-slider"
          />
          <span class="opacity-value">{Math.round(opacity * 100)}%</span>
        </div>
      {/if}

      <!-- Preset swatches -->
      <div class="swatches-section">
        <span class="section-label">{m.template_color_colors()}</span>
        <div class="swatches-grid">
          {#each presetColors as color (color)}
            <button
              type="button"
              class="swatch"
              class:active={value.toLowerCase() === color.toLowerCase()}
              style="background: {color};"
              aria-label={m.template_color_select_color({ color })}
              onclick={() => selectPreset(color)}
            ></button>
          {/each}
        </div>
      </div>

      <!-- Recent colors -->
      {#if recentColors.length > 0}
        <div class="swatches-section">
          <span class="section-label">{m.template_color_recent()}</span>
          <div class="swatches-grid">
            {#each recentColors as color (color)}
              <button
                type="button"
                class="swatch"
                class:active={value.toLowerCase() === color}
                style="background: {color};"
                aria-label={m.template_color_select_color({ color })}
                onclick={() => selectPreset(color)}
              ></button>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </PopoverContent>
</Popover>

<style>
  .color-trigger {
    border: 2px solid color-mix(in srgb, var(--color-border) 50%, transparent);
    border-radius: 5px;
    cursor: pointer;
    padding: 0;
    overflow: hidden;
    transition: border-color 0.15s;
    /* Checkerboard background for transparency */
    background-image:
      linear-gradient(45deg, #ccc 25%, transparent 25%),
      linear-gradient(-45deg, #ccc 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #ccc 75%),
      linear-gradient(-45deg, transparent 75%, #ccc 75%);
    background-size: 8px 8px;
    background-position: 0 0, 0 4px, 4px -4px, -4px 0;
  }

  .color-trigger:hover {
    border-color: var(--color-primary);
  }

  .color-preview {
    display: block;
    width: 100%;
    height: 100%;
    border-radius: 3px;
  }

  .picker-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 10px;
  }

  .spectrum-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .spectrum-input {
    width: 36px;
    height: 36px;
    border: none;
    padding: 0;
    cursor: pointer;
    border-radius: 4px;
    background: none;
  }

  .spectrum-input::-webkit-color-swatch-wrapper { padding: 2px; }
  .spectrum-input::-webkit-color-swatch { border: none; border-radius: 4px; }

  .hex-row {
    display: flex;
    align-items: center;
    gap: 2px;
    flex-grow: 1;
  }

  .none-option {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 28px;
    padding: 4px 6px;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    background: var(--color-background);
    color: var(--color-foreground);
    font-size: 10px;
    text-align: start;
    cursor: pointer;
  }

  .none-option:hover,
  .none-option.active {
    border-color: var(--color-primary);
  }

  .none-swatch {
    width: 18px;
    height: 18px;
    border: 1px solid var(--color-border);
    border-radius: 3px;
    background:
      linear-gradient(135deg, transparent 46%, #ef4444 47%, #ef4444 53%, transparent 54%),
      linear-gradient(45deg, #ccc 25%, transparent 25%),
      linear-gradient(-45deg, #ccc 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #ccc 75%),
      linear-gradient(-45deg, transparent 75%, #ccc 75%);
    background-size: 100% 100%, 8px 8px, 8px 8px, 8px 8px, 8px 8px;
    background-position: center, 0 0, 0 4px, 4px -4px, -4px 0;
  }

  .hex-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--color-muted-foreground);
  }

  .opacity-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .opacity-label {
    font-size: 9px;
    color: var(--color-muted-foreground);
    white-space: nowrap;
  }

  .opacity-slider {
    flex-grow: 1;
    height: 4px;
    -webkit-appearance: none;
    appearance: none;
    background: linear-gradient(to right,
      transparent,
      var(--color-foreground)
    );
    border-radius: 4px;
    outline: none;
    cursor: pointer;
  }

  .opacity-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--color-foreground);
    border: 2px solid var(--color-background);
    cursor: pointer;
    box-shadow: 0 1px 3px rgba(0,0,0,0.3);
  }

  .opacity-value {
    font-size: 9px;
    font-family: monospace;
    color: var(--color-muted-foreground);
    min-width: 30px;
    text-align: end;
  }

  .swatches-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .section-label {
    font-size: 9px;
    font-weight: 600;
    color: var(--color-muted-foreground);
  }

  .swatches-grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    gap: 3px;
  }

  .swatch {
    width: 100%;
    aspect-ratio: 1;
    border: 1.5px solid transparent;
    border-radius: 3px;
    cursor: pointer;
    transition: all 0.1s;
  }

  .swatch:hover {
    transform: scale(1.15);
    border-color: var(--color-foreground);
    z-index: 1;
  }

  .swatch.active {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px var(--color-primary);
  }
</style>
