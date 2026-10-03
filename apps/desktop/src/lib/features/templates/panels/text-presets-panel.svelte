<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * TextPresetsPanel — Text style presets for quick insertion.
 * Provides heading, subheading, body, caption presets.
 */
import type { EditorStore } from "../stores/editor-store.svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

type TextPreset = {
  id: string;
  label: string;
  sampleAr: string;
  sampleEn: string;
  style: {
    fontSize: number;
    fontWeight: string;
    fontFamily: string;
    color: string;
    lineHeight: number;
  };
};

const presets: TextPreset[] = [
  {
    id: "heading1",
    label: m.common_td_heading1(),
    sampleAr: m.common_td_heading1_sample(),
    sampleEn: "Add a heading",
    style: {
      fontSize: 32,
      fontWeight: "700",
      fontFamily: "Rubik",
      color: "#000000",
      lineHeight: 1.2,
    },
  },
  {
    id: "heading2",
    label: m.common_td_heading2(),
    sampleAr: m.common_td_heading2_sample(),
    sampleEn: "Add a subheading",
    style: {
      fontSize: 22,
      fontWeight: "600",
      fontFamily: "Rubik",
      color: "#1a1a1a",
      lineHeight: 1.3,
    },
  },
  {
    id: "heading3",
    label: m.common_td_heading3(),
    sampleAr: m.common_td_heading3_sample(),
    sampleEn: "Add a small heading",
    style: {
      fontSize: 16,
      fontWeight: "600",
      fontFamily: "Rubik",
      color: "#333333",
      lineHeight: 1.4,
    },
  },
  {
    id: "body",
    label: m.common_td_body(),
    sampleAr: m.common_td_body_sample(),
    sampleEn: "Add body text",
    style: {
      fontSize: 14,
      fontWeight: "400",
      fontFamily: "Rubik",
      color: "#333333",
      lineHeight: 1.5,
    },
  },
  {
    id: "body-small",
    label: m.common_td_body_small(),
    sampleAr: m.common_td_body_small_sample(),
    sampleEn: "Add small text",
    style: {
      fontSize: 11,
      fontWeight: "400",
      fontFamily: "Rubik",
      color: "#555555",
      lineHeight: 1.5,
    },
  },
  {
    id: "caption",
    label: m.common_td_caption(),
    sampleAr: m.common_td_caption_sample(),
    sampleEn: "Add a caption",
    style: {
      fontSize: 9,
      fontWeight: "400",
      fontFamily: "Rubik",
      color: "#777777",
      lineHeight: 1.4,
    },
  },
  {
    id: "label",
    label: m.common_td_label(),
    sampleAr: m.common_td_label_sample(),
    sampleEn: "Field Label",
    style: {
      fontSize: 10,
      fontWeight: "600",
      fontFamily: "Rubik",
      color: "#555555",
      lineHeight: 1.3,
    },
  },
  {
    id: "number",
    label: m.common_td_number(),
    sampleAr: m.common_td_number_sample(),
    sampleEn: "1234",
    style: {
      fontSize: 36,
      fontWeight: "700",
      fontFamily: "Rubik",
      color: "#1a1a1a",
      lineHeight: 1.1,
    },
  },
];

function handleDragStart(e: DragEvent, preset: TextPreset) {
  e.dataTransfer?.setData("application/x-docforge-element", "Text");
  e.dataTransfer?.setData(
    "application/x-docforge-text-preset",
    JSON.stringify(preset.style),
  );
  e.dataTransfer?.setData("text/plain", preset.sampleAr);
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "copy";
}

function insertPreset(preset: TextPreset) {
  store.addElement("Text", {
    content: preset.sampleAr,
    ...preset.style,
    textAlign: "start",
  });
}
</script>

<div class="text-presets-panel">
  <div class="presets-list">
    {#each presets as preset (preset.id)}
      <button
        type="button"
        class="preset-card"
        draggable="true"
        ondragstart={(e) => handleDragStart(e, preset)}
        onclick={() => insertPreset(preset)}
      >
        <div class="preset-preview" style="
          font-size: {Math.min(preset.style.fontSize, 24)}px;
          font-weight: {preset.style.fontWeight};
          font-family: {preset.style.fontFamily}, sans-serif;
          color: {preset.style.color};
          line-height: {preset.style.lineHeight};
          direction: inherit;
          text-align: start;
        ">
          {preset.sampleAr}
        </button>
        <span class="preset-label">{preset.label}</span>
      </div>
    {/each}
  </div>

  <div class="px-3 py-2 border-t border-border/20">
    <p class="text-[8px] text-muted-foreground text-center">
      {m.template_msg_drag_text()}
    </p>
  </div>
</div>

<style>
  .text-presets-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .presets-list {
    flex-grow: 1;
    overflow-y: auto;
    padding: 8px 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .presets-list::-webkit-scrollbar { display: none; }

  .preset-card {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 10px 12px;
    border-radius: 8px;
    border: 1px solid color-mix(in srgb, var(--color-border) 25%, transparent);
    background: color-mix(in srgb, var(--color-muted) 10%, transparent);
    cursor: grab;
    transition: all 0.15s ease;
    user-select: none;
  }

  .preset-card:hover {
    background: var(--color-accent);
    border-color: color-mix(in srgb, var(--color-border) 50%, transparent);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  }

  .preset-card:active {
    cursor: grabbing;
    transform: scale(0.98);
  }

  .preset-preview {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .preset-label {
    font-size: 8px;
    color: var(--color-muted-foreground);
    margin-top: 2px;
  }
</style>
