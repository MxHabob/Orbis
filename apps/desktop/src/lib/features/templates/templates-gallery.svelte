<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import type { CreateTemplateRequest } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { createDocument, createNode } from "@docforge/core";
import type { PageSetup, TextNode } from "@docforge/core";

import { l, isAr } from "$lib/core/i18n";
import SearchField from "$lib/shared/components/search-field.svelte";
import { Plus } from "@orbis/icons";
import { fade } from "svelte/transition";

import TemplateGrid from "./components/template-grid.svelte";
import CreateTemplateDialog from "./components/create-template-dialog.svelte";
import ReadyTemplatesSection from "./components/ready-templates-section.svelte";
import {
  createTemplatesQuery,
  createCreateTemplateMutation,
  createDeleteTemplateMutation,
  createPublishTemplateMutation,
  createArchiveTemplateMutation,
} from "./queries";
import { confirmStore } from "$lib/stores";
import { api, extractErrorMessage } from "$lib/api";

let { orgId }: { orgId: string } = $props();

// Dialog & Modals State
let isCreateDialogOpen = $state(false);
let newTemplateName = $state("");
let newTemplateNameAr = $state("");
let newTemplateDocType = $state("CERTIFICATE");
let newTemplateEntityType = $state("enrollment");
let searchQuery = $state("");
let filterEntityType = $state("all");
let filterDocType = $state("all");

// TanStack Query
const templatesQuery = createTemplatesQuery(
  () => orgId,
  () => ({
    documentType: filterDocType !== "all" ? filterDocType : undefined,
    entityType: filterEntityType !== "all" ? filterEntityType : undefined,
  }),
);

const createMutation = createCreateTemplateMutation();
const deleteMutation = createDeleteTemplateMutation();
const publishMutation = createPublishTemplateMutation();
const archiveMutation = createArchiveTemplateMutation();

const rawTemplates = $derived(templatesQuery.data ?? []);
const loading = $derived(templatesQuery.isLoading);
const entityTypes = $derived(
  [...new Set(rawTemplates.map((template) => template.entity_type).filter(Boolean))].sort(),
);
const documentTypes = $derived(
  [...new Set(rawTemplates.map((template) => template.document_type).filter(Boolean))].sort(),
);

function displayType(value: string): string {
  return value
    .toLowerCase()
    .split(/[_\-.]+/)
    .filter(Boolean)
    .map((part) => part[0]!.toUpperCase() + part.slice(1))
    .join(" ");
}

const filteredTemplates = $derived.by(() => {
  if (!searchQuery.trim()) return rawTemplates;
  const q = searchQuery.trim().toLowerCase();
  return rawTemplates.filter(
    (t) =>
      t.name.toLowerCase().includes(q) ||
      (t.name_ar && t.name_ar.includes(q)) ||
      (t.description && t.description.toLowerCase().includes(q)),
  );
});

// Handle Template Creation
function getDefaultPageSetup(): Pick<PageSetup, "size" | "orientation"> {
  if (newTemplateDocType === "ID_CARD")
    return { size: "ID_CARD", orientation: "landscape" };
  if (newTemplateDocType === "EXAM_CARD")
    return { size: "A5", orientation: "portrait" };
  if (
    newTemplateDocType === "RECEIPT" ||
    newTemplateDocType === "TRANSCRIPT"
  )
    return { size: "A4", orientation: "portrait" };
  return { size: "A5", orientation: "landscape" };
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
  document.meta.pageSetup.size = pageSetup.size;
  document.meta.pageSetup.orientation = pageSetup.orientation;

  const page = document.pages[0]!;
  const pageWidth = pageSetup.size === "ID_CARD" ? 85.6 : 210;
  const margin = pageSetup.size === "ID_CARD" ? 5 : 15;
  const contentWidth = pageWidth - margin * 2;
  const header = createNode<TextNode>("Text", {
    props: {
      content: "$org.name_ar",
      fontSize: pageSetup.size === "ID_CARD" ? 7 : 12,
      fontFamily: "Rubik",
      fontWeight: "bold",
      textAlign: "center",
      direction: "rtl",
      position: { x: margin, y: 12, width: contentWidth, height: 12 },
    },
  });
  const title = createNode<TextNode>("Text", {
    props: {
      content: newTemplateNameAr || newTemplateName,
      fontSize: pageSetup.size === "ID_CARD" ? 8 : 18,
      fontFamily: "Rubik",
      fontWeight: "bold",
      textAlign: "center",
      direction: "rtl",
      position: { x: margin, y: 35, width: contentWidth, height: 18 },
    },
  });
  page.children = [header, title];

  return document;
}

async function handleCreateTemplate() {
  if (!newTemplateName.trim()) {
    toast.error(m.shared_ui_template_name_required());
    return;
  }

  try {
    const pageSetup = getDefaultPageSetup();
    const defaultDocJson = JSON.stringify(buildInitialDocument());
    const createRequest: CreateTemplateRequest = {
      name: newTemplateName,
      name_ar: newTemplateNameAr || null,
      description: null,
      document_type: newTemplateDocType,
      entity_type: newTemplateEntityType,
      page_size: pageSetup.size,
      orientation: pageSetup.orientation,
      document_json: defaultDocJson,
      locale: isAr() ? "ar" : "en",
    };

    const response = await createMutation.mutateAsync(createRequest);

    toast.success(m.shared_ui_template_created());
    isCreateDialogOpen = false;

    // Redirect to editor page
    goto(`/${orgId}/templates/${response.id}`);
  } catch (e: unknown) {
    console.error("Failed to create template:", e);
    const msg = extractErrorMessage(e) || m.shared_ui_template_create_failed();
    toast.error(msg);
  }
}

// Trigger Delete
async function confirmDelete(id: string) {
  const confirmed = await confirmStore.prompt({
    title: m.shared_ui_delete_template_confirmation_title(),
    description: m.common_docs_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await deleteMutation.mutateAsync(id);
    toast.success(m.shared_ui_template_deleted());
  } catch (e: unknown) {
    console.error("Failed to delete template:", e);
    const msg = extractErrorMessage(e) || m.shared_ui_template_delete_failed();
    toast.error(msg);
  }
}

// Handle Template Publishing/Activation
async function publishTemplate(id: string) {
  try {
    await publishMutation.mutateAsync(id);
    toast.success(m.settings_templates_publish_success());
  } catch (e: unknown) {
    console.error("Failed to publish template:", e);
    const msg = extractErrorMessage(e) || m.shared_ui_template_publish_failed();
    toast.error(msg);
  }
}

// Handle Template Archiving
async function archiveTemplate(id: string) {
  try {
    await archiveMutation.mutateAsync(id);
    toast.success(m.settings_templates_archive_success());
  } catch (e: unknown) {
    console.error("Failed to archive template:", e);
    const msg = extractErrorMessage(e) || m.shared_ui_template_archive_failed();
    toast.error(msg);
  }
}

// Handle Template Restoring
async function restoreTemplate(id: string) {
  try {
    await publishMutation.mutateAsync(id);
    toast.success(m.settings_templates_restore_success());
  } catch (e: unknown) {
    console.error("Failed to restore template:", e);
    const msg = extractErrorMessage(e) || m.shared_ui_template_restore_failed();
    toast.error(msg);
  }
}

async function setDefaultTemplate(id: string) {
  try {
    await api.updateTemplate(id, { is_default: true });
    await templatesQuery.refetch();
    toast.success(m.settings_templates_set_default_success());
  } catch (e: unknown) {
    console.error("Failed to set default template:", e);
    toast.error(extractErrorMessage(e) || m.shared_ui_template_save_failed());
  }
}
</script>

<div class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide">
  <div class="w-full p-6 md:p-8 flex flex-col gap-8">
    <div class="flex flex-wrap items-center justify-between gap-4" in:fade={{ delay: 50, duration: 200 }}>
      
      <div class="flex flex-wrap items-center gap-3 shrink-0">
        <div class="w-full sm:w-72">
          <SearchField
            bind:value={searchQuery}
            placeholder={m.settings_templates_search_placeholder()}
          />
        </div>

        <Select type="single" bind:value={filterEntityType}>
          <SelectTrigger class="h-9 text-xs bg-background/40 border-border/40 hover:bg-muted/40 transition-colors">
            {filterEntityType === "all" ? m.settings_templates_filter_entity_all() : displayType(filterEntityType)}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.settings_templates_filter_entity_all()}</SelectItem>
            {#each entityTypes as entityType (entityType)}
              <SelectItem value={entityType}>{displayType(entityType)}</SelectItem>
            {/each}
          </SelectContent>
        </Select>

        <Select type="single" bind:value={filterDocType}>
          <SelectTrigger class="h-9 text-xs bg-background/40 border-border/40 hover:bg-muted/40 transition-colors">
            {filterDocType === "all" ? m.settings_templates_filter_doc_all() : displayType(filterDocType)}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.settings_templates_filter_doc_all()}</SelectItem>
            {#each documentTypes as documentType (documentType)}
              <SelectItem value={documentType}>{displayType(documentType)}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Action Button -->
      <div class="flex items-center shrink-0">
        <Button variant="default" size="sm" class="h-9 px-4 text-xs font-semibold shadow-sm" onclick={() => (isCreateDialogOpen = true)}>
          <Plus size={16} class="me-1.5" />
          {m.settings_templates_create_btn()}
        </Button>
      </div>
    </div>

    <!-- Templates Grid -->
    <TemplateGrid
      templates={filteredTemplates}
      {loading}
      {orgId}
      onconfirmdelete={confirmDelete}
      onpublish={publishTemplate}
      onarchive={archiveTemplate}
      onrestore={restoreTemplate}
      onSetDefault={setDefaultTemplate}
      oncreatenew={() => (isCreateDialogOpen = true)}
    />
  </div>

  <ReadyTemplatesSection
    {orgId}
    onimported={async () => {
      await templatesQuery.refetch();
    }}
  />
</div>

<!-- Create Dialog -->
<CreateTemplateDialog
  bind:open={isCreateDialogOpen}
  bind:name={newTemplateName}
  bind:nameAr={newTemplateNameAr}
  bind:docType={newTemplateDocType}
  bind:entityType={newTemplateEntityType}
  onsubmit={handleCreateTemplate}
/>
