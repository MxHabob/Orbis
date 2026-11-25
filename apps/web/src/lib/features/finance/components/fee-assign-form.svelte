<script lang="ts">
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import type { EnrollmentDto, FeeTemplateDto } from "@orbis/sdk";
import * as m from "$lib/paraglide/messages";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { feeAssignmentInputSchema } from "@orbis/schemas";
import { FormField, toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";

let {
  orgId,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

// State
let students = $state<EnrollmentDto[]>([]);
let templates = $state<FeeTemplateDto[]>([]);
let isLoadingDropdowns = $state(false);

// Form state
const form = new FormController({
  initialValues: {
    enrollment_id: "",
    fee_template_id: "",
  },
  schema: feeAssignmentInputSchema,
});

const selectedTemplate = $derived(templates.find((item) => item.id === form.values.fee_template_id));
let dueDate = $state(getLocalDateInputValue());

function getLocalDateInputValue(date = new Date()) {
  return new Date(date.getTime() - date.getTimezoneOffset() * 60_000)
    .toISOString()
    .slice(0, 10);
}

onMount(async () => {
  if (!orgId) return;
  isLoadingDropdowns = true;
  try {
    const [studentList, templateList] = await Promise.all([
      api.listEnrollments(),
      api.listFeeTemplates(),
    ]);
    students = studentList;
    templates = templateList;

    if (studentList.length > 0) {
      form.values.enrollment_id = String(studentList[0].id);
    }
    if (templateList.length > 0) {
      form.values.fee_template_id = String(templateList[0].id);
    }
  } catch (e) {
    toast.error(extractErrorMessage(e));
  } finally {
    isLoadingDropdowns = false;
  }
});

function handleTemplateChange(val: string | null) {
  if (!val) return;
  form.values.fee_template_id = val;
  form.markTouched("fee_template_id");
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (payload) => {
    const enrollment = students.find((item) => item.id === payload.enrollment_id);
    if (!enrollment) {
      throw new Error(m.common_fee_label_student());
    }
    await api.generateStudentFee({
      enrollment_id: enrollment.id,
      student_id: enrollment.student_id,
      period_id: enrollment.period_id,
      fee_template_id: payload.fee_template_id,
      due_date: dueDate,
    });
    toast.success(m.common_fee_toast_success());
    form.reset();
    dueDate = getLocalDateInputValue();
  });
  if (submitted) {
    onSuccess();
  } else if (form.formError) {
    toast.error(form.formError);
  } else if (Object.keys(form.errors).length > 0) {
    toast.error(m.common_fee_toast_error_amount());
  }
}
</script>

<form
  onsubmit={handleSubmit}
  class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide"
>
  <!-- Student Select -->
  <FormField
    id="fee_student"
    label={m.common_fee_label_student()}
    error={form.errors.enrollment_id?.[0]}
    required
  >
    {#snippet children()}
      <SearchableSelect
        value={form.values.enrollment_id}
        items={students}
        placeholder={m.common_fee_dropdown_loading_students()}
        getItemValue={(student) => String(student.id)}
        getItemLabel={(student) => student.student_number || String(student.id)}
        onchange={(value) => {
          if (value) form.values.enrollment_id = value;
          form.markTouched("enrollment_id");
        }}
        disabled={isLoadingDropdowns || form.isSubmitting}
        searchPlaceholder={m.common_fee_label_student()}
        class="w-full"
      />
    {/snippet}
  </FormField>

  <!-- Fee Template Select -->
  <FormField
    id="fee_ledger"
    label={m.common_fee_label_ledger()}
    error={form.errors.fee_template_id?.[0]}
    required
  >
    {#snippet children()}
      <SearchableSelect
        value={form.values.fee_template_id}
        items={templates}
        placeholder={m.common_fee_dropdown_loading_ledgers()}
        getItemValue={(template) => String(template.id)}
        getItemLabel={(template) => template.name_ar || template.name || template.code || String(template.id)}
        onchange={handleTemplateChange}
        disabled={isLoadingDropdowns || form.isSubmitting}
        searchPlaceholder={m.common_fee_label_ledger()}
        class="w-full"
      />
    {/snippet}
  </FormField>

  <!-- Fee Amount -->
  <FormField
    id="fee_amount"
    label={m.common_fee_label_amount()}
  >
    {#snippet children()}
      <Input
        id="fee_amount"
        value={selectedTemplate?.default_amount ?? ""}
        readonly
      />
    {/snippet}
  </FormField>

  <!-- Due Date -->
  <FormField id="fee_due" label={m.common_fee_label_due()}>
    {#snippet children()}
      <Input
        id="fee_due"
        type="date"
        bind:value={dueDate}
        required
        disabled={form.isSubmitting}
      />
    {/snippet}
  </FormField>

  <!-- Form Actions -->
  <div class="flex justify-end gap-2 pt-2">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={form.isSubmitting}
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold"
      disabled={form.isSubmitting || isLoadingDropdowns}
    >
      {form.isSubmitting
        ? m.common_fee_toast_assigning()
        : m.common_fee_button_assign()}
    </Button>
  </div>
</form>
