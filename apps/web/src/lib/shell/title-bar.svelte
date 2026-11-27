<script lang="ts">
import { isDesktopApp } from "$lib/api";
import { api } from "$lib/api/client";
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { authStore, themeStore, uiStore } from "$lib/stores";
import {
  AiCenter,
  ArrowsPointingOut,
  ChevronLeft,
  ChevronRight,
  Maximize,
  Minus,
  Moon,
  Refresh,
  Restore,
  Search,
  Sidebar,
  Sun,
  XMark,
} from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import { Kbd } from "@orbis/ui/kbd";
import { onMount } from "svelte";
import TitleBarMenu from "./title-bar-menu.svelte";
import TitleBarTabs from "./title-bar-tabs.svelte";
import NotificationPopover from "./notification-popover.svelte";
import WhatsNewPopover from "./whats-new-popover.svelte";
import { isStandaloneWindow, STANDALONE_WINDOW_PARAM } from "./window-mode";

const standaloneWindow = $derived(isStandaloneWindow(page.url));
const currentOrgId = $derived(page.params.orgId || authStore.orgId || "");

let appWindow = $state<any>(null);
let isMaximized = $state(false);
let isMac = $state(false);
let isOnline = $state(true);

async function openCurrentPageInWindow() {
  const url = new URL(window.location.href);
  url.searchParams.set(STANDALONE_WINDOW_PARAM, "1");
  const path = `${url.pathname}${url.search}${url.hash}`;
  try {
    await api.openStandaloneWindow(path);
  } catch (error) {
    console.error(
      "Failed to open the current page in a separate window:",
      error,
    );
    toast.error(m.common_titlebar_standalone_window_failed());
  }
}

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
  let unlistenTraySearch: any;
  let unlistenTrayAi: any;
  let isMounted = true;

  if (isDesktopApp) {
    (async () => {
      try {
        const [{ getCurrentWindow }, { listen }] = await Promise.all([
          import("@tauri-apps/api/window"),
          import("@tauri-apps/api/event"),
        ]);
        const win = getCurrentWindow();
        appWindow = win;

        const checkMaximized = async () => {
          isMaximized = await win.isMaximized();
        };
        await checkMaximized();

        const [unlistenRes, unlistenSearch, unlistenAi] = await Promise.all([
          win.onResized(() => {
            checkMaximized();
          }),
          listen("tray-open-search", () => {
            uiStore.setSearch(true);
          }),
          listen("tray-toggle-ai", () => {
            uiStore.toggleAskOrbis();
          }),
        ]);

        if (!isMounted) {
          unlistenRes();
          unlistenSearch();
          unlistenAi();
        } else {
          unlistenResize = unlistenRes;
          unlistenTraySearch = unlistenSearch;
          unlistenTrayAi = unlistenAi;
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
    if (unlistenResize) unlistenResize();
    if (unlistenTraySearch) unlistenTraySearch();
    if (unlistenTrayAi) unlistenTrayAi();
  };
});
</script>

<!-- Single-Row Title & Tab Bar -->
<div
  class="h-10 w-full bg-muted/70 dark:bg-card select-none flex items-center justify-between gap-1 ps-2 text-xs z-50 shrink-0 @container/titlebar"
  class:pe-2={isMac || !appWindow}
  data-tauri-drag-region
>
  <!-- Leading Controls: App Logo Menu + Sidebar Toggle + History Nav -->
  <div
    class="flex items-center gap-0.5 shrink-0"
    class:ps-[76px]={isMac}
    data-tauri-drag-region
  >
    <TitleBarMenu
      orgId={currentOrgId}
      onOpenNewWindow={openCurrentPageInWindow}
      onCloseApp={appWindow ? () => appWindow.close() : undefined}
    />

    {#if authStore.isAuthenticated && !standaloneWindow}
      <Button
        variant={uiStore.focusMode ? "secondary" : "ghost"}
        size="icon-sm"
        class="size-7 text-muted-foreground hover:text-foreground shrink-0"
        onclick={() => uiStore.toggleFocusMode()}
        title={m.common_titlebar_focus_mode_tooltip()}
        aria-label={m.common_titlebar_focus_mode_tooltip()}
        aria-pressed={!uiStore.focusMode}
      >
        <Sidebar size={15} />
      </Button>

      <div class="hidden sm:flex items-center gap-0.5 shrink-0 ms-0.5">
        <Button
          variant="ghost"
          size="icon-sm"
          class="size-7 text-muted-foreground hover:text-foreground"
          onclick={() => window.history.back()}
          title={m.titlebar_menu_back()}
          aria-label={m.titlebar_menu_back()}
        >
          <ChevronLeft size={14} class="rtl:rotate-180" />
        </Button>
        <Button
          variant="ghost"
          size="icon-sm"
          class="size-7 text-muted-foreground hover:text-foreground"
          onclick={() => window.history.forward()}
          title={m.titlebar_menu_forward()}
          aria-label={m.titlebar_menu_forward()}
        >
          <ChevronRight size={14} class="rtl:rotate-180" />
        </Button>
        <Button
          variant="ghost"
          size="icon-sm"
          class="size-7 text-muted-foreground hover:text-foreground"
          onclick={() => window.location.reload()}
          title={m.titlebar_menu_reload()}
          aria-label={m.titlebar_menu_reload()}
        >
          <Refresh size={14} />
        </Button>
      </div>
    {/if}
  </div>

  <!-- Open Tabs Strip + Drag Spacer -->
  {#if authStore.isAuthenticated && !standaloneWindow && currentOrgId}
    <TitleBarTabs orgId={currentOrgId} />
  {:else}
    <div class="flex-1 h-full min-w-0" data-tauri-drag-region></div>
  {/if}

  <!-- Trailing: Global Utility Actions + Flush Windows Caption Controls -->
  <div class="flex items-center h-full shrink-0" data-tauri-drag-region>
    <div class="flex items-center gap-1 px-1.5" data-tauri-drag-region>
      {#if !isOnline}
        <span
          class="px-2 py-0.5 rounded-full bg-warning/15 text-warning text-[11px] font-medium"
        >
          Offline
        </span>
      {/if}

      {#if authStore.isAuthenticated && !standaloneWindow}
        <Button
          variant="outline"
          size="sm"
          onclick={() => uiStore.setSearch(true)}
          class="h-7 px-2 gap-1.5 bg-background/80 border-border/60 text-muted-foreground hover:text-foreground hover:bg-background shrink-0"
          title={m.common_header_search_title()}
          aria-label={m.shared_ui_search()}
        >
          <Search size={13} />
          <span class="hidden @min-[64rem]/titlebar:inline text-xs">
            {m.common_header_search()}
          </span>
          <Kbd class="hidden @min-[76rem]/titlebar:inline-flex">
            {m.common_header_search_shortcut()}
          </Kbd>
        </Button>

        <Button
          variant={uiStore.showAskOrbis ? "secondary" : "outline"}
          size="sm"
          class="h-7 px-2 gap-1.5 border-border/60 shrink-0 {uiStore.showAskOrbis
            ? 'text-foreground'
            : 'bg-background/80 text-muted-foreground hover:text-foreground hover:bg-background'}"
          title={m.shared_ui_ask_orbis()}
          aria-label={m.shared_ui_ask_orbis()}
          onclick={() => uiStore.toggleAskOrbis()}
          aria-pressed={uiStore.showAskOrbis}
        >
          <AiCenter size={14} class="text-primary shrink-0" />
          <span class="hidden whitespace-nowrap @min-[64rem]/titlebar:inline text-xs">
            {m.shared_ui_ask_orbis()}
          </span>
          <Kbd class="hidden @min-[60rem]/titlebar:inline-flex">Ctrl /</Kbd>
        </Button>

        <NotificationPopover orgId={authStore.orgId ?? ""} />

        <WhatsNewPopover />

        {#if isDesktopApp}
          <Button
            variant="ghost"
            size="icon-sm"
            class="size-7 text-muted-foreground hover:text-foreground"
            onclick={openCurrentPageInWindow}
            title={m.common_titlebar_standalone_window()}
            aria-label={m.common_titlebar_standalone_window()}
          >
            <ArrowsPointingOut size={14} />
          </Button>
        {/if}
      {/if}

      <Button
        variant="ghost"
        size="icon-sm"
        class="size-7 text-muted-foreground hover:text-foreground"
        onclick={() => themeStore.toggle()}
        title={m.common_titlebar_theme_toggle()}
        aria-label={m.common_titlebar_theme_toggle()}
      >
        {#if themeStore.resolved === "dark"}
          <Sun size={14} />
        {:else}
          <Moon size={14} />
        {/if}
      </Button>
    </div>

    <!-- Flush Windows/Linux Caption Controls (Min, Max, Close) -->
    {#if !isMac && appWindow}
      <div class="flex items-center h-full border-s border-border/40">
        <Button
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.minimize()}
          class="h-full w-11 rounded-none text-muted-foreground hover:bg-accent hover:text-foreground"
          title={m.common_titlebar_minimize()}
          aria-label={m.shared_ui_minimize()}
        >
          <Minus size={13} class="stroke-[1.75]" />
        </Button>

        <Button
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.toggleMaximize()}
          class="h-full w-11 rounded-none text-muted-foreground hover:bg-accent hover:text-foreground"
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

        <Button
          variant="ghost"
          size="icon-sm"
          onclick={() => appWindow.close()}
          class="h-full w-11 rounded-none text-muted-foreground hover:bg-destructive hover:text-destructive-foreground"
          title={m.common_titlebar_close()}
          aria-label={m.shared_ui_close_window()}
        >
          <XMark size={13} class="stroke-[1.75]" />
        </Button>
      </div>
    {/if}
  </div>
</div>
