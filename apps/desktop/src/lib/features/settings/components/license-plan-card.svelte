
<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { formatDate } from "$lib/core/i18n";
  import { Logo } from "@orbis/icons";

  let {
    planCode,
    planLabel,
    validUntil,
  }: {
    planCode: string;
    planLabel: string;
    validUntil: string | null;
  } = $props();

  const planTheme = $derived.by(() => {
    const plan = planCode.toLowerCase().replace(/[_\s-]+/g, "");

    if (plan.includes("enterprise")) {
      return { color: "#EC4899", glow: "rgba(236, 72, 153, .8)" };
    }
    if (plan.includes("professional")) {
      return { color: "#A821D9", glow: "rgba(168, 33, 217, .8)" };
    }
    if (plan.includes("growth")) {
      return { color: "#84CC16", glow: "rgba(132, 204, 22, .8)" };
    }
    if (plan.includes("starter")) {
      return { color: "#6366F1", glow: "rgba(99, 102, 241, .8)" };
    }
    if (plan.includes("trial") || plan.includes("free")) {
      return { color: "#9CA3AF", glow: "rgba(156, 163, 175, .7)" };
    }

    return { color: "#7C3AED", glow: "rgba(124, 58, 237, .8)" };
  });
</script>

<div class="license-plan-card flex w-full flex-col items-center gap-4 text-center">
  <div
    class="plan-emblem"
    style:--plan-color={planTheme.color}
    style:--plan-glow={planTheme.glow}
    role="img"
    aria-label={planLabel}
  >
    <div class="plan-emblem__logo" aria-hidden="true">
      <Logo size="100%" dotClass="plan-logo-dot" />
    </div>

    <div class="plan-emblem__content">
      <span class="plan-emblem__label">{planLabel}</span>
    </div>
  </div>

  <div class="flex flex-col items-center gap-1">
    <span class="text-xs text-muted-foreground">
      {m.license_valid_until()}
    </span>
    <span class="text-sm font-medium tabular-nums text-foreground">
      {formatDate(validUntil, "datetime") || "—"}
    </span>
  </div>
</div>

<style>
  .plan-emblem {
    position: relative;
    display: grid;
    place-items: center;
    width: min(100%, 10rem);
    aspect-ratio: 642 / 660;
    color: var(--plan-color);
    transition: transform 250ms ease;
  }

  .plan-emblem__logo {
    position: absolute;
    inset: 0;
    display: grid;
    place-items: center;
    pointer-events: none;
  }

  .plan-emblem__logo :global(svg) {
    overflow: visible;
  }

  /*
   * The main logo path keeps its original silhouette and color.
   * Hover glow is applied to the hovered SVG path only.
   */
  .plan-emblem__logo :global(svg > path:first-child) {
    fill: currentColor;
    transition:
      filter 250ms ease,
      opacity 250ms ease;
  }

  .plan-emblem__logo :global(svg > path:last-child) {
    fill: #A821D9;
    transition:
      filter 250ms ease,
      opacity 250ms ease;
  }

  .plan-emblem__content {
    position: relative;
    z-index: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 78%;
    min-width: 0;
    padding: 0.25rem;
    pointer-events: none;
  }

  .plan-emblem__label {
    max-width: 100%;
    overflow-wrap: anywhere;
    text-align: center;
    font-size: clamp(0.75rem, 1.2vw, 1rem);
    font-weight: 700;
    line-height: 1.25;
    letter-spacing: -0.035em;
    color: var(--foreground, #fff);
    text-shadow: 0 0 8px var(--plan-glow);
    transition: text-shadow 250ms ease;
  }

  /* Hover the main curved part: glow only that part */
  .plan-emblem__logo:has(svg > path:first-child:hover)
    :global(svg > path:first-child) {
    filter:
      drop-shadow(0 0 3px var(--plan-color))
      drop-shadow(0 0 7px var(--plan-glow))
      drop-shadow(0 0 14px var(--plan-glow));
  }

  /* Hover the purple dot: glow only the dot */
  .plan-emblem__logo:has(svg > path:last-child:hover)
    :global(svg > path:last-child) {
    filter:
      drop-shadow(0 0 3px #A821D9)
      drop-shadow(0 0 8px #A821D9)
      drop-shadow(0 0 14px rgba(168, 33, 217, .85));
  }

  .plan-emblem__logo :global(svg > path) {
    pointer-events: visiblePainted;
    cursor: pointer;
  }

  @media (prefers-reduced-motion: reduce) {
    .plan-emblem,
    .plan-emblem__logo :global(svg > path),
    .plan-emblem__label {
      transition: none;
    }
  }
</style>
