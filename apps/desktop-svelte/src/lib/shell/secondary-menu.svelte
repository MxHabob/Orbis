<!--
@component SecondaryMenu
Horizontal tab-style navigation bar for detail pages.
Inspired by dashboard's secondary-menu.tsx — unified across Class Detail,
Student Detail, and User Detail.
-->
<script lang="ts">
import { page } from "$app/state";
import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";
import type { Component } from "svelte";

type MenuItem = {
  path: string;
  label: string;
  icon?: any;
};

let {
  items,
  basePath,
}: {
  items: MenuItem[];
  basePath: string;
} = $props();

function isActive(itemPath: string): boolean {
  const unlocalizedPathname = decodeURIComponent(
    deLocalizeUrl(page.url).pathname,
  ).replace(/\/$/, "");
  const cleanBasePath = basePath.replace(/\/$/, "");
  const fullPath = itemPath ? `${cleanBasePath}/${itemPath}` : cleanBasePath;
  const isEqual = unlocalizedPathname === fullPath;
  if (itemPath === "") {
    return unlocalizedPathname === cleanBasePath;
  }
  return (
    unlocalizedPathname === fullPath ||
    unlocalizedPathname.startsWith(`${fullPath}/`)
  );
}
</script>

<nav
  class="flex items-center gap-6 border-b border-border/20 h-10 w-full shrink-0 select-none overflow-x-auto scrollbar-hide"
>
  {#each items as item (item.path)}
    <a
      href={localizeHref(item.path ? `${basePath}/${item.path}` : basePath)}
      class="text-xs font-semibold pb-2 border-b-2 transition-all duration-150
        flex items-center gap-1.5 h-full px-1 cursor-pointer shrink-0
        {isActive(item.path)
        ? 'border-primary text-foreground font-bold'
        : 'border-transparent text-muted-foreground hover:text-foreground'}"
    >
      {#if item.icon}
        <item.icon size={13} />
      {/if}
      <span>{item.label}</span>
    </a>
  {/each}
</nav>
