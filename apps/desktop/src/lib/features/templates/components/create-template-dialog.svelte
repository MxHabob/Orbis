<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { FormField } from "@orbis/ui";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { templateInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

interface Props {
  open: boolean;
  name: string;
  nameAr: string;
  docType: string;
  onsubmit: () => void;
}

let {
  open = $bindable(),
  name = $bindable(),
  nameAr = $bindable(),
  docType = $bindable(),
  onsubmit,
}: Props = $props();

function entityForDocument(type: string): string {
  if (type === "RECEIPT") return "payment";
  if (type === "EXAM_CARD") return "exam";
  if (type === "ID_CARD") return "user";
  if (type === "REPORT" || type === "TRANSCRIPT") return "period_score";
  return "enrollment";
}

function presetForDocument(type: string): string {
  if (type === "RECEIPT") return "receipt";
  if (type === "REPORT" || type === "TRANSCRIPT") return "transcript";
  if (type === "CERTIFICATE" || type === "TERM_CERTIFICATE" || type === "ANNUAL_CERTIFICATE") {
    return "certificate";
  }
  return "blank";
}

const form = new FormController({
  initialValues: {
    name: "",
    name_ar: "",
    description: "",
    entity_type: "",
    document_type: "",
    preset: "",
  },
  schema: templateInputSchema,
});

async function handleSubmit() {
  form.values = {
    name,
    name_ar: nameAr,
    description: "",
    entity_type: entityForDocument(docType),
    document_type: docType,
    preset: presetForDocument(docType),
  };
  if (await form.submit(async () => {})) onsubmit();
}

const documentTypes = [
  "CERTIFICATE",
  "TERM_CERTIFICATE",
  "ANNUAL_CERTIFICATE",
  "REPORT",
  "TRANSCRIPT",
  "ID_CARD",
  "EXAM_CARD",
  "RECEIPT",
];

function documentLabel(type: string) {
  const labels: Record<string, string> = {
    CERTIFICATE: m.template_doc_certificate(),
    ID_CARD: m.template_doc_id_card(),
    RECEIPT: m.template_doc_receipt(),
    EXAM_CARD: m.template_doc_seating(),
    REPORT: m.template_doc_report(),
    TRANSCRIPT: m.template_doc_transcript(),
    TERM_CERTIFICATE: m.template_doc_term_certificate(),
    ANNUAL_CERTIFICATE: m.template_doc_annual_certificate(),
    CUSTOM: m.template_doc_custom(),
  };
  return labels[type] ?? type;
}
</script>

<Dialog bind:open={open}>
  <DialogContent class="sm:max-w-120 text-start">
    <DialogHeader class="gap-0">
      <DialogTitle>{m.settings_templates_create_btn()}</DialogTitle>
    </DialogHeader>

    <div class="flex flex-col gap-5 py-2">
      <BilingualNameFields
        arId="name-ar"
        enId="name-en"
        arLabel={m.template_label_name_ar()}
        enLabel={m.template_label_name_en()}
        bind:arValue={nameAr}
        bind:enValue={name}
        arPlaceholder={m.template_placeholder_name_ar()}
        enPlaceholder={m.template_placeholder_name_en()}
        arError={form.errors.name_ar?.[0]}
        enError={form.errors.name?.[0]}
        kind="label"
        requestSuggestion={suggestLocalTranslation}
        enRequired
        disabled={form.isSubmitting}
        loadingMessage={m.bilingual_entry_loading()}
        failureMessage={m.bilingual_entry_failure()}
      />

      <FormField label={m.template_label_doc_type()} error={form.errors.document_type?.[0]} required>
        <Select type="single" bind:value={docType}>
          <SelectTrigger>{documentLabel(docType)}</SelectTrigger>
          <SelectContent>
            {#each documentTypes as value (value)}
              <SelectItem value={value}>{documentLabel(value)}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </FormField>

      <p class="text-xs text-muted-foreground">
        {m.template_create_hint()}
      </p>
    </div>

    <DialogFooter class="gap-2">
      <Button variant="outline" size="sm" onclick={() => (open = false)}>{m.common_action_cancel()}</Button>
      <Button variant="default" size="sm" onclick={handleSubmit} disabled={form.isSubmitting}>{m.common_form_btn_create()}</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
