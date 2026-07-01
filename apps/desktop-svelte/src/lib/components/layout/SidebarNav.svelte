<!--
@component SidebarNav
Vertical sidebar navigation with icons, badges, and active indicator.
Replaces 626 lines of duplicated code across academics, settings, finance,
and exams layouts.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { Button } from "@orbis/ui-svelte/button";
  import { ChevronRight } from "lucide-svelte";
  import { deLocalizeUrl, localizeHref, getLocale } from "$lib/paraglide/runtime";
  import type { Component } from "svelte";

  type NavItem = {
    path: string;
    label: string;
    icon: Component;
    badge?: number | null;
  };

  let {
    items,
    basePath,
    title,
    description,
  }: {
    items: NavItem[];
    basePath: string;
    title?: string;
    description?: string;
  } = $props();

  function isActive(pathSegment: string): boolean {
    const unlocalizedPathname = deLocalizeUrl(page.url).pathname;
    if (pathSegment === "") {
      return unlocalizedPathname === basePath;
    }
    return (
      unlocalizedPathname === `${basePath}/${pathSegment}` ||
      unlocalizedPathname.startsWith(`${basePath}/${pathSegment}/`)
    );
  }
</script>

<div
  class="h-full border-r border-border bg-card/20 p-4 flex flex-col gap-4 overflow-y-auto scrollbar-hide select-none"
>
  {#if title}
    <div class="flex flex-col gap-1 px-2">
      <h2 class="text-sm font-semibold tracking-tight">{title}</h2>
      {#if description}
        <p class="text-[10px] text-muted-foreground">{description}</p>
      {/if}
    </div>
    <div class="h-px bg-border/60 w-full"></div>
  {/if}

  <div class="flex flex-col gap-1 w-full text-xs">
    {#each items as item (item.path)}
      <Button
        href={localizeHref(
          item.path ? `${basePath}/${item.path}` : basePath,
        )}
        variant="ghost"
        class="w-full flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer h-9
          {isActive(item.path)
          ? 'bg-primary/10 text-primary font-semibold hover:bg-primary/15'
          : 'text-muted-foreground hover:bg-accent hover:text-foreground'}"
      >
        <div class="flex items-center gap-2">
          <item.icon
            size={14}
            class={isActive(item.path)
              ? "text-primary"
              : "text-muted-foreground"}
          />
          <span>{item.label}</span>
        </div>

        {#if item.badge != null}
          <span
            class="text-[9px] px-1.5 py-0.5 rounded-md font-mono bg-muted/80 text-muted-foreground border border-border/40 font-bold"
          >
            {item.badge}
          </span>
        {:else if isActive(item.path)}
          <ChevronRight
            size={10}
            class="text-primary transition-transform duration-200 {getLocale() === 'ar' ? 'rotate-180' : ''}"
          />
        {/if}
      </Button>
    {/each}
  </div>
</div>
