<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

interface Props {
  open: boolean;
  name: string;
  nameAr: string;
  desc: string;
  entityType: string;
  docType: string;
  preset: string;
  schemas?: Record<string, any>;
  onsubmit: () => void;
}

let {
  open = $bindable(),
  name = $bindable(),
  nameAr = $bindable(),
  desc = $bindable(),
  entityType = $bindable(),
  docType = $bindable(),
  preset = $bindable(),
  schemas = {},
  onsubmit,
}: Props = $props();

const entityLabels: Record<string, string> = {
  enrollment: m.template_entity_student_reg(),
  payment: m.template_entity_receipts(),
  exam: m.template_entity_seat_numbers(),
  user: m.template_entity_id_cards(),
  organization: m.template_entity_school(),
  period_score: m.template_entity_period_score(),
  report_card: m.template_entity_report_card(),
  term_certificate: m.template_entity_term_certificate(),
  annual_certificate: m.template_entity_annual_certificate(),
  exam_session: m.template_entity_exam_session(),
};

const entityDocuments: Record<string, string[]> = {
  enrollment: ["CERTIFICATE", "ID_CARD", "REPORT"],
  payment: ["RECEIPT", "REPORT"],
  exam: ["EXAM_CARD", "REPORT"],
  user: ["ID_CARD", "CERTIFICATE", "REPORT"],
  organization: ["CERTIFICATE", "REPORT"],
  period_score: ["REPORT", "TRANSCRIPT"],
  report_card: ["REPORT", "TRANSCRIPT"],
  term_certificate: ["CERTIFICATE", "TERM_CERTIFICATE"],
  annual_certificate: ["CERTIFICATE", "ANNUAL_CERTIFICATE"],
  exam_session: ["EXAM_CARD", "REPORT"],
};

const allEntities = $derived(
  Array.from(new Set([...Object.keys(entityLabels), ...Object.keys(schemas)])),
);
const availableDocumentTypes = $derived(entityDocuments[entityType] ?? ["CUSTOM"]);
const schemaFieldCount = $derived(
  Object.keys(schemas[entityType] ?? {}).filter((key) => key !== "type" && key !== "description").length,
);

$effect(() => {
  if (!availableDocumentTypes.includes(docType)) {
    docType = availableDocumentTypes[0] ?? "CUSTOM";
  }
});

$effect(() => {
  const presetByDocument: Record<string, string> = {
    CERTIFICATE: "certificate",
    TERM_CERTIFICATE: "certificate",
    ANNUAL_CERTIFICATE: "certificate",
    REPORT: "transcript",
    TRANSCRIPT: "transcript",
    RECEIPT: "receipt",
  };
  if (presetByDocument[docType]) {
    preset = presetByDocument[docType];
  }
});

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

    <div class="flex flex-col gap-3 py-1">
      <div class="flex flex-col gap-2">
        <Label for="name-en" class="text-xs font-semibold text-muted-foreground">{m.template_label_name_en()}</Label>
        <Input id="name-en" bind:value={name} placeholder={m.template_placeholder_name_en()} />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="name-ar" class="text-xs font-semibold text-muted-foreground">{m.template_label_name_ar()}</Label>
        <Input id="name-ar" bind:value={nameAr} placeholder={m.template_placeholder_name_ar()} />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="desc" class="text-xs font-semibold text-muted-foreground">{m.template_label_desc()}</Label>
        <Input id="desc" bind:value={desc} placeholder={m.template_placeholder_description()} />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_label_entity()}</Label>
          <Select type="single" bind:value={entityType}>
            <SelectTrigger>
              {entityLabels[entityType] ?? entityType}
            </SelectTrigger>
            <SelectContent>
              {#each allEntities as value (value)}
                <SelectItem value={value}>{entityLabels[value] ?? value}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
          <span class="text-[10px] text-muted-foreground">{m.template_available_fields({ count: String(schemaFieldCount) })}</span>
        </div>

        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_label_doc_type()}</Label>
          <Select type="single" bind:value={docType}>
            <SelectTrigger>
              {documentLabel(docType)}
            </SelectTrigger>
            <SelectContent>
              {#each availableDocumentTypes as value (value)}
                <SelectItem value={value}>{documentLabel(value)}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="flex flex-col gap-2">
        <Label class="text-xs font-semibold text-muted-foreground">{m.template_label_preset()}</Label>
        <Select type="single" bind:value={preset}>
          <SelectTrigger>{preset === "blank" ? m.template_preset_blank() : preset === "certificate" ? m.template_preset_certificate() : preset === "transcript" ? m.template_preset_transcript() : m.template_preset_receipt()}</SelectTrigger>
          <SelectContent>
            <SelectItem value="blank">{m.template_preset_blank()}</SelectItem>
            <SelectItem value="certificate">{m.template_preset_certificate()}</SelectItem>
            <SelectItem value="transcript">{m.template_preset_transcript()}</SelectItem>
            <SelectItem value="receipt">{m.template_preset_receipt()}</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>

    <DialogFooter class="gap-2">
      <Button variant="outline" size="sm" onclick={() => (open = false)}>{m.common_action_cancel()}</Button>
      <Button variant="default" size="sm" onclick={onsubmit}>{m.common_form_btn_create()}</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
