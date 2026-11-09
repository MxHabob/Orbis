<!--
@component AssessmentSchemesManager
Manages assessment schemes (weighted grade composition structures).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, type AssessmentComponent } from "$lib/api";
import AssessmentComponentDialog from "./components/assessment-component-dialog.svelte";
import AssessmentSchemeDialog from "./components/assessment-scheme-dialog.svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import {
  AdjustmentsHorizontal,
  ChevronRight,
  Keyframes,
  Pencil,
  Plus,
  Scale,
  Trash,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Separator } from "@orbis/ui/separator";
import {
  Card,
  CardHeader,
  CardTitle,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";

let { orgId }: { orgId: string } = $props();
const queryClient = useQueryClient();

interface SchemeComponent {
  id: string;
  code: string;
  name: string;
  name_ar?: string | null;
  component_type: string;
  weight: number;
  max_score: number;
  passing_score: number;
  sort_order: number;
  is_required: boolean;
  stored_weight: number | null;
  stored_component_type: string | null;
  displayed_weight: number;
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
  const formulaComponents = Array.isArray(formula?.components)
    ? formula.components
    : Array.isArray(formula)
      ? formula
      : null;
  const storedComponents: unknown[] = Array.isArray(selectedScheme.components)
    ? selectedScheme.components
    : [];
  const usesSchemeComponents = storedComponents.length > 0;
  const components = usesSchemeComponents
    ? storedComponents
    : (formulaComponents ?? storedComponents);
  const schemeMaxScore = Number(selectedScheme.max_score) || 100;

  const usedIds: string[] = [];
  return components.map((component, index) => {
    const value =
      typeof component === "object" && component !== null
        ? (component as Record<string, unknown>)
        : {};
    const baseId =
      (typeof value.id === "string" && value.id) ||
      (typeof value.code === "string" && value.code) ||
      `${selectedScheme.id}-component-${index}`;
    let id = baseId;
    let duplicateIndex = 1;
    while (usedIds.includes(id)) {
      id = `${baseId}-${duplicateIndex++}`;
    }
    usedIds.push(id);

    const maxScore = Number(value.max_score ?? value.maxScore) || 0;
    const storedWeight = Number(value.weight) || 0;
    const storedType = String(
      value.component_type ?? value.componentType ?? "",
    );
    const displayedWeight = usesSchemeComponents
      ? storedWeight === 1
        ? schemeMaxScore > 0
          ? (maxScore / schemeMaxScore) * 100
          : 0
        : storedWeight * 100
      : storedWeight;
    const componentType =
      storedType === "assignment"
        ? "homework"
        : storedType === "coursework"
          ? "classwork"
          : storedType === "final_exam" || storedType === "midterm"
            ? "exam"
            : storedType;
    return {
      id,
      code: String(value.code ?? id),
      name: String(value.name ?? ""),
      name_ar:
        typeof value.name_ar === "string"
          ? value.name_ar
          : typeof value.nameAr === "string"
            ? value.nameAr
            : null,
      component_type: componentType || "exam",
      weight: displayedWeight,
      max_score: maxScore,
      passing_score:
        Number(value.passing_score ?? value.min_score ?? value.minScore) || 0,
      sort_order: Number(value.sort_order ?? value.sortOrder) || index + 1,
      is_required: Boolean(value.is_required ?? value.isRequired ?? true),
      stored_weight: usesSchemeComponents ? storedWeight : null,
      stored_component_type: storedType || null,
      displayed_weight: displayedWeight,
    };
  });
});

function toAssessmentComponents(
  components: SchemeComponent[],
): AssessmentComponent[] {
  return components.map((component, index) => {
    const weight =
      component.stored_weight !== null &&
      component.weight === component.displayed_weight
        ? component.stored_weight
        : component.weight / 100;
    const componentType =
      component.component_type === "homework"
        ? "assignment"
        : component.component_type === "classwork"
          ? "coursework"
          : component.component_type === "exam"
            ? "final_exam"
            : component.component_type;

    return {
      code: component.code,
      name: component.name,
      nameAr: component.name_ar,
      maxScore: String(component.max_score),
      weight: String(weight),
      sortOrder: component.sort_order || index + 1,
      isRequired: component.is_required,
      componentType:
        component.stored_component_type &&
        component.component_type ===
          (component.stored_component_type === "assignment"
            ? "homework"
            : component.stored_component_type === "coursework"
              ? "classwork"
              : component.stored_component_type === "final_exam" ||
                  component.stored_component_type === "midterm"
                ? "exam"
                : component.stored_component_type)
          ? component.stored_component_type
          : componentType,
      minScore:
        component.passing_score > 0 ? String(component.passing_score) : null,
    };
  });
}

async function saveSchemeComponents(components: SchemeComponent[]) {
  if (!selectedScheme) return;

  const request: {
    components: AssessmentComponent[];
    formula?: unknown;
  } = {
    components: toAssessmentComponents(components),
  };
  const formula = selectedScheme.formula;
  if (
    formula &&
    typeof formula === "object" &&
    !Array.isArray(formula) &&
    Array.isArray((formula as { components?: unknown }).components)
  ) {
    request.formula = {
      ...formula,
      components,
    };
  } else if (Array.isArray(formula)) {
    request.formula = components;
  }

  await api.updateAssessmentScheme(selectedScheme.id, request);
}

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
    toast.success(m.settings_ui_assessment_scheme_created());
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
      const id = `comp-${Date.now()}`;
      const newComp: SchemeComponent = {
        id,
        code: id,
        name: componentForm.name.trim(),
        name_ar: null,
        component_type: componentForm.component_type,
        weight: Number(componentForm.weight) || 0,
        max_score: Number(componentForm.max_score) || 100,
        passing_score: Number(componentForm.passing_score) || 50,
        sort_order: current.length + 1,
        is_required: true,
        stored_weight: null,
        stored_component_type: null,
        displayed_weight: Number(componentForm.weight) || 0,
      };
      current.push(newComp);
    }
    await saveSchemeComponents(current);
    await queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
    toast.success(
      editingComponentId
        ? m.settings_ui_assessment_component_updated()
        : m.settings_ui_assessment_component_added(),
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
    toast.success(m.settings_ui_assessment_scheme_deleted());
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
    await saveSchemeComponents(updated);
    await queryClient.invalidateQueries({ queryKey: ["assessment-schemes"] });
    toast.success(m.settings_ui_assessment_component_deleted());
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  }
}

function getTypeLabel(t: string) {
  switch (t) {
    case "exam":
      return m.settings_ui_assessment_type_exam();
    case "quiz":
      return m.settings_ui_assessment_type_quiz();
    case "homework":
      return m.settings_ui_assessment_type_homework();
    case "participation":
      return m.settings_ui_assessment_type_participation();
    case "project":
      return m.settings_ui_assessment_type_project();
    case "practical":
      return m.settings_ui_assessment_type_practical();
    default:
      return t;
  }
}

const componentTypes = [
  { v: "exam", l: m.settings_ui_assessment_type_exam() },
  { v: "quiz", l: m.settings_ui_assessment_type_quiz() },
  { v: "homework", l: m.settings_ui_assessment_type_homework() },
  { v: "participation", l: m.settings_ui_assessment_type_participation() },
  { v: "project", l: m.settings_ui_assessment_type_project() },
  { v: "practical", l: m.settings_ui_assessment_type_practical() },
];
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide text-start">
  {#if schemesQuery.isLoading}
    <SettingsFormSkeleton />
  {:else if schemesQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>{m.settings_ui_assessment_schemes_load_error()}</p>
      <Button
        variant="outline"
        class="mt-4 cursor-pointer"
        onclick={() => schemesQuery.refetch()}
      >
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <!-- Header -->
    <div class="flex items-center justify-between pb-4 border-b border-border/60 w-full select-none">
      <div class="flex items-center gap-2">
        <Keyframes size={18} class="text-primary" />
        <h1 class="text-sm font-bold text-foreground">
          {m.settings_ui_assessment_schemes_title()}
        </h1>
      </div>
      <Button
        size="sm"
        class="px-4 h-8 text-xs cursor-pointer gap-1.5"
        onclick={() => (showCreateDialog = true)}
      >
        <Plus size={14} />
        {m.settings_ui_assessment_scheme_add()}
      </Button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <!-- Schemes List (Left Panel) -->
      <Card class="flex flex-col overflow-hidden">
        <CardHeader class="py-3 px-4">
          <CardTitle class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider text-start select-none">
            {m.settings_ui_assessment_schemes_available({ count: schemes.length })}
          </CardTitle>
        </CardHeader>
        <CardContent class="p-2 flex flex-col gap-1.5">
          {#each schemes as scheme (scheme.id)}
            <div
              role="button"
              tabindex="0"
              class="text-start w-full px-3.5 py-2.5 rounded-lg border transition-all cursor-pointer {selectedSchemeId === scheme.id ? 'border-primary bg-primary/5 shadow-xs' : 'border-transparent hover:border-border hover:bg-muted/40'}"
              onclick={() => (activeSchemeId = scheme.id)}
              onkeydown={(e) =>
                (e.key === "Enter" || e.key === " ") &&
                (activeSchemeId = scheme.id)}
            >
              <div class="flex items-center justify-between gap-2">
                <div class="flex flex-col gap-0.5 min-w-0 flex-1">
                  <span class="text-xs font-semibold text-foreground truncate">
                    {scheme.name}
                  </span>
                  {#if scheme.description}
                    <span class="text-[10px] text-muted-foreground line-clamp-1">
                      {scheme.description}
                    </span>
                  {/if}
                </div>
                <div class="flex items-center gap-1 shrink-0">
                  <button
                    class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                    onclick={(e) => {
                      e.stopPropagation();
                      handleDeleteScheme(scheme.id);
                    }}
                    title={m.settings_ui_assessment_scheme_delete()}
                  >
                    <Trash size={12} />
                  </button>
                  <ChevronRight size={13} class="text-muted-foreground rtl:rotate-180" />
                </div>
              </div>
            </div>
          {:else}
            <div class="text-center py-8 text-muted-foreground text-xs">
              {m.settings_ui_assessment_schemes_empty()}
            </div>
          {/each}
        </CardContent>
      </Card>

      <!-- Components Panel (Right) -->
      <div class="md:col-span-2">
        {#if selectedScheme}
          <Card class="overflow-hidden">
            <CardHeader class="py-3 px-6 flex flex-row items-center justify-between">
              <div class="flex items-center gap-2">
                <Scale size={15} class="text-primary" />
                <CardTitle class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider text-start select-none">
                  {selectedScheme.name}
                </CardTitle>
              </div>
              <CardAction>
                <div class="flex items-center gap-2">
                  <Badge
                    variant={totalWeight === 100 ? "default" : "destructive"}
                    class="text-[10px] font-mono"
                  >
                    {m.settings_ui_assessment_total_weight({ weight: totalWeight })}
                  </Badge>
                  <Button
                    size="sm"
                    variant="outline"
                    class="h-7 text-[10px] cursor-pointer gap-1"
                    onclick={openAddComponent}
                  >
                    <Plus size={11} />
                    {m.settings_ui_assessment_component_add()}
                  </Button>
                </div>
              </CardAction>
            </CardHeader>

            <CardContent class="p-6 flex flex-col gap-4">
              <!-- Weight visual bar -->
              {#if schemeComponents.length > 0}
                <div class="flex rounded-md overflow-hidden h-2.5 bg-muted/40">
                  {#each schemeComponents as comp, i (comp.id)}
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

              <div class="overflow-x-auto rounded-lg border border-border">
                <table class="w-full text-xs">
                  <thead>
                    <tr class="border-b border-border bg-muted/40 text-muted-foreground select-none">
                      <th class="text-start py-2.5 px-4 font-semibold">{m.settings_ui_assessment_component()}</th>
                      <th class="text-center py-2.5 px-3 font-semibold">{m.settings_ui_assessment_type()}</th>
                      <th class="text-center py-2.5 px-3 font-semibold">{m.settings_ui_assessment_weight()}</th>
                      <th class="text-center py-2.5 px-3 font-semibold">{m.settings_ui_assessment_max_score()}</th>
                      <th class="text-center py-2.5 px-3 font-semibold">{m.settings_ui_assessment_passing_score()}</th>
                      <th class="text-center py-2.5 px-3 font-semibold"></th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-border">
                    {#each schemeComponents as comp, i (comp.id)}
                      {@const colors = [
                        "bg-blue-500",
                        "bg-emerald-500",
                        "bg-amber-500",
                        "bg-purple-500",
                        "bg-rose-500",
                        "bg-cyan-500",
                      ]}
                      <tr class="hover:bg-muted/20 transition-colors group">
                        <td class="py-2.5 px-4">
                          <div class="flex items-center gap-2">
                            <div
                              class="w-2 h-2 rounded-full {colors[
                                i % colors.length
                              ]}"
                            ></div>
                            <span class="font-semibold text-foreground">
                              {comp.name}
                            </span>
                          </div>
                        </td>
                        <td class="py-2.5 px-3 text-center">
                          <Badge variant="outline" class="text-[9px]">
                            {getTypeLabel(comp.component_type)}
                          </Badge>
                        </td>
                        <td class="py-2.5 px-3 text-center font-mono font-bold text-foreground">
                          {comp.weight}%
                        </td>
                        <td class="py-2.5 px-3 text-center font-mono text-muted-foreground">
                          {comp.max_score}
                        </td>
                        <td class="py-2.5 px-3 text-center font-mono text-muted-foreground">
                          {comp.passing_score ?? "—"}
                        </td>
                        <td class="py-2.5 px-3 text-center">
                          <div class="flex items-center gap-1 justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                            <button
                              class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                              onclick={() => openEditComponent(comp)}
                              title={m.common_action_edit()}
                            >
                              <Pencil size={12} />
                            </button>
                            <button
                              class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                              onclick={() => handleDeleteComponent(comp.id)}
                              title={m.common_action_delete()}
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
                          {m.settings_ui_assessment_components_empty()}
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            </CardContent>
          </Card>
        {:else}
          <div
            class="flex items-center justify-center h-full text-muted-foreground text-xs py-16"
          >
            {m.settings_ui_assessment_select_scheme()}
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
