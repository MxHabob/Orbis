<!--
@component StudentOrderingSettings
Student ranking engine settings page — rebuild and freeze student ranking for the school.
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
import { ListOrdered, RefreshCw, Lock, Search } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/services/ipc";
import { authStore } from "$lib/stores";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

let { data }: { data: { orgId: string } } = $props();

let students = $state<any[]>([]);
let cycles = $state<any[]>([]);
let isLoading = $state(true);
let isRebuilding = $state(false);
let searchQuery = $state("");
let selectedStrategy = $state("name_ar_asc");
let totalOrdered = $state(0);
let isFrozen = $state(false);
let currentCycleId = $state("");

const strategies = [
  { value: "name_ar_asc", label: m.exams_ordering_strategy_name() },
  { value: "hybrid", label: m.exams_ordering_strategy_hybrid() },
];

async function loadOrdering() {
  if (!currentCycleId) {
    students = [];
    totalOrdered = 0;
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

// Reload when currentCycleId changes
$effect(() => {
  if (currentCycleId) {
    loadOrdering();
  }
});

async function loadCyclesAndData() {
  isLoading = true;
  try {
    const resCycles = await commands.academics.getCycles(data.orgId);
    cycles = resCycles || [];
    if (cycles.length > 0) {
      currentCycleId = cycles[0].id;
    }
  } catch (e) {
    toast.error("Failed to load academic cycles");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadCyclesAndData();
  }
});

async function handleRebuild() {
  if (!currentCycleId) {
    toast.error("يجب تحديد الدورة الأكاديمية أولاً");
    return;
  }
  isRebuilding = true;
  try {
    const result = await commands.exams.rebuildOrdering(
      data.orgId,
      currentCycleId,
      selectedStrategy,
    );
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
  }),
);
</script>

<svelte:head>
  <title>{m.settings_ordering_window_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide select-none animate-in fade-in duration-200">

  <!-- Controls section -->
  <div class="flex flex-col lg:flex-row gap-6 justify-between items-stretch lg:items-end bg-card border border-border rounded-xl p-6">
    <div class="flex flex-wrap items-center gap-6">
      <!-- Cycle Selector -->
      <div class="flex flex-col gap-2 text-start">
        <span class="text-xs font-semibold text-muted-foreground">
          {m.exams_ordering_cycle()}
        </span>
        <Select type="single" value={currentCycleId} onValueChange={(v) => (currentCycleId = v ?? "")}>
          <SelectTrigger class="h-9 text-xs w-64 md:w-72 border border-border/80 bg-background text-foreground text-start flex items-center justify-between px-3">
            <span class="truncate me-2">
              {cycles.find((c) => String(c.id) === currentCycleId)?.name_ar || 
               cycles.find((c) => String(c.id) === currentCycleId)?.name || 
               "اختر الدورة الأكاديمية"}
            </span>
          </SelectTrigger>
          <SelectContent>
            {#each cycles as cycle (cycle.id)}
              <SelectItem value={String(cycle.id)} class="text-xs">
                {cycle.name_ar || cycle.name}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Strategy Selector -->
      <div class="flex flex-col gap-2 text-start">
        <span class="text-xs font-semibold text-muted-foreground">
          {m.exams_ordering_strategy()}
        </span>
        <Select type="single" value={selectedStrategy} onValueChange={(v) => (selectedStrategy = v ?? "")}>
          <SelectTrigger class="h-9 text-xs w-48 md:w-56 border border-border/80 bg-background text-foreground text-start flex items-center justify-between px-3">
            <span class="truncate me-2">
              {strategies.find((s) => s.value === selectedStrategy)?.label || selectedStrategy}
            </span>
          </SelectTrigger>
          <SelectContent>
            {#each strategies as s (s.value)}
              <SelectItem value={s.value} class="text-xs">
                {s.label}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Rebuild & Freeze buttons -->
    <div class="flex items-center gap-2 justify-end lg:self-end h-9 mt-4 lg:mt-0">
      <Button
        variant="outline"
        class="h-9 px-4 text-xs flex items-center gap-1.5 cursor-pointer font-medium"
        disabled={isRebuilding || !currentCycleId}
        onclick={handleRebuild}
      >
        <RefreshCw size={14} class={isRebuilding ? "animate-spin" : ""} />
        {m.exams_ordering_rebuild()}
      </Button>

      <Button
        variant="lime"
        class="h-9 px-4 text-xs flex items-center gap-1.5 cursor-pointer font-semibold"
        disabled={isFrozen || students.length === 0}
        onclick={handleFreeze}
      >
        <Lock size={14} />
        {m.exams_ordering_freeze()}
      </Button>
    </div>
  </div>

  <!-- Search -->
  <div class="relative">
    <Search size={14} class="absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
    <Input placeholder="بحث بالاسم أو الرقم أو الفصل..." class="ps-9 h-9 text-xs" bind:value={searchQuery} />
  </div>

  <!-- Table/Content Section -->
  {#if isLoading}
    <div class="flex flex-col gap-2">
      {#each Array(8) as _}
        <Skeleton class="h-10 w-full rounded-lg" />
      {/each}
    </div>
  {:else if filtered.length === 0}
    <div class="flex-1 flex flex-col items-center justify-center py-16 gap-3 text-center" in:fade={{ duration: 200 }}>
      <div class="h-14 w-14 rounded-2xl bg-muted/50 flex items-center justify-center">
        <ListOrdered size={24} class="text-muted-foreground" />
      </div>
      <p class="text-xs text-muted-foreground max-w-xs">
        اضغط "إعادة بناء الترتيب" لإنشاء ترتيب الطلاب للدورة المحددة.
      </p>
    </div>
  {:else}
    <div class="border border-border rounded-xl bg-card overflow-hidden" in:fade={{ delay: 50, duration: 200 }}>
      <Table>
        <TableHeader>
          <TableRow class="text-[10px]">
            <TableHead class="w-12 text-center">الترتيب</TableHead>
            <TableHead>اسم الطالب</TableHead>
            <TableHead class="w-32">الرقم الأكاديمي</TableHead>
            <TableHead class="w-28">الفصل</TableHead>
            <TableHead class="w-28">المستوى</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#each filtered as student, i (student.id || i)}
            <TableRow class="text-xs">
              <TableCell class="text-center">
                <span class="inline-flex items-center justify-center h-6 w-6 rounded-md bg-primary/10 text-primary text-[10px] font-bold font-mono">
                  {student.order_index || i + 1}
                </span>
              </TableCell>
              <TableCell class="font-medium text-start">{student.student_name_ar || "—"}</TableCell>
              <TableCell class="font-mono text-muted-foreground text-start">{student.academic_number || "—"}</TableCell>
              <TableCell class="text-start">{student.class_id || "—"}</TableCell>
              <TableCell class="text-start">{student.level || "—"}</TableCell>
            </TableRow>
          {/each}
        </TableBody>
      </Table>
    </div>
  {/if}
</div>
