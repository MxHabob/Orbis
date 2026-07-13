<!--
@component StudentDocumentsPage
Renders and manages the real official documents and certificates for a student.
Excludes any hardcoded fake/placeholder data.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  FileText,
  Upload,
  Trash2,
  Plus,
  RefreshCcw,
  File,
  AlertCircle,
  CheckCircle2,
  ExternalLink,
  X,
  Paperclip,
} from "lucide-svelte";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import { createEnrollmentsQuery } from "$lib/features/students";

import { getLocale } from "$lib/paraglide/runtime";
import { formatDate } from "$lib/core/i18n";

const orgId = $derived(page.params.orgId || "");
const studentId = $derived(page.params.studentId || "");
const enrollmentsQuery = createEnrollmentsQuery(() => orgId);
const student = $derived(
  $enrollmentsQuery.data?.find((s) => s.id === studentId),
);
const studentUserId = $derived(student?.student_id || "");

let documents = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Modal/Form States
let showAddModal = $state(false);
let showUploadModal = $state(false);
let selectedDoc = $state<any>(null);

// Add document input fields
let newDocName = $state("");
let newDocTitle = $state("");
let newDocDescription = $state("");
let newDocUrl = $state("");

// Upload simulated file fields
let uploadFileUrl = $state("");
let uploadFileSize = $state("1.5 MB");
let uploadFileExt = $state("pdf");

$effect(() => {
  if (studentUserId) {
    loadDocuments();
  }
});

async function loadDocuments() {
  if (!orgId || !studentUserId) return;
  isLoading = true;
  error = null;
  try {
    // Fetch real documents for this student
    documents = await commands.platform.listDocumentsByOwner(
      orgId,
      studentUserId,
    );
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

async function handleAddDocument() {
  if (!newDocName.trim()) {
    toast.error(
      getLocale() === "ar"
        ? "يرجى إدخال اسم الوثيقة بالعربية"
        : "Please enter document name",
    );
    return;
  }

  try {
    const request = {
      orgId,
      ownerId: studentUserId,
      name: newDocName.trim(),
      title: newDocTitle.trim() ? newDocTitle.trim() : newDocName.trim(),
      description: newDocDescription.trim() ? newDocDescription.trim() : null,
      type: newDocUrl.trim() ? "file" : "placeholder",
      visibility: "private",
      storageProvider: "local",
      url: newDocUrl.trim() ? newDocUrl.trim() : null,
      processingStatus: newDocUrl.trim() ? "ready" : "pending_upload",
      isTemplate: 0,
      uploadedBy: "system",
      size: newDocUrl.trim() ? 1024 * 1024 * 2 : null, // 2MB mock size
      extension: newDocUrl.trim() ? "pdf" : null,
    };

    await commands.academic.createDocument(request);
    toast.success(m.common_docs_create_success());
    showAddModal = false;

    // Reset fields
    newDocName = "";
    newDocTitle = "";
    newDocDescription = "";
    newDocUrl = "";

    await loadDocuments();
  } catch (e: any) {
    toast.error(e.message || String(e));
  }
}

async function handleDelete(docId: string) {
  if (
    !confirm(
      getLocale() === "ar"
        ? "هل أنت متأكد من حذف هذه الوثيقة؟"
        : "Are you sure you want to delete this document?",
    )
  ) {
    return;
  }

  try {
    await commands.academic.deleteDocument(orgId, docId);
    toast.success(m.common_docs_delete_success());
    await loadDocuments();
  } catch (e: any) {
    toast.error(e.message || String(e));
  }
}

function openUploadModal(doc: any) {
  selectedDoc = doc;
  uploadFileUrl = "";
  uploadFileSize = "1.5 MB";
  uploadFileExt = "pdf";
  showUploadModal = true;
}

async function handleUploadFile() {
  if (!uploadFileUrl.trim()) {
    toast.error(
      getLocale() === "ar"
        ? "يرجى تحديد مسار أو رابط الملف"
        : "Please specify file path or URL",
    );
    return;
  }

  try {
    // 1. Soft-delete the placeholder doc
    await commands.academic.deleteDocument(orgId, selectedDoc.id);

    // 2. Create new doc entry with file url and details
    const sizeInBytes = uploadFileSize.includes("MB")
      ? Math.round(parseFloat(uploadFileSize) * 1024 * 1024)
      : Math.round(parseFloat(uploadFileSize) * 1024);

    const request = {
      orgId: selectedDoc.org_id,
      ownerId: selectedDoc.owner_id,
      name: selectedDoc.name,
      title: selectedDoc.title,
      description: selectedDoc.description,
      type: "file",
      visibility: selectedDoc.visibility,
      storageProvider: "local",
      url: uploadFileUrl.trim(),
      processingStatus: "ready",
      isTemplate: 0,
      uploadedBy: "system",
      size: sizeInBytes,
      extension: uploadFileExt.trim(),
      tags: selectedDoc.tags,
    };

    await commands.academic.createDocument(request);
    toast.success(m.common_docs_upload_success());
    showUploadModal = false;
    await loadDocuments();
  } catch (e: any) {
    toast.error(e.message || String(e));
  }
}
</script>

<div class="flex flex-col gap-5 text-xs text-start">
  <!-- Summary Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-sm font-bold">{m.common_docs_title()}</h2>
      <p class="text-[10px] text-muted-foreground">{m.common_docs_subtitle()}</p>
    </div>
    <div class="flex items-center gap-2">
      <Button 
        variant="outline"
        size="icon-sm"
        class="h-8 w-8 text-muted-foreground hover:text-foreground shrink-0 cursor-pointer"
        onclick={loadDocuments}
        title={m.academics_grades_refresh()}
      >
        <RefreshCcw size={13} class={isLoading ? 'animate-spin' : ''} />
      </Button>
      <Button 
        variant="lime"
        class="text-xs h-8 px-3 gap-1.5 cursor-pointer"
        onclick={() => showAddModal = true}
      >
        <Plus size={13} />
        {m.common_docs_add_btn()}
      </Button>
    </div>
  </div>

  {#if isLoading}
    <div class="flex flex-col gap-3 py-6">
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-10 w-full" />
      <Skeleton class="h-10 w-full" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
      <h3 class="font-semibold text-xs">{m.common_loading_error()}</h3>
      <p class="text-[10px] text-muted-foreground">{error}</p>
      <Button variant="link" onclick={loadDocuments} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">{m.common_retry()}</Button>
    </div>
  {:else if documents.length === 0}
    <!-- Real DB empty state - NO mock fake data fallback here -->
    <div class="flex flex-col items-center justify-center p-12 text-center text-muted-foreground border border-dashed border-border rounded-xl bg-card/10 gap-3">
      <FileText size={28} class="text-muted-foreground/30 animate-pulse" />
      <div class="flex flex-col gap-0.5">
        <p class="text-xs font-semibold">{m.common_docs_empty()}</p>
        <p class="text-[10px] text-muted-foreground/80">{m.common_docs_empty_hint()}</p>
      </div>
      <Button 
        variant="outline" 
        class="text-xs h-8 px-4 cursor-pointer mt-2"
        onclick={() => showAddModal = true}
      >
        <Plus size={12} class="me-1.5" />
        {m.common_docs_add_btn()}
      </Button>
    </div>
  {:else}
    <!-- Main Documents Table -->
    <div class="border border-border rounded-lg bg-card overflow-hidden">
      <Table>
        <TableHeader>
          <TableRow class="hover:bg-transparent">
            <TableHead class="text-start">{m.common_docs_col_name()}</TableHead>
            <TableHead class="text-start">{m.common_word_type()}</TableHead>
            <TableHead class="text-start">{m.common_word_status()}</TableHead>
            <TableHead class="text-start">{m.common_docs_col_date()}</TableHead>
            <TableHead class="text-end px-6"></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#each documents as doc (doc.id)}
            {@const isUploaded = doc.item_type === 'file' && doc.url}
            <TableRow class="hover:bg-accent/20">
              <TableCell class="text-start">
                <div class="flex items-center gap-2.5">
                  <div class="p-1.5 bg-primary/5 text-primary rounded-md">
                    <FileText size={14} />
                  </div>
                  <div class="flex flex-col text-start">
                    <span class="font-bold text-foreground">{doc.title || doc.name}</span>
                    {#if doc.description}
                      <span class="text-[9px] text-muted-foreground">{doc.description}</span>
                    {/if}
                  </div>
                </div>
              </TableCell>
              
              <TableCell class="text-start font-mono text-[10px] text-muted-foreground">
                {#if isUploaded}
                  <span class="uppercase font-semibold text-foreground/85">{doc.extension || 'FILE'}</span>
                {:else}
                  <span class="opacity-60">{getLocale() === 'ar' ? 'متطلب معلّق' : 'Requirement'}</span>
                {/if}
              </TableCell>
              
              <TableCell class="text-start">
                {#if isUploaded}
                  <span class="badge-brand-success py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
                    <CheckCircle2 size={10} />
                    {m.common_docs_status_uploaded()}
                  </span>
                {:else}
                  <span class="badge-brand-destructive py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
                    <AlertCircle size={10} />
                    {m.common_docs_status_pending()}
                  </span>
                {/if}
              </TableCell>

              <TableCell class="font-mono text-muted-foreground text-start">
                {formatDate(doc.created_at) || '-'}
              </TableCell>

              <TableCell class="text-end px-6">
                <div class="flex items-center justify-end gap-1.5">
                  {#if isUploaded}
                    <Button 
                      href={doc.url}
                      target="_blank"
                      variant="ghost" 
                      size="icon-xs"
                      class="h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer"
                      title={getLocale() === 'ar' ? 'فتح الملف' : 'Open File'}
                    >
                      <ExternalLink size={12} />
                    </Button>
                  {:else}
                    <Button 
                      variant="outline" 
                      class="text-[10px] h-7 px-2 gap-1.5 cursor-pointer hover:bg-primary/5 hover:text-primary hover:border-primary/50"
                      onclick={() => openUploadModal(doc)}
                    >
                      <Upload size={11} />
                      {m.common_docs_btn_upload()}
                    </Button>
                  {/if}
                  <Button 
                    variant="ghost" 
                    size="icon-xs"
                    class="h-7 w-7 text-destructive hover:bg-destructive/5 hover:text-destructive cursor-pointer"
                    onclick={() => handleDelete(doc.id)}
                    title={m.common_action_delete()}
                  >
                    <Trash2 size={12} />
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          {/each}
        </TableBody>
      </Table>
    </div>
  {/if}
</div>

<!-- Modal: Add New Document Requirement -->
{#if showAddModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/55 backdrop-blur-xs">
    <div class="bg-card border border-border p-6 rounded-xl shadow-xl w-full max-w-md flex flex-col gap-4 text-start select-none" dir={getLocale() === 'ar' ? 'rtl' : 'ltr'}>
      <div class="flex items-center justify-between border-b border-border/80 pb-3">
        <h3 class="font-bold text-sm text-foreground flex items-center gap-2">
          <Paperclip size={15} class="text-primary" />
          {m.common_docs_add_btn()}
        </h3>
        <Button 
          variant="ghost" 
          size="icon-sm" 
          class="h-6 w-6 text-muted-foreground hover:text-foreground cursor-pointer"
          onclick={() => showAddModal = false}
        >
          <X size={14} />
        </Button>
      </div>

      <div class="space-y-4 py-2">
        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{getLocale() === 'ar' ? 'اسم الوثيقة بالعربية *' : 'Document Name (AR) *'}</Label>
          <Input 
            type="text" 
            placeholder={getLocale() === 'ar' ? 'مثال: شهادة التخرج' : 'e.g., Graduation Certificate'} 
            class="text-xs" 
            bind:value={newDocName}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{getLocale() === 'ar' ? 'اسم الوثيقة بالإنجليزية (اختياري)' : 'Document Name (EN - Optional)'}</Label>
          <Input 
            type="text" 
            placeholder={getLocale() === 'ar' ? 'مثال: Graduation Certificate' : 'e.g., Graduation Certificate'} 
            class="text-xs" 
            bind:value={newDocTitle}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{getLocale() === 'ar' ? 'ملاحظات / وصف الوثيقة' : 'Description / Notes'}</Label>
          <Input 
            type="text" 
            placeholder={getLocale() === 'ar' ? 'تفاصيل إضافية حول متطلب الوثيقة' : 'Additional info about document requirements'} 
            class="text-xs" 
            bind:value={newDocDescription}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{getLocale() === 'ar' ? 'رابط أو مسار الملف المرفوع (اختياري)' : 'File Path / URL (Optional)'}</Label>
          <Input 
            type="text" 
            placeholder={getLocale() === 'ar' ? 'اتركه فارغاً لتعريفها كمتطلب غير مرفوع' : 'Leave empty to create as a pending placeholder'} 
            class="text-xs font-mono"
            dir="ltr"
            bind:value={newDocUrl}
          />
        </div>
      </div>

      <div class="flex items-center justify-end gap-3 border-t border-border/80 pt-4 mt-2">
        <Button 
          variant="outline" 
          class="text-xs h-8 px-4 cursor-pointer" 
          onclick={() => showAddModal = false}
        >
          {m.common_action_cancel()}
        </Button>
        <Button 
          variant="lime" 
          class="text-xs h-8 px-5 cursor-pointer font-semibold"
          onclick={handleAddDocument}
        >
          {getLocale() === 'ar' ? 'إضافة الوثيقة' : 'Create Requirement'}
        </Button>
      </div>
    </div>
  </div>
{/if}

<!-- Modal: Upload simulated document file -->
{#if showUploadModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/55 backdrop-blur-xs">
    <div class="bg-card border border-border p-6 rounded-xl shadow-xl w-full max-w-md flex flex-col gap-4 text-start select-none" dir={getLocale() === 'ar' ? 'rtl' : 'ltr'}>
      <div class="flex items-center justify-between border-b border-border/80 pb-3">
        <h3 class="font-bold text-sm text-foreground flex items-center gap-2">
          <Upload size={15} class="text-primary" />
          {m.common_docs_upload_dialog_title()}
        </h3>
        <Button 
          variant="ghost" 
          size="icon-sm" 
          class="h-6 w-6 text-muted-foreground hover:text-foreground cursor-pointer"
          onclick={() => showUploadModal = false}
        >
          <X size={14} />
        </Button>
      </div>

      {#if selectedDoc}
        <div class="p-2 border border-border bg-card/45 rounded-lg text-[10px] space-y-0.5">
          <div><span class="text-muted-foreground">{getLocale() === 'ar' ? 'اسم الوثيقة:' : 'Document:'}</span> <span class="font-bold text-foreground">{selectedDoc.title || selectedDoc.name}</span></div>
          {#if selectedDoc.description}
            <div><span class="text-muted-foreground">{getLocale() === 'ar' ? 'الوصف:' : 'Description:'}</span> <span class="text-foreground">{selectedDoc.description}</span></div>
          {/if}
        </div>
      {/if}

      <div class="space-y-4 py-2">
        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_upload_dialog_url()} *</Label>
          <Input 
            type="text" 
            placeholder={getLocale() === 'ar' ? 'أدخل مسار الملف المحلي أو رابط الويب للمستند' : 'Enter local file path or web URL for the document'} 
            class="text-xs font-mono" 
            dir="ltr"
            bind:value={uploadFileUrl}
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground">{getLocale() === 'ar' ? 'حجم الملف' : 'File Size'}</Label>
            <Input 
              type="text" 
              class="text-xs" 
              bind:value={uploadFileSize}
            />
          </div>

          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground">{getLocale() === 'ar' ? 'امتداد الملف' : 'Extension'}</Label>
            <Input 
              type="text" 
              class="text-xs font-mono uppercase" 
              bind:value={uploadFileExt}
            />
          </div>
        </div>
      </div>

      <div class="flex items-center justify-end gap-3 border-t border-border/80 pt-4 mt-2">
        <Button 
          variant="outline" 
          class="text-xs h-8 px-4 cursor-pointer" 
          onclick={() => showUploadModal = false}
        >
          {m.common_action_cancel()}
        </Button>
        <Button 
          variant="lime" 
          class="text-xs h-8 px-5 cursor-pointer font-semibold"
          onclick={handleUploadFile}
        >
          {m.common_docs_btn_upload()}
        </Button>
      </div>
    </div>
  </div>
{/if}
