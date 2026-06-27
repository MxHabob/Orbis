<!--
@component StudentOrdering
Student ordering engine page — rebuild and freeze student ordering.
-->
<script lang="ts">
import { onMount } from "svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui-svelte/table";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import {
  ListOrdered,
  RefreshCw,
  Lock,
  Search,
  ArrowUpDown,
  Hash,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/services/ipc";
import { authStore } from "$lib/stores";

let { data }: { data: { orgId: string } } = $props();

let students = $state<any[]>([]);
let isLoading = $state(true);
let isRebuilding = $state(false);
let searchQuery = $state("");
let selectedStrategy = $state("name_ar_asc");
let totalOrdered = $state(0);
let isFrozen = $state(false);
let currentCycleId = $state(""); // TODO: get from active academic cycle

const strategies = [
  { value: "name_ar_asc", label: m.exams_ordering_strategy_name() },
  { value: "hybrid", label: m.exams_ordering_strategy_hybrid() },
];

async function loadOrdering() {
  if (!currentCycleId) {
    isLoading = false;
    return;
  }
  isLoading = true;
  try {
    const result = await commands.exams.getOrdering(data.orgId, currentCycleId);
    students = result as any[];
    totalOrdered = students.length;
  } catch (e) {
    // No ordering yet — that's ok
    students = [];
    totalOrdered = 0;
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadOrdering();
  }
});

async function handleRebuild() {
  if (!currentCycleId) {
    toast.error("يجب تحديد الدورة الأكاديمية أولاً");
    return;
  }
  isRebuilding = true;
  try {
    const result = await commands.exams.rebuildOrdering(data.orgId, currentCycleId, selectedStrategy);
    toast.success(`تم إعادة بناء الترتيب — استراتيجية: ${selectedStrategy}`);
    await loadOrdering();
  } catch (e: any) {
    toast.error(e?.message || "Failed to rebuild ordering");
  } finally {
    isRebuilding = false;
  }
}

async function handleFreeze() {
  if (!currentCycleId) return;
  try {
    const userId = authStore.userId || authStore.user?.id || "desktop_user";
    await commands.exams.freezeOrdering(data.orgId, currentCycleId, userId);
    toast.success("تم تجميد الترتيب بنجاح");
    isFrozen = true;
  } catch (e: any) {
    toast.error(e?.message || "Failed to freeze ordering");
  }
}

const filtered = $derived(
  students.filter((s) => {
    const q = searchQuery.toLowerCase();
    return (
      (s.student_name_ar || "").toLowerCase().includes(q) ||
      (s.academic_number || "").toLowerCase().includes(q) ||
      (s.class_id || "").toLowerCase().includes(q)
    );
  })
);
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-5 select-none">
	<!-- Header -->
	<div class="card-tactile bg-gradient-to-br from-card to-card/50 p-5 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4" in:fade={{ duration: 150 }}>
		<div class="flex items-center gap-3">
			<div class="h-10 w-10 rounded-xl bg-emerald-500/10 text-emerald-500 flex items-center justify-center shrink-0">
				<ListOrdered size={20} />
			</div>
			<div class="flex flex-col">
				<h1 class="text-sm font-bold text-foreground">{m.exams_ordering_title()}</h1>
				{#if totalOrdered > 0}
					<p class="text-[10px] text-muted-foreground mt-0.5">
						{m.exams_ordering_result({ count: totalOrdered })}
						{#if isFrozen}
							<span class="ms-1.5 text-amber-500 font-bold">🔒 مجمد</span>
						{/if}
					</p>
				{/if}
			</div>
		</div>

		<div class="flex items-center gap-2">
			<!-- Strategy Selector -->
			<select
				bind:value={selectedStrategy}
				class="h-8 px-2 text-[10px] rounded-lg border border-border bg-background text-foreground focus:ring-1 focus:ring-primary"
			>
				{#each strategies as s}
					<option value={s.value}>{s.label}</option>
				{/each}
			</select>

			<Button
				variant="outline"
				class="h-8 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
				disabled={isRebuilding}
				onclick={handleRebuild}
			>
				<RefreshCw size={12} class={isRebuilding ? "animate-spin" : ""} />
				{m.exams_ordering_rebuild()}
			</Button>

			<Button
				variant="default"
				class="h-8 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
				disabled={isFrozen || students.length === 0}
				onclick={handleFreeze}
			>
				<Lock size={12} />
				{m.exams_ordering_freeze()}
			</Button>
		</div>
	</div>

	<!-- Search -->
	<div class="relative">
		<Search size={14} class="absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
		<Input placeholder="بحث بالاسم أو الرقم..." class="ps-9 h-9 text-xs" bind:value={searchQuery} />
	</div>

	<!-- Table -->
	{#if isLoading}
		<div class="flex flex-col gap-2">
			{#each Array(8) as _}
				<Skeleton class="h-10 w-full rounded-lg" />
			{/each}
		</div>
	{:else if filtered.length === 0}
		<div class="flex-1 flex flex-col items-center justify-center py-16 gap-3 text-center" in:fade={{ duration: 200 }}>
			<div class="h-14 w-14 rounded-2xl bg-muted/50 flex items-center justify-center">
				<ArrowUpDown size={24} class="text-muted-foreground" />
			</div>
			<p class="text-xs text-muted-foreground max-w-xs">
				اضغط "إعادة بناء الترتيب" لإنشاء ترتيب الطلاب
			</p>
		</div>
	{:else}
		<div class="card-tactile bg-card overflow-hidden" in:fade={{ delay: 50, duration: 200 }}>
			<Table>
				<TableHeader>
					<TableRow class="text-[10px]">
						<TableHead class="w-12">الترتيب</TableHead>
						<TableHead>اسم الطالب</TableHead>
						<TableHead class="w-24">الرقم الأكاديمي</TableHead>
						<TableHead class="w-20">الفصل</TableHead>
						<TableHead class="w-20">المستوى</TableHead>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each filtered as student, i (student.id || i)}
						<TableRow class="text-xs">
							<TableCell>
								<span class="inline-flex items-center justify-center h-6 w-6 rounded-md bg-primary/10 text-primary text-[10px] font-bold font-mono">
									{student.order_index || i + 1}
								</span>
							</TableCell>
							<TableCell class="font-medium">{student.student_name_ar || "—"}</TableCell>
							<TableCell class="font-mono text-muted-foreground">{student.academic_number || "—"}</TableCell>
							<TableCell>{student.class_id || "—"}</TableCell>
							<TableCell>{student.level || "—"}</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/if}
</div>
