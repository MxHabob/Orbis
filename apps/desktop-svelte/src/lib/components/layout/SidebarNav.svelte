<!--
@component SidebarNav
Vertical sidebar navigation with support for groups, custom actions, active indicators,
and collapsibility.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { Button } from "@orbis/ui-svelte/button";
  import { deLocalizeUrl, localizeHref, getLocale } from "$lib/paraglide/runtime";
  import type { Snippet } from "svelte";

  type NavItem = {
    path?: string;
    label: string;
    icon?: any;
    badge?: number | string | null;
    onclick?: (e: MouseEvent) => void;
    isActive?: boolean;
  };

  type NavGroup = {
    title?: string;
    items: NavItem[];
  };

  let {
    items = [],
    groups,
    basePath = "",
    title,
    headerAction,
  }: {
    items?: NavItem[];
    groups?: NavGroup[];
    basePath?: string;
    title?: string;
    headerAction?: Snippet;
  } = $props();

  function isActive(pathSegment: string): boolean {
    if (!basePath) return false;
    const unlocalizedPathname = deLocalizeUrl(page.url).pathname;
    if (pathSegment === "") {
      return unlocalizedPathname === basePath;
    }
    return (
      unlocalizedPathname === `${basePath}/${pathSegment}` ||
      unlocalizedPathname.startsWith(`${basePath}/${pathSegment}/`)
    );
  }

  function collapseSidebar(e: MouseEvent) {
    const target = e.currentTarget as HTMLElement;
    target.dispatchEvent(
      new CustomEvent("collapsesidebar", { bubbles: true, composed: true }),
    );
  }

  const resolvedGroups = $derived(
    groups || [{ items }]
  );
</script>

<div
  class="h-full flex flex-col overflow-y-auto scrollbar-hide select-none"
>
  <!-- Header -->
  <div class="h-14 flex items-center justify-between pb-4 border-b border-border/60 shrink-0">
    <div class="flex items-center gap-2 min-w-0">
      <h2 class="text-sm font-semibold tracking-tight text-foreground truncate">
        {title || "Navigation"}
      </h2>
      {#if headerAction}
        {@render headerAction()}
      {/if}
    </div>
    <Button
      size="icon-xs"
      class="shrink-0"
      onclick={collapseSidebar}
      title={getLocale() === 'ar' ? 'طي القائمة' : 'Collapse Sidebar'}
    >
      <svg
        width="11"
        height="11"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="transition-transform duration-200 {getLocale() === 'ar' ? 'rotate-180' : ''}"
      >
        <path d="m11 17-5-5 5-5M18 17l-5-5 5-5" />
      </svg>
    </Button>
  </div>

  <!-- Groups and Items -->
  <div class="flex flex-col gap-4 w-full p-3 text-xs">
    {#each resolvedGroups as group, groupIdx (group.title ?? groupIdx)}
      <div class="flex flex-col gap-1 w-full">
        {#if group.title}
          <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider px-3.5 py-1 text-start">
            {group.title}
          </span>
        {/if}

        {#each group.items as item (item.path ?? item.label)}
          <Button
            href={item.path !== undefined ? localizeHref(item.path ? `${basePath}/${item.path}` : basePath) : undefined}
            onclick={item.onclick}
            variant="ghost"
            size="lg"
            class="w-full !justify-start px-3.5 rounded-lg text-start transition-all duration-150 cursor-pointer border
              {item.isActive ?? isActive(item.path ?? '')
              ? 'bg-primary/5 text-primary border-primary/20 font-medium'
              : 'text-muted-foreground/90 hover:bg-accent/50 hover:text-foreground border-transparent'}"
          >
            <span class="truncate">{item.label}</span>

            {#if item.badge != null}
              <div class="flex-grow"></div>
              <span
                class="text-[9px] px-1.5 py-0.5 rounded-md font-mono bg-muted/80 text-muted-foreground border border-border/40 font-bold"
              >
                {item.badge}
              </span>
            {/if}
          </Button>
        {/each}
      </div>

      {#if groupIdx < resolvedGroups.length - 1}
        <div class="h-px bg-border/60 mx-3.5 my-1"></div>
      {/if}
    {/each}
  </div>
</div>
