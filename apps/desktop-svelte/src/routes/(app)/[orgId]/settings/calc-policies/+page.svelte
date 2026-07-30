<!--
@component CalculationPoliciesSettingsPage
Manages calculation policies (formulas for how grades are computed per period).
Uses the Assessment Engine's calculation_policy CRUD operations.
-->
<script lang="ts">
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Badge } from "@orbis/ui-svelte/badge";
import { Separator } from "@orbis/ui-svelte/separator";
import { Textarea } from "@orbis/ui-svelte/textarea";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import { toast } from "svelte-sonner";
import {
  Calculator,
  Plus,
  Trash2,
  Loader2,
  Pencil,
  FlaskConical,
} from "lucide-svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);
const queryClient = useQueryClient();

const policiesQuery = createQuery({
  queryKey: ["calc-policies", orgId],
  queryFn: () => commands.assessment.listCalculationPolicies(),
});

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
      await commands.assessment.updateCalculationPolicy(editingId, form);
      toast.success("تم تحديث سياسة الحساب بنجاح");
    } else {
      await commands.assessment.createCalculationPolicy(form);
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
    await commands.assessment.deleteCalculationPolicy(id);
    toast.success("تم حذف سياسة الحساب");
    queryClient.invalidateQueries({ queryKey: ["calc-policies"] });
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  }
}

function getRuleLabel(t: string) {
  switch (t) {
    case "weighted_sum": return "المجموع الموزون";
    case "simple_average": return "المتوسط البسيط";
    case "custom_ast": return "صيغة مخصصة (AST)";
    default: return t;
  }
}

function getRoundingLabel(r: string) {
  switch (r) {
    case "round_half_up": return "تقريب للأعلى";
    case "round_half_down": return "تقريب للأسفل";
    case "truncate": return "قطع الكسور";
    case "ceiling": return "تقريب للسقف";
    case "floor": return "تقريب للقاع";
    default: return r;
  }
}
</script>

<svelte:head>
  <title>سياسات الحساب | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide bg-background">
  {#if $policiesQuery.isLoading}
    <SettingsFormSkeleton />
  {:else if $policiesQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>فشل تحميل سياسات الحساب</p>
      <Button variant="outline" class="mt-4 cursor-pointer" onclick={() => $policiesQuery.refetch()}>
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
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
      {#each ($policiesQuery.data ?? []) as policy (policy.id)}
        <div class="border border-border/40 rounded-lg p-5 hover:border-border hover:shadow-sm transition-all group">
          <div class="flex items-start justify-between mb-3">
            <div class="flex items-center gap-2">
              <FlaskConical size={14} class="text-primary" />
              <h3 class="text-xs font-semibold text-foreground">{policy.name}</h3>
            </div>
            <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
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

          <div class="bg-muted/30 rounded-md p-2.5 font-mono text-[10px] text-muted-foreground leading-relaxed">
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

<!-- Create/Edit Dialog -->
<Dialog bind:open={showDialog}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle class="text-sm">{editingId ? "تعديل سياسة الحساب" : "إنشاء سياسة حساب جديدة"}</DialogTitle>
    </DialogHeader>
    <div class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">اسم السياسة</Label>
        <Input bind:value={form.name} placeholder="مثال: المجموع الموزون الافتراضي" class="text-xs" />
      </div>

      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">نوع القاعدة</Label>
        <Select type="single" value={form.rule_type} onValueChange={(v) => { if (v) form.rule_type = v; }}>
          <SelectTrigger class="text-xs h-9">{getRuleLabel(form.rule_type)}</SelectTrigger>
          <SelectContent class="text-xs">
            <SelectItem value="weighted_sum">المجموع الموزون</SelectItem>
            <SelectItem value="simple_average">المتوسط البسيط</SelectItem>
            <SelectItem value="custom_ast">صيغة مخصصة (AST)</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">صيغة الحساب</Label>
        <Textarea bind:value={form.formula_expression} class="text-xs font-mono min-h-[60px]" placeholder="SUM(score * weight) / SUM(weight)" />
      </div>

      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">سياسة التقريب</Label>
        <Select type="single" value={form.rounding_policy} onValueChange={(v) => { if (v) form.rounding_policy = v; }}>
          <SelectTrigger class="text-xs h-9">{getRoundingLabel(form.rounding_policy)}</SelectTrigger>
          <SelectContent class="text-xs">
            <SelectItem value="round_half_up">تقريب للأعلى</SelectItem>
            <SelectItem value="round_half_down">تقريب للأسفل</SelectItem>
            <SelectItem value="truncate">قطع الكسور</SelectItem>
            <SelectItem value="ceiling">تقريب للسقف</SelectItem>
            <SelectItem value="floor">تقريب للقاع</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
    <DialogFooter>
      <Button size="sm" class="text-xs cursor-pointer" onclick={handleSave} disabled={isSaving || !form.name.trim()}>
        {#if isSaving}<Loader2 size={12} class="animate-spin me-1" />{/if}
        {editingId ? "تحديث" : "إنشاء"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
