<!--
@component StudentTable
Renders the list of students in a high-density, interactive data table.
-->
<script lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import { studentsStore } from "$lib/stores/students.svelte";
import type { Enrollment } from "$lib/services/ipc";
import { ArrowUpRight, GraduationCap } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";

let { students }: { students: Enrollment[] } = $props();

function getClassName(classId: string): string {
  const cls = studentsStore.classes.find((c) => c.id === classId);
  return cls ? String(cls.name || cls.name_ar || classId) : m.table_not_enrolled();
}

function handleSelect(studentId: string) {
  studentsStore.selectedId = studentId;
}
</script>

<div class="w-full h-full overflow-auto scrollbar-hide select-none">
	{#if students.length === 0}
		<div class="h-full w-full flex flex-col items-center justify-center p-12 text-center text-muted-foreground gap-3">
			<GraduationCap size={32} class="text-muted-foreground/30" />
			<div class="flex flex-col gap-1">
				<p class="text-xs font-semibold">{m.table_no_students()}</p>
				<p class="text-[10px] text-muted-foreground/80">{m.table_no_students_hint()}</p>
			</div>
		</div>
	{:else}
		<Table class="text-xs">
			<TableHeader class="bg-card sticky top-0 z-10">
				<TableRow class="hover:bg-transparent">
					<TableHead class="text-start">{m.table_student()}</TableHead>
					<TableHead class="text-start">{m.table_student_number()}</TableHead>
					<TableHead class="text-start">{m.table_class()}</TableHead>
					<TableHead class="text-start">{m.table_email()}</TableHead>
					<TableHead class="text-start">{m.table_status()}</TableHead>
					<TableHead class="text-center w-12"></TableHead>
				</TableRow>
			</TableHeader>
			<TableBody>
				{#each students as student (student.id)}
					{@const isSelected = studentsStore.selectedId === student.id}
					<TableRow
						class="cursor-pointer transition-colors duration-150 {isSelected ? 'bg-primary/5 hover:bg-primary/8' : 'hover:bg-accent/40'}"
						onclick={() => handleSelect(String(student.id))}
					>
						<TableCell class="font-medium">
							<div class="flex items-center gap-3">
								<Avatar class="h-8 w-8 ring-1 ring-border">
									<AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
									<AvatarFallback class="bg-primary/5 text-primary text-[10px] font-bold">
										{String(student.full_name || "S").slice(0, 1)}
									</AvatarFallback>
								</Avatar>
								<div class="flex flex-col text-start">
									<span class="font-bold text-foreground">{student.full_name || student.name_ar || ''}</span>
									{#if student.name_en}
										<span class="text-[10px] text-muted-foreground font-mono">{student.name_en}</span>
									{/if}
								</div>
							</div>
						</TableCell>
						<TableCell class="font-mono text-start">{student.student_number || student.id || '-'}</TableCell>
						<TableCell class="text-start">{getClassName(String(student.class_id))}</TableCell>
						<TableCell class="text-start font-mono text-muted-foreground">{student.email || '-'}</TableCell>
						<TableCell class="text-start">
							{#if String(student.status).toLowerCase() === 'active'}
								<span class="badge-brand-lime py-0.5 px-2 text-[10px]">{m.status_active()}</span>
							{:else if String(student.status).toLowerCase() === 'withdrawn'}
								<span class="badge-brand-destructive py-0.5 px-2 text-[10px]">{m.status_withdrawn()}</span>
							{:else}
								<span class="badge-brand-default py-0.5 px-2 text-[10px]">{student.status || m.status_undefined()}</span>
							{/if}
						</TableCell>
						<TableCell class="text-center">
							<div class="flex items-center justify-center text-muted-foreground/60 hover:text-primary transition-colors">
								<ArrowUpRight size={14} />
							</div>
						</TableCell>
					</TableRow>
				{/each}
			</TableBody>
		</Table>
	{/if}
</div>

