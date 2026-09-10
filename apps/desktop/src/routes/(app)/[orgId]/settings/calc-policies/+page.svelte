<!--
@component CalculationPoliciesSettingsPage
Manages calculation policies (formulas for how grades are computed per period).
Uses the Assessment Engine's calculation_policy CRUD operations.
-->
<script lang="ts">
import { api, extractErrorMessage } from "$lib/api";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Separator } from "@orbis/ui/separator";
import { Textarea } from "@orbis/ui/textarea";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import {
  Calculator,
  Plus,
  Trash2,
  Loader2,
  Pencil,
  FlaskConical,
} from "@lucide/svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import CalculationPolicyDialog from "$lib/features/settings/components/calculation-policy-dialog.svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);
const queryClient = useQueryClient();

const policiesQuery = createQuery(() => ({
  queryKey: ["calc-policies", orgId],
  queryFn: () => api.assessment.listCalculationPolicies(),
}));

// ── Create / Edit Dialog ─────────────────────────────────────
let showDialog = $state(false);
let editingId = $state<string | null>(null);
let isSaving = $state(false);
let form = $state({
  name: "",
  rule_type: "weighted_sum",
  formula_expression: "SUM(score * weight) / SUM(weight)",
  rounding_policy: "round_half_up",
  target_period_id: "",
});

function resetForm() {
  editingId = null;
  form = {
    name: "",
    rule_type: "weighted_sum",
    formula_expression: "SUM(score * weight) / SUM(weight)",
    rounding_policy: "round_half_up",
    target_period_id: "",
  };
}

function openCreate() {
  resetForm();
  showDialog = true;
}

function openEdit(policy: any) {
  editingId = policy.id;
  form = {
    name: policy.name,
    rule_type: policy.rule_type,
    formula_expression: policy.formula_expression,
    rounding_policy: policy.rounding_policy,
    target_period_id: policy.target_period_id || "",
  };
  showDialog = true;
}

async function handleSave() {
  if (!form.name.trim()) return;
  isSaving = true;
  try {
    if (editingId) {
      await api.assessment.updateCalculationPolicy(editingId, form);
      toast.success("تم تحديث سياسة الحساب بنجاح");
    } else {
      await api.assessment.createCalculationPolicy(form);
      toast.success("تم إنشاء سياسة الحساب بنجاح");
    }
    queryClient.invalidateQueries({ queryKey: ["calc-policies"] });
    showDialog = false;
    resetForm();
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSaving = false;
  }
}

async function handleDelete(id: string) {
  try {
    await api.assessment.deleteCalculationPolicy(id);
    toast.success("تم حذف سياسة الحساب");
    queryClient.invalidateQueries({ queryKey: ["calc-policies"] });
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  }
}

function getRuleLabel(t: string) {
  switch (t) {
    case "weighted_sum":
      return "المجموع الموزون";
    case "simple_average":
      return "المتوسط البسيط";
    case "custom_ast":
      return "صيغة مخصصة (AST)";
    default:
      return t;
  }
}

function getRoundingLabel(r: string) {
  switch (r) {
    case "round_half_up":
      return "تقريب للأعلى";
    case "round_half_down":
      return "تقريب للأسفل";
    case "truncate":
      return "قطع الكسور";
    case "ceiling":
      return "تقريب للسقف";
    case "floor":
      return "تقريب للقاع";
    default:
      return r;
  }
}
</script>

<svelte:head>
  <title>سياسات الحساب | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  {#if policiesQuery.isLoading}
    <SettingsFormSkeleton />
  {:else if policiesQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>فشل تحميل سياسات الحساب</p>
      <Button variant="outline" class="mt-4 cursor-pointer" onclick={() => policiesQuery.refetch()}>
        إعادة المحاولة
      </Button>
    </div>
  {:else}
    <!-- Header -->
    <div class="flex items-center justify-between pb-5 border-b border-border/40 w-full select-none">
      <div class="flex items-center gap-2">
        <Calculator size={18} class="text-primary" />
        <h1 class="text-sm font-semibold text-foreground">سياسات حساب الدرجات</h1>
      </div>
      <Button size="sm" class="px-4 h-8 text-xs cursor-pointer" onclick={openCreate}>
        <Plus size={14} class="me-1" />
        إضافة سياسة
      </Button>
    </div>

    <!-- Policies Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 min-w-0">
      {#each (policiesQuery.data ?? []) as policy (policy.id)}
        <div class="border border-border/40 rounded-lg p-5 hover:border-border hover:shadow-sm transition-all group flex flex-col justify-between min-w-0 overflow-hidden bg-card">
          <div>
            <div class="flex items-start justify-between mb-3 min-w-0">
              <div class="flex items-center gap-2 min-w-0 truncate">
                <FlaskConical size={14} class="text-primary shrink-0" />
                <h3 class="text-xs font-semibold text-foreground truncate">{policy.name}</h3>
              </div>
              <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
                <button
                  class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                  onclick={() => openEdit(policy)}
                >
                  <Pencil size={12} />
                </button>
                <button
                  class="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                  onclick={() => handleDelete(policy.id)}
                >
                  <Trash2 size={12} />
                </button>
              </div>
            </div>

            <div class="flex flex-wrap gap-1.5 mb-3">
              <Badge variant="outline" class="text-[9px]">{getRuleLabel(policy.rule_type)}</Badge>
              <Badge variant="secondary" class="text-[9px]">{getRoundingLabel(policy.rounding_policy)}</Badge>
              <Badge variant="secondary" class="text-[9px]">v{policy.version}</Badge>
            </div>
          </div>

          <div class="bg-muted/30 rounded-md p-2.5 font-mono text-[10px] text-muted-foreground leading-relaxed break-all max-h-24 overflow-y-auto w-full min-w-0">
            {policy.formula_expression}
          </div>
        </div>
      {:else}
        <div class="col-span-full text-center py-12 text-muted-foreground text-xs">
          لا توجد سياسات حساب. أنشئ سياسة جديدة للبدء.
        </div>
      {/each}
    </div>
  {/if}
</div>

<CalculationPolicyDialog bind:open={showDialog} form={form} editingId={editingId} isSaving={isSaving} getRuleLabel={getRuleLabel} getRoundingLabel={getRoundingLabel} onsubmit={handleSave} />
