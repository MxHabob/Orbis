<!--
@component StudentSidebarFilters
Renders the left panel filtering list including status items and classes list.
-->
<script lang="ts">
import { studentsStore } from "$lib/stores/students.svelte";
import {
  Users,
  GraduationCap,
  ShieldCheck,
  ShieldAlert,
  ListFilter,
} from "lucide-svelte";

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
</script>

<div class="flex flex-col gap-6 w-full text-xs">
	<!-- Status Filters Group -->
	<div class="flex flex-col gap-1.5">
		<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider px-2">الحالة الأكاديمية</span>
		
		<button
			class="flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer"
			class:nav-item-active={studentsStore.statusFilter === 'all'}
			class:nav-item={studentsStore.statusFilter !== 'all'}
			onclick={() => studentsStore.statusFilter = 'all'}
		>
			<div class="flex items-center gap-2">
				<ListFilter size={14} />
				<span>الكل</span>
			</div>
			<span class="text-[10px] px-1.5 py-0.5 rounded bg-muted/60 text-muted-foreground font-mono">
				{getCountByStatus('all')}
			</span>
		</button>

		<button
			class="flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer"
			class:nav-item-active={studentsStore.statusFilter === 'active'}
			class:nav-item={studentsStore.statusFilter !== 'active'}
			onclick={() => studentsStore.statusFilter = 'active'}
		>
			<div class="flex items-center gap-2">
				<ShieldCheck size={14} class="text-emerald-500" />
				<span>الطلاب النشطين</span>
			</div>
			<span class="text-[10px] px-1.5 py-0.5 rounded bg-muted/60 text-muted-foreground font-mono">
				{getCountByStatus('active')}
			</span>
		</button>

		<button
			class="flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer"
			class:nav-item-active={studentsStore.statusFilter === 'withdrawn'}
			class:nav-item={studentsStore.statusFilter !== 'withdrawn'}
			onclick={() => studentsStore.statusFilter = 'withdrawn'}
		>
			<div class="flex items-center gap-2">
				<ShieldAlert size={14} class="text-amber-500" />
				<span>الطلاب المنسحبين</span>
			</div>
			<span class="text-[10px] px-1.5 py-0.5 rounded bg-muted/60 text-muted-foreground font-mono">
				{getCountByStatus('withdrawn')}
			</span>
		</button>
	</div>

	<!-- Classes Filters Group -->
	<div class="flex flex-col gap-1.5">
		<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider px-2">الصفوف الدراسية</span>

		<button
			class="flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer"
			class:nav-item-active={studentsStore.selectedClassId === null}
			class:nav-item={studentsStore.selectedClassId !== null}
			onclick={() => studentsStore.selectedClassId = null}
		>
			<div class="flex items-center gap-2">
				<Users size={14} />
				<span>كل الصفوف</span>
			</div>
		</button>

		{#if studentsStore.classes.length === 0}
			<p class="text-[11px] text-muted-foreground px-2 italic mt-1">لا توجد صفوف مسجلة حالياً.</p>
		{:else}
			{#each studentsStore.classes as cls (cls.id)}
				{@const count = getCountByClass(String(cls.id))}
				<button
					class="flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer"
					class:nav-item-active={studentsStore.selectedClassId === cls.id}
					class:nav-item={studentsStore.selectedClassId !== cls.id}
					onclick={() => studentsStore.selectedClassId = String(cls.id)}
				>
					<div class="flex items-center gap-2">
						<GraduationCap size={14} class={studentsStore.selectedClassId === cls.id ? 'text-primary' : 'text-muted-foreground'} />
						<span>{cls.name || cls.name_ar || cls.id}</span>
					</div>
					{#if count > 0}
						<span class="text-[10px] px-1.5 py-0.5 rounded bg-muted/60 text-muted-foreground font-mono">
							{count}
						</span>
					{/if}
				</button>
			{/each}
		{/if}
	</div>
</div>
