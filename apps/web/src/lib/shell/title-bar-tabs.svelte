<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { deLocalizeUrl } from "$lib/paraglide/runtime";
import { tabsStore } from "$lib/stores";
import { Plus, XMark } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { onMount, untrack } from "svelte";
import { getRouteTitle } from "./route-titles";

let {
  orgId,
}: {
  orgId: string;
} = $props();

let tabStripEl = $state<HTMLDivElement | null>(null);
let draggingTabId = $state<string | null>(null);
let suppressClick = false;

const currentPath = $derived(deLocalizeUrl(page.url).pathname);
const currentTabTitle = $derived(
  orgId
    ? getRouteTitle({
        pathname: currentPath,
        orgId,
        studentId: page.params.studentId,
      })
    : "Orbis",
);

$effect(() => {
  if (!orgId) return;
  const path = currentPath;
  const title = currentTabTitle;
  untrack(() => tabsStore.syncRoute(orgId, path, title));
});

function handleKeydown(e: KeyboardEvent) {
  if (!orgId) return;
  const mod = e.ctrlKey || e.metaKey;
  if (!mod) return;

  const key = e.key.toLowerCase();
  if (key === "t" && e.shiftKey) {
    e.preventDefault();
    tabsStore.reopenLastClosed(orgId);
  } else if (key === "t" && !e.shiftKey) {
    e.preventDefault();
    tabsStore.openNewTab(orgId, `/${orgId}`, m.common_nav_overview());
  } else if (key === "w" && !e.shiftKey) {
    e.preventDefault();
    if (tabsStore.activeTabId) {
      tabsStore.closeTab(orgId, tabsStore.activeTabId);
    }
  }
}

function handleLinkClickIntent(e: MouseEvent) {
  const anchor = (e.target as HTMLElement | null)?.closest(
    "a[href]",
  ) as HTMLAnchorElement | null;
  if (!anchor) return;
  if (e.ctrlKey || e.metaKey || e.button === 1) {
    tabsStore.markNextNavigationAsNewTab();
  }
}

function startTabDrag(e: PointerEvent, tabId: string) {
  if (e.button !== 0 || tabsStore.tabs.length < 2) return;
  if ((e.target as HTMLElement | null)?.closest("button")) return;

  const startX = e.clientX;
  let active = false;

  const onMove = (moveEvent: PointerEvent) => {
    if (!active) {
      if (Math.abs(moveEvent.clientX - startX) < 6) return;
      active = true;
      draggingTabId = tabId;
    }

    if (!tabStripEl) return;
    const nodes = tabStripEl.querySelectorAll<HTMLElement>("[data-tab-id]");
    let targetId: string | null = null;

    for (const node of nodes) {
      const rect = node.getBoundingClientRect();
      if (moveEvent.clientX >= rect.left && moveEvent.clientX <= rect.right) {
        targetId = node.dataset.tabId ?? null;
        break;
      }
    }

    if (targetId && targetId !== tabId) {
      tabsStore.moveTab(orgId, tabId, targetId);
    }
  };

  const onEnd = () => {
    window.removeEventListener("pointermove", onMove);
    window.removeEventListener("pointerup", onEnd);
    window.removeEventListener("pointercancel", onEnd);

    if (active) {
      suppressClick = true;
      requestAnimationFrame(() => {
        suppressClick = false;
      });
    }
    draggingTabId = null;
  };

  window.addEventListener("pointermove", onMove);
  window.addEventListener("pointerup", onEnd);
  window.addEventListener("pointercancel", onEnd);
}

function handleTabKeydown(e: KeyboardEvent, tabId: string) {
  if (e.key === "Enter" || e.key === " ") {
    e.preventDefault();
    tabsStore.activateTab(orgId, tabId);
    return;
  }

  if (e.altKey && (e.key === "ArrowLeft" || e.key === "ArrowRight")) {
    e.preventDefault();
    const isRtl =
      tabStripEl !== null && getComputedStyle(tabStripEl).direction === "rtl";
    const step = e.key === "ArrowLeft" ? (isRtl ? 1 : -1) : isRtl ? -1 : 1;
    const idx = tabsStore.tabs.findIndex((t) => t.id === tabId);
    const target = tabsStore.tabs[idx + step];
    if (target) {
      tabsStore.moveTab(orgId, tabId, target.id);
    }
  }
}

onMount(() => {
  window.addEventListener("click", handleLinkClickIntent, true);
  window.addEventListener("auxclick", handleLinkClickIntent, true);
  return () => {
    window.removeEventListener("click", handleLinkClickIntent, true);
    window.removeEventListener("auxclick", handleLinkClickIntent, true);
  };
});
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  bind:this={tabStripEl}
  role="tablist"
  class="flex items-end h-full min-w-0 flex-1 overflow-x-auto scrollbar-hide px-1 gap-1"
  data-tauri-drag-region
>
  {#each tabsStore.tabs as tab (tab.id)}
    {@const active = tab.id === tabsStore.activeTabId}
    {@const isDragging = draggingTabId === tab.id}
    <div
      data-tab-id={tab.id}
      role="tab"
      tabindex="0"
      aria-selected={active}
      onpointerdown={(e) => startTabDrag(e, tab.id)}
      onclick={() => {
        if (suppressClick) return;
        tabsStore.activateTab(orgId, tab.id);
      }}
      onauxclick={(e) => {
        if (e.button === 1) {
          e.preventDefault();
          tabsStore.closeTab(orgId, tab.id);
        }
      }}
      onkeydown={(e) => handleTabKeydown(e, tab.id)}
      class="group relative flex items-center justify-between gap-2 h-9 self-end pb-1 min-w-[104px] max-w-[196px] px-2.5 rounded-t-md font-medium transition-colors duration-150 shrink-0 select-none {isDragging
        ? 'z-20 cursor-grabbing bg-background text-foreground ring-1 ring-primary/50 shadow-xs'
        : active
          ? 'bg-background text-foreground z-10 cursor-pointer'
          : 'text-muted-foreground hover:text-foreground hover:bg-background/60 cursor-pointer'}"
    >
      <div class="flex items-center gap-1.5 min-w-0">
        <span
          class="size-1.5 rounded-full shrink-0 transition-colors duration-150 {active
            ? 'bg-primary'
            : 'bg-transparent'}"
        ></span>
        <span class="truncate text-xs leading-normal">{tab.title}</span>
      </div>
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={(e) => {
          e.stopPropagation();
          tabsStore.closeTab(orgId, tab.id);
        }}
        class="size-4 p-0 rounded-sm text-muted-foreground/70 hover:bg-foreground/10 hover:text-foreground transition-opacity shrink-0 {active
          ? 'opacity-80 hover:opacity-100'
          : 'opacity-0 group-hover:opacity-100'}"
        aria-label={m.titlebar_menu_close_tab()}
        title={m.titlebar_menu_close_tab()}
      >
        <XMark size={11} />
      </Button>
    </div>
  {/each}

  <Button
    variant="ghost"
    size="icon-sm"
    onclick={() =>
      tabsStore.openNewTab(orgId, `/${orgId}`, m.common_nav_overview())}
    class="size-7 text-muted-foreground hover:text-foreground self-center shrink-0"
    title={m.titlebar_menu_new_tab()}
    aria-label={m.titlebar_menu_new_tab()}
  >
    <Plus size={14} />
  </Button>

  <div class="flex-1 h-full min-w-6" data-tauri-drag-region></div>
</div>
