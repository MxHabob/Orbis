<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { isDesktopApp } from "$lib/core/ipc";
import {
  academicContext,
  authStore,
  uiStore,
  themeStore,
  workflowInbox,
} from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Logo } from "@orbis/ui/logo";
import {
  Minimize2,
  Square,
  Copy,
  X,
  GraduationCap,
  Sun,
  Moon,
  Search,
  Bell,
} from "@lucide/svelte";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";

import WhatsNewPopover from "./whats-new-popover.svelte";
import TitleBreadcrumbs from "./title-breadcrumbs.svelte";

// Window states
let appWindow = $state<any>(null);
let isMaximized = $state(false);
let isMac = $state(false);
let isOnline = $state(true);

// Academic period — reads from unified academicContext (populated by bootstrap)
const periods = $derived(academicContext.allPeriods);
const currentPeriodName = $derived.by(() => {
  const active = periods.find(
    (c) => String(c.id) === academicContext.effectivePeriodId,
  );
  if (active) {
    return active.name;
  }
  return m.titlebar_period();
});

// Phase label for the phase badge
const phaseLabel = $derived.by(() => {
  const phase = academicContext.periodPhase;
  if (!phase) return "";
  switch (phase) {
    case "pre_start":
      return m.common_phase_pre_start
        ? m.common_phase_pre_start()
        : "Pre-Start";
    case "active":
      return m.common_phase_active ? m.common_phase_active() : "Active";
    case "exams":
      return m.common_phase_exams ? m.common_phase_exams() : "Exams";
    case "grade_entry":
      return m.common_phase_grade_entry
        ? m.common_phase_grade_entry()
        : "Grade Entry";
    case "closed":
      return m.common_phase_closed ? m.common_phase_closed() : "Closed";
    default:
      return "";
  }
});

const phaseBadgeVariant = $derived.by(() => {
  const phase = academicContext.periodPhase;
  switch (phase) {
    case "active":
      return "success" as const;
    case "grade_entry":
      return "brand" as const;
    case "exams":
      return "secondary" as const;
    case "closed":
      return "outline" as const;
    default:
      return "ghost" as const;
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

// No $effect for loading periods — academicContext is populated by bootstrap.
// Periods are reactive via $derived above.
// No selectPeriod function — the title bar is now a read-only display.
// Period switching is done via the sidebar YearSwitcher for year-level,
// and inside academic pages (e.g. Control) for period-level.
</script>

<div 
  class="h-13 w-full bg-card border-b border-border/60  select-none flex items-center justify-between px-3.5 text-xs z-50 shrink-0"
  data-tauri-drag-region
>
  <!-- Start Side: Spacing for Mac traffic lights OR Logo + Period Switcher + Breadcrumb -->
  <div class="flex items-center gap-2.5 min-w-0 max-w-[55%]" class:ps-[76px]={isMac} data-tauri-drag-region>
    {#if !isMac}
      <!-- App Logo -->
      <Logo class="w-6 h-6 text-foreground shrink-0" data-tauri-drag-region />
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
        title={m.common_header_search_title()}
        aria-label="Search"
      >
        <Search size={14} class="text-muted-foreground stroke-[2px]" />
        <span class="whitespace-nowrap">{m.common_header_search()}</span>
        <kbd class="ml-1 rounded px-1 py-0.5 font-mono text-[9px] font-medium text-zinc-400">{m.common_header_search_shortcut()}</kbd>
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
          title={m.common_titlebar_inbox ? m.common_titlebar_inbox() : "Inbox"}
          onclick={() => {
            const orgId = authStore.orgId;
            if (orgId) window.location.href = localizeHref(`/${orgId}/settings/approvals`);
          }}
        >
          <Bell size={14} class="stroke-[2px]" />
          <span class="absolute -top-0.5 -inset-e-0.5 size-4 bg-destructive text-destructive-foreground text-[10px] rounded-full flex items-center justify-center font-semibold">
            {workflowInbox.totalCount}
          </span>
        </Button>
      {:else}
        <Button
          variant="ghost"
          size="icon-sm"
          class="rounded-[8px] shrink-0"
          title={m.common_titlebar_inbox ? m.common_titlebar_inbox() : "Inbox"}
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
      title={m.common_titlebar_theme_toggle()}
    >
      {#if themeStore.resolved === "dark"}
        <Sun size={14} class="stroke-[2px]" />
      {:else}
        <Moon size={14} class="stroke-[2px]" />
      {/if}
    </Button>

    <!-- Focus Mode Toggle -->
    {#if authStore.isAuthenticated}
      <Button 
        variant={"secondary"}
        size="sm"
        onclick={() => uiStore.toggleFocusMode()} 
        class="font-semibold text-xs rounded-[8px] px-3 gap-2 shrink-0 {uiStore.focusMode ? 'bg-primary text-primary-foreground border-primary hover:bg-primary/95 hover:text-primary-foreground' : 'bg-muted/40 text-foreground border-border/60 hover:bg-muted/70'}"
        title={m.common_titlebar_focus_mode_tooltip()}
      >
        <span class="hidden sm:inline">{m.common_titlebar_focus_mode()}</span>
      </Button>
    {/if}

    <!-- Windows Window Controls (Min, Max, Close) -->
    {#if !isMac && appWindow}
      <div class="flex items-center border-s border-border/40 ms-1.5 ps-1.5 h-6">
        <!-- Minimize -->
        <Button 
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.minimize()} 
          class="text-muted-foreground hover:text-foreground hover:bg-muted"
          title={m.common_titlebar_minimize()}
        >
          <Minimize2 size={12} />
        </Button>

        <!-- Maximize / Restore -->
        <Button 
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.toggleMaximize()} 
          class="text-muted-foreground hover:text-foreground hover:bg-muted"
          title={isMaximized ? m.common_titlebar_restore() : m.common_titlebar_maximize()}
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
          title={m.common_titlebar_close()}
        >
          <X size={12} />
        </Button>
      </div>
    {/if}
  </div>
</div>

