<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import type { CreateTemplateRequest, TemplateDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { createDocument } from "@docforge/core";

import { l, isAr } from "$lib/core/i18n";
import SearchField from "$lib/shared/components/search-field.svelte";
import { Plus, RefreshCw, FileText } from "@lucide/svelte";
import { fade } from "svelte/transition";

// Component imports
import {
  TemplateGrid,
  CreateTemplateDialog,
  DeleteTemplateDialog,
} from "$lib/features/templates";
import { confirmStore } from "$lib/stores";

// State runes
let templates = $state<TemplateDto[]>([]);
let dataSchemas = $state<Record<string, any>>({});
let loading = $state(true);
let searchQuery = $state("");
let filterEntityType = $state("all");
let filterDocType = $state("all");

// Dialog & Modals State
let isCreateDialogOpen = $state(false);
let newTemplateName = $state("");
let newTemplateNameAr = $state("");
let newTemplateDesc = $state("");
let newTemplateEntityType = $state("enrollment");
let newTemplateDocType = $state("CERTIFICATE");
let newTemplatePreset = $state("blank");

// Dynamic parameters from route
const orgId: string = $derived(page.params.orgId ?? "");

// Load templates from database
async function loadTemplates() {
  if (!orgId) return;
  loading = true;
  try {
    const listFilter: any = {};
    if (filterEntityType !== "all") listFilter.entity_type = filterEntityType;
    if (filterDocType !== "all") listFilter.document_type = filterDocType;

    const response = await api.listTemplates(listFilter);
    let result = response || [];
    if (searchQuery.trim() !== "") {
      const q = searchQuery.trim().toLowerCase();
      result = result.filter(
        (t) =>
          t.name.toLowerCase().includes(q) ||
          (t.name_ar && t.name_ar.includes(q)) ||
          (t.description && t.description.toLowerCase().includes(q))
      );
    }
    templates = result;
  } catch (e: unknown) {
    console.error("Failed to load templates:", e);
    const msg = (e as any)?.message || "Failed to load templates";
    toast.error(msg);
  } finally {
    loading = false;
  }
}

// React to query changes
$effect(() => {
  if (orgId) {
    loadTemplates();
  }
});

// Handle Template Creation
function getDefaultPageSetup() {
  if (newTemplateDocType === "ID_CARD") return { size: "ID_CARD", orientation: "landscape" };
  if (newTemplateDocType === "EXAM_CARD") return { size: "A5", orientation: "portrait" };
  if (newTemplateDocType === "RECEIPT") return { size: "A4", orientation: "portrait" };
  return { size: "A4", orientation: "portrait" };
}

function buildInitialDocument() {
  const document = createDocument(
    newTemplateDocType,
    newTemplateEntityType,
    isAr() ? "ar" : "en",
  );
  document.meta.name = newTemplateName;
  document.meta.nameAr = newTemplateNameAr || undefined;
  const pageSetup = getDefaultPageSetup();
  document.meta.pageSetup.size = pageSetup.size as any;
  document.meta.pageSetup.orientation = pageSetup.orientation as any;

  const schema = dataSchemas[newTemplateEntityType] ?? {};
  for (const [field, definition] of Object.entries(schema)) {
    const type = (definition as any)?.type;
    if (["string", "number", "boolean", "date", "image", "array"].includes(type)) {
      document.variables[field] = {
        source: field,
        type,
        label: field,
        labelAr: (definition as any)?.label,
      } as any;
    }
  }

  const text = (id: string, content: string, y: number, fontSize: number) => ({
    type: "Text",
    id,
    props: {
      content,
      fontSize,
      fontFamily: "Tahoma",
      fontWeight: fontSize >= 20 ? "bold" : "normal",
      textAlign: "center",
      direction: "rtl",
    },
    position: { x: 15, y, width: "100%", height: 18 },
    positionMode: "absolute",
  });

  const page = document.pages[0]!;
  if (newTemplatePreset === "certificate") {
    page.children = [
      { type: "Header", id: "header", props: { height: 25 }, children: [text("header-text", "$org.name_ar", 8, 18)] },
      text("title", newTemplateNameAr || newTemplateName, 65, 26),
      text("student", "{{student.nameAr}}", 110, 20),
      { type: "Footer", id: "footer", props: { height: 15, showPageNumber: true }, children: [text("footer-text", "{{student.nationalId}}", 270, 10)] },
    ] as any;
  } else if (newTemplatePreset === "transcript") {
    page.children = [
      text("title", newTemplateNameAr || newTemplateName, 15, 22),
      {
        type: "Table",
        id: "results-table",
        props: {
          dataSource: "subjects",
          columns: [
            { header: "المادة", field: "nameAr", width: "40%" },
            { header: "الدرجة", field: "score", width: "30%" },
            { header: "التقدير", field: "grade", width: "30%" },
          ],
          showHeader: true,
          showGrid: true,
          direction: "rtl",
        },
        position: { x: 15, y: 55, width: "100%", height: 100 },
        positionMode: "absolute",
      },
    ] as any;
  } else if (newTemplatePreset === "receipt") {
    page.children = [
      text("title", newTemplateNameAr || newTemplateName, 20, 22),
      text("payer", "{{payerName}}", 70, 16),
      text("amount", "{{amount | number:'0.00'}} {{currency}}", 110, 24),
      { type: "QRCode", id: "receipt-qr", props: { data: "{{receiptNumber}}", size: 25, errorCorrection: "M" }, position: { x: 85, y: 160, width: 25, height: 25 }, positionMode: "absolute" },
    ] as any;
  }

  return document;
}

async function handleCreateTemplate() {
  if (!newTemplateName.trim()) {
    toast.error("Template name in English is required");
    return;
  }

  try {
    const pageSetup = getDefaultPageSetup();
    const defaultDocJson = JSON.stringify(buildInitialDocument());
    const createRequest: CreateTemplateRequest = {
      name: newTemplateName,
      name_ar: newTemplateNameAr || null,
      description: newTemplateDesc || null,
      document_type: newTemplateDocType,
      entity_type: newTemplateEntityType,
      page_size: pageSetup.size,
      orientation: pageSetup.orientation,
      document_json: defaultDocJson,
      locale: isAr() ? "ar" : "en",
    };

    const response = await api.createTemplate(createRequest);

    toast.success("Template created successfully");
    isCreateDialogOpen = false;

    // Redirect to editor page
    goto(`/${orgId}/templates/${response.id}`);
  } catch (e: unknown) {
    console.error("Failed to create template:", e);
    const msg = (e as any)?.message || "Failed to create template";
    toast.error(msg);
  }
}

// Trigger Delete
async function confirmDelete(id: string) {
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete() + "؟",
    description: m.common_docs_delete_confirm
      ? m.common_docs_delete_confirm()
      : "Are you sure you want to delete this template?",
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await api.deleteTemplate(id);
    toast.success("Template deleted successfully");
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to delete template:", e);
    const msg = (e as any)?.message || "Failed to delete template";
    toast.error(msg);
  }
}

// Handle Template Publishing/Activation
async function publishTemplate(id: string) {
  try {
    await api.publishTemplate(id);
    toast.success(m.settings_templates_publish_success());
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to publish template:", e);
    const msg = (e as any)?.message || "Failed to publish template";
    toast.error(msg);
  }
}

// Handle Template Archiving
async function archiveTemplate(id: string) {
  try {
    await api.archiveTemplate(id);
    toast.success(m.settings_templates_archive_success());
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to archive template:", e);
    const msg = (e as any)?.message || "Failed to archive template";
    toast.error(msg);
  }
}

// Handle Template Restoring
async function restoreTemplate(id: string) {
  try {
    await api.publishTemplate(id);
    toast.success(m.settings_templates_restore_success());
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to restore template:", e);
    const msg = (e as any)?.message || "Failed to restore template";
    toast.error(msg);
  }
}
</script>

<svelte:head>
  <title>{m.settings_nav_templates()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide">
  <div class="w-full p-6 md:p-8 flex flex-col gap-8">
    <div class="flex flex-wrap items-center justify-between gap-4 select-none" in:fade={{ delay: 50, duration: 200 }}>
      
      <div class="flex flex-wrap items-center gap-3 shrink-0">
        <div class="w-full sm:w-72">
          <SearchField
            bind:value={searchQuery}
            oninput={loadTemplates}
            placeholder={m.settings_templates_search_placeholder()}
          />
        </div>

          <Select type="single" bind:value={filterEntityType} onValueChange={loadTemplates}>
            <SelectTrigger class="h-9 text-xs bg-background/40 border-border/40 hover:bg-muted/40 transition-colors">
              {({ all: m.settings_templates_filter_entity_all(), enrollment: m.common_bulk_card_students_tag_enrollment(), payment: m.settings_templates_filter_entity_payment(), exam: m.settings_templates_filter_entity_exam(), user: m.common_agent_default_user_name(), organization: m.settings_templates_filter_entity_organization() } as any)[filterEntityType] || m.common_print_multi_up_entity_type()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">{m.settings_templates_filter_entity_all()}</SelectItem>
              <SelectItem value="enrollment">{m.common_bulk_card_students_tag_enrollment()}</SelectItem>
              <SelectItem value="payment">{m.settings_templates_filter_entity_payment()}</SelectItem>
              <SelectItem value="exam">{m.settings_templates_filter_entity_exam()}</SelectItem>
              <SelectItem value="user">{m.common_agent_default_user_name()}</SelectItem>
              <SelectItem value="organization">{m.settings_templates_filter_entity_organization()}</SelectItem>
            </SelectContent>
          </Select>

          <Select type="single" bind:value={filterDocType} onValueChange={loadTemplates}>
            <SelectTrigger class="h-9 text-xs bg-background/40 border-border/40 hover:bg-muted/40 transition-colors">
              {({ all: m.settings_templates_filter_doc_all(), CERTIFICATE: m.settings_templates_filter_doc_certificate(), ID_CARD: m.settings_templates_filter_doc_id_card(), RECEIPT: m.finance_layout_nav_payments(), EXAM_CARD: m.settings_templates_filter_doc_exam_card(), REPORT: m.common_nav_reports() } as any)[filterDocType] || m.settings_permissions_col_doc_type()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">{m.settings_templates_filter_doc_all()}</SelectItem>
              <SelectItem value="CERTIFICATE">{m.settings_templates_filter_doc_certificate()}</SelectItem>
              <SelectItem value="ID_CARD">{m.settings_templates_filter_doc_id_card()}</SelectItem>
              <SelectItem value="RECEIPT">{m.finance_layout_nav_payments()}</SelectItem>
              <SelectItem value="EXAM_CARD">{m.settings_templates_filter_doc_exam_card()}</SelectItem>
              <SelectItem value="REPORT">{m.common_nav_reports()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

      <!-- Right side (in LTR) / Left side (in RTL): Action Button -->
      <div class="flex items-center shrink-0">
        <Button variant="default" size="sm" class="h-9 px-4 text-xs font-semibold shadow-sm" onclick={() => (isCreateDialogOpen = true)}>
          <Plus size={16} class="me-1.5" />
          {m.settings_templates_create_btn()}
        </Button>
      </div>
    </div>

    <!-- Templates Grid -->
    <TemplateGrid
      {templates}
      {loading}
      {orgId}
      onconfirmdelete={confirmDelete}
      onpublish={publishTemplate}
      onarchive={archiveTemplate}
      onrestore={restoreTemplate}
      oncreatenew={() => (isCreateDialogOpen = true)}
    />
  </div>
</div>

<!-- Create Dialog -->
<CreateTemplateDialog
  bind:open={isCreateDialogOpen}
  bind:name={newTemplateName}
  bind:nameAr={newTemplateNameAr}
  bind:desc={newTemplateDesc}
  bind:entityType={newTemplateEntityType}
  bind:docType={newTemplateDocType}
  bind:preset={newTemplatePreset}
  schemas={dataSchemas}
  onsubmit={handleCreateTemplate}
/>
