<script lang="ts">
let {
  edge,
  width,
  min,
  max,
  label,
  onresize,
}: {
  edge: "start" | "end";
  width: number;
  min: number;
  max: number;
  label: string;
  onresize: (width: number) => void;
} = $props();

let drag: { pointerId: number; x: number; width: number; sign: number } | null = null;

function resize(value: number) {
  onresize(Math.max(min, Math.min(max, value)));
}

function sign(element: HTMLElement) {
  const ltr = getComputedStyle(element).direction !== "rtl";
  return (edge === "end") === ltr ? 1 : -1;
}

function start(event: PointerEvent & { currentTarget: HTMLElement }) {
  if (event.button !== 0) return;
  const handle = event.currentTarget;
  drag = { pointerId: event.pointerId, x: event.clientX, width, sign: sign(handle) };
  handle.setPointerCapture(event.pointerId);
}

function move(event: PointerEvent) {
  if (drag?.pointerId === event.pointerId) resize(drag.width + (event.clientX - drag.x) * drag.sign);
}

function keyboard(event: KeyboardEvent & { currentTarget: HTMLElement }) {
  if (event.key === "Home") resize(min);
  else if (event.key === "End") resize(max);
  else if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
    resize(width + (event.key === "ArrowRight" ? 16 : -16) * sign(event.currentTarget));
  } else return;
  event.preventDefault();
  event.stopPropagation();
}
</script>

<!-- A focusable separator is the ARIA window splitter pattern. -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
<div
  class="resize-handle"
  class:at-end={edge === "end"}
  role="separator"
  tabindex="0"
  aria-orientation="vertical"
  aria-label={label}
  aria-valuenow={Math.round(width)}
  aria-valuemin={min}
  aria-valuemax={max}
  onpointerdown={start}
  onpointermove={move}
  onlostpointercapture={() => { drag = null; }}
  onkeydown={keyboard}
></div>

<style>
  .resize-handle {
    position: absolute;
    z-index: 2;
    inset-block: 0;
    inset-inline-start: 0;
    width: 7px;
    cursor: col-resize;
    touch-action: none;
  }
  .at-end { inset-inline: auto 0; }
  .resize-handle:hover, .resize-handle:focus-visible {
    background: color-mix(in srgb, var(--color-primary) 28%, transparent);
  }
  @media (max-width: 900px) { .resize-handle { display: none; } }
</style>
