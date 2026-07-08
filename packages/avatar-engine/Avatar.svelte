<script lang="ts">
import { onDestroy } from "svelte";
import { spring } from "svelte/motion";
import { AvatarController } from "./AvatarController.svelte";
import { idleAnimation } from "./animations/idle.js";
import { blinkAnimation } from "./animations/blink.js";
import { thinkAnimation } from "./animations/think.js";
import { happyAnimation } from "./animations/happy.js";
import { errorAnimation } from "./animations/error.js";

const {
  controller = new AvatarController(),
  hat = "none", // "none" | "crown" | "graduation" | "firefighter"
  width = "100%",
  height = "100%",
  class: className = "",
}: {
  controller?: AvatarController;
  hat?: "none" | "crown" | "graduation" | "firefighter";
  width?: string;
  height?: string;
  class?: string;
} = $props();

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
  switch (controller.state) {
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

// Watch for state/animation changes and update spring settings + targets
$effect(() => {
  const config = activeAnimation;

  // Apply configuration springs settings dynamically
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

  // Set target spring positions if we are NOT hovered in idle state
  if (!(controller.state === "idle" && isHovered)) {
    bodyScaleSpring.set(config.bodyScale ?? 1.0);
    bodyXSpring.set(0);
    bodyYSpring.set(config.bodyY ?? 0);
    eyeScaleXSpring.set(config.eyeScaleX ?? 1.0);
    eyeScaleYSpring.set(config.eyeScaleY ?? 1.0);
    eyeXSpring.set(config.eyeX ?? 0);
    eyeYSpring.set(config.eyeY ?? 0);
  }

  // Morph hand-drawn eyebrows based on state
  if (controller.state === "happy") {
    leftEyebrowYSpring.set(-6);
    rightEyebrowYSpring.set(-6);
    leftEyebrowRotateSpring.set(-5);
    rightEyebrowRotateSpring.set(5);
  } else if (controller.state === "thinking") {
    leftEyebrowYSpring.set(1);
    rightEyebrowYSpring.set(-4);
    leftEyebrowRotateSpring.set(8);
    rightEyebrowRotateSpring.set(-10);
  } else if (controller.state === "error") {
    leftEyebrowYSpring.set(2);
    rightEyebrowYSpring.set(2);
    leftEyebrowRotateSpring.set(-15);
    rightEyebrowRotateSpring.set(15);
  } else {
    // Idle / blinking
    leftEyebrowYSpring.set(0);
    rightEyebrowYSpring.set(0);
    leftEyebrowRotateSpring.set(0);
    rightEyebrowRotateSpring.set(0);
  }

  isShaking = config.bodyShake ?? false;
});

// Mouse move handler for premium interactive parallax effect
function handleMouseMove(e: MouseEvent) {
  if (!containerEl || controller.state !== "idle") return;

  const rect = containerEl.getBoundingClientRect();
  const centerX = rect.left + rect.width / 2;
  const centerY = rect.top + rect.height / 2;

  const relX = (e.clientX - centerX) / (rect.width / 2);
  const relY = (e.clientY - centerY) / (rect.height / 2);

  mouseX = Math.max(-1.0, Math.min(1.0, relX));
  mouseY = Math.max(-1.0, Math.min(1.0, relY));
  isHovered = true;

  // Apply interactive shifts (Parallax: eyes & face features move most, head moves medium)
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

  // Reset springs smoothly back to idle parameters
  const config = activeAnimation;
  bodyScaleSpring.set(config.bodyScale ?? 1.0);
  bodyXSpring.set(0);
  bodyYSpring.set(config.bodyY ?? 0);
  eyeXSpring.set(config.eyeX ?? 0);
  eyeYSpring.set(config.eyeY ?? 0);
}

onDestroy(() => {
  controller.destroy();
});
</script>

<div
  bind:this={containerEl}
  onmousemove={handleMouseMove}
  onmouseleave={handleMouseLeave}
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
    <!-- Main Head Casing (Notion clean circle, no border, filled with theme-aligned muted background) -->
    <g
      id="avatar-head"
      style="transform: translate({$bodyXSpring}px, {$bodyYSpring}px) scale({$bodyScaleSpring}); transform-origin: 100px 120px;"
      class={controller.state === "idle" && !isHovered ? "idle-breathing" : ""}
    >
      <!-- Head Circle -->
      <circle cx="100" cy="120" r="58" fill="var(--muted, #f1f5f9)" />

      <!-- Minimalist Hand-Drawn Face Features matching the image -->
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

        <!-- Notion Style Nose starting from the eyebrow intersection -->
        <path
          d="M 102 100 L 75 146 L 104 147"
          fill="none"
          stroke="var(--foreground, #18181b)"
          stroke-width="4.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />

        <!-- Eyes: Happy Arches vs Regular Dots -->
        {#if controller.state === "happy"}
          <!-- Smiling happy arches -->
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
          <!-- Standard blinking dots -->
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

      <!-- Layer 3: Dynamic Role Hats (rendered on top of head, moving with head) -->
      {#if hat === "crown"}
        <!-- Gold Crown (Admin Role) -->
        <g id="hat-crown" style="transform: translate(0, -6px);">
          <path
            d="M 70 70 L 60 38 L 82 54 L 100 28 L 118 54 L 140 38 L 130 70 Z"
            fill="#fbbf24"
            stroke="var(--foreground, #18181b)"
            stroke-width="4.5"
            stroke-linejoin="round"
          />
          <!-- Little jewels on points -->
          <circle cx="60" cy="38" r="3.5" fill="#ef4444" stroke="var(--foreground, #18181b)" stroke-width="1.5" />
          <circle cx="100" cy="28" r="3.5" fill="#3b82f6" stroke="var(--foreground, #18181b)" stroke-width="1.5" />
          <circle cx="140" cy="38" r="3.5" fill="#10b981" stroke="var(--foreground, #18181b)" stroke-width="1.5" />
        </g>
      {:else if hat === "graduation"}
        <!-- Graduation Cap (Teacher Role) -->
        <g id="hat-graduation" style="transform: translate(0, -4px);">
          <!-- Cap base -->
          <path
            d="M 74 52 L 74 69 C 74 76, 126 76, 126 69 L 126 52"
            fill="#1f2937"
            stroke="var(--foreground, #18181b)"
            stroke-width="4.5"
            stroke-linejoin="round"
          />
          <!-- Diamond board -->
          <polygon
            points="100,32 148,48 100,64 52,48"
            fill="#111827"
            stroke="var(--foreground, #18181b)"
            stroke-width="4.5"
            stroke-linejoin="round"
          />
          <!-- Tassel -->
          <path
            d="M 100 48 L 78 56 L 76 74"
            fill="none"
            stroke="#fbbf24"
            stroke-width="2.5"
            stroke-linecap="round"
          />
          <circle cx="76" cy="74" r="3" fill="#fbbf24" />
        </g>
      {:else if hat === "firefighter"}
        <!-- Red Firefighter Hat (Staff/Fun Role) -->
        <g id="hat-firefighter" style="transform: translate(0, -4px);">
          <!-- Crown -->
          <path
            d="M 70 70 C 70 40, 130 40, 130 70 Z"
            fill="#ef4444"
            stroke="var(--foreground, #18181b)"
            stroke-width="4.5"
            stroke-linejoin="round"
          />
          <!-- Brim -->
          <path
            d="M 54 72 Q 100 64 146 72 C 150 73, 150 77, 146 77 Q 100 69 54 77 C 50 77, 50 73, 54 72 Z"
            fill="#ef4444"
            stroke="var(--foreground, #18181b)"
            stroke-width="4.5"
            stroke-linejoin="round"
          />
          <!-- Shield/Badge -->
          <polygon
            points="94,56 106,56 108,68 92,68"
            fill="#fbbf24"
            stroke="var(--foreground, #18181b)"
            stroke-width="2"
            stroke-linejoin="round"
          />
          <circle cx="100" cy="62" r="2" fill="#ef4444" />
        </g>
      {/if}
    </g>
  </svg>
</div>

<style>
  .avatar-svg {
    overflow: visible;
  }

  /* Keyframe shake for error state */
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

  /* Slow breathing translation on idle when not interactive */
  .idle-breathing {
    animation: breath-animation 5s ease-in-out infinite;
  }

  @keyframes breath-animation {
    0% { transform: translate(0, 0px) scale(1); }
    50% { transform: translate(0, -2.5px) scale(1.012); }
    100% { transform: translate(0, 0px) scale(1); }
  }
</style>
