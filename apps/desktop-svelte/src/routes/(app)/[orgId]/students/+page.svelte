<!--
@component StudentsDirectoryPage
Main workspace for students. Integrates Left sidebar filters, middle data table,
and right detailed student inspector.
-->
<script lang="ts">
import MainViewport from "$lib/shell/main-viewport.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import {
  Plus,
  Search,
  FilterX,
  Download,
  Printer,
  Upload,
  ShieldAlert,
} from "lucide-svelte";
import { studentsStore, createEnrollmentsQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import { academicContext, authStore } from "$lib/stores";
import StudentsTableSkeleton from "$lib/shared/skeletons/students-table-skeleton.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import SidebarNav from "$lib/shell/sidebar-nav.svelte";
import StudentTable from "$lib/features/students/student-table.svelte";
import StudentInspector from "$lib/features/students/student-inspector.svelte";
import { toast } from "svelte-sonner";
import { goto } from "$app/navigation";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/core/ipc";
import { debounce } from "$lib/shared/utils/debounce";

let { data }: { data: { orgId: string } } = $props();

// TanStack Queries
const enrollmentsQuery = createEnrollmentsQuery(data.orgId);
const classesQuery = createClassesQuery(data.orgId);

// Derived data from queries
const enrollments = $derived($enrollmentsQuery.data ?? []);
const classes = $derived($classesQuery.data ?? []);
const filteredStudents = $derived(studentsStore.filterList(enrollments));

// Capability-based enrollment
const canCreateStudent = $derived(
  !academicContext.isReadOnly && academicContext.canEnroll,
);
const canRequestLateEnrollment = $derived(
  !academicContext.isReadOnly &&
    !academicContext.canEnroll &&
    ["admin", "registrar", "super_admin"].includes(authStore.user?.role ?? ""),
);
let showOverrideDialog = $state(false);
const isLoading = $derived(
  $enrollmentsQuery.isLoading || $classesQuery.isLoading,
);
const error = $derived($enrollmentsQuery.error || $classesQuery.error);

// Helper to count students matching a criteria
function getCountByClass(classId: string): number {
  return enrollments.filter((s) => s.class_id === classId).length;
}

function getCountByStatus(status: string): number {
  if (status === "all") return enrollments.length;
  return enrollments.filter(
    (s) => String(s.status || "").toLowerCase() === status,
  ).length;
}

// Local search input with debounce to prevent excessive $derived recalculations
let localSearch = $state("");
const updateSearch = debounce((q: string) => {
  studentsStore.searchQuery = q;
}, 150);

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
							...classes.map(cls => {
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
					bind:value={localSearch}
					oninput={() => updateSearch(localSearch)}
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
						{#if canCreateStudent}
							<Button 
								variant="lime" 
								class="text-xs h-9 px-3" 
								onclick={() => goto(localizeHref(`/${data.orgId}/students/new`))}
							>
								<Plus size={14} class="me-1.5" />
								{m.students_new()}
							</Button>
						{:else if canRequestLateEnrollment}
							<Button 
								variant="outline" 
								class="text-xs h-9 px-3" 
								onclick={() => { showOverrideDialog = true; }}
							>
								<ShieldAlert size={14} class="me-1.5" />
								{m.students_request_late_enrollment ? m.students_request_late_enrollment() : "Request Late Enrollment"}
							</Button>
						{/if}
					</div>
				</div>

				<!-- Student Grid -->
				<div class="flex-1 min-h-0 flex flex-col">
					{#if isLoading}
						<StudentsTableSkeleton />
					{:else if error}
						<div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3 bg-card border border-border rounded-lg">
							<p class="text-xs text-destructive font-semibold">{m.students_load_error({ error: extractErrorMessage(error) })}</p>
							<Button variant="outline" class="text-xs" onclick={() => { $enrollmentsQuery.refetch(); $classesQuery.refetch(); }}>{m.students_retry()}</Button>
						</div>
					{:else}
						<StudentTable
							students={filteredStudents}
							classes={classes}
							enableSelection={true}
							bind:selectedIds={selectedStudentIds}
							orgId={data.orgId}
						/>
					{/if}
				</div>
			</div>
		{/snippet}

	</MainViewport>

	{#snippet rightSnippet()}
		<div class="h-full border-l border-border bg-card/45 flex flex-col overflow-hidden">
			<StudentInspector 
				student={enrollments.find(e => e.id === studentsStore.selectedId)} 
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

