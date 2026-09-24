<!--
  @component StudentDocuments
  Student official documents and certificate attachments.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import {
  Grid,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Skeleton } from "@orbis/ui/skeleton";
import { ArrowForward, ArrowUpTray, CheckCircle, Clip, DocumentPage, InfoCircle, Plus, Refresh, Trash, XMark } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { createEnrollmentsQuery } from "$lib/features/students";
import { formatDate } from "$lib/core/i18n";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();
const queryClient = useQueryClient();

const enrollmentsQuery = createEnrollmentsQuery(() => orgId);
const student = $derived(
  enrollmentsQuery.data?.find((s) => s.id === studentId),
);
const studentUserId = $derived(student?.student_id || "");

const documentsQuery = createQuery(() => ({
  queryKey: ["student-documents", studentUserId],
  queryFn: () => api.listDocuments(null, studentUserId, null, null),
  enabled: !!studentUserId,
  staleTime: 3 * 60 * 1000,
}));

const documents = $derived((documentsQuery.data as any[] | undefined) ?? []);
const isLoading = $derived(documentsQuery.isLoading || enrollmentsQuery.isLoading);
const error = $derived(
  documentsQuery.error ? extractErrorMessage(documentsQuery.error) : null,
);

let showAddModal = $state(false);
let showUploadModal = $state(false);
let selectedDoc = $state<any>(null);

let newDocName = $state("");
let newDocTitle = $state("");
let newDocDescription = $state("");
let newDocUrl = $state("");

let uploadFileUrl = $state("");
let uploadFileSize = $state("1.5 MB");
let uploadFileExt = $state("pdf");

async function handleAddDocument() {
  if (!newDocName.trim()) {
    toast.error(m.common_docs_form_name_required());
    return;
  }

  try {
    const request = {
      owner_id: studentUserId,
      name: newDocName.trim(),
      item_type: newDocUrl.trim() ? "file" : "placeholder",
      key: newDocUrl.trim(),
    };

    await api.uploadDocument(request);
    toast.success(m.common_docs_create_success());
    showAddModal = false;

    newDocName = "";
    newDocTitle = "";
    newDocDescription = "";
    newDocUrl = "";

    await queryClient.invalidateQueries({
      queryKey: ["student-documents", studentUserId],
    });
  } catch (e: any) {
    toast.error(e.message || String(e));
  }
}

async function handleDelete(docId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete() + "؟",
    description: m.common_docs_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) {
    return;
  }

  try {
    await api.deleteDocument(docId);
    toast.success(m.common_docs_delete_success());
    await queryClient.invalidateQueries({
      queryKey: ["student-documents", studentUserId],
    });
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
    toast.error(m.common_docs_upload_url_required());
    return;
  }

  try {
    await api.deleteDocument(selectedDoc.id);

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

    await api.uploadDocument(request);
    toast.success(m.common_docs_upload_success());
    showUploadModal = false;
    await queryClient.invalidateQueries({
      queryKey: ["student-documents", studentUserId],
    });
  } catch (e: any) {
    toast.error(e.message || String(e));
  }
}
</script>

<div class="flex flex-col gap-5 text-xs text-start">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-sm font-bold">{m.common_docs_title()}</h2>
      <p class="text-[10px] text-muted-foreground">{m.common_docs_subtitle()}</p>
    </div>
    <div class="flex items-center gap-2">
      <Button 
        variant="outline" 
        size="sm"
        class="h-8 w-8 p-0 text-muted-foreground hover:text-foreground shrink-0 cursor-pointer"
        onclick={() => documentsQuery.refetch()}
        title={m.academics_grades_refresh()}
      >
        <Refresh size={13} class={isLoading ? 'animate-spin' : ''} />
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
      <Button variant="link" onclick={() => documentsQuery.refetch()} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">{m.common_retry()}</Button>
    </div>
  {:else if documents.length === 0}
    <div class="flex flex-col items-center justify-center p-12 text-center text-muted-foreground border border-dashed border-border rounded-xl bg-card/10 gap-3">
      <DocumentPage size={28} class="text-muted-foreground/30 animate-pulse" />
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
    <div class="border border-border rounded-lg bg-card overflow-hidden">
      <Grid>
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
                    <DocumentPage size={14} />
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
                  <span class="opacity-60">{m.common_docs_status_requirement()}</span>
                {/if}
              </TableCell>
              
              <TableCell class="text-start">
                {#if isUploaded}
                  <span class="badge-brand-success py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
                    <CheckCircle size={10} />
                    {m.common_docs_status_uploaded()}
                  </span>
                {:else}
                  <span class="badge-brand-destructive py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
                    <InfoCircle size={10} />
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
                      size="sm"
                      class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground cursor-pointer"
                      title={m.common_docs_action_open_file()}
                    >
                      <ArrowForward size={12} />
                    </Button>
                  {:else}
                    <Button 
                      variant="outline" 
                      class="text-[10px] h-7 px-2 gap-1.5 cursor-pointer hover:bg-primary/5 hover:text-primary hover:border-primary/50"
                      onclick={() => openUploadModal(doc)}
                    >
                      <ArrowUpTray size={11} />
                      {m.common_docs_btn_upload()}
                    </Button>
                  {/if}
                  <Button 
                    variant="ghost" 
                    size="sm"
                    class="h-7 w-7 p-0 text-destructive hover:bg-destructive/5 hover:text-destructive cursor-pointer"
                    onclick={() => handleDelete(doc.id)}
                    title={m.common_action_delete()}
                  >
                    <Trash size={12} />
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          {/each}
        </TableBody>
      </Grid>
    </div>
  {/if}
</div>

<!-- Modal: Add New Document Requirement -->
{#if showAddModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/55 backdrop-blur-xs">
    <div class="bg-card border border-border p-6 rounded-xl shadow-xl w-full max-w-md flex flex-col gap-4 text-start select-none">
      <div class="flex items-center justify-between border-b border-border/80 pb-3">
        <h3 class="font-bold text-sm text-foreground flex items-center gap-2">
          <Clip size={15} class="text-primary" />
          {m.common_docs_add_btn()}
        </h3>
        <Button 
          variant="ghost" 
          size="sm" 
          class="h-6 w-6 p-0 text-muted-foreground hover:text-foreground cursor-pointer"
          onclick={() => showAddModal = false}
        >
          <XMark size={14} />
        </Button>
      </div>

      <div class="space-y-4 py-2">
        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_form_name_ar_label()}</Label>
          <Input 
            type="text" 
            placeholder={m.common_docs_form_name_ar_placeholder()} 
            class="text-xs" 
            bind:value={newDocName}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_form_name_en_label()}</Label>
          <Input 
            type="text" 
            placeholder={m.common_docs_form_name_ar_placeholder()} 
            class="text-xs" 
            bind:value={newDocTitle}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_form_desc_label()}</Label>
          <Input 
            type="text" 
            placeholder={m.common_docs_form_desc_placeholder()} 
            class="text-xs" 
            bind:value={newDocDescription}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_form_url_label()}</Label>
          <Input 
            type="text" 
            placeholder={m.common_docs_form_url_placeholder()} 
            class="text-xs font-mono" 
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
          {m.common_docs_action_create_req()}
        </Button>
      </div>
    </div>
  </div>
{/if}

<!-- Modal: Upload File -->
{#if showUploadModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/55 backdrop-blur-xs">
    <div class="bg-card border border-border p-6 rounded-xl shadow-xl w-full max-w-md flex flex-col gap-4 text-start select-none">
      <div class="flex items-center justify-between border-b border-border/80 pb-3">
        <h3 class="font-bold text-sm text-foreground flex items-center gap-2">
          <ArrowUpTray size={15} class="text-primary" />
          {m.common_docs_upload_dialog_title()}
        </h3>
        <Button 
          variant="ghost" 
          size="sm" 
          class="h-6 w-6 p-0 text-muted-foreground hover:text-foreground cursor-pointer"
          onclick={() => showUploadModal = false}
        >
          <XMark size={14} />
        </Button>
      </div>

      {#if selectedDoc}
        <div class="p-2 border border-border bg-card/45 rounded-lg text-[10px] space-y-0.5">
          <div><span class="text-muted-foreground">{m.common_docs_detail_doc_label()}</span> <span class="font-bold text-foreground">{selectedDoc.title || selectedDoc.name}</span></div>
          {#if selectedDoc.description}
            <div><span class="text-muted-foreground">{m.common_docs_detail_desc_label()}</span> <span class="text-foreground">{selectedDoc.description}</span></div>
          {/if}
        </div>
      {/if}

      <div class="space-y-4 py-2">
        <div class="flex flex-col gap-1.5">
          <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_upload_dialog_url()} *</Label>
          <Input 
            type="text" 
            placeholder={m.common_docs_form_upload_url_placeholder()} 
            class="text-xs font-mono" 
            bind:value={uploadFileUrl}
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_form_file_size()}</Label>
            <Input 
              type="text" 
              class="text-xs" 
              bind:value={uploadFileSize}
            />
          </div>

          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground">{m.common_docs_form_file_ext()}</Label>
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
