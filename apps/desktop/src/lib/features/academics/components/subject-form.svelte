<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { createSubjectInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import InputColor from "$lib/shared/components/input-color.svelte";
import { toast } from "@orbis/ui";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

type SubjectDraft = {
  name: string;
  name_ar: string;
  code: string;
  credit_hours: number;
  color: string | null;
};

const form = new FormController<SubjectDraft>({
  initialValues: { name: "", name_ar: "", code: "", credit_hours: 3, color: null },
  schema: createSubjectInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}
let {
  onSuccess,
  onCancel,
}: {
  orgId: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (draft) => {
    await api.createSubject({
      code: draft.code,
      name: draft.name || draft.name_ar || "",
      name_ar: draft.name_ar || draft.name || undefined,
      credit_hours: String(draft.credit_hours ?? 3),
      is_core: true,
    });
  });
  if (submitted) {
    form.reset();
    onSuccess();
  } else if (form.formError) {
    toast.error(form.formError);
  } else if (Object.keys(form.errors).length > 0) {
    toast.error(m.common_subject_toast_error_name());
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-xl font-bold text-foreground">
      {m.common_subject_sheet_title()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
    <BilingualNameFields
      arId="sub_name_ar"
      enId="sub_name_en"
      arLabel={m.common_subject_label_name_ar()}
      enLabel={m.common_subject_label_name_en()}
      bind:arValue={form.values.name_ar}
      bind:enValue={form.values.name}
      arPlaceholder={m.common_subject_placeholder_name_ar()}
      enPlaceholder={m.common_subject_placeholder_name_en()}
      arError={fieldError("name_ar")}
      enError={fieldError("name")}
      onArBlur={() => form.markTouched("name_ar")}
      onEnBlur={() => form.markTouched("name")}
      kind="label"
      requestSuggestion={suggestLocalTranslation}
      arRequired
      disabled={form.isSubmitting}
      loadingMessage={m.bilingual_entry_loading()}
      failureMessage={m.bilingual_entry_failure()}
    >
      {#snippet arControl({ id, placeholder, required, disabled, ariaInvalid, ariaDescribedBy, onInput, onBlur })}
        <InputColor
          {id}
          {placeholder}
          bind:value={form.values.name_ar}
          bind:color={form.values.color}
          onvalueinput={onInput}
          onblur={onBlur}
          {required}
          {disabled}
          ariaInvalid={ariaInvalid}
          ariaDescribedBy={ariaDescribedBy}
        />
      {/snippet}
    </BilingualNameFields>

    <!-- Subject Code -->
    <FormField id="sub_code" label={m.common_subject_label_code()} error={fieldError("code")} required>
      {#snippet children()}
      <Input 
        id="sub_code"
        type="text" 
        bind:value={form.values.code}
        onblur={() => form.markTouched("code")}
        aria-invalid={Boolean(fieldError("code"))}
        aria-describedby={fieldError("code") ? "sub_code_error" : undefined}
        placeholder={m.common_subject_placeholder_code()}
      />
      {/snippet}
    </FormField>

    <!-- Credit Hours -->
    <div class="flex flex-col gap-2 text-start">
      <label for="sub_weight" class="text-sm font-medium text-foreground">
        {m.common_subject_label_periods()}
      </label>
      <Input 
        id="sub_weight"
        type="number" 
        bind:value={form.values.credit_hours}
        min="1"
        max="20"
      />
    </div>
  </div>

  <!-- Form Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
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
      disabled={form.isSubmitting}
    >
      {form.isSubmitting ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
