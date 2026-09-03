<!--
@component AssessmentSchemesSettingsPage
Manages assessment schemes (weighted grade composition structures).
Each scheme has components (homework, quizzes, exams, etc.) with weights.
Uses the Assessment Engine's assessment_scheme CRUD operations.
-->
<script lang="ts">
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Separator } from "@orbis/ui/separator";
import { toast } from "@orbis/ui";
import {
  Layers,
  Plus,
  Trash2,
  Loader2,
  ChevronRight,
  Pencil,
  Weight,
} from "@lucide/svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import AssessmentSchemeDialog from "$lib/features/settings/components/assessment-scheme-dialog.svelte";
import AssessmentComponentDialog from "$lib/features/settings/components/assessment-component-dialog.svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);
const queryClient = useQueryClient();

// ── Queries ──────────────────────────────────────────────────
const schemesQuery = createQuery(() => ({
  queryKey: ["assessment-schemes", orgId],
  queryFn: () => commands.assessment.listAssessmentSchemes(),
}));

let selectedSchemeId = $state<string | null>(null);
const selectedScheme = $derived(
  (schemesQuery.data as any[] | undefined)?.find((s: any) => s.id === selectedSchemeId) ?? null,
);

const componentsQuery = createQuery(() => ({
  queryKey: ["scheme-components", selectedSchemeId],
  queryFn: () => (selectedSchemeId ? commands.assessment.listComponents(selectedSchemeId) : []),
  enabled: !!selectedSchemeId,
}));

// ── Weight summary ───────────────────────────────────────────
const totalWeight = $derived(
  ((componentsQuery.data ?? []) as any[]).reduce((sum: number, c: any) => sum + (c.weight || 0), 0),
);

// ── Create Scheme Dialog ─────────────────────────────────────
let showCreateDialog = $state(false);
let newSchemeName = $state("");
let newSchemeDesc = $state("");
let isCreating = $state(false);

async function handleCreateScheme() {
  if (!newSchemeName.trim()) return;
  isCreating = true;
  try {
    await commands.assessment.createAssessmentScheme({
      name: newSchemeName,
      description: newSchemeDesc || null,
    });
    toast.success("تم إنشاء هيكل التقييم بنجاح");
    queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
    showCreateDialog = false;
    newSchemeName = "";
    newSchemeDesc = "";
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isCreating = false;
  }
}

// ── Create Component Dialog ──────────────────────────────────
let showComponentDialog = $state(false);
let editingComponentId = $state<string | null>(null);
let componentForm = $state({
  name: "",
  component_type: "exam",
  weight: 30,
  max_score: 100,
  passing_score: 50,
});

function resetComponentForm() {
  editingComponentId = null;
  componentForm = { name: "", component_type: "exam", weight: 30, max_score: 100, passing_score: 50 };
}

function openAddComponent() {
  resetComponentForm();
  showComponentDialog = true;
}

function openEditComponent(c: any) {
  editingComponentId = c.id;
  componentForm = {
    name: c.name,
    component_type: c.component_type,
    weight: c.weight,
    max_score: c.max_score,
    passing_score: c.passing_score ?? 50,
  };
  showComponentDialog = true;
}

let isSavingComponent = $state(false);

async function handleSaveComponent() {
  if (!componentForm.name.trim() || !selectedSchemeId) return;
  isSavingComponent = true;
  try {
    if (editingComponentId) {
      await commands.assessment.updateComponent(editingComponentId, componentForm);
      toast.success("تم تحديث المكوّن بنجاح");
    } else {
      await commands.assessment.createComponent({
        scheme_id: selectedSchemeId,
        ...componentForm,
      });
      toast.success("تم إضافة المكوّن بنجاح");
    }
    queryClient.invalidateQueries({ queryKey: ["scheme-components"] });
    showComponentDialog = false;
    resetComponentForm();
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSavingComponent = false;
  }
}

// ── Delete ───────────────────────────────────────────────────
async function handleDeleteScheme(id: string) {
  try {
    await commands.assessment.deleteAssessmentScheme(id);
    toast.success("تم حذف هيكل التقييم");
    if (selectedSchemeId === id) selectedSchemeId = null;
    queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  }
}

async function handleDeleteComponent(id: string) {
  try {
    await commands.assessment.deleteComponent(id);
    toast.success("تم حذف المكوّن");
    queryClient.invalidateQueries({ queryKey: ["scheme-components"] });
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  }
}

function getTypeLabel(t: string) {
  switch (t) {
    case "exam": return "اختبار";
    case "quiz": return "اختبار قصير";
    case "homework": return "واجبات";
    case "participation": return "مشاركة";
    case "project": return "مشروع";
    case "practical": return "عملي";
    default: return t;
  }
}

const componentTypes = [
  { v: "exam", l: "اختبار" },
  { v: "quiz", l: "اختبار قصير" },
  { v: "homework", l: "واجبات" },
  { v: "participation", l: "مشاركة" },
  { v: "project", l: "مشروع" },
  { v: "practical", l: "عملي" },
];
</script>

<svelte:head>
  <title>هياكل التقييم | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  {#if schemesQuery.isLoading}
    <SettingsFormSkeleton />
  {:else if schemesQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>فشل تحميل هياكل التقييم</p>
      <Button variant="outline" class="mt-4 cursor-pointer" onclick={() => schemesQuery.refetch()}>
        إعادة المحاولة
      </Button>
    </div>
  {:else}
    <!-- Header -->
    <div class="flex items-center justify-between pb-5 border-b border-border/40 w-full select-none">
      <div class="flex items-center gap-2">
        <Layers size={18} class="text-primary" />
        <h1 class="text-sm font-semibold text-foreground">هياكل التقييم والأوزان</h1>
      </div>
      <Button size="sm" class="px-4 h-8 text-xs cursor-pointer" onclick={() => (showCreateDialog = true)}>
        <Plus size={14} class="me-1" />
        إضافة هيكل
      </Button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <!-- Schemes List (Left Panel) -->
      <div class="flex flex-col gap-2">
        {#each (schemesQuery.data ?? []) as scheme (scheme.id)}
          <div
            role="button"
            tabindex="0"
            class="text-start w-full px-4 py-3 rounded-lg border transition-all cursor-pointer {selectedSchemeId === scheme.id ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/40 hover:border-border hover:bg-muted/30'}"
            onclick={() => (selectedSchemeId = scheme.id)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && (selectedSchemeId = scheme.id)}
          >
            <div class="flex items-center justify-between">
              <div class="flex flex-col gap-0.5">
                <span class="text-xs font-semibold text-foreground">{scheme.name}</span>
                {#if scheme.description}
                  <span class="text-[10px] text-muted-foreground line-clamp-1">{scheme.description}</span>
                {/if}
              </div>
              <div class="flex items-center gap-1">
                <button
                  class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                  onclick={(e) => { e.stopPropagation(); handleDeleteScheme(scheme.id); }}
                >
                  <Trash2 size={12} />
                </button>
                <ChevronRight size={14} class="text-muted-foreground" />
              </div>
            </div>
          </div>
        {:else}
          <div class="text-center py-8 text-muted-foreground text-xs">
            لا توجد هياكل تقييم. أضف هيكل جديد للبدء.
          </div>
        {/each}
      </div>

      <!-- Components Panel (Right) -->
      <div class="md:col-span-2">
        {#if selectedScheme}
          <div class="border border-border/40 rounded-lg p-5">
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center gap-2">
                <Weight size={16} class="text-primary" />
                <h2 class="text-sm font-semibold">{selectedScheme.name}</h2>
              </div>
              <div class="flex items-center gap-2">
                <Badge
                  variant={totalWeight === 100 ? "default" : "destructive"}
                  class="text-[10px]"
                >
                  الوزن الإجمالي: {totalWeight}%
                </Badge>
                <Button size="sm" variant="outline" class="h-7 text-[10px] cursor-pointer" onclick={openAddComponent}>
                  <Plus size={12} class="me-1" />
                  إضافة مكوّن
                </Button>
              </div>
            </div>

            <Separator class="mb-4 bg-border/40" />

            {#if componentsQuery.isLoading}
              <div class="flex items-center justify-center py-8">
                <Loader2 size={20} class="animate-spin text-muted-foreground" />
              </div>
            {:else}
              <!-- Weight visual bar -->
              {#if (componentsQuery.data ?? []).length > 0}
                <div class="flex rounded-md overflow-hidden h-3 mb-5 bg-muted/30">
                  {#each (componentsQuery.data ?? []) as comp, i}
                    {@const colors = [
                      "bg-blue-500", "bg-emerald-500", "bg-amber-500",
                      "bg-purple-500", "bg-rose-500", "bg-cyan-500",
                    ]}
                    <div
                      class="{colors[i % colors.length]} transition-all"
                      style="width: {comp.weight}%"
                      title="{comp.name}: {comp.weight}%"
                    ></div>
                  {/each}
                </div>
              {/if}

              <div class="overflow-x-auto">
                <table class="w-full text-xs">
                  <thead>
                    <tr class="border-b border-border/40 text-muted-foreground">
                      <th class="text-start py-2 font-medium">المكوّن</th>
                      <th class="text-center py-2 font-medium">النوع</th>
                      <th class="text-center py-2 font-medium">الوزن</th>
                      <th class="text-center py-2 font-medium">الدرجة القصوى</th>
                      <th class="text-center py-2 font-medium">درجة النجاح</th>
                      <th class="text-center py-2 font-medium"></th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each (componentsQuery.data ?? []) as comp, i (comp.id)}
                      {@const colors = [
                        "bg-blue-500", "bg-emerald-500", "bg-amber-500",
                        "bg-purple-500", "bg-rose-500", "bg-cyan-500",
                      ]}
                      <tr class="border-b border-border/20 hover:bg-muted/20 transition-colors group">
                        <td class="py-2.5">
                          <div class="flex items-center gap-2">
                            <div class="w-2 h-2 rounded-full {colors[i % colors.length]}"></div>
                            <span class="font-semibold text-foreground">{comp.name}</span>
                          </div>
                        </td>
                        <td class="py-2.5 text-center">
                          <Badge variant="outline" class="text-[9px]">{getTypeLabel(comp.component_type)}</Badge>
                        </td>
                        <td class="py-2.5 text-center font-mono font-semibold">{comp.weight}%</td>
                        <td class="py-2.5 text-center font-mono">{comp.max_score}</td>
                        <td class="py-2.5 text-center font-mono">{comp.passing_score ?? "—"}</td>
                        <td class="py-2.5 text-center">
                          <div class="flex items-center gap-0.5 justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                            <button
                              class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                              onclick={() => openEditComponent(comp)}
                            >
                              <Pencil size={12} />
                            </button>
                            <button
                              class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                              onclick={() => handleDeleteComponent(comp.id)}
                            >
                              <Trash2 size={12} />
                            </button>
                          </div>
                        </td>
                      </tr>
                    {:else}
                      <tr>
                        <td colspan="6" class="text-center py-6 text-muted-foreground">
                          لا توجد مكوّنات. أضف مكوّن تقييم جديد.
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
            اختر هيكل تقييم من القائمة لعرض المكوّنات
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<AssessmentSchemeDialog bind:open={showCreateDialog} bind:name={newSchemeName} bind:description={newSchemeDesc} isCreating={isCreating} onsubmit={handleCreateScheme} />

<AssessmentComponentDialog bind:open={showComponentDialog} form={componentForm} editingId={editingComponentId} componentTypes={componentTypes} isSaving={isSavingComponent} onsubmit={handleSaveComponent} />
