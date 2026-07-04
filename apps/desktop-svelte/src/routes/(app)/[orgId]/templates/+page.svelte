<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { l, isAr } from "$lib/core/i18n";
import SearchField from "$lib/shared/components/search-field.svelte";
import { Plus, RefreshCw, FileText } from "lucide-svelte";
import { fade } from "svelte/transition";

// Component imports
import {
  TemplateGrid,
  CreateTemplateDialog,
  DeleteTemplateDialog,
} from "$lib/features/templates";

// State runes
let templates = $state<any[]>([]);
let loading = $state(true);
let searchQuery = $state("");
let filterEntityType = $state("all");
let filterDocType = $state("all");

// Dialog & Dialog State
let isCreateDialogOpen = $state(false);
let newTemplateName = $state("");
let newTemplateNameAr = $state("");
let newTemplateDesc = $state("");
let newTemplateEntityType = $state("enrollment");
let newTemplateDocType = $state("CERTIFICATE");
let newTemplatePageSize = $state("A4");
let newTemplateOrientation = $state("portrait");

// Alert Dialog delete state
let isDeleteDialogOpen = $state(false);
let templateToDeleteId = $state("");

// Dynamic parameters from route
const orgId: string = $derived(page.params.orgId ?? "");

// Load templates from database
async function loadTemplates() {
  if (!orgId) return;
  loading = true;
  try {
    const listParams: any = {};
    if (filterEntityType !== "all") listParams.entity_type = filterEntityType;
    if (filterDocType !== "all") listParams.document_type = filterDocType;
    if (searchQuery.trim() !== "") listParams.search = searchQuery.trim();

    const response = await commands.documentTemplates.getTemplates(
      orgId,
      listParams,
    );
    templates = response || [];
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

// Trigger load on mounting
onMount(() => {
  if (orgId) {
    loadTemplates();
  }
});

// Handle Template Creation
async function handleCreateTemplate() {
  if (!newTemplateName.trim()) {
    toast.error("Template name in English is required");
    return;
  }

  try {
    const defaultDocJson = JSON.stringify({
      version: "1.0",
      meta: {
        name: newTemplateName,
        nameAr: newTemplateNameAr || null,
        documentType: newTemplateDocType,
        entityType: newTemplateEntityType,
        locale: isAr() ? "ar" : "en",
        pageSetup: {
          size: newTemplatePageSize,
          orientation: newTemplateOrientation,
          margins: { top: 15, bottom: 15, left: 15, right: 15 },
          unit: "mm",
        },
      },
      variables: {},
      assets: {},
      root: {
        type: "Page",
        id: "page-root",
        children: [
          {
            type: "Frame",
            id: "content-root",
            props: {
              layout: "vertical",
              gap: 10,
            },
            children: [
              {
                type: "Text",
                id: "main-title",
                props: {
                  content: newTemplateNameAr || newTemplateName,
                  fontSize: 24,
                  fontFamily: "Tahoma",
                  fontWeight: "bold",
                  textAlign: "center",
                },
              },
            ],
          },
        ],
      },
    });

    const createRequest = {
      name: newTemplateName,
      name_ar: newTemplateNameAr || null,
      description: newTemplateDesc || null,
      description_ar: null,
      document_type: newTemplateDocType,
      entity_type: newTemplateEntityType,
      category_id: null,
      code: null,
      page_size: newTemplatePageSize,
      orientation: newTemplateOrientation,
      margins_json: JSON.stringify({
        top: 15,
        bottom: 15,
        left: 15,
        right: 15,
      }),
      document_json: defaultDocJson,
      locale: isAr() ? "ar" : "en",
      tags: null,
    };

    const userId = "current-user";
    const response = await commands.documentTemplates.createTemplate(
      orgId,
      userId,
      createRequest,
    );

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
function confirmDelete(id: string) {
  templateToDeleteId = id;
  isDeleteDialogOpen = true;
}

async function handleDeleteTemplate() {
  if (!templateToDeleteId) return;

  try {
    await commands.documentTemplates.deleteTemplate(orgId, templateToDeleteId);
    toast.success("Template deleted successfully");
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to delete template:", e);
    const msg = (e as any)?.message || "Failed to delete template";
    toast.error(msg);
  } finally {
    isDeleteDialogOpen = false;
    templateToDeleteId = "";
  }
}

// Handle Template Publishing/Activation
async function publishTemplate(id: string) {
  try {
    const userId = "current-user";
    await commands.documentTemplates.publishTemplate(orgId, id, userId);
    toast.success(m.templates_publish_success());
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
    const userId = "current-user";
    await commands.documentTemplates.archiveTemplate(orgId, id, userId);
    toast.success(m.templates_archive_success());
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
    const userId = "current-user";
    await commands.documentTemplates.restoreTemplate(orgId, id, userId);
    toast.success(m.templates_restore_success());
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to restore template:", e);
    const msg = (e as any)?.message || "Failed to restore template";
    toast.error(msg);
  }
}
</script>

<svelte:head>
  <title>{m.templates_window_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="w-full p-6 flex flex-col gap-6">
    <!-- Header -->
    <div
      class="bg-card border border-border rounded-xl p-5 flex flex-col md:flex-row md:items-center justify-between gap-4"
      in:fade={{ duration: 150 }}
    >
      <div class="flex items-center gap-3 text-start">
        <div class="h-10 w-10 rounded-xl bg-primary/10 text-primary flex items-center justify-center shrink-0">
          <FileText size={22} />
        </div>
        <div>
          <h1 class="text-sm font-bold text-foreground">{m.templates_page_title()}</h1>
          <p class="text-[10px] text-muted-foreground mt-0.5">
            {m.templates_page_desc()}
          </p>
        </div>
      </div>
    </div>

    <!-- Filters Section -->
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 select-none shrink-0" in:fade={{ delay: 50, duration: 200 }}>
      <div class="flex items-center gap-3 flex-wrap">
        <!-- Search bar -->
        <SearchField
          bind:value={searchQuery}
          onchange={loadTemplates}
          placeholder={m.templates_search_placeholder()}
        />

        <div class="flex items-center gap-2">
          <!-- Entity Type Select -->
          <div class="min-w-[140px]">
            <Select type="single" bind:value={filterEntityType} onValueChange={loadTemplates}>
              <SelectTrigger class="h-9 text-xs bg-background/50">
                {({ all: m.settings_templates_filter_entity_all(), enrollment: m.settings_templates_filter_entity_enrollment(), payment: m.settings_templates_filter_entity_payment(), exam: m.settings_templates_filter_entity_exam(), user: m.settings_templates_filter_entity_user(), organization: m.settings_templates_filter_entity_organization() } as any)[filterEntityType] || m.settings_templates_filter_entity_trigger()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">{m.settings_templates_filter_entity_all()}</SelectItem>
                <SelectItem value="enrollment">{m.settings_templates_filter_entity_enrollment()}</SelectItem>
                <SelectItem value="payment">{m.settings_templates_filter_entity_payment()}</SelectItem>
                <SelectItem value="exam">{m.settings_templates_filter_entity_exam()}</SelectItem>
                <SelectItem value="user">{m.settings_templates_filter_entity_user()}</SelectItem>
                <SelectItem value="organization">{m.settings_templates_filter_entity_organization()}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <!-- Doc Type Select -->
          <div class="min-w-[140px]">
            <Select type="single" bind:value={filterDocType} onValueChange={loadTemplates}>
              <SelectTrigger class="h-9 text-xs bg-background/50">
                {({ all: m.settings_templates_filter_doc_all(), CERTIFICATE: m.settings_templates_filter_doc_certificate(), ID_CARD: m.settings_templates_filter_doc_id_card(), RECEIPT: m.settings_templates_filter_doc_receipt(), EXAM_CARD: m.settings_templates_filter_doc_exam_card(), REPORT: m.settings_templates_filter_doc_report() } as any)[filterDocType] || m.settings_templates_filter_doc_trigger()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">{m.settings_templates_filter_doc_all()}</SelectItem>
                <SelectItem value="CERTIFICATE">{m.settings_templates_filter_doc_certificate()}</SelectItem>
                <SelectItem value="ID_CARD">{m.settings_templates_filter_doc_id_card()}</SelectItem>
                <SelectItem value="RECEIPT">{m.settings_templates_filter_doc_receipt()}</SelectItem>
                <SelectItem value="EXAM_CARD">{m.settings_templates_filter_doc_exam_card()}</SelectItem>
                <SelectItem value="REPORT">{m.settings_templates_filter_doc_report()}</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2 flex-nowrap shrink-0">
        <Button variant="lime" size="sm" class="h-9 text-xs font-semibold" onclick={() => (isCreateDialogOpen = true)}>
          <Plus size={16} class="me-1.5" />
          {m.templates_create_btn()}
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
  bind:pageSize={newTemplatePageSize}
  bind:orientation={newTemplateOrientation}
  onsubmit={handleCreateTemplate}
/>

<!-- Delete Dialog -->
<DeleteTemplateDialog
  bind:open={isDeleteDialogOpen}
  onconfirm={handleDeleteTemplate}
/>
