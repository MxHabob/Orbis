<script lang="ts">
import { onMount } from "svelte";
import { isDesktopApp } from "$lib/services/ipc";
import {
  academicStore,
  authStore,
  uiStore,
  themeStore,
  studentsStore,
} from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import {
  Minimize2,
  Square,
  Copy,
  X,
  GraduationCap,
  Sparkles,
  Wifi,
  WifiOff,
  ChevronDown,
  ChevronLeft,
  ChevronRight,
  Sun,
  Moon,
  Search,
  Megaphone,
} from "lucide-svelte";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Breadcrumb,
  BreadcrumbList,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbSeparator,
  BreadcrumbPage,
} from "@orbis/ui-svelte/breadcrumb";
import { commands } from "$lib/services/ipc";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { toast } from "svelte-sonner";
import { page } from "$app/state";
import WhatsNewDialog from "./WhatsNewDialog.svelte";

// Window states
let appWindow = $state<any>(null);
let isMaximized = $state(false);
let isMac = $state(false);
let isOnline = $state(true);

// Academic cycles list
let cycles = $state<any[]>([]);
let currentCycleName = $derived.by(() => {
  const active = cycles.find(
    (c) => String(c.id) === academicStore.selectedCycleId,
  );
  if (active) {
    return active.name_ar && getLocale() === "ar"
      ? active.name_ar
      : active.name;
  }
  return m.titlebar_cycle();
});

let showWhatsNew = $state(false);

// Breadcrumb items generator based on active route
const breadcrumbs = $derived.by(() => {
  const pathname = page.url.pathname;
  const currentOrgId = page.params.orgId || authStore.orgId || "";
  const studentId = page.params.studentId;

  const items: { label: string; href?: string }[] = [];

  // Base dashboard link
  items.push({ label: m.breadcrumb_home(), href: `/${currentOrgId}` });

  if (pathname.includes("/reports")) {
    items.push({
      label: m.breadcrumb_reports(),
      href: `/${currentOrgId}/reports`,
    });
  } else if (pathname.includes("/students")) {
    items.push({
      label: m.breadcrumb_students(),
      href: `/${currentOrgId}/students`,
    });

    if (pathname.includes("/students/new")) {
      items.push({ label: m.breadcrumb_new_student() });
    } else if (studentId) {
      const student = studentsStore.list.find((s) => s.id === studentId);
      const studentName =
        (student?.full_name as string) ||
        (student?.name_ar as string) ||
        m.breadcrumb_student_profile();
      items.push({
        label: studentName,
        href: `/${currentOrgId}/students/${studentId}`,
      });

      if (pathname.includes("/edit")) {
        items.push({ label: m.breadcrumb_edit_data() });
      } else if (pathname.includes("/grades")) {
        items.push({ label: m.breadcrumb_grades() });
      } else if (pathname.includes("/fees")) {
        items.push({ label: m.breadcrumb_fees() });
      } else if (pathname.includes("/attendance")) {
        items.push({ label: m.breadcrumb_attendance() });
      } else if (pathname.includes("/behavior")) {
        items.push({ label: m.breadcrumb_behavior() });
      }
    }
  } else if (pathname.includes("/academics")) {
    items.push({
      label: m.breadcrumb_academics(),
      href: `/${currentOrgId}/academics`,
    });
    if (pathname.includes("/exams")) {
      items.push({
        label: m.breadcrumb_exams(),
        href: `/${currentOrgId}/academics/exams`,
      });
    }
  } else if (pathname.includes("/finance")) {
    items.push({
      label: m.breadcrumb_finance(),
      href: `/${currentOrgId}/finance`,
    });
  } else if (pathname.includes("/settings")) {
    items.push({
      label: m.breadcrumb_settings(),
      href: `/${currentOrgId}/settings`,
    });
  }

  return items;
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

$effect(() => {
  if (authStore.isAuthenticated && authStore.orgId) {
    commands.academics
      .getCycles(authStore.orgId)
      .then((list) => {
        cycles = list;
        if (!academicStore.selectedCycleId && list.length > 0) {
          const activeCycle =
            list.find((c: any) => c.is_current === 1) || list[0];
          if (activeCycle) {
            academicStore.selectedCycleId = String(activeCycle.id);
          }
        }
      })
      .catch((err) => {
        console.error("Failed to load cycles in TitleBar:", err);
      });
  } else {
    cycles = [];
  }
});

function selectCycle(cycleId: string) {
  academicStore.selectedCycleId = cycleId;
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
      <div class="relative flex items-center z-50 shrink-0">
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

    <!-- Breadcrumb -->
    {#if authStore.isAuthenticated}
      <div class="w-px h-4 bg-border/60 mx-1 shrink-0" data-tauri-drag-region></div>
      <Breadcrumb class="min-w-0 overflow-hidden">
        <BreadcrumbList class="flex items-center gap-1 text-[10px] font-semibold text-muted-foreground select-none flex-nowrap">
          {#each breadcrumbs as item, index}
            <BreadcrumbItem class="min-w-0">
              {#if item.href && index < breadcrumbs.length - 1}
                <BreadcrumbLink href={localizeHref(item.href)} class="hover:text-foreground transition-colors duration-150 truncate max-w-[120px] block">
                  {item.label}
                </BreadcrumbLink>
              {:else}
                <BreadcrumbPage class="text-foreground font-bold truncate max-w-[120px] block">
                  {item.label}
                </BreadcrumbPage>
              {/if}
            </BreadcrumbItem>
            {#if index < breadcrumbs.length - 1}
              <BreadcrumbSeparator class="opacity-40 shrink-0">
                {#if getLocale() === "ar"}
                  <ChevronLeft size={10} />
                {:else}
                  <ChevronRight size={10} />
                {/if}
              </BreadcrumbSeparator>
            {/if}
          {/each}
        </BreadcrumbList>
      </Breadcrumb>
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

      <!-- What's New Megaphone (Original Header Design) -->
      <Button
        variant="outline"
        size="icon-sm"
        class="rounded-[8px] shrink-0"
        title="What's New"
        aria-label="What's new"
        onclick={() => showWhatsNew = true}
      >
        <Megaphone size={14} class="text-foreground stroke-[2px]" />
      </Button>
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

<WhatsNewDialog bind:open={showWhatsNew} />
