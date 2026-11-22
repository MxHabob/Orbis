<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { AdjustmentsHorizontal, ArrowPath } from "@orbis/icons";
import { Checkbox } from "@orbis/ui/checkbox";
import { assessmentComponentInputSchema, type AssessmentComponentInput } from "@orbis/schemas";
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

const form = new FormController<AssessmentComponentInput, AssessmentComponentInput>({
  initialValues: untrack(() => ({
    code: component?.code ?? "",
    name: component?.name ?? "",
    name_ar: component?.name_ar ?? "",
    component_type: component?.component_type ?? "exam",
    weight: component?.weight ?? 30,
    max_score: component?.max_score ?? 100,
    passing_score: component?.passing_score ?? 50,
    sort_order: component?.sort_order ?? 1,
    is_required: component?.is_required ?? true,
  })),
  schema: assessmentComponentInputSchema,
});

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
        <Input bind:value={form.values.name} placeholder={m.settings_ui_assessment_component_example()} class="text-xs" onblur={() => form.markTouched("name")} />
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.common_form_full_name_ar()}</Label>
        <Input bind:value={form.values.name_ar} class="text-xs" />
      </div>
      <div class="flex flex-col gap-1.5 sm:col-span-2">
        <Label class="text-xs">{m.settings_ui_assessment_type()}</Label>
        <div class="grid grid-cols-3 gap-1.5 sm:grid-cols-6">
          {#each componentTypes as ct (ct.v)}
            <button type="button" class="text-[10px] py-1.5 rounded-md border transition-all cursor-pointer {form.values.component_type === ct.v ? 'border-primary bg-primary/10 text-primary font-semibold' : 'border-border/40 text-muted-foreground hover:border-border'}" onclick={() => (form.values.component_type = ct.v)}>
              {ct.l}
            </button>
          {/each}
        </div>
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.settings_ui_assessment_weight_percent()}</Label>
        <Input type="number" min="0" max="100" bind:value={form.values.weight} class="text-xs font-mono" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.settings_ui_assessment_max_score()}</Label>
        <Input type="number" min="1" max="1000" bind:value={form.values.max_score} class="text-xs font-mono" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">{m.settings_ui_assessment_passing_score()}</Label>
        <Input type="number" min="0" max="1000" bind:value={form.values.passing_score} class="text-xs font-mono" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">Sort Order</Label>
        <Input type="number" min="1" bind:value={form.values.sort_order} class="text-xs font-mono" />
      </div>
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs">Code</Label>
        <Input bind:value={form.values.code} class="text-xs font-mono" />
      </div>
      <div class="flex items-center gap-2 sm:col-span-2">
        <Checkbox checked={form.values.is_required} onCheckedChange={(value) => (form.values.is_required = value)} />
        <Label class="text-xs">Required</Label>
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
