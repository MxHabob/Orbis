<script lang="ts">
import { onDestroy } from "svelte";
import { createSubscriber } from "svelte/reactivity";
import { AvatarController } from "../core/avatar-controller.js";
import { getAvatarDrawing } from "../core/avatar-svg.js";
import { type AvatarHat, resolveAvatarHat } from "../core/types.js";
import AvatarDrawing from "./avatar-drawing.svelte";

const {
  controller,
  hat = "random",
  width = "100%",
  height = "100%",
  class: className = "",
  label = "Orbis avatar",
  autoBlink = true,
  faceColor,
  featureColor,
}: {
  controller?: AvatarController;
  hat?: AvatarHat;
  width?: string;
  height?: string;
  class?: string;
  label?: string;
  autoBlink?: boolean;
  faceColor?: string;
  featureColor?: string;
} = $props();

const fallbackController = new AvatarController();
const activeController = $derived(controller ?? fallbackController);
const subscribeToState = createSubscriber((update) =>
  activeController.subscribe(() => update()),
);
const state = $derived.by(() => {
  subscribeToState();
  return activeController.state;
});
const resolvedHat = $derived(resolveAvatarHat(hat));
const drawing = $derived(
  getAvatarDrawing({ hat: resolvedHat, state, faceColor, featureColor }),
);

$effect(() => {
  if (!controller) fallbackController.setAutoBlink(autoBlink);
});

onDestroy(() => fallbackController.destroy());
</script>

<div
  class={`avatar-root ${className}`}
  style:width
  style:height
  role="img"
  aria-label={label}
  data-state={state}
>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 200 200"
    width="100%"
    height="100%"
    class="avatar-svg"
  >
    <AvatarDrawing elements={drawing} />
  </svg>
</div>

<style>
  .avatar-root {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    overflow: visible;
    user-select: none;
  }

  .avatar-root :global(svg) {
    display: block;
    overflow: visible;
  }

  .avatar-root[data-state="happy"] :global(svg) {
    animation: happy-bounce 520ms ease-out;
  }

  .avatar-root[data-state="error"] :global(svg) {
    animation: error-shake 280ms ease-in-out;
  }

  .avatar-root[data-state="celebrating"] :global(svg) {
    animation: celebrate 680ms ease-out;
    transform-origin: center;
  }

  .avatar-root[data-state="curious"] :global(svg) {
    animation: curious-tilt 880ms ease-in-out;
    transform-origin: center;
  }

  .avatar-root[data-state="sleepy"] :global(svg) {
    animation: sleepy-breathe 2.2s ease-in-out infinite;
    transform-origin: center;
  }

  @keyframes happy-bounce {
    0%, 100% { transform: translateY(0) scale(1); }
    45% { transform: translateY(-3px) scale(1.035); }
  }

  @keyframes celebrate {
    0%, 100% { transform: translateY(0) scale(1) rotate(0); }
    24% { transform: translateY(-8px) scale(1.08) rotate(-5deg); }
    52% { transform: translateY(0) scale(1) rotate(2deg); }
    76% { transform: translateY(-5px) scale(1.04) rotate(5deg); }
  }

  @keyframes curious-tilt {
    0%, 100% { transform: rotate(0); }
    30%, 65% { transform: rotate(8deg); }
  }

  @keyframes sleepy-breathe {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.025); }
  }

  @keyframes error-shake {
    0%, 100% { transform: translateX(0); }
    25% { transform: translateX(-2px) rotate(-1deg); }
    75% { transform: translateX(2px) rotate(1deg); }
  }

  @media (prefers-reduced-motion: reduce) {
    .avatar-root :global(svg) {
      animation: none !important;
    }
  }
</style>
