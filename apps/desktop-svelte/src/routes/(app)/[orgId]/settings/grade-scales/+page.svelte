<!--
@component GradeScalesSettingsPage
Manages organization grade scales (percentage, GPA, pass/fail) and their items.
Uses the Assessment Engine's grade_scale CRUD operations.
-->
<script lang="ts">
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Separator } from "@orbis/ui-svelte/separator";
import { toast } from "svelte-sonner";
import {
  Scale,
  Plus,
  Trash2,
  Loader2,
  ChevronRight,
  Award,
  Pencil,
} from "@lucide/svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import GradeScaleDialog from "$lib/features/settings/components/grade-scale-dialog.svelte";
import GradeScaleItemDialog from "$lib/features/settings/components/grade-scale-item-dialog.svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);
const queryClient = useQueryClient();

// ── Queries ──────────────────────────────────────────────────
const scalesQuery = createQuery(() => ({
  queryKey: ["grade-scales", orgId],
  queryFn: () => commands.assessment.listGradeScales(),
}));

let selectedScaleId = $state<string | null>(null);
const selectedScale = $derived(
  scalesQuery.data?.find((s: any) => s.id === selectedScaleId) ?? null,
);

const itemsQuery = createQuery(() => ({
  queryKey: ["grade-scale-items", selectedScaleId],
  queryFn: () => (selectedScaleId ? commands.assessment.listGradeScaleItems(selectedScaleId) : []),
  enabled: !!selectedScaleId,
}));

// ── Create Scale Dialog ──────────────────────────────────────
let showCreateDialog = $state(false);
let newScaleName = $state("");
let newScaleType = $state("percentage");
let isCreating = $state(false);

async function handleCreateScale() {
  if (!newScaleName.trim()) return;
  isCreating = true;
  try {
    await commands.assessment.createGradeScale({
      name: newScaleName,
      type: newScaleType,
    });
    toast.success("تم إنشاء مقياس الدرجات بنجاح");
    queryClient.invalidateQueries({ queryKey: ["grade-scales"] });
    showCreateDialog = false;
    newScaleName = "";
    newScaleType = "percentage";
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isCreating = false;
  }
}

// ── Create Item Dialog ───────────────────────────────────────
let showItemDialog = $state(false);
let newItem = $state({ grade_label: "", min_score: 0, max_score: 100, gpa_points: 0, is_passing: true });
let isCreatingItem = $state(false);

async function handleCreateItem() {
  if (!selectedScaleId || !newItem.grade_label.trim()) return;
  isCreatingItem = true;
  try {
    await commands.assessment.createGradeScaleItem({
      scale_id: selectedScaleId,
      ...newItem,
    });
    toast.success("تم إضافة عنصر التقدير بنجاح");
    queryClient.invalidateQueries({ queryKey: ["grade-scale-items"] });
    showItemDialog = false;
    newItem = { grade_label: "", min_score: 0, max_score: 100, gpa_points: 0, is_passing: true };
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isCreatingItem = false;
  }
}

// ── Delete ───────────────────────────────────────────────────
async function handleDeleteScale(id: string) {
  try {
    await commands.assessment.deleteGradeScale(id);
    toast.success("تم حذف مقياس الدرجات");
    if (selectedScaleId === id) selectedScaleId = null;
    queryClient.invalidateQueries({ queryKey: ["grade-scales"] });
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  }
}

async function handleDeleteItem(id: string) {
  try {
    await commands.assessment.deleteGradeScaleItem(id);
    toast.success("تم حذف عنصر التقدير");
    queryClient.invalidateQueries({ queryKey: ["grade-scale-items"] });
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  }
}

function getTypeLabel(t: string) {
  switch (t) {
    case "percentage": return "نسبة مئوية";
    case "gpa_4_0": return "GPA 4.0";
    case "pass_fail": return "ناجح/راسب";
    default: return t;
  }
}
</script>

<svelte:head>
  <title>مقاييس الدرجات | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide bg-background">
  {#if scalesQuery.isLoading}
    <SettingsFormSkeleton />
  {:else if scalesQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>فشل تحميل مقاييس الدرجات</p>
      <Button variant="outline" class="mt-4 cursor-pointer" onclick={() => scalesQuery.refetch()}>
        إعادة المحاولة
      </Button>
    </div>
  {:else}
    <!-- Header -->
    <div class="flex items-center justify-between pb-5 border-b border-border/40 w-full select-none">
      <div class="flex items-center gap-2">
        <Scale size={18} class="text-primary" />
        <h1 class="text-sm font-semibold text-foreground">مقاييس الدرجات والتقديرات</h1>
      </div>
      <Button size="sm" class="px-4 h-8 text-xs cursor-pointer" onclick={() => (showCreateDialog = true)}>
        <Plus size={14} class="me-1" />
        إضافة مقياس
      </Button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <!-- Scales List (Left Panel) -->
      <div class="flex flex-col gap-2">
        {#each scalesQuery.data ?? [] as scale (scale.id)}
          <div
            role="button"
            tabindex="0"
            class="text-start w-full px-4 py-3 rounded-lg border transition-all cursor-pointer {selectedScaleId === scale.id ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/40 hover:border-border hover:bg-muted/30'}"
            onclick={() => (selectedScaleId = scale.id)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && (selectedScaleId = scale.id)}
          >
            <div class="flex items-center justify-between">
              <div class="flex flex-col gap-0.5">
                <span class="text-xs font-semibold text-foreground">{scale.name}</span>
                <Badge variant="outline" class="text-[10px] w-fit">{getTypeLabel(scale.type)}</Badge>
              </div>
              <div class="flex items-center gap-1">
                <button
                  class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                  onclick={(e) => { e.stopPropagation(); handleDeleteScale(scale.id); }}
                >
                  <Trash2 size={12} />
                </button>
                <ChevronRight size={14} class="text-muted-foreground" />
              </div>
            </div>
          </div>
        {:else}
          <div class="text-center py-8 text-muted-foreground text-xs">
            لا توجد مقاييس. أضف مقياس جديد للبدء.
          </div>
        {/each}
      </div>

      <!-- Items Panel (Right) -->
      <div class="md:col-span-2">
        {#if selectedScale}
          <div class="border border-border/40 rounded-lg p-5">
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center gap-2">
                <Award size={16} class="text-primary" />
                <h2 class="text-sm font-semibold">{selectedScale.name}</h2>
                <Badge variant="secondary" class="text-[10px]">{getTypeLabel(selectedScale.type)}</Badge>
              </div>
              <Button size="sm" variant="outline" class="h-7 text-[10px] cursor-pointer" onclick={() => (showItemDialog = true)}>
                <Plus size={12} class="me-1" />
                إضافة تقدير
              </Button>
            </div>

            <Separator class="mb-4 bg-border/40" />

            {#if itemsQuery.isLoading}
              <div class="flex items-center justify-center py-8">
                <Loader2 size={20} class="animate-spin text-muted-foreground" />
              </div>
            {:else}
              <div class="overflow-x-auto">
                <table class="w-full text-xs">
                  <thead>
                    <tr class="border-b border-border/40 text-muted-foreground">
                      <th class="text-start py-2 font-medium">التقدير</th>
                      <th class="text-center py-2 font-medium">أدنى درجة</th>
                      <th class="text-center py-2 font-medium">أعلى درجة</th>
                      <th class="text-center py-2 font-medium">نقاط GPA</th>
                      <th class="text-center py-2 font-medium">ناجح</th>
                      <th class="text-center py-2 font-medium"></th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each itemsQuery.data ?? [] as item (item.id)}
                      <tr class="border-b border-border/20 hover:bg-muted/20 transition-colors">
                        <td class="py-2.5 font-semibold text-foreground">{item.grade_label}</td>
                        <td class="py-2.5 text-center font-mono">{item.min_score}</td>
                        <td class="py-2.5 text-center font-mono">{item.max_score}</td>
                        <td class="py-2.5 text-center font-mono">{item.gpa_points}</td>
                        <td class="py-2.5 text-center">
                          <Badge variant={item.is_passing ? "default" : "destructive"} class="text-[9px]">
                            {item.is_passing ? "ناجح" : "راسب"}
                          </Badge>
                        </td>
                        <td class="py-2.5 text-center">
                          <button
                            class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                            onclick={() => handleDeleteItem(item.id)}
                          >
                            <Trash2 size={12} />
                          </button>
                        </td>
                      </tr>
                    {:else}
                      <tr>
                        <td colspan="6" class="text-center py-6 text-muted-foreground">
                          لا توجد تقديرات. أضف عنصر تقدير جديد.
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>
        {:else}
          <div class="flex items-center justify-center h-full text-muted-foreground text-xs py-16">
            اختر مقياس من القائمة لعرض التقديرات
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<GradeScaleDialog bind:open={showCreateDialog} bind:name={newScaleName} bind:scaleType={newScaleType} isCreating={isCreating} onsubmit={handleCreateScale} />
<GradeScaleItemDialog bind:open={showItemDialog} item={newItem} isCreating={isCreatingItem} onsubmit={handleCreateItem} />
