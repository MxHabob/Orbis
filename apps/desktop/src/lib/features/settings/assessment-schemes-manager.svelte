<!--
@component AssessmentSchemesManager
Manages assessment schemes (weighted grade composition structures).
-->
<script lang="ts">
  import { api, extractErrorMessage } from "$lib/api";
  import AssessmentComponentDialog from "./components/assessment-component-dialog.svelte";
  import AssessmentSchemeDialog from "./components/assessment-scheme-dialog.svelte";
  import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
  import { AdjustmentsHorizontal, ChevronRight, Keyframes, Pencil, Plus, Scale, Trash } from "@orbis/icons";
  import { toast } from "@orbis/ui";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";
  import { Separator } from "@orbis/ui/separator";
  import { createQuery, useQueryClient } from "@tanstack/svelte-query";

  let { orgId }: { orgId: string } = $props();
  const queryClient = useQueryClient();

  interface SchemeComponent {
    id: string;
    name: string;
    component_type: string;
    weight: number;
    max_score: number;
    passing_score: number;
  }

  // ── Queries ──────────────────────────────────────────────────
  const schemesQuery = createQuery(() => ({
    queryKey: ["assessment-schemes", orgId],
    queryFn: () => api.listAssessmentSchemes(),
  }));

  let activeSchemeId = $state<string | null>(null);

  const schemes = $derived((schemesQuery.data as any[] | undefined) ?? []);
  const selectedScheme = $derived(
    (activeSchemeId ? schemes.find((s: any) => s.id === activeSchemeId) : null) ??
      schemes[0] ??
      null,
  );
  const selectedSchemeId = $derived(selectedScheme?.id ?? null);

  const schemeComponents = $derived.by<SchemeComponent[]>(() => {
    if (!selectedScheme) return [];
    const formula = selectedScheme.formula as any;
    if (formula) {
      if (Array.isArray(formula.components)) {
        return formula.components;
      }
      if (Array.isArray(formula)) {
        return formula;
      }
    }
    return [];
  });

  // ── Weight summary ───────────────────────────────────────────
  const totalWeight = $derived(
    schemeComponents.reduce(
      (sum: number, c: SchemeComponent) => sum + (Number(c.weight) || 0),
      0,
    ),
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
      const created = await api.createAssessmentScheme({
        name: newSchemeName.trim(),
        description: newSchemeDesc.trim() || null,
        max_score: 100,
        formula: {
          type: "weighted_average",
          components: [],
        },
      });
      toast.success("تم إنشاء هيكل التقييم بنجاح");
      activeSchemeId = created.id;
      await queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
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
    componentForm = {
      name: "",
      component_type: "exam",
      weight: 30,
      max_score: 100,
      passing_score: 50,
    };
  }

  function openAddComponent() {
    resetComponentForm();
    showComponentDialog = true;
  }

  function openEditComponent(c: SchemeComponent) {
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
    if (!componentForm.name.trim() || !selectedScheme) return;
    isSavingComponent = true;
    try {
      const current = [...schemeComponents];
      if (editingComponentId) {
        const idx = current.findIndex((c) => c.id === editingComponentId);
        if (idx >= 0) {
          current[idx] = {
            ...current[idx],
            name: componentForm.name.trim(),
            component_type: componentForm.component_type,
            weight: Number(componentForm.weight) || 0,
            max_score: Number(componentForm.max_score) || 100,
            passing_score: Number(componentForm.passing_score) || 50,
          };
        }
      } else {
        const newComp: SchemeComponent = {
          id: `comp-${Date.now()}`,
          name: componentForm.name.trim(),
          component_type: componentForm.component_type,
          weight: Number(componentForm.weight) || 0,
          max_score: Number(componentForm.max_score) || 100,
          passing_score: Number(componentForm.passing_score) || 50,
        };
        current.push(newComp);
      }
      await api.updateAssessmentScheme(selectedScheme.id, {
        formula: {
          type: "weighted_average",
          components: current,
        },
      });
      await queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
      toast.success(
        editingComponentId
          ? "تم تحديث المكوّن بنجاح"
          : "تم إضافة المكوّن بنجاح",
      );
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
      await api.deleteAssessmentScheme(id);
      toast.success("تم حذف هيكل التقييم");
      if (activeSchemeId === id) activeSchemeId = null;
      await queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
    } catch (err: unknown) {
      toast.error(extractErrorMessage(err));
    }
  }

  async function handleDeleteComponent(id: string) {
    if (!selectedScheme) return;
    try {
      const updated = schemeComponents.filter((c) => c.id !== id);
      await api.updateAssessmentScheme(selectedScheme.id, {
        formula: {
          type: "weighted_average",
          components: updated,
        },
      });
      await queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
      toast.success("تم حذف المكوّن");
    } catch (err: unknown) {
      toast.error(extractErrorMessage(err));
    }
  }

  function getTypeLabel(t: string) {
    switch (t) {
      case "exam":
        return "اختبار";
      case "quiz":
        return "اختبار قصير";
      case "homework":
        return "واجبات";
      case "participation":
        return "مشاركة";
      case "project":
        return "مشروع";
      case "practical":
        return "عملي";
      default:
        return t;
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

<div
  class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide"
>
  {#if schemesQuery.isLoading}
    <SettingsFormSkeleton />
  {:else if schemesQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>فشل تحميل هياكل التقييم</p>
      <Button
        variant="outline"
        class="mt-4 cursor-pointer"
        onclick={() => schemesQuery.refetch()}
      >
        إعادة المحاولة
      </Button>
    </div>
  {:else}
    <!-- Header -->
    <div
      class="flex items-center justify-between pb-5 border-b border-border/40 w-full select-none"
    >
      <div class="flex items-center gap-2">
        <Keyframes size={18} class="text-primary" />
        <h1 class="text-sm font-semibold text-foreground">
          هياكل التقييم والأوزان
        </h1>
      </div>
      <Button
        size="sm"
        class="px-4 h-8 text-xs cursor-pointer"
        onclick={() => (showCreateDialog = true)}
      >
        <Plus size={14} class="me-1" />
        إضافة هيكل
      </Button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <!-- Schemes List (Left Panel) -->
      <div class="flex flex-col gap-2">
        {#each schemes as scheme (scheme.id)}
          <div
            role="button"
            tabindex="0"
            class="text-start w-full px-4 py-3 rounded-lg border transition-all cursor-pointer {selectedSchemeId === scheme.id ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/40 hover:border-border hover:bg-muted/30'}"
            onclick={() => (activeSchemeId = scheme.id)}
            onkeydown={(e) =>
              (e.key === "Enter" || e.key === " ") &&
              (activeSchemeId = scheme.id)}
          >
            <div class="flex items-center justify-between">
              <div class="flex flex-col gap-0.5">
                <span class="text-xs font-semibold text-foreground"
                  >{scheme.name}</span
                >
                {#if scheme.description}
                  <span class="text-[10px] text-muted-foreground line-clamp-1"
                    >{scheme.description}</span
                  >
                {/if}
              </div>
              <div class="flex items-center gap-1">
                <button
                  class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleDeleteScheme(scheme.id);
                  }}
                >
                  <Trash size={12} />
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
                <Scale size={16} class="text-primary" />
                <h2 class="text-sm font-semibold">{selectedScheme.name}</h2>
              </div>
              <div class="flex items-center gap-2">
                <Badge
                  variant={totalWeight === 100 ? "default" : "destructive"}
                  class="text-[10px]"
                >
                  الوزن الإجمالي: {totalWeight}%
                </Badge>
                <Button
                  size="sm"
                  variant="outline"
                  class="h-7 text-[10px] cursor-pointer"
                  onclick={openAddComponent}
                >
                  <Plus size={12} class="me-1" />
                  إضافة مكوّن
                </Button>
              </div>
            </div>

            <Separator class="mb-4 bg-border/40" />

            <!-- Weight visual bar -->
            {#if schemeComponents.length > 0}
              <div class="flex rounded-md overflow-hidden h-3 mb-5 bg-muted/30">
                {#each schemeComponents as comp, i}
                  {@const colors = [
                    "bg-blue-500",
                    "bg-emerald-500",
                    "bg-amber-500",
                    "bg-purple-500",
                    "bg-rose-500",
                    "bg-cyan-500",
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
                  {#each schemeComponents as comp, i (comp.id)}
                    {@const colors = [
                      "bg-blue-500",
                      "bg-emerald-500",
                      "bg-amber-500",
                      "bg-purple-500",
                      "bg-rose-500",
                      "bg-cyan-500",
                    ]}
                    <tr
                      class="border-b border-border/20 hover:bg-muted/20 transition-colors group"
                    >
                      <td class="py-2.5">
                        <div class="flex items-center gap-2">
                          <div
                            class="w-2 h-2 rounded-full {colors[
                              i % colors.length
                            ]}"
                          ></div>
                          <span class="font-semibold text-foreground"
                            >{comp.name}</span
                          >
                        </div>
                      </td>
                      <td class="py-2.5 text-center">
                        <Badge variant="outline" class="text-[9px]"
                          >{getTypeLabel(comp.component_type)}</Badge
                        >
                      </td>
                      <td class="py-2.5 text-center font-mono font-semibold"
                        >{comp.weight}%</td
                      >
                      <td class="py-2.5 text-center font-mono"
                        >{comp.max_score}</td
                      >
                      <td class="py-2.5 text-center font-mono"
                        >{comp.passing_score ?? "—"}</td
                      >
                      <td class="py-2.5 text-center">
                        <div
                          class="flex items-center gap-0.5 justify-center opacity-0 group-hover:opacity-100 transition-opacity"
                        >
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
                            <Trash size={12} />
                          </button>
                        </div>
                      </td>
                    </tr>
                  {:else}
                    <tr>
                      <td
                        colspan="6"
                        class="text-center py-6 text-muted-foreground"
                      >
                        لا توجد مكوّنات. أضف مكوّن تقييم جديد.
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>
        {:else}
          <div
            class="flex items-center justify-center h-full text-muted-foreground text-xs py-16"
          >
            اختر هيكل تقييم من القائمة لعرض المكوّنات
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<AssessmentSchemeDialog
  bind:open={showCreateDialog}
  bind:name={newSchemeName}
  bind:description={newSchemeDesc}
  {isCreating}
  onsubmit={handleCreateScheme}
/>

<AssessmentComponentDialog
  bind:open={showComponentDialog}
  form={componentForm}
  editingId={editingComponentId}
  {componentTypes}
  isSaving={isSavingComponent}
  onsubmit={handleSaveComponent}
/>
