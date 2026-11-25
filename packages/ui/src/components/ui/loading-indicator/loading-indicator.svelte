<script lang="ts">
import {
  cn,
  type WithElementRef,
  type WithoutChildren,
} from "../../../utils.js";
import type { HTMLAttributes } from "svelte/elements";

export type LoadingIndicatorSize = "sm" | "md" | "lg";

export type LoadingIndicatorProps = Omit<
  WithoutChildren<WithElementRef<HTMLAttributes<HTMLSpanElement>>>,
  "role" | "aria-label" | "aria-live" | "aria-busy"
> & {
  size?: LoadingIndicatorSize | number;
  label?: string;
  color?: string;
  containerColor?: string;
};

let {
  ref = $bindable(null),
  class: className,
  style,
  size = "md",
  label = "Loading",
  color = "var(--color-primary-foreground)",
  containerColor = "var(--color-primary)",
  ...restProps
}: LoadingIndicatorProps = $props();

const sizeValue = $derived(
  typeof size === "number"
    ? `${size}px`
    : size === "sm"
      ? "1rem"
      : size === "lg"
        ? "4rem"
        : "3rem",
);
</script>

<script module lang="ts">
// Use the same angular samples for every silhouette so SVG can interpolate the curves.
function silhouette(radius: (angle: number) => number) {
  const points = Array.from({ length: 72 }, (_, index) => {
    const angle = index * Math.PI / 36;
    const distance = radius(angle);
    return [50 + distance * Math.cos(angle), 50 + distance * Math.sin(angle)];
  });
  const segments = points.map(([x, y], index) => {
    const [nextX, nextY] = points[(index + 1) % points.length];
    return `Q ${x.toFixed(2)} ${y.toFixed(2)} ${((x + nextX) / 2).toFixed(2)} ${((y + nextY) / 2).toFixed(2)}`;
  });
  const [lastX, lastY] = points[points.length - 1];
  const [firstX, firstY] = points[0];
  return `M ${((lastX + firstX) / 2).toFixed(2)} ${((lastY + firstY) / 2).toFixed(2)} ${segments.join(" ")} Z`;
}

function oval(angle: number, width: number, height: number) {
  return 1 / Math.hypot(Math.cos(angle) / width, Math.sin(angle) / height);
}

// Material 3 Expressive's indeterminate shape sequence, drawn in the web viewBox.
const softBurst = silhouette((angle) => 32 + 4.5 * Math.cos(angle * 10));
const cookie9 = silhouette((angle) => 34 + 3 * Math.sin(angle * 9));
const pentagon = silhouette((angle) => {
  const sector = Math.PI * 2 / 5;
  const offset = (angle + Math.PI / 2) % sector - sector / 2;
  return 29 / Math.cos(offset * 0.94);
});
const pill = silhouette((angle) => {
  const x = Math.abs(Math.cos(angle));
  const y = Math.abs(Math.sin(angle));
  return 18 * x < 20 * y
    ? 18 / y
    : 20 * x + Math.sqrt(Math.max(0, 18 ** 2 - (20 * y) ** 2));
});
const sunny = silhouette((angle) => 34 + 3.5 * Math.cos(angle * 8));
const cookie4 = silhouette((angle) => 31 - 6 * Math.cos(angle * 4));
const ovalPath = silhouette((angle) => oval(angle - Math.PI / 4, 38, 25));
const shapes = [softBurst, cookie9, pentagon, pill, sunny, cookie4, ovalPath];
const shapeStyles = shapes.map((shape, index) => `--shape-${index}: path("${shape}")`).join("; ");
</script>

<span
  bind:this={ref}
  data-slot="loading-indicator"
  data-size={typeof size === "string" ? size : undefined}
  class={cn("loading-indicator", className)}
  style={`--loading-indicator-size: ${sizeValue}; --loading-indicator-color: ${color}; --loading-indicator-container-color: ${containerColor}; ${style ?? ""}`}
  role="status"
  aria-live="polite"
  aria-label={label}
  aria-busy="true"
  {...restProps}
>
  <svg class="loading-indicator__shape" viewBox="0 0 100 100" aria-hidden="true">
    <g class="loading-indicator__turn">
      <path d={softBurst} style={shapeStyles} />
    </g>
  </svg>
</span>

<style>
  .loading-indicator {
    display: inline-flex;
    width: var(--loading-indicator-size);
    height: var(--loading-indicator-size);
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    color: var(--loading-indicator-color);
    background: var(--loading-indicator-container-color);
    border-radius: 50%;
    vertical-align: middle;
    contain: size layout paint;
  }

  .loading-indicator__shape {
    width: 100%;
    height: 100%;
    fill: currentColor;
    animation: loading-indicator-rotate 4666ms linear infinite;
  }

  .loading-indicator__turn {
    transform-origin: 50% 50%;
    animation: loading-indicator-turn 2600ms linear(0, 0.2 12%, 0.7 27%, 1.07 51%, 1 75%, 1) infinite;
  }

  .loading-indicator__shape path {
    animation: loading-indicator-morph 4550ms linear(0, 0.2 12%, 0.7 27%, 1 44%, 1) infinite;
  }

  @keyframes loading-indicator-rotate {
    to {
      rotate: 360deg;
    }
  }

  @keyframes loading-indicator-turn {
    0% { rotate: 90deg; }
    25% { rotate: 180deg; }
    50% { rotate: 270deg; }
    75% { rotate: 360deg; }
    100% { rotate: 450deg; }
  }

  @keyframes loading-indicator-morph {
    0%, 100% { d: var(--shape-0); }
    14.285714% { d: var(--shape-1); }
    28.571429% { d: var(--shape-2); }
    42.857143% { d: var(--shape-3); }
    57.142857% { d: var(--shape-4); }
    71.428571% { d: var(--shape-5); }
    85.714286% { d: var(--shape-6); }
  }

  @media (prefers-reduced-motion: reduce) {
    .loading-indicator__shape,
    .loading-indicator__turn,
    .loading-indicator__shape path {
      animation: none;
    }
  }
</style>
