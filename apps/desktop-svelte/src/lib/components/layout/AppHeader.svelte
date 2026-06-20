<!--
@component AppHeader
Three-zone header bar following GitButler's AppHeader pattern.
Left zone (actions) | Center zone (selector) | Right zone (actions).
-->
<script lang="ts">
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Search,
  ChevronDown,
  ChevronLeft,
  ChevronRight,
  Play,
  Megaphone,
} from "lucide-svelte";
import { studentsStore } from "$lib/stores";
import { isDesktopApp } from "$lib/services/ipc";
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import {
  Breadcrumb,
  BreadcrumbList,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbSeparator,
  BreadcrumbPage,
} from "@orbis/ui-svelte/breadcrumb";

let {
  orgId,
}: {
  orgId: string;
  orgName?: string;
} = $props();

const isDesktop = isDesktopApp();

// Breadcrumb items generator based on active route
const breadcrumbs = $derived.by(() => {
  const pathname = page.url.pathname;
  const currentOrgId = orgId || page.params.orgId || "";
  const studentId = page.params.studentId;

  const items: { label: string; href?: string }[] = [];

  // Base dashboard link
  items.push({ label: m.breadcrumb_home(), href: `/${currentOrgId}` });

  if (pathname.includes("/reports")) {
    items.push({ label: m.breadcrumb_reports(), href: `/${currentOrgId}/reports` });
  } else if (pathname.includes("/students")) {
    items.push({ label: m.breadcrumb_students(), href: `/${currentOrgId}/students` });

    if (pathname.includes("/students/new")) {
      items.push({ label: m.breadcrumb_new_student() });
    } else if (studentId) {
      const student = studentsStore.list.find((s) => s.id === studentId);
      const studentName =
        (student?.full_name as string) || (student?.name_ar as string) || m.breadcrumb_student_profile();
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
    items.push({ label: m.breadcrumb_academics(), href: `/${currentOrgId}/academics` });
  } else if (pathname.includes("/finance")) {
    items.push({ label: m.breadcrumb_finance(), href: `/${currentOrgId}/finance` });
  } else if (pathname.includes("/settings")) {
    items.push({ label: m.breadcrumb_settings(), href: `/${currentOrgId}/settings` });
  }

  return items;
});
</script>

<div
	class="flex h-13 shrink-0 items-center justify-between gap-3 px-[14px]"
	class:pt-1={isDesktop}
	data-tauri-drag-region={isDesktop || undefined}
>
	<!-- Left Zone -->
	<div
		class="flex min-w-0 items-center justify-start gap-4"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<!-- Breadcrumb -->
		<Breadcrumb>
			<BreadcrumbList class="flex items-center gap-1.5 text-[11px] font-medium text-muted-foreground select-none">
				{#each breadcrumbs as item, index}
					<BreadcrumbItem>
						{#if item.href && index < breadcrumbs.length - 1}
							<BreadcrumbLink href={item.href} class="hover:text-foreground transition-colors duration-150 truncate max-w-[120px]">
								{item.label}
							</BreadcrumbLink>
						{:else}
							<BreadcrumbPage class="text-foreground font-semibold truncate max-w-[120px]">
								{item.label}
							</BreadcrumbPage>
						{/if}
					</BreadcrumbItem>
					{#if index < breadcrumbs.length - 1}
						<BreadcrumbSeparator class="opacity-40">
							{#if getLocale() === "ar"}
								<ChevronLeft size={11} />
							{:else}
								<ChevronRight size={11} />
							{/if}
						</BreadcrumbSeparator>
					{/if}
				{/each}
			</BreadcrumbList>
		</Breadcrumb>
	</div>

	<!-- Right Zone -->
	<div
		class="flex items-center gap-2"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<!-- Monitors Dropdown -->
		<DropdownMenu>
			<DropdownMenuTrigger class="outline-none">
				<button
					class="flex h-8 items-center gap-1.5 rounded-[8px] border border-zinc-200 bg-white px-3 font-semibold text-xs text-foreground shadow-[0_2px_0_0_#e4e4e7] hover:bg-zinc-50 active:translate-y-[1.5px] active:shadow-none transition-all dark:border-zinc-800 dark:bg-zinc-900 dark:shadow-[0_2px_0_0_#27272a] dark:hover:bg-zinc-800 shrink-0 cursor-pointer select-none"
				>
					<span>0 Monitors</span>
					<ChevronDown size={14} class="opacity-60" />
				</button>
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" sideOffset={8} class="w-48 p-1 z-50">
				<div class="px-2 py-1.5 text-[10px] font-bold text-muted-foreground uppercase tracking-wider">Monitors</div>
				<DropdownMenuSeparator />
				<div class="px-2 py-3 text-xs text-muted-foreground text-center">No monitors configured</div>
			</DropdownMenuContent>
		</DropdownMenu>

		<!-- Play Button -->
		<button
			class="flex h-8 w-8 items-center justify-center rounded-[8px] border border-zinc-200 bg-white shadow-[0_2px_0_0_#e4e4e7] hover:bg-zinc-50 active:translate-y-[1.5px] active:shadow-none transition-all dark:border-zinc-800 dark:bg-zinc-900 dark:shadow-[0_2px_0_0_#27272a] dark:hover:bg-zinc-800 shrink-0 cursor-pointer select-none"
			title="Play"
		>
			<Play size={14} class="fill-current text-foreground stroke-[2px]" />
		</button>

		<!-- Search Button -->
		<button
			class="flex h-8 items-center gap-2 rounded-[8px] border border-zinc-200 bg-white px-3 font-semibold text-xs text-foreground shadow-[0_2px_0_0_#e4e4e7] hover:bg-zinc-50 active:translate-y-[1.5px] active:shadow-none transition-all dark:border-zinc-800 dark:bg-zinc-900 dark:shadow-[0_2px_0_0_#27272a] dark:hover:bg-zinc-800 shrink-0 cursor-pointer select-none"
			title={m.header_search_title()}
			aria-label="Search"
		>
			<Search size={14} class="text-muted-foreground stroke-[2px]" />
			<span class="whitespace-nowrap">{m.header_search()}</span>
			<kbd class="ml-1 rounded border border-zinc-200 bg-zinc-50 px-1 py-0.5 font-mono text-[9px] font-medium text-zinc-400 dark:border-zinc-800 dark:bg-zinc-850 dark:text-zinc-500">{m.header_search_shortcut()}</kbd>
		</button>

		<!-- Ask Seer Button -->
		<button
			class="flex h-8 items-center gap-2 rounded-[8px] border border-zinc-200 bg-white px-3 font-semibold text-xs text-foreground shadow-[0_2px_0_0_#e4e4e7] hover:bg-zinc-50 active:translate-y-[1.5px] active:shadow-none transition-all dark:border-zinc-800 dark:bg-zinc-900 dark:shadow-[0_2px_0_0_#27272a] dark:hover:bg-zinc-800 shrink-0 cursor-pointer select-none"
			title="Ask Seer"
			aria-label="Ask Seer"
		>
			<svg class="h-3.5 w-3.5 text-foreground shrink-0 stroke-[2px]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round">
				<path d="M12 3l8.5 14.7a1 1 0 0 1-.9 1.3H4.4a1 1 0 0 1-.9-1.3L12 3z" />
				<path d="M12 15a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
			</svg>
			<span class="whitespace-nowrap">Ask Seer</span>
			<kbd class="ml-1 rounded border border-zinc-200 bg-zinc-50 px-1 py-0.5 font-mono text-[9px] font-medium text-zinc-400 dark:border-zinc-800 dark:bg-zinc-850 dark:text-zinc-500">Ctrl /</kbd>
		</button>

		<!-- Megaphone Button -->
		<button
			class="flex h-8 w-8 items-center justify-center rounded-[8px] border border-zinc-200 bg-white shadow-[0_2px_0_0_#e4e4e7] hover:bg-zinc-50 active:translate-y-[1.5px] active:shadow-none transition-all dark:border-zinc-800 dark:bg-zinc-900 dark:shadow-[0_2px_0_0_#27272a] dark:hover:bg-zinc-800 shrink-0 cursor-pointer select-none"
			title="What's New"
			aria-label="What's new"
		>
			<Megaphone size={14} class="text-foreground stroke-[2px]" />
		</button>
	</div>
</div>

