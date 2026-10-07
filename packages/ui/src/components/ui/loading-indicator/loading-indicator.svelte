<script lang="ts">
import { cn, type WithElementRef, type WithoutChildren } from "../../../utils.js";
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
        : "2rem",
);
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
  <span class="loading-indicator__container" aria-hidden="true">
    <span class="loading-indicator__shape"></span>
  </span>
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
    vertical-align: middle;
    contain: size layout paint;
  }

  .loading-indicator__container {
    display: flex;
    width: 100%;
    height: 100%;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    border-radius: 50%;
    background: var(--loading-indicator-container-color);
  }

  .loading-indicator__shape {
    display: block;
    width: 54%;
    height: 54%;
    background: currentColor;
    clip-path: polygon(
      50% 24.6%, 61.4% 7.3%, 62.7% 28%, 81.2% 18.8%, 72% 37.3%,
      92.7% 38.6%, 75.4% 50%, 92.7% 61.4%, 72% 62.7%, 81.2% 81.2%,
      62.7% 72%, 61.4% 92.7%, 50% 75.4%, 38.6% 92.7%, 37.3% 72%,
      18.8% 81.2%, 28% 62.7%, 7.3% 61.4%, 24.6% 50%, 7.3% 38.6%,
      28% 37.3%, 18.8% 18.8%, 37.3% 28%, 38.6% 7.3%
    );
    animation:
      loading-indicator-morph 4.55s cubic-bezier(0.2, 0.75, 0.35, 1) infinite,
      loading-indicator-rotate 4.666s linear infinite;
  }

  @keyframes loading-indicator-morph {
    0%,
    100% {
      clip-path: polygon(
        50% 24.6%, 61.4% 7.3%, 62.7% 28%, 81.2% 18.8%, 72% 37.3%,
        92.7% 38.6%, 75.4% 50%, 92.7% 61.4%, 72% 62.7%, 81.2% 81.2%,
        62.7% 72%, 61.4% 92.7%, 50% 75.4%, 38.6% 92.7%, 37.3% 72%,
        18.8% 81.2%, 28% 62.7%, 7.3% 61.4%, 24.6% 50%, 7.3% 38.6%,
        28% 37.3%, 18.8% 18.8%, 37.3% 28%, 38.6% 7.3%
      );
    }
    14.2857% {
      clip-path: polygon(
        50% 6.8%, 58.3% 19.1%, 68.3% 18.3%, 79.2% 20.8%,
        76.1% 35%, 89.9% 39.3%, 86.7% 50%, 80.9% 58.3%,
        87.4% 71.6%, 72.6% 72.6%, 68.3% 81.7%, 60.7% 89.9%,
        50% 80.1%, 39.3% 89.9%, 31.7% 81.7%, 27.4% 72.6%,
        12.6% 71.6%, 19.1% 58.3%, 13.3% 50%, 10.1% 39.3%,
        23.9% 35%, 20.8% 20.8%, 31.7% 18.3%, 41.7% 19.1%
      );
    }
    28.5714% {
      clip-path: polygon(
        50% 6.8%, 59.9% 12.9%, 65.5% 23.2%, 72.6% 27.4%,
        84.6% 30%, 91.6% 38.9%, 86.7% 50%, 79.3% 57.8%,
        78.9% 66.7%, 79.2% 79.2%, 71.2% 86.7%, 59% 83.8%,
        50% 80.1%, 41% 83.8%, 28.8% 86.7%, 20.8% 79.2%,
        21.1% 66.7%, 20.7% 57.8%, 13.3% 50%, 8.4% 38.9%,
        15.4% 30%, 27.4% 27.4%, 34.5% 23.2%, 40.1% 12.9%
      );
    }
    42.8571% {
      clip-path: polygon(
        50% 8.6%, 60.4% 11.3%, 68.3% 18.3%, 72.6% 27.4%,
        73.6% 36.4%, 73% 43.8%, 72.6% 50%, 73% 56.2%,
        73.6% 63.6%, 72.6% 72.6%, 68.3% 81.7%, 60.4% 88.7%,
        50% 91.4%, 39.6% 88.7%, 31.7% 81.7%, 27.4% 72.6%,
        26.4% 63.6%, 27% 56.2%, 27.4% 50%, 27% 43.8%,
        26.4% 36.4%, 27.4% 27.4%, 31.7% 18.3%, 39.6% 11.3%
      );
    }
    57.1428% {
      clip-path: polygon(
        50% 5.8%, 57.8% 20.9%, 70.2% 15%, 75.9% 24.1%,
        78.5% 33.5%, 91.7% 38.8%, 79.1% 50%, 91.7% 61.2%,
        78.5% 66.4%, 75.9% 75.9%, 70.2% 85%, 57.8% 79.1%,
        50% 94.2%, 42.2% 79.1%, 29.8% 85%, 24.1% 75.9%,
        21.5% 66.5%, 8.3% 61.2%, 20.9% 50%, 8.3% 38.8%,
        21.5% 33.6%, 24.1% 24.1%, 29.8% 15%, 42.2% 20.9%
      );
    }
    71.4285% {
      clip-path: polygon(
        50% 6.8%, 58.6% 17.8%, 66.7% 21.1%, 80.6% 19.4%,
        78.9% 33.3%, 82.2% 41.4%, 93.2% 50%, 82.2% 58.6%,
        78.9% 66.7%, 80.6% 80.6%, 66.7% 78.9%, 58.6% 82.2%,
        50% 93.2%, 41.4% 82.2%, 33.3% 78.9%, 19.4% 80.6%,
        21.1% 66.7%, 17.8% 58.6%, 6.8% 50%, 17.8% 41.4%,
        21.1% 33.3%, 19.4% 19.4%, 33.3% 21.1%, 41.4% 17.8%
      );
    }
    85.7142% {
      clip-path: polygon(
        50% 7.7%, 60.5% 10.7%, 68.2% 18.5%, 71.6% 28.4%,
        71.4% 37.7%, 69.7% 44.7%, 68.8% 50%, 69.7% 55.3%,
        71.4% 62.3%, 71.6% 71.6%, 68.2% 81.5%, 60.5% 89.3%,
        50% 92.3%, 39.5% 89.3%, 31.8% 81.5%, 28.4% 71.6%,
        28.6% 62.3%, 30.3% 55.3%, 31.2% 50%, 30.3% 44.7%,
        28.6% 37.7%, 28.4% 28.4%, 31.8% 18.5%, 39.5% 10.7%
      );
    }
  }

  @keyframes loading-indicator-rotate {
    to {
      transform: rotate(360deg);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .loading-indicator__shape {
      animation: none;
    }
  }
</style>
