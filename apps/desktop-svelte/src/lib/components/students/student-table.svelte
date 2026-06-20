<!--
@component StudentTable
Renders the list of students using the DataTable infrastructure.
Matches the column layout and styling of the React dashboard precisely.
-->
<script lang="ts">
import { DataTable, type ColumnDef, type CellContext } from "$lib/components/table";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import { studentsStore } from "$lib/stores/students.svelte";
import type { Enrollment } from "$lib/services/ipc";
import { ArrowUpRight, GraduationCap } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { formatDate } from "$lib/i18n-utils";

let { students }: { students: Enrollment[] } = $props();

// ── Helpers ────────────────────────────────────────────────────────

function getClassName(classId: string): string {
	const cls = studentsStore.classes.find((c) => c.id === classId);
	return cls ? String(cls.name || cls.name_ar || classId) : m.table_not_enrolled();
}

// ── Column Definitions ─────────────────────────────────────────────

const columns: ColumnDef<Enrollment>[] = [
	{
		id: "name",
		header: m.table_student(),
		accessorKey: "full_name",
		size: 250,
		minSize: 200,
		maxSize: 400,
		sticky: true,
		sortable: true,
		skeleton: { type: "avatar-text", width: "w-32" },
		cell: nameCell,
	},
	{
		id: "studentNumber",
		header: m.table_student_number(),
		accessorKey: "student_number",
		size: 150,
		minSize: 100,
		maxSize: 200,
		sortable: true,
		skeleton: { type: "text", width: "w-20" },
		className: "font-mono",
		cell: studentNumberCell,
	},
	{
		id: "class",
		header: m.table_class(),
		accessorFn: (row) => getClassName(String(row.class_id)),
		size: 150,
		minSize: 100,
		maxSize: 200,
		sortable: true,
		skeleton: { type: "text", width: "w-24" },
	},
	{
		id: "status",
		header: m.table_status(),
		accessorKey: "status",
		size: 120,
		minSize: 100,
		maxSize: 150,
		sortable: true,
		skeleton: { type: "badge", width: "w-16" },
		cell: statusCell,
	},
	{
		id: "sex",
		header: m.form_sex(),
		accessorKey: "sex",
		size: 100,
		minSize: 80,
		maxSize: 120,
		sortable: true,
		skeleton: { type: "text", width: "w-12" },
		cell: sexCell,
	},
	{
		id: "enrollmentDate",
		header: "Enrolled At",
		accessorKey: "enrollment_date",
		size: 150,
		minSize: 120,
		maxSize: 200,
		sortable: true,
		skeleton: { type: "text", width: "w-24" },
		cell: enrollmentDateCell,
	},
];
</script>

<!-- ── Custom Cell Snippets ──────────────────────────────────────── -->

{#snippet nameCell(ctx: CellContext<Enrollment>)}
	{@const student = ctx.row}
	<div class="flex items-center gap-2.5">
		<Avatar class="h-7 w-7 ring-1 ring-border flex-shrink-0">
			<AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
			<AvatarFallback class="bg-primary/5 text-primary text-[9px] font-bold">
				{String(student.full_name || "S").slice(0, 1)}
			</AvatarFallback>
		</Avatar>
		<div class="flex flex-col text-start min-w-0">
			<span class="text-xs font-bold text-foreground truncate">{student.full_name || student.name_ar || ''}</span>
			{#if student.name_en}
				<span class="text-[10px] text-muted-foreground font-mono truncate">{student.name_en}</span>
			{/if}
		</div>
	</div>
{/snippet}

{#snippet studentNumberCell(ctx: CellContext<Enrollment>)}
	<span class="inline-flex items-center rounded-md border border-border px-2 py-0.5 text-[10px] font-mono font-medium text-foreground bg-background">
		{ctx.row.student_number || ctx.row.id || '-'}
	</span>
{/snippet}

{#snippet statusCell(ctx: CellContext<Enrollment>)}
	{@const status = String(ctx.row.status || '').toLowerCase()}
	{#if status === 'active'}
		<span class="badge-brand-lime py-0.5 px-2 text-[10px]">{m.status_active()}</span>
	{:else if status === 'withdrawn'}
		<span class="badge-brand-destructive py-0.5 px-2 text-[10px]">{m.status_withdrawn()}</span>
	{:else}
		<span class="badge-brand-default py-0.5 px-2 text-[10px]">{ctx.row.status || m.status_undefined()}</span>
	{/if}
{/snippet}

{#snippet sexCell(ctx: CellContext<Enrollment>)}
	{@const sex = String(ctx.row.sex || '').toLowerCase()}
	<span class="capitalize text-xs">
		{#if sex === 'male'}
			{m.form_male()}
		{:else if sex === 'female'}
			{m.form_female()}
		{:else}
			{ctx.row.sex || '-'}
		{/if}
	</span>
{/snippet}

{#snippet enrollmentDateCell(ctx: CellContext<Enrollment>)}
	{@const date = ctx.row.enrollment_date as string | undefined}
	<span class="text-xs text-muted-foreground">
		{#if date}
			{formatDate(date)}
		{:else}
			-
		{/if}
	</span>
{/snippet}

<!-- ── DataTable ─────────────────────────────────────────────────── -->

<DataTable
	{columns}
	data={students}
	tableId="students"
	getRowId={(s) => String(s.id)}
	height="100%"
	enableSorting={true}
	enableSelection={false}
	enableVirtualization={students.length > 100}
	enableExport={false}
	enableColumnToggle={false}
	enablePersistence={true}
	onRowClick={(row) => studentsStore.selectedId = String(row.id)}
>
	{#snippet empty()}
		<div class="flex flex-col items-center justify-center gap-3 text-muted-foreground">
			<GraduationCap size={32} class="text-muted-foreground/30" />
			<div class="flex flex-col gap-1 text-center">
				<p class="text-xs font-semibold">{m.table_no_students()}</p>
				<p class="text-[10px] text-muted-foreground/80">{m.table_no_students_hint()}</p>
			</div>
		</div>
	{/snippet}

	{#snippet rowActions({ row })}
		<div class="flex items-center justify-center text-muted-foreground/60 hover:text-primary transition-colors">
			<ArrowUpRight size={14} />
		</div>
	{/snippet}
</DataTable>
