<script lang="ts">
import { onMount } from "svelte";
import { isDesktopApp } from "$lib/core/ipc";
import {
  academicContext,
  authStore,
  uiStore,
  themeStore,
  workflowInbox,
} from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Minimize2,
  Square,
  Copy,
  X,
  GraduationCap,
  Wifi,
  WifiOff,
  ChevronDown,
  Sun,
  Moon,
  Search,
  Bell,
} from "lucide-svelte";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { toast } from "svelte-sonner";
import { page } from "$app/state";
import WhatsNewPopover from "./whats-new-popover.svelte";
import TitleBreadcrumbs from "./title-breadcrumbs.svelte";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui-svelte/dropdown-menu";

// Window states
let appWindow = $state<any>(null);
let isMaximized = $state(false);
let isMac = $state(false);
let isOnline = $state(true);

// Academic cycle — reads from unified academicContext (populated by bootstrap)
const cycles = $derived(academicContext.allCycles);
const currentCycleName = $derived.by(() => {
  const active = cycles.find(
    (c) => String(c.id) === academicContext.effectiveCycleId,
  );
  if (active) {
    return active.name_ar && getLocale() === "ar"
      ? active.name_ar
      : active.name;
  }
  return m.titlebar_cycle();
});

// Phase label for the phase badge
const phaseLabel = $derived.by(() => {
  const phase = academicContext.cyclePhase;
  if (!phase) return "";
  switch (phase) {
    case "pre_start": return m.phase_pre_start ? m.phase_pre_start() : "Pre-Start";
    case "active": return m.phase_active ? m.phase_active() : "Active";
    case "exams": return m.phase_exams ? m.phase_exams() : "Exams";
    case "grade_entry": return m.phase_grade_entry ? m.phase_grade_entry() : "Grade Entry";
    case "closed": return m.phase_closed ? m.phase_closed() : "Closed";
    default: return "";
  }
});

const phaseBadgeVariant = $derived.by(() => {
  const phase = academicContext.cyclePhase;
  switch (phase) {
    case "active": return "lime" as const;
    case "grade_entry": return "brand" as const;
    case "exams": return "secondary" as const;
    case "closed": return "outline" as const;
    default: return "ghost" as const;
  }
});

onMount(() => {
  isOnline = navigator.onLine;
  const setOnline = () => {
    isOnline = true;
  };
  const setOffline = () => {
    isOnline = false;
  };

  window.addEventListener("online", setOnline);
  window.addEventListener("offline", setOffline);

  isMac = navigator.userAgent.toLowerCase().includes("mac");

  let unlistenResize: any;
  let isMounted = true;

  if (isDesktopApp()) {
    (async () => {
      try {
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        const win = getCurrentWindow();
        appWindow = win;

        const checkMaximized = async () => {
          isMaximized = await win.isMaximized();
        };
        await checkMaximized();

        const unlisten = await win.onResized(() => {
          checkMaximized();
        });

        if (!isMounted) {
          unlisten();
        } else {
          unlistenResize = unlisten;
        }
      } catch (e) {
        console.error("Failed to initialize Tauri window in TitleBar:", e);
      }
    })();
  }

  return () => {
    isMounted = false;
    window.removeEventListener("online", setOnline);
    window.removeEventListener("offline", setOffline);
    if (unlistenResize) {
      unlistenResize();
    }
  };
});

// No $effect for loading cycles — academicContext is populated by bootstrap.
// Cycles are reactive via $derived above.

function selectCycle(cycleId: string) {
  academicContext.selectedCycleId = cycleId;
  const active = cycles.find((c) => String(c.id) === cycleId);
  if (active) {
    const name =
      active.name_ar && getLocale() === "ar" ? active.name_ar : active.name;
    toast.success(m.titlebar_cycle_selected({ name }));
  }
}
</script>

<div 
  class="h-13 w-full bg-card border-b border-border/60  select-none flex items-center justify-between px-3.5 text-xs z-50 shrink-0"
  data-tauri-drag-region
>
  <!-- Start Side: Spacing for Mac traffic lights OR Logo + Cycle Switcher + Breadcrumb -->
  <div class="flex items-center gap-2.5 min-w-0 max-w-[55%]" class:ps-[76px]={isMac} data-tauri-drag-region>
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
      <div class="relative flex items-center z-50 shrink-0 gap-1.5">
        <!-- Cycle Switcher -->
        <DropdownMenu>
          <DropdownMenuTrigger class="flex items-center gap-1 {academicContext.isReadOnly ? 'bg-muted/60' : 'bg-muted/40'} hover:bg-muted/70 text-[11px] font-semibold text-foreground px-2.5 py-1 rounded-lg border border-border/60 transition-colors duration-150 cursor-pointer">
            <GraduationCap size={13} class="text-primary" />
            <span>{currentCycleName}</span>
            {#if academicContext.isReadOnly}
              <span class="text-[10px] opacity-60">🔒</span>
            {/if}
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
                {#if academicContext.effectiveCycleId === String(cycle.id)}
                  <span class="w-1.5 h-1.5 rounded-full bg-primary"></span>
                {/if}
              </DropdownMenuItem>
            {/each}
          </DropdownMenuContent>
        </DropdownMenu>

        <!-- Phase Badge -->
        {#if phaseLabel}
          <Badge variant={phaseBadgeVariant} class="text-[10px] px-1.5 py-0 h-5">
            {phaseLabel}
          </Badge>
        {/if}
      </div>
    {/if}

    <!-- Breadcrumb -->
    {#if authStore.isAuthenticated}
      <div class="w-px h-4 bg-border/60 mx-1 shrink-0" data-tauri-drag-region></div>
      <TitleBreadcrumbs />
    {/if}
  </div>

  <!-- Center Side: Draggable spacer -->
  <div class="flex-1 h-full min-w-0" data-tauri-drag-region></div>

  <!-- End Side: Header Buttons, Sync Indicator, Theme, Focus Mode, Window Controls -->
  <div class="flex items-center gap-2 shrink-0" data-tauri-drag-region>
    {#if authStore.isAuthenticated}

      <!-- Search Button (Original Header Design) -->
      <Button
        variant="outline"
        size="sm"
        onclick={() => uiStore.setSearch(true)}
        class="font-semibold text-xs text-foreground rounded-[8px] px-3 gap-2 shrink-0"
        title={m.header_search_title()}
        aria-label="Search"
      >
        <Search size={14} class="text-muted-foreground stroke-[2px]" />
        <span class="whitespace-nowrap">{m.header_search()}</span>
        <kbd class="ml-1 rounded px-1 py-0.5 font-mono text-[9px] font-medium text-zinc-400">{m.header_search_shortcut()}</kbd>
      </Button>

      <!-- Ask Orbis Button (Original Header Design) -->
      <Button
        variant="outline"
        size="sm"
        class="font-semibold text-xs text-foreground px-3 gap-2 shrink-0 {uiStore.showAskOrbis ? 'bg-zinc-150 dark:bg-zinc-850/80 border-zinc-300 dark:border-zinc-700/80 text-foreground font-bold' : ''}"
        title="Ask Orbis"
        aria-label="Ask Orbis"
        onclick={() => uiStore.toggleAskOrbis()}
      >
        <svg width="14" height="14" class="text-foreground shrink-0 stroke-[2px]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 3l8.5 14.7a1 1 0 0 1-.9 1.3H4.4a1 1 0 0 1-.9-1.3L12 3z" />
          <path d="M12 15a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
        </svg>
        <span class="whitespace-nowrap">Ask Orbis</span>
        <kbd class="ml-1 rounded px-1 py-0.5 font-mono text-[9px] font-medium text-zinc-400">Ctrl /</kbd>
      </Button>

      <!-- Workflow Inbox Bell -->
      {#if workflowInbox.hasActions}
        <Button
          variant="ghost"
          size="icon-sm"
          class="relative rounded-[8px] shrink-0"
          title={m.titlebar_inbox ? m.titlebar_inbox() : "Inbox"}
          onclick={() => {
            const orgId = authStore.orgId;
            if (orgId) window.location.href = localizeHref(`/${orgId}/settings/approvals`);
          }}
        >
          <Bell size={14} class="stroke-[2px]" />
          <span class="absolute -top-0.5 -end-0.5 size-4 bg-destructive text-destructive-foreground text-[10px] rounded-full flex items-center justify-center font-semibold">
            {workflowInbox.totalCount}
          </span>
        </Button>
      {:else}
        <Button
          variant="ghost"
          size="icon-sm"
          class="rounded-[8px] shrink-0"
          title={m.titlebar_inbox ? m.titlebar_inbox() : "Inbox"}
          onclick={() => {
            const orgId = authStore.orgId;
            if (orgId) window.location.href = localizeHref(`/${orgId}/settings/approvals`);
          }}
        >
          <Bell size={14} class="stroke-[2px] text-muted-foreground" />
        </Button>
      {/if}

      <!-- What's New Megaphone (Original Header Design) -->
      <WhatsNewPopover />
    {/if}



    <!-- Theme Toggle -->
    <Button 
      variant="outline"
      size="icon-sm"
      onclick={() => themeStore.toggle()} 
      class="rounded-[8px] shrink-0"
      title={m.titlebar_theme_toggle()}
    >
      {#if themeStore.resolved === "dark"}
        <Sun size={14} class="stroke-[2px]" />
      {:else}
        <Moon size={14} class="stroke-[2px]" />
      {/if}
    </Button>

    <!-- Focus Mode Toggle -->
    <Button 
      variant={"secondary"}
      size="sm"
      onclick={() => uiStore.toggleFocusMode()} 
      class="font-semibold text-xs rounded-[8px] px-3 gap-2 shrink-0 {uiStore.focusMode ? 'bg-primary text-primary-foreground border-primary hover:bg-primary/95 hover:text-primary-foreground' : 'bg-muted/40 text-foreground border-border/60 hover:bg-muted/70'}"
      title={m.titlebar_focus_mode_tooltip()}
    >
      <span class="hidden sm:inline">{m.titlebar_focus_mode()}</span>
    </Button>

    <!-- Windows Window Controls (Min, Max, Close) -->
    {#if !isMac && appWindow}
      <div class="flex items-center border-s border-border/40 ms-1.5 ps-1.5 h-6">
        <!-- Minimize -->
        <Button 
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.minimize()} 
          class="text-muted-foreground hover:text-foreground hover:bg-muted"
          title={m.titlebar_minimize()}
        >
          <Minimize2 size={12} />
        </Button>

        <!-- Maximize / Restore -->
        <Button 
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.toggleMaximize()} 
          class="text-muted-foreground hover:text-foreground hover:bg-muted"
          title={isMaximized ? m.titlebar_restore() : m.titlebar_maximize()}
        >
          {#if isMaximized}
            <Copy size={11} class="rotate-180" />
          {:else}
            <Square size={11} />
          {/if}
        </Button>

        <!-- Close -->
        <Button 
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.close()} 
          class="text-muted-foreground hover:bg-destructive hover:text-destructive-foreground"
          title={m.titlebar_close()}
        >
          <X size={12} />
        </Button>
      </div>
    {/if}
  </div>
</div>


