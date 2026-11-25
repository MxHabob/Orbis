<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { AdjustmentsHorizontal, ArrowPath } from "@orbis/icons";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  assessmentComponentInputSchema,
  type AssessmentComponentInput,
} from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { untrack } from "svelte";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
let {
  open = $bindable(false),
  component,
  editingId = null,
  componentTypes,
  isSaving = false,
  onsubmit,
}: {
  open: boolean;
  component?: AssessmentComponentInput | null;
  editingId?: string | null;
  componentTypes: { v: string; l: string }[];
  isSaving?: boolean;
  onsubmit: (payload: AssessmentComponentInput) => void;
} = $props();

function defaultValues(
  value?: AssessmentComponentInput | null,
): AssessmentComponentInput {
  return {
    code: value?.code ?? "",
    name: value?.name ?? "",
    name_ar: value?.name_ar ?? "",
    component_type: value?.component_type ?? "exam",
    weight: value?.weight ?? 30,
    max_score: value?.max_score ?? 100,
    passing_score: value?.passing_score ?? 50,
    sort_order: value?.sort_order ?? 1,
    is_required: value?.is_required ?? true,
  };
}

const form = new FormController<
  AssessmentComponentInput,
  AssessmentComponentInput
>({
  initialValues: untrack(() => defaultValues(component)),
  schema: assessmentComponentInputSchema,
});

$effect(() => {
  if (open) {
    (form.reset as (values: AssessmentComponentInput) => void)(
      defaultValues(component),
    );
  }
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

function submit() {
  void form.submit(async (payload) => onsubmit(payload));
}
</script>
<Dialog bind:open>
  <DialogContent class="max-w-lg">
    <DialogHeader>
      <DialogTitle class="text-sm">
        {editingId ? m.common_action_edit() : m.settings_ui_assessment_component_add()}
      </DialogTitle>
    </DialogHeader>
    <div class="grid grid-cols-1 gap-4 py-2 sm:grid-cols-2">
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.settings_ui_assessment_component_name()}</Label>
        <Input
          bind:value={form.values.name}
          placeholder={m.settings_ui_assessment_component_example()}
          class="text-xs"
          onblur={() => form.markTouched("name")}
        />
        {#if fieldError("name")}
          <p class="text-[11px] text-destructive">{fieldError("name")}</p>
        {/if}
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.common_form_full_name_ar()}</Label>
        <Input
          bind:value={form.values.name_ar}
          class="text-xs"
          onblur={() => form.markTouched("name_ar")}
        />
        {#if fieldError("name_ar")}
          <p class="text-[11px] text-destructive">{fieldError("name_ar")}</p>
        {/if}
      </div>
      <div class="flex flex-col gap-1.5 sm:col-span-2">
        <Label class="text-xs">{m.settings_ui_assessment_type()}</Label>
        <div class="grid grid-cols-3 gap-1.5 sm:grid-cols-6">
          {#each componentTypes as ct (ct.v)}
            <button
              type="button"
              class="text-[10px] py-1.5 rounded-md border transition-all cursor-pointer {form.values.component_type === ct.v ? 'border-primary bg-primary/10 text-primary font-semibold' : 'border-border/40 text-muted-foreground hover:border-border'}"
              onclick={() => form.setField("component_type", ct.v)}
            >
              {ct.l}
            </button>
          {/each}
        </div>
        {#if fieldError("component_type")}
          <p class="text-[11px] text-destructive">{fieldError("component_type")}</p>
        {/if}
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.settings_ui_assessment_weight_percent()}</Label>
        <Input
          type="number"
          min="0"
          max="100"
          bind:value={form.values.weight}
          class="text-xs font-mono"
          onblur={() => form.markTouched("weight")}
        />
        {#if fieldError("weight")}
          <p class="text-[11px] text-destructive">{fieldError("weight")}</p>
        {/if}
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.settings_ui_assessment_max_score()}</Label>
        <Input
          type="number"
          min="1"
          max="1000"
          bind:value={form.values.max_score}
          class="text-xs font-mono"
          onblur={() => form.markTouched("max_score")}
        />
        {#if fieldError("max_score")}
          <p class="text-[11px] text-destructive">{fieldError("max_score")}</p>
        {/if}
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.settings_ui_assessment_passing_score()}</Label>
        <Input
          type="number"
          min="0"
          max="1000"
          bind:value={form.values.passing_score}
          class="text-xs font-mono"
          onblur={() => form.markTouched("passing_score")}
        />
        {#if fieldError("passing_score")}
          <p class="text-[11px] text-destructive">{fieldError("passing_score")}</p>
        {/if}
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.common_sort_order()}</Label>
        <Input
          type="number"
          min="1"
          bind:value={form.values.sort_order}
          class="text-xs font-mono"
          onblur={() => form.markTouched("sort_order")}
        />
        {#if fieldError("sort_order")}
          <p class="text-[11px] text-destructive">{fieldError("sort_order")}</p>
        {/if}
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.common_code()}</Label>
        <Input
          bind:value={form.values.code}
          class="text-xs font-mono"
          onblur={() => form.markTouched("code")}
        />
        {#if fieldError("code")}
          <p class="text-[11px] text-destructive">{fieldError("code")}</p>
        {/if}
      </div>
      <div class="flex items-center gap-2 sm:col-span-2">
        <Checkbox
          checked={form.values.is_required}
          onCheckedChange={(value) => form.setField("is_required", Boolean(value))}
        />
        <Label class="text-xs">{m.common_required()}</Label>
      </div>
    </div>
    {#if form.formError}
      <p class="text-xs text-destructive">{form.formError}</p>
    {/if}
    <DialogFooter>
      <Button size="sm" class="text-xs cursor-pointer" onclick={submit} disabled={isSaving || form.isSubmitting}>
        {#if isSaving || form.isSubmitting}<LoadingIndicator class="me-1" size={12} />{/if}
        {editingId ? m.common_action_update() : m.common_action_add()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
