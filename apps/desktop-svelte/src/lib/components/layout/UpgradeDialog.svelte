<!--
@component UpgradeDialog
A premium modal dialog for upgrading to the Business Plan.
Triggered by the Zap icon in the sidebar. Matches Sentry's upgrade layout.
-->
<script lang="ts">
import { uiStore } from "$lib/stores";
import { Check, X } from "lucide-svelte";

function handleClose() {
  uiStore.setUpgradeDialog(false);
}

function handleBackdropClick(e: MouseEvent) {
  if (e.target === e.currentTarget) {
    handleClose();
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    handleClose();
  }
}
</script>

<svelte:window onkeydown={handleKeyDown} />

<div
  role="dialog"
  aria-modal="true"
  onclick={handleBackdropClick}
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[2px] transition-all duration-300"
>
  <div class="relative bg-[#211f26] border border-zinc-800/80 rounded-2xl w-full max-w-[800px] flex overflow-hidden shadow-[0_24px_50px_rgba(0,0,0,0.65)] select-none">
    
    <!-- Top-Right Geometric Slanted Corner Graphic -->
    <svg class="absolute top-0 right-0 h-32 w-64 text-[#d5cee8] opacity-95 overflow-visible z-0 pointer-events-none" viewBox="0 0 250 110" fill="currentColor">
      <path d="M70 0 L250 0 L250 110 L220 110 L220 85 L190 85 L190 55 L160 55 L160 25 L70 25 Z" />
    </svg>

    <!-- Bottom-Left Geometric Slanted Corner Graphic -->
    <svg class="absolute bottom-0 left-0 h-32 w-64 text-[#d5cee8] opacity-95 overflow-visible z-0 pointer-events-none" viewBox="0 0 250 110" fill="currentColor">
      <path d="M0 110 L0 0 L20 0 L20 30 L50 30 L50 65 L85 65 L85 90 L250 90 L250 110 Z" />
    </svg>

    <!-- Close Button Top Right (Outside Graphic area if possible, or overlayed) -->
    <button
      type="button"
      onclick={handleClose}
      class="absolute top-4 right-4 z-20 w-7 h-7 flex items-center justify-center rounded-md text-zinc-500 hover:text-zinc-200 hover:bg-zinc-800/40 transition-colors cursor-pointer"
      title="Close dialog"
    >
      <X size={15} strokeWidth={2.5} />
    </button>

    <!-- Left Column (Pitch & Action) -->
    <div class="w-[60%] flex flex-col justify-between p-10 pr-6 z-10 text-start">
      <div class="flex flex-col gap-4">
        <!-- Badge -->
        <span class="text-[9.5px] font-bold tracking-widest text-[#a199ff] uppercase">
          Next Stop - Business Plan
        </span>

        <!-- Headline -->
        <h2 class="text-[23px] font-extrabold text-zinc-100 leading-tight">
          Try the upgrade. Regret less.
        </h2>

        <!-- Description Paragraphs -->
        <div class="flex flex-col gap-3.5 text-zinc-300 text-[11.5px] leading-relaxed pr-2">
          <p>
            We added a bunch of new features that made Orbis a whole lot better. If we do say so ourselves.
          </p>
          <p>
            To get more out of Orbis and life in general, switch over to our new and improved Business Plan. You'll get transactions, attachments, access to Performance, and features that won't be released on our legacy plans.
          </p>
          <p>
            Confusing, we know. Basically, your Orbis organization is perfect just the way it is, but upgrade now if you want it to be even better.
          </p>
          <p class="font-semibold text-zinc-200">
            Upgrade to our Business Plan today.
          </p>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-3 mt-6">
        <button
          type="button"
          class="px-5 py-2.5 text-[11.5px] font-bold rounded-lg bg-[#6366f1] hover:bg-[#5356e0] active:scale-[0.98] text-white cursor-pointer transition-all duration-150 shadow-md"
        >
          Upgrade now
        </button>
        <button
          type="button"
          onclick={handleClose}
          class="px-5 py-2.5 text-[11.5px] font-bold rounded-lg bg-[#272431]/80 border border-zinc-700/60 hover:bg-[#2e2a3b] hover:text-zinc-200 active:scale-[0.98] text-zinc-300 cursor-pointer transition-all duration-150"
        >
          Maybe Later
        </button>
      </div>
    </div>

    <!-- Right Column (Features & Current Plan) -->
    <div class="w-[40%] flex flex-col justify-between p-10 pl-8 border-l border-zinc-800/80 z-10 text-start bg-[#1d1b22]/30">
      <!-- Features List -->
      <div class="flex flex-col">
        <h3 class="text-[10px] font-bold text-zinc-400 uppercase tracking-widest mb-4.5">
          Features Include
        </h3>
        
        <ul class="flex flex-col gap-3">
          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-zinc-200">
            <div class="w-4 h-4 rounded-full border border-zinc-700 flex items-center justify-center text-zinc-400 shrink-0">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>Event Volume Controls</span>
          </li>
          
          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-zinc-200">
            <div class="w-4 h-4 rounded-full border border-zinc-700 flex items-center justify-center text-zinc-400 shrink-0">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>Unlimited Custom Dashboards</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-zinc-200">
            <div class="w-4 h-4 rounded-full border border-zinc-700 flex items-center justify-center text-zinc-400 shrink-0">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>SAML2 Sign In</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-zinc-200">
            <div class="w-4 h-4 rounded-full border border-zinc-700 flex items-center justify-center text-zinc-400 shrink-0">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>Advanced Integrations</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11px] font-semibold text-zinc-200">
            <div class="w-4 h-4 rounded-full border border-zinc-700 flex items-center justify-center text-zinc-400 shrink-0">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
              </svg>
            </div>
            <span>Extended Data Retention</span>
          </li>

          <li class="flex items-center gap-2.5 text-[11.5px] text-zinc-400 italic font-medium">
            <div class="w-4 h-4 flex items-center justify-center text-zinc-400 shrink-0">
              <Check size={11} strokeWidth={3} />
            </div>
            <span>And more...</span>
          </li>
        </ul>
      </div>

      <!-- Current Plan Status -->
      <div class="mt-8 flex flex-col gap-0.5 select-none">
        <span class="text-[9px] font-bold text-zinc-500 uppercase tracking-widest">
          Current Plan
        </span>
        <span class="text-[15px] font-extrabold text-zinc-200">
          Developer
        </span>
        <a
          href="#compare-plans"
          class="text-[11px] font-semibold text-[#a199ff] hover:text-[#b0a9ff] hover:underline mt-1.5 transition-colors"
        >
          Learn more and compare plans
        </a>
      </div>
    </div>

  </div>
</div>
