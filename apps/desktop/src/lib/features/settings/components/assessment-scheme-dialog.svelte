<script lang="ts">
import { LoadingIndicator, Switch } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { AdjustmentsHorizontal } from "@orbis/icons";
import { Label } from "@orbis/ui/label";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui/select";
import { assessmentSchemeInputSchema, type AssessmentSchemeInput } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { untrack } from "svelte";
import type { AssessmentSchemeDto } from "$lib/api";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";

let {
  open = $bindable(false),
  scheme = null,
  isCreating = false,
  onsubmit,
}: {
  open: boolean;
  scheme?: AssessmentSchemeDto | null;
  isCreating?: boolean;
  onsubmit: (payload: AssessmentSchemeInput) => void;
} = $props();

const form = new FormController<AssessmentSchemeInput, AssessmentSchemeInput>({
  initialValues: untrack(() => ({
    name: scheme?.name ?? "",
    name_ar: scheme?.name_ar ?? "",
    description: scheme?.description ?? "",
    max_score: Number(scheme?.max_score ?? 100),
    level: scheme?.level ?? "term",
    institution_scope: scheme?.institution_scope ?? "all",
    is_default: scheme?.is_default ?? false,
  })),
  schema: assessmentSchemeInputSchema,
});

function submit() {
  void form.submit(async (payload) => onsubmit(payload));
}
</script>

<Dialog bind:open>
  <DialogContent class="max-w-lg">
    <DialogHeader>
      <DialogTitle class="text-sm">
        {scheme ? m.common_action_edit() : m.settings_ui_assessment_scheme_create_title()}
      </DialogTitle>
    </DialogHeader>
    <div class="grid grid-cols-1 gap-4 py-2 sm:grid-cols-2">
      <FormField label={m.settings_ui_assessment_scheme_name()} required>
        {#snippet children()}
          <Input bind:value={form.values.name} placeholder={m.settings_ui_assessment_scheme_example()} class="text-xs" onblur={() => form.markTouched("name")} />
        {/snippet}
      </FormField>
      <FormField label={m.common_form_full_name_ar()}>
        {#snippet children()}
          <Input bind:value={form.values.name_ar} class="text-xs" />
        {/snippet}
      </FormField>
      <div class="sm:col-span-2">
        <FormField label={m.settings_ui_assessment_scheme_description_optional()}>
          {#snippet children()}
            <Input bind:value={form.values.description} placeholder={m.settings_ui_assessment_scheme_description_example()} class="text-xs" />
          {/snippet}
        </FormField>
      </div>
      <FormField label={m.settings_ui_assessment_max_score()} required>
        {#snippet children()}
          <Input type="number" min="1" max="1000" bind:value={form.values.max_score} class="text-xs font-mono" />
        {/snippet}
      </FormField>
      <FormField label={m.common_assessment_label_type()} required>
        {#snippet children()}
          <Select type="single" value={form.values.level} onValueChange={(value) => (form.values.level = (value as AssessmentSchemeInput["level"]) ?? "term")}>
            <SelectTrigger class="h-9 w-full text-xs">{form.values.level}</SelectTrigger>
            <SelectContent>
              <SelectItem value="monthly_period">monthly_period</SelectItem>
              <SelectItem value="term">term</SelectItem>
              <SelectItem value="annual">annual</SelectItem>
              <SelectItem value="coursework">coursework</SelectItem>
              <SelectItem value="midterm">midterm</SelectItem>
              <SelectItem value="final">final</SelectItem>
              <SelectItem value="module_assessment">module_assessment</SelectItem>
              <SelectItem value="certificate">certificate</SelectItem>
              <SelectItem value="certificate_cumulative">certificate_cumulative</SelectItem>
            </SelectContent>
          </Select>
        {/snippet}
      </FormField>
      <FormField label={m.settings_general_institution_type()} required>
        {#snippet children()}
          <Select type="single" value={form.values.institution_scope} onValueChange={(value) => (form.values.institution_scope = (value as AssessmentSchemeInput["institution_scope"]) ?? "all")}>
            <SelectTrigger class="h-9 w-full text-xs">{form.values.institution_scope}</SelectTrigger>
            <SelectContent>
              <SelectItem value="all">all</SelectItem>
              <SelectItem value="school">school</SelectItem>
              <SelectItem value="university">university</SelectItem>
              <SelectItem value="institute">institute</SelectItem>
            </SelectContent>
          </Select>
        {/snippet}
      </FormField>
      <div class="flex items-center justify-between rounded-md border border-border px-3 py-2">
        <Label class="text-xs">Default Scheme</Label>
        <Switch checked={form.values.is_default} onCheckedChange={(value) => (form.values.is_default = value)} />
      </div>
    </div>
    {#if form.formError}
      <p class="text-xs text-destructive">{form.formError}</p>
    {/if}
    <DialogFooter>
      <Button size="sm" class="text-xs cursor-pointer" onclick={submit} disabled={isCreating || form.isSubmitting}>
        {#if isCreating || form.isSubmitting}<LoadingIndicator class="me-1" size={12} />{/if}
        {scheme ? m.common_action_update() : m.common_action_create()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
