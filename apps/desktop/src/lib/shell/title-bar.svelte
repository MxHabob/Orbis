<script lang="ts">
  import { isDesktopApp } from "$lib/api";
  import * as m from "$lib/paraglide/messages";
  import { localizeHref } from "$lib/paraglide/runtime";
  import { authStore, themeStore, uiStore, workflowInbox } from "$lib/stores";
  import { Bell, Eye, Maximize, Minus, Moon, Restore, Search, Sparkles, Sun, XMark } from "@orbis/icons";
  import { Button } from "@orbis/ui/button";
  import { Logo } from "@orbis/icons";
  import { onMount } from "svelte";

  import TitleBreadcrumbs from "./title-breadcrumbs.svelte";
  import WhatsNewPopover from "./whats-new-popover.svelte";

  // Window states
  let appWindow = $state<any>(null);
  let isMaximized = $state(false);
  let isMac = $state(false);
  let isOnline = $state(true);

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

    if (isDesktopApp) {
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
  class="h-13 w-full bg-card border-b border-border/60 select-none flex items-center justify-between px-3.5 text-xs z-50 shrink-0"
  data-tauri-drag-region
>
  <!-- Start Side: Spacing for Mac traffic lights OR Logo + Period Switcher + Breadcrumb -->
  <div
    class="flex items-center gap-2.5 min-w-0 max-w-[55%]"
    class:ps-[76px]={isMac}
    data-tauri-drag-region
  >
    {#if !isMac}
      <!-- App Logo -->
      <Logo class="w-6 h-6 text-foreground shrink-0" data-tauri-drag-region />
    {/if}

    <!-- Breadcrumb -->
    {#if authStore.isAuthenticated}
      <div
        class="w-px h-4 bg-border/60 mx-1 shrink-0"
        data-tauri-drag-region
      ></div>
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
        <kbd
          class="ml-1 rounded px-1 py-0.5 font-mono text-[9px] font-medium text-zinc-400"
          >{m.common_header_search_shortcut()}</kbd
        >
      </Button>

      <!-- Ask Orbis Button (Original Header Design) -->
      <Button
        variant="outline"
        size="sm"
        class="font-semibold text-xs text-foreground px-3 gap-2 shrink-0 {uiStore.showAskOrbis
          ? 'bg-zinc-150 dark:bg-zinc-850/80 border-zinc-300 dark:border-zinc-700/80 text-foreground font-bold'
          : ''}"
        title="Ask Orbis"
        aria-label="Ask Orbis"
        onclick={() => uiStore.toggleAskOrbis()}
      >
        <svg
          width="14"
          height="14"
          class="text-foreground shrink-0 stroke-[2px]"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path
            d="M12 3l8.5 14.7a1 1 0 0 1-.9 1.3H4.4a1 1 0 0 1-.9-1.3L12 3z"
          />
          <path d="M12 15a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
        </svg>
        <span class="whitespace-nowrap">Ask Orbis</span>
        <kbd
          class="ml-1 rounded px-1 py-0.5 font-mono text-[9px] font-medium text-zinc-400"
          >Ctrl /</kbd
        >
      </Button>

      <!-- Workflow Inbox AppNotification -->
      {#if workflowInbox.hasActions}
        <Button
          variant="ghost"
          size="icon-sm"
          class="relative rounded-[8px] shrink-0"
          title={m.common_titlebar_inbox()}
          onclick={() => {
            const orgId = authStore.orgId;
            if (orgId)
              window.location.href = localizeHref(
                `/${orgId}/settings/approvals`,
              );
          }}
        >
          <Bell size={14} class="stroke-[2px]" />
          <span
            class="absolute -top-0.5 -inset-e-0.5 size-4 bg-destructive text-destructive-foreground text-[10px] rounded-full flex items-center justify-center font-semibold"
          >
            {workflowInbox.totalCount}
          </span>
        </Button>
      {:else}
        <Button
          variant="ghost"
          size="icon-sm"
          class="rounded-[8px] shrink-0"
          title={m.common_titlebar_inbox()}
          onclick={() => {
            const orgId = authStore.orgId;
            if (orgId)
              window.location.href = localizeHref(
                `/${orgId}/settings/approvals`,
              );
          }}
        >
          <Bell size={14} class="stroke-[2px] text-muted-foreground" />
        </Button>
      {/if}

      <!-- What's New AppNotification (Original Header Design) -->
      <WhatsNewPopover />
    {/if}

    <!-- Theme Toggle -->
    <Button
      variant="outline"
      size="icon-sm"
      onclick={() => themeStore.toggle()}
      class="rounded-[8px] shrink-0 cursor-pointer text-muted-foreground hover:text-foreground"
      title={m.common_titlebar_theme_toggle()}
      aria-label="Toggle Theme"
    >
      {#if themeStore.resolved === "dark"}
        <Sun size={14} class="stroke-[2px] text-amber-400" />
      {:else}
        <Moon size={14} class="stroke-[2px] text-indigo-500" />
      {/if}
    </Button>

    <!-- Focus Mode Toggle -->
    {#if authStore.isAuthenticated}
      <Button
        variant={"secondary"}
        size="sm"
        onclick={() => uiStore.toggleFocusMode()}
        class="font-semibold text-xs rounded-[8px] px-3 gap-2 shrink-0 {uiStore.focusMode
          ? 'bg-primary text-primary-foreground border-primary hover:bg-primary/95 hover:text-primary-foreground'
          : 'bg-muted/40 text-foreground border-border/60 hover:bg-muted/70'}"
        title={m.common_titlebar_focus_mode_tooltip()}
      >
        <span class="hidden sm:inline">{m.common_titlebar_focus_mode()}</span>
      </Button>
    {/if}

    <!-- Windows Window Controls (Min, Max, Close) -->
    {#if !isMac && appWindow}
      <div
        class="flex items-center border-s border-border/40 ms-1.5 ps-1.5 h-6 gap-0.5"
      >
        <!-- Minimize -->
        <button
          type="button"
          onclick={() => appWindow.minimize()}
          class="h-7 w-7 inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/80 transition-colors cursor-pointer"
          title={m.common_titlebar_minimize()}
          aria-label="Minimize"
        >
          <Minus size={13} class="stroke-[2px]" />
        </button>

        <!-- Expand / Restore -->
        <button
          type="button"
          onclick={() => appWindow.toggleMaximize()}
          class="h-7 w-7 inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/80 transition-colors cursor-pointer"
          title={isMaximized
            ? m.common_titlebar_restore()
            : m.common_titlebar_maximize()}
          aria-label="Maximize / Restore"
        >
          {#if isMaximized}
            <Restore size={12} class="stroke-[1.6]" />
          {:else}
            <Maximize size={12} class="stroke-[1.6]" />
          {/if}
        </button>

        <!-- Close -->
        <button
          type="button"
          onclick={() => appWindow.close()}
          class="h-7 w-7 inline-flex items-center justify-center rounded-md text-muted-foreground hover:bg-destructive hover:text-destructive-foreground transition-colors cursor-pointer"
          title={m.common_titlebar_close()}
          aria-label="Close"
        >
          <XMark size={13} class="stroke-[2px]" />
        </button>
      </div>
    {/if}
  </div>
</div>
