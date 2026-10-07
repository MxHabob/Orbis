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

const COLLAPSED_WIDTH = 3;

// Convert px to rem using root font-size (cached to avoid DOM reads per render)
let _cachedBaseFontSize: number | null = null;
function pxToRem(px: number): number {
  if (typeof document === "undefined") return px / 16;
  _cachedBaseFontSize ??= parseFloat(
    getComputedStyle(document.documentElement).fontSize,
  );
  return px / _cachedBaseFontSize;
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
	<div class="relative flex h-full w-full flex-1 items-stretch">
		{#if left}
			<div
				class="relative flex h-full shrink-0 flex-col"
				class:me-2={finalLeftWidth > COLLAPSED_WIDTH}
				class:overflow-hidden={finalLeftWidth === COLLAPSED_WIDTH}
				bind:this={leftDiv}
				style:width="{finalLeftWidth}rem"
				style:min-width="{finalLeftWidth}rem"
			>
				{#if finalLeftWidth === COLLAPSED_WIDTH}
					<!-- Collapsed vertical strip -->
					<div class="h-full w-full flex flex-col items-center py-4 select-none">
						<Button
							class="h-8 w-8 flex items-center justify-center cursor-pointer duration-150 mb-4"
							onclick={() => {
								leftPreferredWidth = pxToRem(leftWidth.default);
								localStorage.setItem(`resizer:viewport-${name}-left`, String(leftPreferredWidth));
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
							localStorage.setItem(`resizer:viewport-${name}-left`, String(COLLAPSED_WIDTH));
						}}
					>
						{@render left()}
					</div>
				{/if}
				<Resizer
					viewport={leftDiv}
					edge="end"
					minWidth={leftMinWidth}
					maxWidth={leftMaxWidth}
					defaultValue={pxToRem(leftWidth.default)}
					collapsedWidth={COLLAPSED_WIDTH}
					persistId="viewport-{name}-left"
					onWidth={(w) => (leftPreferredWidth = w)}
				/>
			</div>
		{/if}

		<div class="relative flex h-full min-w-0 flex-1 flex-col overflow-hidden" class:no-left={!left}>
			{@render middle()}
		</div>

		{#if right}
			<div
				class="relative flex h-full shrink-0 flex-col"
				class:ms-2={finalRightWidth > COLLAPSED_WIDTH}
				class:overflow-hidden={finalRightWidth === COLLAPSED_WIDTH}
				bind:this={rightDiv}
				style:width="{finalRightWidth}rem"
				style:min-width="{finalRightWidth}rem"
			>
				<Resizer
					viewport={rightDiv}
					edge="start"
					minWidth={rightMinWidth}
					maxWidth={rightMaxWidth}
					defaultValue={pxToRem(rightWidth.default)}
					collapsedWidth={COLLAPSED_WIDTH}
					persistId="viewport-{name}-right"
					onWidth={(w) => (rightPreferredWidth = w)}
				/>
				{#if finalRightWidth === COLLAPSED_WIDTH}
					<!-- Collapsed vertical strip -->
					<div class="h-full w-full flex flex-col items-center py-4 bg-card/30 border-s border-border/80 select-none">
						<button
							type="button"
							class="h-8 w-8 flex items-center justify-center rounded-lg hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer transition-colors duration-150 mb-4"
							onclick={() => {
								rightPreferredWidth = pxToRem(rightWidth.default);
								localStorage.setItem(`resizer:viewport-${name}-right`, String(rightPreferredWidth));
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
