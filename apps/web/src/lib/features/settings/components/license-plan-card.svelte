<script lang="ts">
import { Logo } from "@orbis/icons";

let {
  planCode,
  planLabel,
  validUntil: _validUntil = null,
}: {
  planCode: string;
  planLabel: string;
  validUntil?: string | null;
} = $props();

let pointerX = $state(0);
let pointerY = $state(0);
let isHovered = $state(false);

const planTheme = $derived.by(() => {
  const plan = planCode.toLowerCase().replace(/[_\s-]+/g, "");

  if (plan.includes("enterprise")) {
    return {
      beam: "bg-brand-pink/20",
      accent: "text-brand-pink",
      spotlight: "oklch(from var(--brand-pink) l c h / 0.28)",
    };
  }
  if (plan.includes("professional")) {
    return {
      beam: "bg-brand-purple/25",
      accent: "text-brand-purple",
      spotlight: "oklch(from var(--brand-purple) l c h / 0.3)",
    };
  }
  if (plan.includes("growth")) {
    return {
      beam: "bg-brand-lime/20",
      accent: "text-brand-lime-dark",
      spotlight: "oklch(from var(--brand-lime) l c h / 0.28)",
    };
  }
  if (plan.includes("starter")) {
    return {
      beam: "bg-brand-indigo/20",
      accent: "text-brand-indigo",
      spotlight: "oklch(from var(--brand-indigo) l c h / 0.28)",
    };
  }
  if (plan.includes("trial") || plan.includes("free")) {
    return {
      beam: "bg-muted-foreground/15",
      accent: "text-muted-foreground",
      spotlight: "oklch(from var(--foreground) l c h / 0.1)",
    };
  }

  return {
    beam: "bg-primary/20",
    accent: "text-primary",
    spotlight: "oklch(from var(--primary) l c h / 0.28)",
  };
});

function handlePointerMove(event: PointerEvent & { currentTarget: HTMLDivElement }) {
  const rect = event.currentTarget.getBoundingClientRect();
  pointerX = event.clientX - rect.left;
  pointerY = event.clientY - rect.top;
  if (!isHovered) isHovered = true;
}

function handlePointerLeave() {
  isHovered = false;
}
</script>

<div
  onpointermove={handlePointerMove}
  onpointerleave={handlePointerLeave}
  class="relative flex min-h-[10.5rem] w-full flex-col justify-between overflow-hidden rounded-xl border border-border/70 bg-card p-4 text-start transition-colors duration-200 hover:border-border"
  role="region"
  aria-label={planLabel}
>
  <!-- Architectural vertical grid lines -->
  <div
    aria-hidden="true"
    class="pointer-events-none absolute inset-0 opacity-35"
    style="background-image: repeating-linear-gradient(to right, oklch(from var(--border) l c h / 0.45) 0 1px, transparent 1px 28px);"
  ></div>

  <!-- Ambient diagonal beam -->
  <div
    aria-hidden="true"
    class="pointer-events-none absolute -top-14 -end-10 h-36 w-52 -rotate-25 rounded-full blur-2xl opacity-60 {planTheme.beam}"
  ></div>

  <!-- Cursor-tracking radial spotlight -->
  <div
    aria-hidden="true"
    class="pointer-events-none absolute inset-0 transition-opacity duration-300 ease-out {isHovered
      ? 'opacity-100'
      : 'opacity-0'}"
    style="background: radial-gradient(220px circle at {pointerX}px {pointerY}px, {planTheme.spotlight}, transparent 72%);"
  ></div>

  <!-- Watermark emblem -->
  <div
    aria-hidden="true"
    class="pointer-events-none absolute -bottom-5 -end-4 opacity-[0.07] {planTheme.accent}"
  >
    <Logo size={108} />
  </div>

  <!-- Top Logo -->
  <div class="relative z-10 flex items-center">
    <div class="flex size-8 items-center justify-center rounded-md border border-border/60 bg-background/80 backdrop-blur-sm">
      <Logo size={16} />
    </div>
  </div>

  <!-- Plan Name -->
  <div class="relative z-10 pt-6">
    <h3 class="truncate text-lg font-bold tracking-tight text-foreground">
      {planLabel}
    </h3>
  </div>
</div>
