<!--
@component AvatarHats
Premium Notion/Raycast-style Vector Hats matching avatar head geometry (cy=120 r=58).
Designs use soft depth shadows, seamless head wrapping, and zero harsh sticker outlines.
-->
<script lang="ts">
import type { AvatarHat } from "../core/types.js";

const { hat = "none" }: { hat: AvatarHat } = $props();
</script>

{#if hat === "crown"}
  <!-- 👑 Royal Gold Crown -->
  <g id="hat-crown" style="transform: translate(0, 4px);">
    <!-- Base shadow -->
    <path d="M 66 68 Q 100 74 134 68 Z" fill="rgba(0,0,0,0.15)" />
    <!-- Crown Body -->
    <path
      d="M 66 66 L 54 36 L 78 50 L 100 24 L 122 50 L 146 36 L 134 66 Q 100 72 66 66 Z"
      fill="#F59E0B"
      stroke="#B45309"
      stroke-width="1.5"
      stroke-linejoin="round"
    />
    <!-- Velvet Base Rim -->
    <path
      d="M 66 65 Q 100 71 134 65 L 132 68 Q 100 74 68 68 Z"
      fill="#92400E"
    />
    <!-- Jewels -->
    <circle cx="54" cy="36" r="3.5" fill="#EF4444" stroke="#7F1D1D" stroke-width="1" />
    <circle cx="100" cy="24" r="4" fill="#3B82F6" stroke="#1E3A8A" stroke-width="1" />
    <circle cx="146" cy="36" r="3.5" fill="#10B981" stroke="#064E3B" stroke-width="1" />
  </g>

{:else if hat === "graduation"}
  <!-- 🎓 Mortarboard Graduation Cap -->
  <g id="hat-graduation" style="transform: translate(0, 6px);">
    <!-- Skull Cap Base -->
    <path
      d="M 74 54 L 74 68 C 74 74, 126 74, 126 68 L 126 54 Z"
      fill="#1E293B"
      stroke="#0F172A"
      stroke-width="1"
    />
    <!-- Diamond Board -->
    <polygon
      points="100,34 148,46 100,58 52,46"
      fill="#0F172A"
      stroke="#334155"
      stroke-width="1.5"
    />
    <polygon
      points="100,36 144,46 100,56 56,46"
      fill="#1E293B"
    />
    <!-- Tassel -->
    <path
      d="M 100 46 C 84 50, 74 56, 72 74"
      fill="none"
      stroke="#FBBF24"
      stroke-width="2.5"
      stroke-linecap="round"
    />
    <circle cx="72" cy="74" r="3.5" fill="#FBBF24" stroke="#B45309" stroke-width="1" />
  </g>

{:else if hat === "firefighter"}
  <!-- 👨‍🚒 Modern Fire Safety Helmet -->
  <g id="hat-firefighter" style="transform: translate(0, 6px);">
    <!-- Rear Flare Shade -->
    <path
      d="M 48 70 Q 100 60 152 70 Q 100 76 48 70 Z"
      fill="#7F1D1D"
    />
    <!-- Main Shell -->
    <path
      d="M 68 64 C 68 36, 132 36, 132 64 Q 100 70 68 64 Z"
      fill="#EF4444"
      stroke="#991B1B"
      stroke-width="1.5"
    />
    <!-- Front Brim Visor -->
    <path
      d="M 50 66 Q 100 58 150 66 Q 100 72 50 66 Z"
      fill="#DC2626"
      stroke="#7F1D1D"
      stroke-width="1"
    />
    <!-- Crest Badge -->
    <path
      d="M 93 48 L 107 48 L 105 62 L 95 62 Z"
      fill="#FBBF24"
      stroke="#B45309"
      stroke-width="1"
    />
    <circle cx="100" cy="54" r="2.5" fill="#DC2626" />
  </g>

{:else if hat === "wizard"}
  <!-- 🧙‍♂️ Arcane Star Wizard Hat -->
  <g id="hat-wizard" style="transform: translate(0, 0px);">
    <!-- Brim -->
    <path
      d="M 48 72 Q 100 62 152 72 Q 100 78 48 72 Z"
      fill="#4338CA"
      stroke="#312E81"
      stroke-width="1.5"
    />
    <!-- Cone -->
    <path
      d="M 66 70 Q 84 38 100 16 Q 112 36 134 70 Q 100 64 66 70 Z"
      fill="#4F46E5"
      stroke="#3730A3"
      stroke-width="1.5"
    />
    <!-- Belt Band -->
    <path d="M 69 64 Q 100 58 131 64 L 130 68 Q 100 62 70 68 Z" fill="#FBBF24" />
    <!-- Stars -->
    <polygon points="98,34 100,38 104,38 101,40 102,44 98,42 94,44 95,40 92,38 96,38" fill="#FDE047" />
    <polygon points="82,48 84,52 88,52 85,54 86,58 82,56 78,58 79,54 76,52 80,52" fill="#FDE047" />
  </g>

{:else if hat === "detective"}
  <!-- 🕵️‍♂️ Inspector Tweed Cap -->
  <g id="hat-detective" style="transform: translate(0, 4px);">
    <!-- Dome Base -->
    <path
      d="M 62 66 C 62 40, 138 40, 138 66 Q 100 70 62 66 Z"
      fill="#92400E"
      stroke="#78350F"
      stroke-width="1.5"
    />
    <!-- Front Brim Visor -->
    <path
      d="M 50 66 Q 100 56 150 66 Q 100 73 50 66 Z"
      fill="#B45309"
      stroke="#78350F"
      stroke-width="1.5"
    />
    <!-- Ribbon Bow -->
    <path
      d="M 94 44 C 96 38, 104 38, 106 44 L 100 46 Z"
      fill="#FEF3C7"
      stroke="#78350F"
      stroke-width="1"
    />
  </g>

{:else if hat === "chef"}
  <!-- 👨‍🍳 Executive Culinary Toque -->
  <g id="hat-chef" style="transform: translate(0, -2px);">
    <!-- Puffed Folds -->
    <path
      d="M 74 58 C 56 42, 60 22, 82 20 C 90 10, 110 10, 118 20 C 140 22, 144 42, 126 58 Z"
      fill="#F8FAFC"
      stroke="#CBD5E1"
      stroke-width="1.5"
    />
    <!-- Crease folds -->
    <path d="M 88 22 C 86 36, 84 50, 84 56" fill="none" stroke="#E2E8F0" stroke-width="2" />
    <path d="M 100 16 C 100 32, 100 48, 100 56" fill="none" stroke="#E2E8F0" stroke-width="2" />
    <path d="M 112 22 C 114 36, 116 50, 116 56" fill="none" stroke="#E2E8F0" stroke-width="2" />
    <!-- Band -->
    <path
      d="M 76 56 L 124 56 L 122 68 Q 100 72 78 68 Z"
      fill="#E2E8F0"
      stroke="#94A3B8"
      stroke-width="1.5"
    />
  </g>

{:else if hat === "astronaut"}
  <!-- 🧑‍🚀 Space Orbital Helmet -->
  <g id="hat-astronaut" style="transform: translate(0, 2px);">
    <!-- Shell -->
    <path
      d="M 66 66 C 66 26, 134 26, 134 66 Q 100 70 66 66 Z"
      fill="#F1F5F9"
      stroke="#CBD5E1"
      stroke-width="2"
    />
    <!-- Glass Visor -->
    <path
      d="M 76 44 C 76 32, 124 32, 124 44 C 124 60, 76 60, 76 44 Z"
      fill="#0284C7"
      stroke="#0369A1"
      stroke-width="1.5"
    />
    <!-- Reflection Sheen -->
    <path
      d="M 82 40 Q 96 34 116 38"
      fill="none"
      stroke="#7DD3FC"
      stroke-width="2.5"
      stroke-linecap="round"
    />
  </g>

{:else if hat === "headset"}
  <!-- 🎧 Studio Pro AI Headphones -->
  <g id="hat-headset" style="transform: translate(0, 4px);">
    <!-- Arch Band -->
    <path
      d="M 52 105 C 52 46, 148 46, 148 105"
      fill="none"
      stroke="#1E293B"
      stroke-width="5"
      stroke-linecap="round"
    />
    <!-- Ear Cups -->
    <rect x="40" y="96" width="18" height="26" rx="8" fill="#3B82F6" stroke="#1D4ED8" stroke-width="1.5" />
    <rect x="142" y="96" width="18" height="26" rx="8" fill="#3B82F6" stroke="#1D4ED8" stroke-width="1.5" />
    <!-- Boom Mic -->
    <path d="M 46 116 Q 62 136 86 134" fill="none" stroke="#0F172A" stroke-width="2.5" stroke-linecap="round" />
    <circle cx="86" cy="134" r="3.5" fill="#EF4444" stroke="#991B1B" stroke-width="1" />
  </g>

{:else if hat === "party"}
  <!-- 🥳 Celebration Party Cone -->
  <g id="hat-party" style="transform: translate(0, 0px);">
    <path
      d="M 68 70 L 100 18 L 132 70 Q 100 76 68 70 Z"
      fill="#EC4899"
      stroke="#BE185D"
      stroke-width="1.5"
    />
    <!-- Stripes -->
    <path d="M 76 56 Q 100 52 124 56" fill="none" stroke="#FBBF24" stroke-width="3" />
    <path d="M 84 42 Q 100 38 116 42" fill="none" stroke="#3B82F6" stroke-width="3" />
    <!-- Fluffy Top Pom-Pom -->
    <circle cx="100" cy="16" r="5" fill="#FBBF24" stroke="#D97706" stroke-width="1" />
  </g>

{:else if hat === "pirate"}
  <!-- 🏴‍☠️ Captain Tricorn Hat -->
  <g id="hat-pirate" style="transform: translate(0, 4px);">
    <path
      d="M 52 68 Q 100 48 148 68 Q 100 74 52 68 Z"
      fill="#111827"
      stroke="#374151"
      stroke-width="1.5"
    />
    <path
      d="M 64 64 C 64 42, 136 42, 136 64 Z"
      fill="#1F2937"
    />
    <!-- Gold Trim -->
    <path d="M 54 67 Q 100 49 146 67" fill="none" stroke="#FBBF24" stroke-width="2" />
    <!-- Skull Badge -->
    <circle cx="100" cy="54" r="4" fill="#FFFFFF" />
    <circle cx="98.5" cy="53.5" r="1" fill="#111827" />
    <circle cx="101.5" cy="53.5" r="1" fill="#111827" />
  </g>

{:else if hat === "ninja"}
  <!-- 🥷 Shadow Ninja Headband -->
  <g id="hat-ninja" style="transform: translate(0, 6px);">
    <!-- Wrapped Band -->
    <path
      d="M 64 64 Q 100 60 136 64 L 134 72 Q 100 68 66 72 Z"
      fill="#18181B"
      stroke="#27272A"
      stroke-width="1"
    />
    <!-- Metallic Forehead Plate -->
    <rect x="86" y="63" width="28" height="10" rx="3" fill="#94A3B8" stroke="#475569" stroke-width="1" />
    <!-- Metal Rivets -->
    <circle cx="89" cy="68" r="1" fill="#1E293B" />
    <circle cx="111" cy="68" r="1" fill="#1E293B" />
    <!-- Floating Ribbon Tails -->
    <path d="M 134 68 Q 146 72 154 66 L 152 72 Q 144 76 134 72 Z" fill="#18181B" />
  </g>

{:else if hat === "tophat"}
  <!-- 🎩 Gentleman Top Hat -->
  <g id="hat-tophat" style="transform: translate(0, 2px);">
    <!-- Wide Brim -->
    <path
      d="M 54 68 Q 100 62 146 68 Q 100 74 54 68 Z"
      fill="#18181B"
      stroke="#27272A"
      stroke-width="1.5"
    />
    <!-- Tall Cylinder -->
    <path
      d="M 72 66 L 70 28 L 130 28 L 128 66 Z"
      fill="#27272A"
      stroke="#18181B"
      stroke-width="1.5"
    />
    <!-- Red Satin Ribbon -->
    <path d="M 71 58 L 129 58 L 128 66 L 72 66 Z" fill="#DC2626" />
  </g>

{:else if hat === "viking"}
  <!-- 🛡️ Nordic Horned Viking Helmet -->
  <g id="hat-viking" style="transform: translate(0, 4px);">
    <!-- Left Horn -->
    <path d="M 68 62 C 54 52, 42 32, 40 24 C 48 30, 62 44, 70 54 Z" fill="#F8FAFC" stroke="#CBD5E1" stroke-width="1.5" />
    <!-- Right Horn -->
    <path d="M 132 62 C 146 52, 158 32, 160 24 C 152 30, 138 44, 130 54 Z" fill="#F8FAFC" stroke="#CBD5E1" stroke-width="1.5" />
    <!-- Metal Cap -->
    <path
      d="M 66 64 C 66 40, 134 40, 134 64 Q 100 70 66 64 Z"
      fill="#64748B"
      stroke="#334155"
      stroke-width="1.5"
    />
    <path d="M 100 42 L 100 66" fill="none" stroke="#475569" stroke-width="3" />
  </g>

{:else if hat === "pilot"}
  <!-- Classic airline pilot cap -->
  <g id="hat-pilot" style="transform: translate(0, 4px);">
    <!-- Cap crown -->
    <path
      d="M 66 64 C 68 42, 84 32, 100 32 C 116 32, 132 42, 134 64 Q 100 70 66 64 Z"
      fill="#172554"
      stroke="#0F172A"
      stroke-width="1.5"
    />
    <!-- Gold band -->
    <path d="M 67 57 Q 100 50 133 57 L 133 64 Q 100 70 67 64 Z" fill="#F8FAFC" />
    <path d="M 68 58 Q 100 52 132 58 L 132 61 Q 100 56 68 61 Z" fill="#F59E0B" />
    <!-- Cap visor -->
    <path
      d="M 50 66 Q 100 56 150 66 Q 100 75 50 66 Z"
      fill="#0F172A"
      stroke="#020617"
      stroke-width="1.5"
    />
    <!-- Pilot wings badge -->
    <path
      d="M 91 43 L 96 47 L 100 42 L 104 47 L 109 43 L 106 52 L 94 52 Z"
      fill="#FBBF24"
      stroke="#B45309"
      stroke-width="1"
      stroke-linejoin="round"
    />
  </g>

{:else if hat === "propeller"}
  <!-- 🚁 Retro Propeller Beanie -->
  <g id="hat-propeller" style="transform: translate(0, 4px);">
    <!-- Beanie Segments -->
    <path d="M 66 66 C 66 44, 100 42, 100 66 Z" fill="#3B82F6" />
    <path d="M 100 66 C 100 42, 134 44, 134 66 Z" fill="#EF4444" />
    <!-- Brim -->
    <path d="M 64 65 Q 100 58 136 65 Q 100 70 64 65 Z" fill="#FBBF24" />
    <!-- Propeller Shaft -->
    <line x1="100" y1="42" x2="100" y2="34" stroke="#1E293B" stroke-width="2" />
    <!-- Propeller Blades -->
    <ellipse cx="90" cy="34" rx="10" ry="2" fill="#F8FAFC" stroke="#94A3B8" stroke-width="1" />
    <ellipse cx="110" cy="34" rx="10" ry="2" fill="#F8FAFC" stroke="#94A3B8" stroke-width="1" />
    <circle cx="100" cy="34" r="2.5" fill="#EF4444" />
  </g>

{:else if hat === "santa"}
  <!-- 🎅 Festive Holiday Santa Cap -->
  <g id="hat-santa" style="transform: translate(0, 2px);">
    <!-- Floppy Red Cone -->
    <path
      d="M 66 64 Q 100 38 134 64 Q 150 48 156 56 C 158 64, 142 72, 132 68 Z"
      fill="#DC2626"
      stroke="#991B1B"
      stroke-width="1.5"
    />
    <!-- White Fluffy Brim -->
    <path
      d="M 62 64 Q 100 56 138 64 C 142 70, 58 70, 62 64 Z"
      fill="#F8FAFC"
      stroke="#CBD5E1"
      stroke-width="1.5"
    />
    <!-- Pom-Pom Tail Ball -->
    <circle cx="156" cy="58" r="7" fill="#F8FAFC" stroke="#CBD5E1" stroke-width="1.5" />
  </g>
{/if}
