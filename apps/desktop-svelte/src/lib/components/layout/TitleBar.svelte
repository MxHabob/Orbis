<script lang="ts">
import { onMount, onDestroy } from "svelte";
import { isDesktopApp } from "$lib/services/ipc";
import { academicStore, authStore, uiStore, themeStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import {
  Minimize2,
  Square,
  Copy,
  X,
  GraduationCap,
  Play,
  Pause,
  RotateCcw,
  Sparkles,
  Wifi,
  WifiOff,
  ChevronDown,
  Sun,
  Moon
} from "lucide-svelte";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui-svelte/dropdown-menu";
import { commands } from "$lib/services/ipc";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { toast } from "svelte-sonner";

// Window states
let appWindow = $state<any>(null);
let isMaximized = $state(false);
let isMac = $state(false);
let isOnline = $state(true);

// Academic cycles list
let cycles = $state<any[]>([]);
let currentCycleName = $derived.by(() => {
  const active = cycles.find(c => String(c.id) === academicStore.selectedCycleId);
  if (active) {
    return active.name_ar && getLocale() === "ar" ? active.name_ar : active.name;
  }
  return m.titlebar_cycle();
});

// Classroom Timer states
let timerActive = $state(false);
let timerPaused = $state(false);
let timerRemaining = $state(0);
let timerMax = $state(0);
let timerInterval = $state<any>(null);

let timerProgress = $derived(
  timerMax > 0 ? (timerRemaining / timerMax) * 100 : 0
);

let formattedTime = $derived.by(() => {
  const mins = Math.floor(timerRemaining / 60);
  const secs = timerRemaining % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
});

onMount(async () => {
  isOnline = navigator.onLine;
  const setOnline = () => { isOnline = true; };
  const setOffline = () => { isOnline = false; };
  
  window.addEventListener("online", setOnline);
  window.addEventListener("offline", setOffline);

  isMac = navigator.userAgent.toLowerCase().includes("mac");

  let unlistenResize: any;

  if (isDesktopApp()) {
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const win = getCurrentWindow();
      appWindow = win;
      
      const checkMaximized = async () => {
        isMaximized = await win.isMaximized();
      };
      await checkMaximized();
      
      unlistenResize = await win.onResized(() => {
        checkMaximized();
      });
    } catch (e) {
      console.error("Failed to initialize Tauri window in TitleBar:", e);
    }
  }

  return () => {
    window.removeEventListener("online", setOnline);
    window.removeEventListener("offline", setOffline);
    if (unlistenResize) {
      unlistenResize();
    }
  };
});

$effect(() => {
  if (authStore.isAuthenticated && authStore.orgId) {
    commands.academics.getCycles(authStore.orgId)
      .then(list => {
        cycles = list;
        if (!academicStore.selectedCycleId && list.length > 0) {
          const activeCycle = list.find((c: any) => c.is_current === 1) || list[0];
          if (activeCycle) {
            academicStore.selectedCycleId = String(activeCycle.id);
          }
        }
      })
      .catch(err => {
        console.error("Failed to load cycles in TitleBar:", err);
      });
  } else {
    cycles = [];
  }
});

function selectCycle(cycleId: string) {
  academicStore.selectedCycleId = cycleId;
  const active = cycles.find(c => String(c.id) === cycleId);
  if (active) {
    const name = active.name_ar && getLocale() === "ar" ? active.name_ar : active.name;
    toast.success(getLocale() === "ar" ? `تم تحديد الدورة: ${name}` : `Selected cycle: ${name}`);
  }
}

function startTimer(seconds: number) {
  if (timerInterval) clearInterval(timerInterval);
  timerRemaining = seconds;
  timerMax = seconds;
  timerActive = true;
  timerPaused = false;

  timerInterval = setInterval(() => {
    if (!timerPaused) {
      if (timerRemaining > 0) {
        timerRemaining -= 1;
      } else {
        completeTimer();
      }
    }
  }, 1000);
}

function completeTimer() {
  const maxMinutes = Math.round(timerMax / 60);
  const timeLabel = getLocale() === "ar" ? `${maxMinutes} دقيقة` : `${maxMinutes}m`;
  
  resetTimer();
  
  toast.success(m.titlebar_timer_finished(), {
    description: m.titlebar_timer_finished_desc({ time: timeLabel }),
    duration: 10000,
  });
  
  // Play alert beep sound
  try {
    const audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
    const oscillator = audioCtx.createOscillator();
    const gainNode = audioCtx.createGain();
    
    oscillator.type = "sine";
    oscillator.frequency.setValueAtTime(880, audioCtx.currentTime); // A5 note
    gainNode.gain.setValueAtTime(0.15, audioCtx.currentTime);
    
    oscillator.connect(gainNode);
    gainNode.connect(audioCtx.destination);
    
    oscillator.start();
    setTimeout(() => oscillator.stop(), 800);
  } catch (e) {
    console.error("Failed to play audio alert:", e);
  }
}

function togglePauseTimer() {
  timerPaused = !timerPaused;
}

function resetTimer() {
  if (timerInterval) clearInterval(timerInterval);
  timerInterval = null;
  timerRemaining = 0;
  timerMax = 0;
  timerActive = false;
  timerPaused = false;
}

onDestroy(() => {
  if (timerInterval) clearInterval(timerInterval);
});
</script>

<div 
  class="h-11 w-full bg-card border-b border-border/60 select-none flex items-center justify-between px-3 text-xs z-50 shrink-0"
  data-tauri-drag-region
>
  <!-- Left Side: Spacing for Mac traffic lights OR Logo + Cycle Switcher -->
  <div class="flex items-center gap-3 shrink-0" class:pl-[76px]={isMac} data-tauri-drag-region>
    {#if !isMac}
      <!-- App Logo icon -->
      <div class="w-5 h-5 flex items-center justify-center text-primary shrink-0" data-tauri-drag-region>
        <svg class="w-4.5 h-4.5 text-primary" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 3l8.5 14.7a1 1 0 0 1-.9 1.3H4.4a1 1 0 0 1-.9-1.3L12 3z" />
          <path d="M12 15a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
        </svg>
      </div>
    {/if}

    <!-- Cycle Switcher (only shown when authenticated) -->
    {#if authStore.isAuthenticated && cycles.length > 0}
      <div class="relative flex items-center z-50">
        <DropdownMenu>
          <DropdownMenuTrigger class="flex items-center gap-1 bg-muted/40 hover:bg-muted/70 text-[11px] font-semibold text-foreground px-2.5 py-1 rounded-lg border border-border/60 transition-colors duration-150 cursor-pointer">
            <GraduationCap size={13} class="text-primary" />
            <span>{currentCycleName}</span>
            <ChevronDown size={10} class="opacity-60" />
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start" class="w-56 bg-popover border border-border rounded-lg shadow-lg py-1 z-50">
            <div class="px-2.5 py-1.5 text-[10px] font-semibold text-muted-foreground border-b border-border/40 mb-1">
              {m.titlebar_cycle_select()}
            </div>
            {#each cycles as cycle}
              <DropdownMenuItem 
                onclick={() => selectCycle(String(cycle.id))}
                class="flex items-center justify-between text-xs px-2.5 py-1.5 cursor-pointer rounded-md hover:bg-accent hover:text-accent-foreground"
              >
                <span>{cycle.name_ar && getLocale() === "ar" ? cycle.name_ar : cycle.name}</span>
                {#if academicStore.selectedCycleId === String(cycle.id)}
                  <span class="w-1.5 h-1.5 rounded-full bg-primary"></span>
                {/if}
              </DropdownMenuItem>
            {/each}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    {/if}
  </div>

  <!-- Center Side: Classroom Timer & Sync Indicator -->
  <div class="flex items-center gap-4 max-w-xl flex-1 justify-center min-w-0" data-tauri-drag-region>
    <!-- Classroom Timer -->
    <div class="flex items-center gap-2 bg-muted/30 border border-border/50 rounded-lg px-2.5 py-1 relative overflow-hidden shrink-0">
      <!-- Background progress bar for timer -->
      {#if timerActive && timerMax > 0}
        <div class="absolute bottom-0 left-0 h-[3px] bg-primary/45 transition-all duration-1000" style="width: {timerProgress}%"></div>
      {/if}

      <div class="flex items-center gap-1.5 select-none text-[11px] font-medium text-muted-foreground">
        <span class="w-1.5 h-1.5 rounded-full {timerActive && !timerPaused ? 'bg-emerald-500' : timerPaused ? 'bg-amber-500' : 'bg-muted-foreground/30'}"></span>
        {#if timerActive}
          <span class="font-mono text-foreground font-bold tracking-wider">{formattedTime}</span>
        {:else}
          <span>{m.titlebar_timer()}</span>
        {/if}
      </div>

      <!-- Preset Buttons & Custom Selector -->
      {#if !timerActive}
        <div class="flex items-center gap-1">
          <button onclick={() => startTimer(60)} class="text-[10px] hover:bg-muted px-1.5 py-0.5 rounded cursor-pointer transition-colors text-muted-foreground hover:text-foreground">{m.titlebar_timer_preset({ minutes: "1" })}</button>
          <button onclick={() => startTimer(180)} class="text-[10px] hover:bg-muted px-1.5 py-0.5 rounded cursor-pointer transition-colors text-muted-foreground hover:text-foreground">{m.titlebar_timer_preset({ minutes: "3" })}</button>
          <button onclick={() => startTimer(300)} class="text-[10px] hover:bg-muted px-1.5 py-0.5 rounded cursor-pointer transition-colors text-muted-foreground hover:text-foreground">{m.titlebar_timer_preset({ minutes: "5" })}</button>
          <DropdownMenu>
            <DropdownMenuTrigger class="text-[10px] hover:bg-muted px-1.5 py-0.5 rounded cursor-pointer transition-colors text-muted-foreground hover:text-foreground flex items-center gap-0.5">
              <span>{m.titlebar_timer_custom()}</span>
              <ChevronDown size={8} />
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-32 bg-popover border border-border rounded-lg shadow-lg py-1 z-50">
              <DropdownMenuItem onclick={() => startTimer(600)} class="text-xs px-2.5 py-1.5 cursor-pointer rounded-md hover:bg-accent">{m.titlebar_timer_preset({ minutes: "10" })}</DropdownMenuItem>
              <DropdownMenuItem onclick={() => startTimer(900)} class="text-xs px-2.5 py-1.5 cursor-pointer rounded-md hover:bg-accent">{m.titlebar_timer_preset({ minutes: "15" })}</DropdownMenuItem>
              <DropdownMenuItem onclick={() => startTimer(1800)} class="text-xs px-2.5 py-1.5 cursor-pointer rounded-md hover:bg-accent">{m.titlebar_timer_preset({ minutes: "30" })}</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      {:else}
        <!-- Active Timer Controls -->
        <div class="flex items-center gap-1.5 ml-1">
          <button onclick={togglePauseTimer} class="text-[10px] hover:bg-muted p-1 rounded cursor-pointer text-muted-foreground hover:text-foreground flex items-center" title={timerPaused ? m.titlebar_timer_resume() : m.titlebar_timer_pause()}>
            {#if timerPaused}
              <Play size={10} fill="currentColor" />
            {:else}
              <Pause size={10} fill="currentColor" />
            {/if}
          </button>
          <button onclick={resetTimer} class="text-[10px] hover:bg-muted p-1 rounded cursor-pointer text-muted-foreground hover:text-foreground flex items-center" title={m.titlebar_timer_reset()}>
            <RotateCcw size={10} />
          </button>
        </div>
      {/if}
    </div>

    <!-- Sync and Network Status -->
    <div class="flex items-center gap-1.5 text-[10px] text-muted-foreground select-none shrink-0" data-tauri-drag-region>
      {#if isOnline}
        <Wifi size={12} class="text-emerald-500" />
        <span class="hidden sm:inline opacity-75">{m.titlebar_status_synced()}</span>
      {:else}
        <WifiOff size={12} class="text-amber-500 animate-pulse" />
        <span class="hidden sm:inline font-semibold text-amber-500">{m.titlebar_status_offline()}</span>
      {/if}
    </div>
  </div>

  <!-- Right Side: Focus Mode Toggle + Custom Windows Controls -->
  <div class="flex items-center gap-1.5 shrink-0" data-tauri-drag-region>
    <!-- Theme Toggle -->
    <button 
      onclick={() => themeStore.toggle()} 
      class="flex items-center justify-center w-8 h-8 rounded-lg bg-muted/40 border border-border/60 hover:bg-muted/70 text-foreground transition-all duration-200 cursor-pointer"
      title={m.titlebar_theme_toggle()}
    >
      {#if themeStore.resolved === "dark"}
        <Sun size={13} class="text-amber-500" />
      {:else}
        <Moon size={13} class="text-foreground" />
      {/if}
    </button>

    <!-- Focus Mode -->
    <button 
      onclick={() => uiStore.toggleFocusMode()} 
      class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-[11px] font-semibold border transition-all duration-200 cursor-pointer {uiStore.focusMode ? 'bg-primary text-primary-foreground border-primary' : 'bg-muted/40 text-foreground border-border/60 hover:bg-muted/70'}"
      title={m.titlebar_focus_mode_tooltip()}
    >
      <Sparkles size={12} class={uiStore.focusMode ? 'text-primary-foreground animate-pulse' : 'text-primary'} />
      <span>{m.titlebar_focus_mode()}</span>
    </button>

    <!-- Windows Window Controls (Min, Max, Close) -->
    {#if !isMac && appWindow}
      <div class="flex items-center border-s border-border/40 ms-1.5 ps-1.5 h-6">
        <!-- Minimize -->
        <button 
          onclick={() => appWindow.minimize()} 
          class="w-7 h-7 hover:bg-muted rounded-md flex items-center justify-center text-muted-foreground hover:text-foreground cursor-pointer transition-colors duration-150"
          title={m.titlebar_minimize()}
        >
          <Minimize2 size={12} />
        </button>

        <!-- Maximize / Restore -->
        <button 
          onclick={() => appWindow.toggleMaximize()} 
          class="w-7 h-7 hover:bg-muted rounded-md flex items-center justify-center text-muted-foreground hover:text-foreground cursor-pointer transition-colors duration-150"
          title={isMaximized ? m.titlebar_restore() : m.titlebar_maximize()}
        >
          {#if isMaximized}
            <Copy size={11} class="rotate-180" />
          {:else}
            <Square size={11} />
          {/if}
        </button>

        <!-- Close -->
        <button 
          onclick={() => appWindow.close()} 
          class="w-7 h-7 hover:bg-red-500 hover:text-white rounded-md flex items-center justify-center text-muted-foreground cursor-pointer transition-colors duration-150"
          title={m.titlebar_close()}
        >
          <X size={12} />
        </button>
      </div>
    {/if}
  </div>
</div>
