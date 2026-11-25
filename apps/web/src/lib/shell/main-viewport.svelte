<!--
@component MainViewport
A multi-panel split view that manages resizing of panels.
Inspired by GitButler's MainViewport.

Accepts `left`, `middle`, and optionally `right` snippets.
Each side panel is resizable via drag handles, and widths
are persisted to localStorage.

@example
```svelte
<MainViewport
  name="students"
  leftWidth={{ default: 280, min: 220 }}
  rightWidth={{ default: 300, min: 200 }}
>
  {#snippet left()}...{/snippet}
  {#snippet middle()}...{/snippet}
  {#snippet right()}...{/snippet}
</MainViewport>
```
-->
<script lang="ts">
import { untrack } from "svelte";
import { Button } from "@orbis/ui";
import Resizer from "./resizer.svelte";
import SashLayer from "./sash-layer.svelte";
import type { Snippet } from "svelte";
import { m } from "$lib/paraglide/messages";
import { page } from "$app/state";
import { isStandaloneWindow } from "./window-mode";

type PanelSize = {
  default: number;
  min: number;
};

let {
  name,
  left,
  leftWidth,
  middle,
  right,
  rightWidth = { default: 0, min: 0 },
  leftLabel,
  rightLabel,
}: {
  name: string;
  left?: Snippet;
  leftWidth: PanelSize;
  middle: Snippet;
  right?: Snippet;
  rightWidth?: PanelSize;
  leftLabel?: string;
  rightLabel?: string;
} = $props();

const standaloneWindow = $derived(isStandaloneWindow(page.url));
const COLLAPSED_WIDTH = 3;
const panelId = $props.id();
let viewportWidth = $state(0);
const compact = $derived(viewportWidth > 0 && viewportWidth < 760);
let visiblePanel = $state<"left" | "right" | null>(null);

function togglePanel(panel: "left" | "right") {
  visiblePanel = visiblePanel === panel ? null : panel;
}

// Convert px to rem using the current root font-size.
function pxToRem(px: number): number {
  if (typeof document === "undefined") return px / 16;
  const baseFontSize = parseFloat(
    getComputedStyle(document.documentElement).fontSize,
  );
  return px / (baseFontSize || 16);
}

function persistPanelWidth(persistId: string, widthRem: number) {
  const baseFontSize =
    typeof document === "undefined"
      ? 16
      : parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
  localStorage.setItem(
    `resizer:viewport-${persistId}:px`,
    String(widthRem * baseFontSize),
  );
}

// Panel preferred widths (in rem)
let leftPreferredWidth = $state(untrack(() => pxToRem(leftWidth.default)));
let rightPreferredWidth = $state(untrack(() => pxToRem(rightWidth.default)));

// Panel DOM refs
let leftDiv = $state<HTMLDivElement>();
let rightDiv = $state<HTMLDivElement>();

// Computed constraints (in rem)
const leftMinWidth = $derived(pxToRem(leftWidth.min));
const leftMaxWidth = $derived(pxToRem(leftWidth.min * 3));
const rightMinWidth = $derived(right ? pxToRem(rightWidth.min) : 0);
const rightMaxWidth = $derived(right ? pxToRem(rightWidth.min * 3) : 0);

// Final computed widths
const finalLeftWidth = $derived(
  left
    ? leftPreferredWidth <= COLLAPSED_WIDTH
      ? COLLAPSED_WIDTH
      : Math.max(leftMinWidth, Math.min(leftMaxWidth, leftPreferredWidth))
    : 0,
);
const finalRightWidth = $derived(
  right
    ? rightPreferredWidth <= COLLAPSED_WIDTH
      ? COLLAPSED_WIDTH
      : Math.max(rightMinWidth, Math.min(rightMaxWidth, rightPreferredWidth))
    : 0,
);

function getFallbackLabel(
  viewportName: string,
  side: "left" | "right",
): string {
  if (side === "left") {
    if (viewportName.includes("exams")) return m.shared_ui_examinations();
    if (viewportName.includes("academics")) return m.shared_ui_academics();
    if (viewportName.includes("finance")) return m.shared_ui_finance();
    return m.shared_ui_filters();
  } else {
    return m.shared_ui_details();
  }
}
</script>

<SashLayer>
	<div bind:clientWidth={viewportWidth}
    class="relative flex h-full w-full min-w-0 flex-1 items-stretch"
    class:flex-col={compact}>
    {#if compact && !standaloneWindow && (left || right)}
      <div class="flex shrink-0 flex-wrap gap-2 border-b border-border px-3 py-2">
        {#if left}
          <Button size="sm" variant="outline" aria-controls={panelId + "-left"} aria-expanded={visiblePanel === "left"} onclick={() => togglePanel("left")}>
            {leftLabel || getFallbackLabel(name, "left")}
          </Button>
        {/if}
        {#if right}
          <Button size="sm" variant="outline" aria-controls={panelId + "-right"} aria-expanded={visiblePanel === "right"} onclick={() => togglePanel("right")}>
            {rightLabel || getFallbackLabel(name, "right")}
          </Button>
        {/if}
      </div>
    {/if}
		{#if left && !standaloneWindow}
			<div
				class="relative min-w-0 shrink-0 flex-col {compact ? 'w-full max-h-[40%] overflow-y-auto' : right ? 'h-full max-w-[30%]' : 'h-full max-w-[35%]'}"
				class:flex={!compact || visiblePanel === "left"}
				class:hidden={compact && visiblePanel !== "left"}
				class:me-2={!compact && finalLeftWidth > COLLAPSED_WIDTH}
				class:overflow-hidden={!compact && finalLeftWidth === COLLAPSED_WIDTH}
				id={panelId + "-left"}
				bind:this={leftDiv}
				style:width={compact ? undefined : `${finalLeftWidth}rem`}
			>
				{#if !compact && finalLeftWidth === COLLAPSED_WIDTH}
					<!-- Collapsed vertical strip -->
					<div class="h-full w-full flex flex-col items-center py-4 select-none">
						<Button
							class="h-8 w-8 flex items-center justify-center cursor-pointer duration-150 mb-4"
							onclick={() => {
								leftPreferredWidth = pxToRem(leftWidth.default);
								persistPanelWidth(`${name}-left`, leftPreferredWidth);
							}}
							title={m.shared_ui_expand_sidebar()}
						>
                          <svg
                            width="18"
                            height="18"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="transition-transform duration-200 rtl:rotate-180"
                          >
                            <rect width="18" height="18" x="3" y="3" rx="2" />
                            <path d="M9 3v18" />
                            <path d="m14 9 3 3-3 3" />
                          </svg>
						</Button>
						<span class="vertical-text text-[10px] font-bold text-muted-foreground tracking-wider uppercase whitespace-nowrap">
							{leftLabel || getFallbackLabel(name, "left")}
						</span>
					</div>
				{:else}
					<div
						class="flex h-full flex-col overflow-hidden"
						oncollapsesidebar={() => {
							leftPreferredWidth = COLLAPSED_WIDTH;
							persistPanelWidth(`${name}-left`, COLLAPSED_WIDTH);
						}}
					>
						{@render left()}
					</div>
				{/if}
				{#if !compact}
					<Resizer
						viewport={leftDiv}
						label={leftLabel || getFallbackLabel(name, "left")}
						edge="end"
						minWidth={leftMinWidth}
						maxWidth={leftMaxWidth}
						defaultValue={pxToRem(leftWidth.default)}
						collapsedWidth={COLLAPSED_WIDTH}
						persistId="viewport-{name}-left"
						onWidth={(w) => (leftPreferredWidth = w)}
					/>
				{/if}
			</div>
		{/if}

		<div
			class="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden @container/workspace"
			class:no-left={!left || standaloneWindow}
		>
			{@render middle()}
		</div>

		{#if right && !standaloneWindow}
			<div
				class="relative min-w-0 shrink-0 flex-col {compact ? 'w-full max-h-[40%] overflow-y-auto' : left ? 'h-full max-w-[30%]' : 'h-full max-w-[35%]'}"
				class:flex={!compact || visiblePanel === "right"}
				class:hidden={compact && visiblePanel !== "right"}
				class:ms-2={!compact && finalRightWidth > COLLAPSED_WIDTH}
				class:overflow-hidden={!compact && finalRightWidth === COLLAPSED_WIDTH}
				id={panelId + "-right"}
				bind:this={rightDiv}
				style:width={compact ? undefined : `${finalRightWidth}rem`}
			>
				{#if !compact}
					<Resizer
						viewport={rightDiv}
						label={rightLabel || getFallbackLabel(name, "right")}
						edge="start"
						minWidth={rightMinWidth}
						maxWidth={rightMaxWidth}
						defaultValue={pxToRem(rightWidth.default)}
						collapsedWidth={COLLAPSED_WIDTH}
						persistId="viewport-{name}-right"
						onWidth={(w) => (rightPreferredWidth = w)}
					/>
				{/if}
				{#if !compact && finalRightWidth === COLLAPSED_WIDTH}
					<!-- Collapsed vertical strip -->
					<div class="h-full w-full flex flex-col items-center py-4 bg-card/30 border-s border-border/80 select-none">
						<button
							type="button"
							class="h-8 w-8 flex items-center justify-center rounded-lg hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer transition-colors duration-150 mb-4"
							onclick={() => {
								rightPreferredWidth = pxToRem(rightWidth.default);
								persistPanelWidth(`${name}-right`, rightPreferredWidth);
							}}
							title={m.shared_ui_expand_sidebar()}
						>
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<rect width="18" height="18" x="3" y="3" rx="2" />
								<path d="M15 3v18" />
							</svg>
						</button>
						
						<span class="vertical-text text-[10px] font-bold text-muted-foreground tracking-wider uppercase whitespace-nowrap">
							{rightLabel || getFallbackLabel(name, "right")}
						</span>
					</div>
				{:else}
					<div class="flex h-full flex-col overflow-hidden">
						{@render right()}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</SashLayer>

<style>
	.vertical-text {
		writing-mode: vertical-rl;
		text-orientation: mixed;
		transform: rotate(180deg);
	}

	:global(.resizer-dragging) {
		transition: none !important;
	}
</style>
