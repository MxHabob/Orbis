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
  UserCircle,
  LogOut,
} from "lucide-svelte";
import { authStore, studentsStore } from "$lib/stores";
import { isDesktopApp } from "$lib/services/ipc";
import { page } from "$app/state";
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
  items.push({ label: "الرئيسية", href: `/${currentOrgId}` });

  if (pathname.includes("/reports")) {
    items.push({ label: "التقارير", href: `/${currentOrgId}/reports` });
  } else if (pathname.includes("/students")) {
    items.push({ label: "الطلاب", href: `/${currentOrgId}/students` });

    if (pathname.includes("/students/new")) {
      items.push({ label: "طالب جديد" });
    } else if (studentId) {
      const student = studentsStore.list.find((s) => s.id === studentId);
      const studentName =
        student?.full_name || student?.name_ar || "ملف الطالب";
      items.push({
        label: studentName,
        href: `/${currentOrgId}/students/${studentId}`,
      });

      if (pathname.includes("/edit")) {
        items.push({ label: "تعديل البيانات" });
      } else if (pathname.includes("/grades")) {
        items.push({ label: "الدرجات" });
      } else if (pathname.includes("/fees")) {
        items.push({ label: "الرسوم" });
      } else if (pathname.includes("/attendance")) {
        items.push({ label: "الحضور" });
      } else if (pathname.includes("/behavior")) {
        items.push({ label: "السلوك" });
      }
    }
  } else if (pathname.includes("/academics")) {
    items.push({ label: "الأكاديميات", href: `/${currentOrgId}/academics` });
  } else if (pathname.includes("/finance")) {
    items.push({ label: "المالية", href: `/${currentOrgId}/finance` });
  } else if (pathname.includes("/settings")) {
    items.push({ label: "الإعدادات", href: `/${currentOrgId}/settings` });
  }

  return items;
});
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
			title="Search (⌘K)"
			aria-label="Search"
		>
			<Search size={15} strokeWidth={2} />
			<span class="whitespace-nowrap">Search...</span>
			<kbd class="ml-2 inline-flex h-4.5 items-center rounded border border-border bg-muted px-1 font-mono text-[10px] text-muted-foreground">⌘K</kbd>
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
							<ChevronLeft size={11} />
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
		<DropdownMenu>
			<DropdownMenuTrigger asChild>
				<Button
					variant="ghost"
					size="sm"
					class="max-w-full gap-1.5 px-2.5 text-[12px] font-semibold"
					title="Switch organization"
				>
					<School size={15} strokeWidth={2} class="text-muted-foreground" />
					<span class="truncate">{orgName}</span>
					<ChevronDown size={12} strokeWidth={2} class="shrink-0 text-muted-foreground opacity-60" />
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent align="center" sideOffset={8} class="w-56">
				<DropdownMenuItem class="cursor-pointer">
					<UserCircle size={14} strokeWidth={1.8} />
					<span>{orgName}</span>
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem class="cursor-pointer">
					<School size={14} strokeWidth={1.8} />
					<span>Switch organization</span>
				</DropdownMenuItem>
				<DropdownMenuItem class="cursor-pointer text-destructive focus:text-destructive">
					<LogOut size={14} strokeWidth={1.8} />
					<span>Logout</span>
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	</div>

	<!-- Right Zone -->
	<div
		class="flex min-w-0 items-center justify-end gap-2"
		data-tauri-drag-region={isDesktop || undefined}
	>
		<Button
			variant="ghost"
			size="icon"
			title="Notifications"
			aria-label="Notifications"
		>
			<Bell size={15} strokeWidth={2} />
		</Button>
	</div>
</div>
