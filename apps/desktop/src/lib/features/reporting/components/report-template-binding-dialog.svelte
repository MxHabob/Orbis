<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage } from "$lib/api";
import type {
  ReportDefinitionDto,
  TemplateBindingDto,
  TemplateDto,
} from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { isAr } from "$lib/core/i18n";

interface Props {
  open: boolean;
  definition: ReportDefinitionDto | null;
  orgId: string;
  onOpenChange: (open: boolean) => void;
}

let { open, definition, orgId, onOpenChange }: Props = $props();

let templates = $state<TemplateDto[]>([]);
let bindings = $state<TemplateBindingDto[]>([]);
let locale = $state<"ar" | "en">("ar");
let templateId = $state("none");
let isLoading = $state(false);
let isSaving = $state(false);
let loadError = $state("");
let loadedDefinitionId = "";

const currentBinding = $derived(
  bindings.find(
    (binding) =>
      binding.locale.toLowerCase() === locale &&
      binding.format.toLowerCase() === "pdf",
  ),
);

$effect(() => {
  if (!open) {
    loadedDefinitionId = "";
    return;
  }
  if (definition && loadedDefinitionId !== definition.id) {
    loadedDefinitionId = definition.id;
    locale = isAr() ? "ar" : "en";
    void loadConfiguration(definition);
  }
});

$effect(() => {
  templateId = currentBinding?.template_id ?? "none";
});

async function loadConfiguration(report: ReportDefinitionDto) {
  isLoading = true;
  loadError = "";
  try {
    const [availableTemplates, reportBindings] = await Promise.all([
      api.listTemplates({ status: "published" }),
      api.listTemplateBindings(report.id),
    ]);
    templates = availableTemplates.filter(
      (template) =>
        template.status.toLowerCase() === "published" &&
        template.document_type.toLowerCase() === "report",
    );
    bindings = reportBindings;
  } catch (error: unknown) {
    loadError =
      extractErrorMessage(error) ||
      m.operations_ui_report_template_load_error();
    toast.error(loadError);
  } finally {
    isLoading = false;
  }
}

async function saveBinding() {
  if (!definition || templateId === "none") return;
  isSaving = true;
  try {
    await api.createTemplateBinding(definition.id, templateId, locale, "pdf");
    bindings = await api.listTemplateBindings(definition.id);
    toast.success(m.operations_ui_report_template_saved());
    onOpenChange(false);
  } catch (error: unknown) {
    toast.error(
      extractErrorMessage(error) ||
        m.operations_ui_report_template_save_error(),
    );
  } finally {
    isSaving = false;
  }
}
</script>

<Dialog open={open} onOpenChange={onOpenChange}>
  <DialogContent class="sm:max-w-lg">
    <DialogHeader>
      <DialogTitle>{m.operations_ui_report_template_title()}</DialogTitle>
      <DialogDescription>
        {m.operations_ui_report_template_description({
          name: definition?.name_ar || definition?.name || "",
        })}
      </DialogDescription>
    </DialogHeader>

    {#if isLoading}
      <div class="py-8 text-center text-sm text-muted-foreground" aria-live="polite">
        {m.common_loading()}
      </div>
    {:else if loadError}
      <div class="py-6 text-center text-sm text-destructive" role="alert">
        {loadError}
      </div>
    {:else if templates.length === 0}
      <div class="space-y-3 py-4 text-sm text-muted-foreground">
        <p>{m.operations_ui_report_template_no_published()}</p>
        <Button variant="outline" href={`/${orgId}/templates`}>
          {m.operations_ui_report_template_open_templates()}
        </Button>
      </div>
    {:else}
      <div class="space-y-4 py-2">
        <div class="space-y-1.5">
          <label for="report-template-locale" class="text-sm font-medium">
            {m.operations_ui_report_template_language()}
          </label>
          <Select
            type="single"
            value={locale}
            onValueChange={(value) => {
              if (value === "ar" || value === "en") locale = value;
            }}
          >
            <SelectTrigger id="report-template-locale" class="w-full">
              {locale === "ar"
                ? m.operations_ui_report_template_arabic()
                : m.operations_ui_report_template_english()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ar">{m.operations_ui_report_template_arabic()}</SelectItem>
              <SelectItem value="en">{m.operations_ui_report_template_english()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="space-y-1.5">
          <label for="report-template-choice" class="text-sm font-medium">
            {m.operations_ui_report_template_pdf()}
          </label>
          <Select
            type="single"
            value={templateId}
            onValueChange={(value) => (templateId = value)}
          >
            <SelectTrigger id="report-template-choice" class="w-full">
              {templates.find((template) => template.id === templateId)?.name_ar ||
                templates.find((template) => template.id === templateId)?.name ||
                m.operations_ui_report_template_choose()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="none">{m.operations_ui_report_template_choose()}</SelectItem>
              {#each templates as template (template.id)}
                <SelectItem value={template.id}>
                  {template.name_ar || template.name}
                  <span class="ms-2 text-xs text-muted-foreground">
                    {template.locale.toLowerCase() === locale
                      ? m.operations_ui_report_template_matching_language()
                      : template.locale}
                  </span>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
          <p class="text-xs text-muted-foreground">
            {m.operations_ui_report_template_binding_hint()}
          </p>
        </div>
      </div>
    {/if}

    <DialogFooter>
      <Button variant="outline" onclick={() => onOpenChange(false)} disabled={isSaving}>
        {m.common_action_cancel()}
      </Button>
      {#if templates.length > 0}
        <Button
          onclick={saveBinding}
          disabled={isLoading || isSaving || templateId === "none" || !!loadError}
        >
          {isSaving
            ? m.operations_ui_saving()
            : m.operations_ui_report_template_save()}
        </Button>
      {/if}
    </DialogFooter>
  </DialogContent>
</Dialog>
