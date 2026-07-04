<!--
@component StudentsDirectoryPage
Main workspace for students. Integrates Left sidebar filters, middle data table,
and right detailed student inspector.
-->
<script lang="ts">
import { onMount } from "svelte";
import MainViewport from "$lib/components/layout/MainViewport.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, Search, FilterX, Download, Printer, Upload } from "lucide-svelte";
import { studentsStore } from "$lib/stores";
import PrintDialog from "$lib/components/shared/PrintDialog.svelte";
import SidebarNav from "$lib/components/layout/SidebarNav.svelte";
import StudentTable from "$lib/components/students/student-table.svelte";
import StudentInspector from "$lib/components/students/student-inspector.svelte";
import { toast } from "svelte-sonner";
import { goto } from "$app/navigation";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber } from "$lib/i18n-utils";

let { data }: { data: { orgId: string } } = $props();

// Helper to count students matching a criteria
function getCountByClass(classId: string): number {
  return studentsStore.list.filter((s) => s.class_id === classId).length;
}

function getCountByStatus(status: string): number {
  if (status === "all") return studentsStore.list.length;
  return studentsStore.list.filter(
    (s) => String(s.status || "").toLowerCase() === status,
  ).length;
}

// Load data when page mounts
onMount(() => {
  if (data.orgId) {
    studentsStore.load(data.orgId);
  }
});

// Clear active filters
function clearFilters() {
  studentsStore.selectedClassId = null;
  studentsStore.statusFilter = "active";
  studentsStore.searchQuery = "";
}

const hasActiveFilters = $derived(
  studentsStore.selectedClassId !== null ||
    studentsStore.statusFilter !== "active" ||
    studentsStore.searchQuery !== "",
);

let printDialogOpen = $state(false);
let selectedStudentIds = $state<Set<string>>(new Set());
</script>

<svelte:head>
	<title>{m.page_title_students()}</title>
</svelte:head>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
	<MainViewport
		name="students-workspace"
		leftWidth={{ default: 240, min: 200 }}
		right={studentsStore.selectedId ? rightSnippet : undefined}
		rightWidth={{ default: 360, min: 300 }}
	>
		<!-- Left Panel: Sidebar Filters -->
		{#snippet left()}
			<SidebarNav
				title={m.students_filter_panel()}
				groups={[
					{
						title: m.filter_academic_status(),
						items: [
							{
								label: m.filter_all(),
								badge: formatNumber(getCountByStatus('all')),
								isActive: studentsStore.statusFilter === 'all',
								onclick: () => studentsStore.statusFilter = 'all'
							},
							{
								label: m.filter_active(),
								badge: formatNumber(getCountByStatus('active')),
								isActive: studentsStore.statusFilter === 'active',
								onclick: () => studentsStore.statusFilter = 'active'
							},
							{
								label: m.filter_withdrawn(),
								badge: formatNumber(getCountByStatus('withdrawn')),
								isActive: studentsStore.statusFilter === 'withdrawn',
								onclick: () => studentsStore.statusFilter = 'withdrawn'
							}
						]
					},
					{
						title: m.filter_classes(),
						items: [
							{
								label: m.filter_all_classes(),
								isActive: studentsStore.selectedClassId === null,
								onclick: () => studentsStore.selectedClassId = null
							},
							...studentsStore.classes.map(cls => {
								const count = getCountByClass(String(cls.id));
								return {
									label: String(cls.name || cls.name_ar || cls.id),
									badge: count > 0 ? formatNumber(count) : null,
									isActive: studentsStore.selectedClassId === cls.id,
									onclick: () => studentsStore.selectedClassId = String(cls.id)
								};
							})
						]
					}
				]}
			>
				{#snippet headerAction()}
					{#if hasActiveFilters}
						<Button 
							variant="ghost" 
							size="icon-xs" 
							class="h-5 w-5 text-muted-foreground hover:text-foreground cursor-pointer" 
							onclick={clearFilters}
							title={m.students_clear_filters()}
						>
							<FilterX size={12} />
						</Button>
					{/if}
				{/snippet}
			</SidebarNav>
		{/snippet}

		<!-- Middle Panel: Directory datagrid -->
		{#snippet middle()}
			<div class="h-full flex flex-col p-6 overflow-hidden gap-4">
				<div class="flex items-center justify-between shrink-0">
				<div class="relative w-full max-w-sm shrink-0">
					<Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						type="text"
						placeholder={m.students_search_placeholder()}
						class="input-sunken ps-9 text-xs py-2"
						bind:value={studentsStore.searchQuery}
					/>
				</div>

					<div class="flex items-center gap-2">
						{#if selectedStudentIds.size > 0}
							<Button
								variant="outline"
								class="text-xs h-9 px-3"
								onclick={() => printDialogOpen = true}
							>
								<Printer size={13} class="me-1.5" />
								<span>{m.print_templates()} ({selectedStudentIds.size})</span>
							</Button>
						{/if}
						<Button
							variant="outline"
							class="text-xs h-9 px-3"
							onclick={() => goto(localizeHref(`/${data.orgId}/bulk-input`))}
						>
							<Upload size={13} class="me-1.5" />
							{m.nav_bulk_input()}
						</Button>
						<Button
							variant="outline"
							class="text-xs h-9 px-3"
							onclick={() => toast.info(m.students_export_coming_soon())}
						>
							<Download size={13} class="me-1.5" />
							{m.students_export()}
						</Button>
						<Button 
							variant="lime" 
							class="text-xs h-9 px-3" 
							onclick={() => goto(localizeHref(`/${data.orgId}/students/new`))}
						>
							<Plus size={14} class="me-1.5" />
							{m.students_new()}
						</Button>
					</div>
				</div>

				<!-- Student Grid -->
				<div class="flex-1 min-h-0 flex flex-col">
					{#if studentsStore.isLoading}
						<div class="h-full w-full p-4 flex flex-col gap-3 bg-card border border-border rounded-lg">
							<Skeleton class="h-10 w-full" />
							<Skeleton class="h-8 w-full" />
							<Skeleton class="h-8 w-full" />
							<Skeleton class="h-8 w-full" />
							<Skeleton class="h-8 w-full" />
						</div>
					{:else if studentsStore.error}
						<div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3 bg-card border border-border rounded-lg">
							<p class="text-xs text-destructive font-semibold">{m.students_load_error({ error: studentsStore.error })}</p>
							<Button variant="outline" class="text-xs" onclick={() => studentsStore.load(data.orgId)}>{m.students_retry()}</Button>
						</div>
					{:else}
						<StudentTable
							students={studentsStore.filteredList}
							enableSelection={true}
							bind:selectedIds={selectedStudentIds}
						/>
					{/if}
				</div>
			</div>
		{/snippet}

	</MainViewport>

	{#snippet rightSnippet()}
		<div class="h-full border-l border-border bg-card/45 flex flex-col overflow-hidden">
			<StudentInspector 
				studentId={studentsStore.selectedId!} 
				orgId={data.orgId}
				onClose={() => studentsStore.selectedId = null} 
			/>
		</div>
	{/snippet}

	<PrintDialog
		bind:open={printDialogOpen}
		entityType="enrollment"
		entityIds={Array.from(selectedStudentIds)}
	/>

</div>

