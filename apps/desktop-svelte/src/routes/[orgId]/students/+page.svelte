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
import { Plus, Search, FilterX, Download } from "lucide-svelte";
import { studentsStore } from "$lib/stores";
import StudentSidebarFilters from "$lib/components/students/student-sidebar-filters.svelte";
import StudentTable from "$lib/components/students/student-table.svelte";
import StudentInspector from "$lib/components/students/student-inspector.svelte";
import { toast } from "svelte-sonner";
import { goto } from "$app/navigation";

let { data }: { data: { orgId: string } } = $props();

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
</script>

<svelte:head>
	<title>دليل الطلاب | Orbis</title>
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
			<div class="h-full border-r border-border bg-card/20 p-4 flex flex-col gap-4 overflow-y-auto scrollbar-hide">
				<div class="flex items-center justify-between">
					<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider">لوحة التصفية</span>
					{#if hasActiveFilters}
						<Button 
							variant="ghost" 
							size="icon" 
							class="h-6 w-6 text-muted-foreground hover:text-foreground" 
							onclick={clearFilters}
							title="مسح التصفية"
						>
							<FilterX size={13} />
						</Button>
					{/if}
				</div>
				<StudentSidebarFilters />
			</div>
		{/snippet}

		<!-- Middle Panel: Directory datagrid -->
		{#snippet middle()}
			<div class="h-full flex flex-col p-6 overflow-hidden gap-4">
				<div class="flex items-center justify-between shrink-0">
				<div class="relative w-full max-w-sm shrink-0">
					<Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						type="text"
						placeholder="البحث بالاسم، الرقم الأكاديمي..."
						class="input-sunken ps-9 text-xs py-2"
						bind:value={studentsStore.searchQuery}
					/>
				</div>

					<div class="flex items-center gap-2">
						<Button
							variant="ghost"
							class="btn-tactile text-xs h-9 px-3"
							onclick={() => toast.info("ميزة التصدير قيد التحضير")}
						>
							<Download size={13} class="me-1.5" />
							تصدير
						</Button>
						<Button 
							variant="lime" 
							class="btn-tactile-lime text-xs h-9 px-3" 
							onclick={() => goto(`/${data.orgId}/students/new`)}
						>
							<Plus size={14} class="me-1.5" />
							طالب جديد
						</Button>
					</div>
				</div>

				<!-- Student Grid -->
				<div class="flex-1 min-h-0 bg-card rounded-lg border border-border overflow-hidden">
					{#if studentsStore.isLoading}
						<div class="h-full w-full p-4 flex flex-col gap-3">
							<Skeleton class="h-10 w-full" />
							<Skeleton class="h-8 w-full" />
							<Skeleton class="h-8 w-full" />
							<Skeleton class="h-8 w-full" />
							<Skeleton class="h-8 w-full" />
						</div>
					{:else if studentsStore.error}
						<div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
							<p class="text-xs text-destructive font-semibold">فشل تحميل البيانات: {studentsStore.error}</p>
							<Button variant="ghost" class="btn-tactile text-xs" onclick={() => studentsStore.load(data.orgId)}>إعادة المحاولة</Button>
						</div>
					{:else}
						<StudentTable students={studentsStore.filteredList} />
					{/if}
				</div>
			</div>
		{/snippet}

	</MainViewport>

	{#snippet rightSnippet()}
		<div class="h-full border-l border-border bg-card/45 flex flex-col overflow-hidden">
			<StudentInspector 
				studentId={studentsStore.selectedId!} 
				onClose={() => studentsStore.selectedId = null} 
			/>
		</div>
	{/snippet}

</div>
