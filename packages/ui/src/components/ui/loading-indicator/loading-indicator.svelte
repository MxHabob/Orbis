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
    transform-origin: center;
    animation:
      loading-indicator-rotate 1.35s linear infinite,
      loading-indicator-breathe 2.5s ease-in-out infinite;
  }

  @keyframes loading-indicator-rotate {
    to {
      rotate: 360deg;
    }
  }

  @keyframes loading-indicator-breathe {
    0%,
    100% {
      scale: 0.88;
      opacity: 0.8;
    }
    50% {
      scale: 1;
      opacity: 1;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .loading-indicator__shape {
      animation: none;
    }
  }
</style>
