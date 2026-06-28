<!--
@component ExamsLayout
Wraps all Exams routes with a resizable sidebar layout using MainViewport.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";
import {
  ClipboardList,
  FileText,
  BarChart3,
  ChevronRight,
} from "lucide-svelte";
import MainViewport from "$lib/components/layout/MainViewport.svelte";
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

let sessionsCount = $state(0);

async function loadSidebarStats() {
  try {
    // TODO: Wire to IPC commands when available
    sessionsCount = 0;
  } catch (e) {
    console.error("Error loading exam sidebar stats:", e);
  }
}

$effect(() => {
  const _path = page.url.pathname;
  if (data.orgId) {
    loadSidebarStats();
  }
});

function isActive(pathSegment: string): boolean {
  const unlocalizedPathname = deLocalizeUrl(page.url).pathname;
  const baseUrl = `/${data.orgId}/academics/exams`;
  if (pathSegment === "") {
    return unlocalizedPathname === baseUrl;
  }
  return (
    unlocalizedPathname === `${baseUrl}/${pathSegment}` ||
    unlocalizedPathname.startsWith(`${baseUrl}/${pathSegment}/`)
  );
}

const navItems = $derived([
  {
    path: "",
    label: m.exams_layout_nav_overview(),
    icon: BarChart3,
  },
  {
    path: "sessions",
    label: m.exams_layout_nav_sessions(),
    icon: ClipboardList,
    badge: sessionsCount > 0 ? sessionsCount : null,
  },
  {
    path: "reports",
    label: m.exams_layout_nav_reports(),
    icon: FileText,
  },
]);
</script>

<div class="h-full w-full bg-background overflow-hidden">
	<MainViewport
		name="exams-workspace"
		leftWidth={{ default: 220, min: 180 }}
	>
		{#snippet left()}
			<div class="h-full border-r border-border bg-card/20 p-4 flex flex-col gap-4 overflow-y-auto scrollbar-hide select-none">
				<div class="flex flex-col gap-1 w-full text-xs">
					{#each navItems as item (item.path)}
						<Button
							href={localizeHref(`/${data.orgId}/academics/exams/${item.path}`)}
							variant="ghost"
							class="w-full flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer h-9
								{isActive(item.path) ? 'bg-primary/10 text-primary font-semibold hover:bg-primary/15' : 'text-muted-foreground hover:bg-accent hover:text-foreground'}"
						>
							<div class="flex items-center gap-2">
								<item.icon size={14} class={isActive(item.path) ? 'text-primary' : 'text-muted-foreground'} />
								<span>{item.label}</span>
							</div>

							{#if item.badge !== undefined && item.badge !== null}
								<span class="text-[9px] px-1.5 py-0.5 rounded-md font-mono bg-muted/80 text-muted-foreground border border-border/40 font-bold">
									{item.badge}
								</span>
							{:else if isActive(item.path)}
								<ChevronRight size={10} class="text-primary" />
							{/if}
						</Button>
					{/each}
				</div>
			</div>
		{/snippet}

		{#snippet middle()}
			<div class="h-full w-full overflow-hidden">
				{@render children()}
			</div>
		{/snippet}
	</MainViewport>
</div>
