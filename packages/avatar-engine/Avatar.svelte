<script lang="ts">
import { onDestroy } from "svelte";
import { spring } from "svelte/motion";
import { AvatarController } from "./AvatarController.svelte";
import AvatarHats from "./AvatarHats.svelte";
import { ALL_AVATAR_HATS, type AvatarHat } from "./types.js";
import { idleAnimation } from "./animations/idle.js";
import { blinkAnimation } from "./animations/blink.js";
import { thinkAnimation } from "./animations/think.js";
import { happyAnimation } from "./animations/happy.js";
import { errorAnimation } from "./animations/error.js";

let {
  controller,
  hat = "random",
  width = "100%",
  height = "100%",
  class: className = "",
}: {
  controller?: AvatarController;
  hat?: AvatarHat;
  width?: string;
  height?: string;
  class?: string;
} = $props();

const fallbackController = new AvatarController();
const activeController = $derived(controller ?? fallbackController);

// Idiomatic Svelte 5 Rune: pure reactivity derived state
const resolvedHat = $derived.by<AvatarHat>(() => {
  if (hat === "random") {
    const idx = Math.floor(Math.random() * ALL_AVATAR_HATS.length);
    return ALL_AVATAR_HATS[idx];
  }
  return hat;
});

// Create spring stores for interactive expressions
const bodyScaleSpring = spring(1.0);
const bodyYSpring = spring(0);
const bodyXSpring = spring(0);
const eyeScaleXSpring = spring(1.0);
const eyeScaleYSpring = spring(1.0);
const eyeXSpring = spring(0);
const eyeYSpring = spring(0);

// Eyebrow custom springs for hand-drawn expressions
const leftEyebrowYSpring = spring(0);
const rightEyebrowYSpring = spring(0);
const leftEyebrowRotateSpring = spring(0);
const rightEyebrowRotateSpring = spring(0);

let isShaking = $state(false);
let containerEl = $state<HTMLDivElement | null>(null);
let mouseX = $state(0);
let mouseY = $state(0);
let isHovered = $state(false);

// Derives current animation config based on controller state
const activeAnimation = $derived.by(() => {
  switch (activeController.state) {
    case "blinking":
      return blinkAnimation;
    case "thinking":
      return thinkAnimation;
    case "happy":
      return happyAnimation;
    case "error":
      return errorAnimation;
    default:
      return idleAnimation;
  }
});

// Sync animation spring targets on state change
$effect(() => {
  const config = activeAnimation;

  if (config.springs) {
    if (config.springs.body) {
      bodyScaleSpring.stiffness = config.springs.body.stiffness;
      bodyScaleSpring.damping = config.springs.body.damping;
      bodyXSpring.stiffness = config.springs.body.stiffness;
      bodyXSpring.damping = config.springs.body.damping;
      bodyYSpring.stiffness = config.springs.body.stiffness;
      bodyYSpring.damping = config.springs.body.damping;
    }
    if (config.springs.eyes) {
      eyeScaleXSpring.stiffness = config.springs.eyes.stiffness;
      eyeScaleXSpring.damping = config.springs.eyes.damping;
      eyeScaleYSpring.stiffness = config.springs.eyes.stiffness;
      eyeScaleYSpring.damping = config.springs.eyes.damping;
      eyeXSpring.stiffness = config.springs.eyes.stiffness;
      eyeXSpring.damping = config.springs.eyes.damping;
      eyeYSpring.stiffness = config.springs.eyes.stiffness;
      eyeYSpring.damping = config.springs.eyes.damping;
    }
  }

  if (!(activeController.state === "idle" && isHovered)) {
    bodyScaleSpring.set(config.bodyScale ?? 1.0);
    bodyXSpring.set(0);
    bodyYSpring.set(config.bodyY ?? 0);
    eyeScaleXSpring.set(config.eyeScaleX ?? 1.0);
    eyeScaleYSpring.set(config.eyeScaleY ?? 1.0);
    eyeXSpring.set(config.eyeX ?? 0);
    eyeYSpring.set(config.eyeY ?? 0);
  }

  if (activeController.state === "happy") {
    leftEyebrowYSpring.set(-6);
    rightEyebrowYSpring.set(-6);
    leftEyebrowRotateSpring.set(-5);
    rightEyebrowRotateSpring.set(5);
  } else if (activeController.state === "thinking") {
    leftEyebrowYSpring.set(1);
    rightEyebrowYSpring.set(-4);
    leftEyebrowRotateSpring.set(8);
    rightEyebrowRotateSpring.set(-10);
  } else if (activeController.state === "error") {
    leftEyebrowYSpring.set(2);
    rightEyebrowYSpring.set(2);
    leftEyebrowRotateSpring.set(-15);
    rightEyebrowRotateSpring.set(15);
  } else {
    leftEyebrowYSpring.set(0);
    rightEyebrowYSpring.set(0);
    leftEyebrowRotateSpring.set(0);
    rightEyebrowRotateSpring.set(0);
  }

  isShaking = config.bodyShake ?? false;
});

// Mouse move handler for interactive parallax effect
function handleMouseMove(e: MouseEvent) {
  if (!containerEl || activeController.state !== "idle") return;

  const rect = containerEl.getBoundingClientRect();
  const centerX = rect.left + rect.width / 2;
  const centerY = rect.top + rect.height / 2;

  const relX = (e.clientX - centerX) / (rect.width / 2);
  const relY = (e.clientY - centerY) / (rect.height / 2);

  mouseX = Math.max(-1.0, Math.min(1.0, relX));
  mouseY = Math.max(-1.0, Math.min(1.0, relY));
  isHovered = true;

  eyeXSpring.set(mouseX * 7.0);
  eyeYSpring.set(mouseY * 5.5);
  bodyXSpring.set(mouseX * 3.0);
  bodyYSpring.set(mouseY * 2.5);
  bodyScaleSpring.set(1.01);
}

function handleMouseLeave() {
  isHovered = false;
  mouseX = 0;
  mouseY = 0;

  const config = activeAnimation;
  bodyScaleSpring.set(config.bodyScale ?? 1.0);
  bodyXSpring.set(0);
  bodyYSpring.set(config.bodyY ?? 0);
  eyeXSpring.set(config.eyeX ?? 0);
  eyeYSpring.set(config.eyeY ?? 0);
}

onDestroy(() => {
  fallbackController.destroy();
});
</script>

<div
  bind:this={containerEl}
  onmousemove={(e) => handleMouseMove(e)}
  onmouseleave={() => handleMouseLeave()}
  class="relative flex items-center justify-center select-none {className}"
  style:width
  style:height
  role="presentation"
>
  <svg
    class="avatar-svg {isShaking ? 'shake' : ''}"
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 200 200"
    width="100%"
    height="100%"
  >
    <!-- Main Head Casing -->
    <g
      id="avatar-head"
      style="transform: translate({$bodyXSpring}px, {$bodyYSpring}px) scale({$bodyScaleSpring}); transform-origin: 100px 120px;"
      class={activeController.state === "idle" && !isHovered ? "idle-breathing" : ""}
    >
      <!-- Head Circle -->
      <circle cx="100" cy="120" r="58" fill="var(--muted, #f1f5f9)" />

      <!-- Minimalist Hand-Drawn Face Features -->
      <g id="face-features" style="transform: translate({$eyeXSpring * 0.4}px, {$eyeYSpring * 0.4}px); transform-origin: 100px 120px;">
        <!-- Left Eyebrow -->
        <path
          d="M 68 100 C 72 88, 96 88, 102 100"
          fill="none"
          stroke="var(--foreground, #18181b)"
          stroke-width="4.5"
          stroke-linecap="round"
          style="transform: translate(0, {$leftEyebrowYSpring}px) rotate({$leftEyebrowRotateSpring}deg); transform-origin: 85px 94px;"
        />
        
        <!-- Right Eyebrow -->
        <path
          d="M 102 100 C 108 88, 132 88, 142 100"
          fill="none"
          stroke="var(--foreground, #18181b)"
          stroke-width="4.5"
          stroke-linecap="round"
          style="transform: translate(0, {$rightEyebrowYSpring}px) rotate({$rightEyebrowRotateSpring}deg); transform-origin: 122px 94px;"
        />

        <!-- Notion Style Nose -->
        <path
          d="M 102 100 L 75 146 L 104 147"
          fill="none"
          stroke="var(--foreground, #18181b)"
          stroke-width="4.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />

        <!-- Eyes -->
        {#if activeController.state === "happy"}
          <path
            d="M 78 116 Q 86 109 94 116"
            fill="none"
            stroke="var(--foreground, #18181b)"
            stroke-width="4.5"
            stroke-linecap="round"
            style="transform: translate({$eyeXSpring * 0.6}px, {$eyeYSpring * 0.6}px);"
          />
          <path
            d="M 112 118 Q 120 111 128 118"
            fill="none"
            stroke="var(--foreground, #18181b)"
            stroke-width="4.5"
            stroke-linecap="round"
            style="transform: translate({$eyeXSpring * 0.6}px, {$eyeYSpring * 0.6}px);"
          />
        {:else}
          <circle
            cx="86"
            cy="114"
            r="5.5"
            fill="var(--foreground, #18181b)"
            style="transform: translate({$eyeXSpring * 0.6}px, {$eyeYSpring * 0.6}px) scale({$eyeScaleXSpring}, {$eyeScaleYSpring}); transform-origin: 86px 114px;"
          />
          <circle
            cx="120"
            cy="117"
            r="5.5"
            fill="var(--foreground, #18181b)"
            style="transform: translate({$eyeXSpring * 0.6}px, {$eyeYSpring * 0.6}px) scale({$eyeScaleXSpring}, {$eyeScaleYSpring}); transform-origin: 120px 117px;"
          />
        {/if}
      </g>

      <!-- Layer 3: Render Hats Component -->
      <AvatarHats hat={resolvedHat} />
    </g>
  </svg>
</div>

<style>
  .avatar-svg {
    overflow: visible;
  }

  .shake {
    animation: shake-animation 0.15s infinite alternate;
  }

  @keyframes shake-animation {
    0% { transform: translate(-1.5px, -1px) rotate(-0.5deg); }
    10% { transform: translate(1px, 1.5px) rotate(0.5deg); }
    20% { transform: translate(-1.5px, 0px) rotate(-1deg); }
    30% { transform: translate(1px, -1.5px) rotate(0deg); }
    40% { transform: translate(-1px, 1.5px) rotate(0.5deg); }
    50% { transform: translate(1.5px, -1px) rotate(-0.5deg); }
    60% { transform: translate(-1px, 1px) rotate(0deg); }
    70% { transform: translate(1.5px, 1.5px) rotate(0.5deg); }
    80% { transform: translate(-1.5px, -1px) rotate(-1deg); }
    90% { transform: translate(1px, 1px) rotate(0deg); }
    100% { transform: translate(-1px, -1px) rotate(0.5deg); }
  }

  .idle-breathing {
    animation: breath-animation 5s ease-in-out infinite;
  }

  @keyframes breath-animation {
    0% { transform: translate(0, 0px) scale(1); }
    50% { transform: translate(0, -2.5px) scale(1.012); }
    100% { transform: translate(0, 0px) scale(1); }
  }
</style>
