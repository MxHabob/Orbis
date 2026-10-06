<script lang="ts">
  import { isDesktopApp } from "$lib/api";
  import * as m from "$lib/paraglide/messages";
  import { localizeHref } from "$lib/paraglide/runtime";
  import { authStore, themeStore, uiStore, workflowInbox } from "$lib/stores";
  import { Bell, Maximize, Minus, Moon, Restore, Search, Sun, XMark } from "@orbis/icons";
  import { Button } from "@orbis/ui/button";
  import { Kbd } from "@orbis/ui/kbd";
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
  <div class="flex items-center gap-1.5 shrink-0" data-tauri-drag-region>
    {#if authStore.isAuthenticated}
      <Button
        variant="outline"
        size="default"
        onclick={() => uiStore.setSearch(true)}
        class="shrink-0"
        title={m.common_header_search_title()}
        aria-label={m.shared_ui_search()}
      >
        <Search size={14} class="text-muted-foreground" />
        <span>{m.common_header_search()}</span>
        <Kbd>{m.common_header_search_shortcut()}</Kbd>
      </Button>

      <Button
        variant="outline"
        size="default"
        class="shrink-0"
        title={m.shared_ui_ask_orbis()}
        aria-label={m.shared_ui_ask_orbis()}
        onclick={() => uiStore.toggleAskOrbis()}
        aria-pressed={uiStore.showAskOrbis}
      >
        <svg
          width="14"
          height="14"
          class="text-muted-foreground shrink-0"
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
        <span class="whitespace-nowrap">{m.shared_ui_ask_orbis()}</span>
        <Kbd>Ctrl /</Kbd>
      </Button>

      <!-- Workflow Inbox AppNotification -->
      {#if workflowInbox.hasActions}
        <Button
          variant="outline"
          size="icon-sm"
          class="relative shrink-0"
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
          variant="outline"
          size="icon-sm"
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
      title={m.common_titlebar_theme_toggle()}
      aria-label={m.common_titlebar_theme_toggle()}
    >
      {#if themeStore.resolved === "dark"}
        <Sun size={14} class="text-muted-foreground" />
      {:else}
        <Moon size={14} class="text-muted-foreground" />
      {/if}
    </Button>

    <!-- Focus Mode Toggle -->
    {#if authStore.isAuthenticated}
      <Button
        variant={uiStore.focusMode ? "default" : "outline"}
        size="default"
        onclick={() => uiStore.toggleFocusMode()}
        title={m.common_titlebar_focus_mode_tooltip()}
        aria-label={m.common_titlebar_focus_mode_tooltip()}
        aria-pressed={uiStore.focusMode}
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
        <Button
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.minimize()}
          title={m.common_titlebar_minimize()}
          aria-label={m.shared_ui_minimize()}
        >
          <Minus size={13} class="stroke-[2px]" />
        </Button>

        <!-- Expand / Restore -->
        <Button
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.toggleMaximize()}
          title={isMaximized
            ? m.common_titlebar_restore()
            : m.common_titlebar_maximize()}
          aria-label={m.shared_ui_maximize_restore()}
        >
          {#if isMaximized}
            <Restore size={12} class="stroke-[1.6]" />
          {:else}
            <Maximize size={12} class="stroke-[1.6]" />
          {/if}
        </Button>

        <!-- Close -->
        <Button
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.close()}
          class="hover:bg-destructive hover:text-destructive-foreground"
          title={m.common_titlebar_close()}
          aria-label={m.shared_ui_close_window()}
        >
          <XMark size={13} class="stroke-[2px]" />
        </Button>
      </div>
    {/if}
  </div>
</div>
