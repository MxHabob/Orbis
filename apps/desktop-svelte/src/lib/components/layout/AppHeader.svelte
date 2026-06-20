<!--
@component AppHeader
Three-zone header bar following GitButler's AppHeader pattern.
Left zone (actions) | Center zone (selector) | Right zone (actions).
Both left and right use flex-basis: 0 so center stays perfectly centered.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Search,
  Bell,
  School,
  ChevronDown,
  ChevronLeft,
  ChevronRight,
  UserCircle,
  LogOut,
  Languages,
  Sun,
  Moon,
  Monitor,
} from "lucide-svelte";
import { authStore, studentsStore, themeStore } from "$lib/stores";
import { isDesktopApp } from "$lib/services/ipc";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref, setLocale } from "$lib/paraglide/runtime";
import {
  Breadcrumb,
  BreadcrumbList,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbSeparator,
  BreadcrumbPage,
} from "@orbis/ui-svelte/breadcrumb";
import OrgSwitcher from "./OrgSwitcher.svelte";

let {
  orgId,
  orgName = "Orbis School",
}: {
  orgId: string;
  orgName?: string;
} = $props();

const isDesktop = isDesktopApp();
const userInitials = $derived(
  (authStore.user?.full_name ?? "User")
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() ?? "")
    .join("") || "U",
);

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

function switchLanguage(locale: "ar" | "en") {
  const newPath = localizeHref(page.url.pathname, { locale });
  setLocale(locale, { reload: false });
  window.location.href = newPath;
}
</script>

<div
	class="grid h-13 shrink-0 grid-cols-[1fr_auto_1fr] items-center gap-3 px-[14px]"
	class:pt-1={isDesktop}
	data-tauri-drag-region={isDesktop || undefined}
>
	<!-- Left Zone -->
	<div
		class="flex min-w-0 items-center justify-start gap-4"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<Button
			variant="outline"
			size="sm"
			class="h-8 gap-1.5 px-2.5 text-[12px] text-muted-foreground shrink-0"
			title={m.header_search_title()}
			aria-label="Search"
		>
			<Search size={15} strokeWidth={2} />
			<span class="whitespace-nowrap">{m.header_search()}</span>
			<kbd class="ml-2 inline-flex h-4.5 items-center rounded border border-border bg-muted px-1 font-mono text-[10px] text-muted-foreground">{m.header_search_shortcut()}</kbd>
		</Button>

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

	<!-- Center Zone -->
	<div
		class="flex min-w-0 justify-center overflow-hidden"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<OrgSwitcher {orgId} {orgName} />
	</div>

	<!-- Right Zone -->
	<div
		class="flex min-w-0 items-center justify-end gap-2"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<!-- Language Switcher Dropdown -->
		<DropdownMenu>
			<DropdownMenuTrigger>
				<Button
					variant="ghost"
					size="sm"
					class="gap-1.5 px-2.5 h-8 text-[12px] font-semibold text-muted-foreground hover:text-foreground"
					title={m.header_switch_language()}
				>
					<Languages size={15} strokeWidth={2} class="text-muted-foreground" />
					<span class="uppercase font-mono">{getLocale()}</span>
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" sideOffset={8} class="w-36">
				<DropdownMenuItem class="cursor-pointer flex items-center justify-between" onSelect={() => switchLanguage("ar")}>
					<span>{m.header_language_ar()}</span>
					{#if getLocale() === "ar"}
						<span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
					{/if}
				</DropdownMenuItem>
				<DropdownMenuItem class="cursor-pointer flex items-center justify-between" onSelect={() => switchLanguage("en")}>
					<span>{m.header_language_en()}</span>
					{#if getLocale() === "en"}
						<span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
					{/if}
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>

		<!-- Theme Switcher Dropdown -->
		<DropdownMenu>
			<DropdownMenuTrigger>
				<Button
					variant="ghost"
					size="icon"
					class="h-8 w-8 text-muted-foreground hover:text-foreground shrink-0"
					title={m.header_switch_theme()}
				>
					{#if themeStore.resolved === "light"}
						<Sun size={15} strokeWidth={2} />
					{:else}
						<Moon size={15} strokeWidth={2} />
					{/if}
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" sideOffset={8} class="w-36">
				<DropdownMenuItem class="cursor-pointer flex items-center justify-between" onSelect={() => themeStore.set("light")}>
					<span class="flex items-center gap-2">
						<Sun size={13} />
						<span>{m.theme_light()}</span>
					</span>
					{#if themeStore.current === "light"}
						<span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
					{/if}
				</DropdownMenuItem>
				<DropdownMenuItem class="cursor-pointer flex items-center justify-between" onSelect={() => themeStore.set("dark")}>
					<span class="flex items-center gap-2">
						<Moon size={13} />
						<span>{m.theme_dark()}</span>
					</span>
					{#if themeStore.current === "dark"}
						<span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
					{/if}
				</DropdownMenuItem>
				<DropdownMenuItem class="cursor-pointer flex items-center justify-between" onSelect={() => themeStore.set("system")}>
					<span class="flex items-center gap-2">
						<Monitor size={13} />
						<span>{m.theme_system()}</span>
					</span>
					{#if themeStore.current === "system"}
						<span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
					{/if}
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>

		<Button
			variant="ghost"
			size="icon"
			title={m.header_notifications()}
			aria-label={m.header_notifications()}
		>
			<Bell size={15} strokeWidth={2} />
		</Button>
	</div>
</div>
