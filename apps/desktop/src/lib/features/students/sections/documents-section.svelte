<!--
  @component DocumentsSection
  Dynamic document requirements checklist and media archiving hub.
  Allows uploading scans and PDF documents directly into the student's digital archive.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Checkbox } from "@orbis/ui/checkbox";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui/card";
import {
  CheckCircle,
  DocumentCheck,
  Lock,
  ArrowUpTray,
  Trash,
  ArrowPath,
} from "@orbis/icons";
import { l } from "$lib/core/i18n";
import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();

const docItems = $derived.by(() => {
  if (state.document.fulfillments.length > 0) {
    return state.document.fulfillments.map((f) => ({
      id: f.requirement.code,
      label: f.requirement.name_ar || f.requirement.name_en || f.requirement.code,
      isMandatory: f.requirement.is_mandatory,
      blocksEnrollment: f.requirement.blocks_enrollment,
      isFulfilled: f.is_fulfilled || state.document.submittedDocs.includes(f.requirement.code),
      condition: f.requirement.condition,
      fileName: state.document.attachments[f.requirement.code]?.fileName || f.document_name,
      fileSize: state.document.attachments[f.requirement.code]?.fileSize,
      attachment: state.document.attachments[f.requirement.code],
    }));
  }

  if (state.document.requirements.length > 0) {
    const selectedClassId = state.personal.classId;
    const selectedClassLevel = state.selectedClass?.level;

    return state.document.requirements
      .filter((r) => {
        if (r.target_id) {
          const matches = r.target_id === selectedClassId || r.target_id === selectedClassLevel;
          if (!matches && selectedClassId) return false;
        }
        return true;
      })
      .map((r) => ({
        id: r.code,
        label: r.name_ar || r.name_en || r.code,
        isMandatory: r.is_mandatory,
        blocksEnrollment: r.blocks_enrollment,
        isFulfilled: state.document.submittedDocs.includes(r.code) || !!state.document.attachments[r.code],
        condition: r.condition,
        fileName: state.document.attachments[r.code]?.fileName,
        fileSize: state.document.attachments[r.code]?.fileSize,
        attachment: state.document.attachments[r.code],
      }));
  }

  if (state.document.orgRequiredDocuments.length > 0) {
    return state.document.orgRequiredDocuments.map((name) => ({
      id: name,
      label: name,
      isMandatory: true,
      blocksEnrollment: false,
      isFulfilled: state.document.submittedDocs.includes(name) || !!state.document.attachments[name],
      condition: "all_students",
      fileName: state.document.attachments[name]?.fileName,
      fileSize: state.document.attachments[name]?.fileSize,
      attachment: state.document.attachments[name],
    }));
  }

  return [];
});

let fileInputRefs: Record<string, HTMLInputElement> = {};

function triggerFileUpload(code: string) {
  fileInputRefs[code]?.click();
}

function handleFilePicked(code: string, event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    state.document.uploadDocumentFile(code, file);
  }
}

function formatBytes(bytes?: number) {
  if (!bytes) return "";
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
</script>

<Card class="w-full border-border/80 shadow-xs">
  <CardHeader class="py-3 px-6 flex flex-row items-center justify-between border-b border-border/40">
    <div class="flex items-center gap-2">
      <DocumentCheck class="h-4 w-4 text-primary" />
      <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
        {m.common_form_documents_title()}
      </CardTitle>
    </div>
    {#if docItems.length > 0}
      {@const fulfilledCount = docItems.filter(f => f.isFulfilled).length}
      <Badge variant={fulfilledCount === docItems.length ? "lime" : "outline"} class="text-[10px] font-mono">
        {fulfilledCount} / {docItems.length} {l("مستند موثق", "archived")}
      </Badge>
    {/if}
  </CardHeader>

  <CardContent class="p-6">
    {#if docItems.length === 0}
      <div class="py-8 flex flex-col items-center justify-center text-center gap-2.5">
        <div class="w-10 h-10 rounded-full bg-muted/50 border border-border/60 flex items-center justify-center text-muted-foreground">
          <DocumentCheck class="w-5 h-5 text-muted-foreground/80" />
        </div>
        <div class="flex flex-col gap-1 max-w-sm">
          <span class="text-xs font-bold text-foreground">
            {l("لا توجد متطلبات وثائق مسجلة", "No Document Requirements Configured")}
          </span>
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {l("لم يتم تحديد أي وثائق أو مستندات مطلوبة للقبول والتسجيل في إعدادات المؤسسة حالياً.", "No admission documents are configured in institutional settings.")}
          </span>
        </div>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each docItems as doc (doc.id)}
        {@const isChecked = doc.isFulfilled}
        {@const isUploading = state.document.isUploading[doc.id]}

        <div 
          class="flex flex-col p-4 border rounded-lg bg-background hover:bg-accent/5 transition-all duration-150 gap-3 text-start select-none {isChecked ? 'border-primary/40 bg-primary/5 shadow-xs' : 'border-border'}"
        >
          <!-- Top Row: Checkbox, Title, and Verified Check -->
          <div class="flex items-start justify-between gap-2">
            <div class="flex items-start gap-3">
              <Checkbox 
                id={`doc_${doc.id}`} 
                checked={isChecked}
                onCheckedChange={() => state.document.toggleDoc(doc.id)}
                class="mt-0.5"
              />
              <div class="flex flex-col gap-0.5">
                <Label for={`doc_${doc.id}`} class="text-xs font-bold text-foreground cursor-pointer leading-snug">
                  {doc.label}
                </Label>
                <div class="flex items-center gap-1.5 flex-wrap">
                  {#if doc.isMandatory}
                    <Badge variant="destructive" class="text-[9px] px-1.5 py-0 h-4 font-bold">
                      {l("إلزامي للقبول", "Mandatory")}
                    </Badge>
                  {:else}
                    <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4 font-medium">
                      {l("اختياري", "Optional")}
                    </Badge>
                  {/if}

                  {#if doc.blocksEnrollment}
                    <Badge variant="outline" class="text-[9px] px-1.5 py-0 h-4 text-amber-500 border-amber-500/30 flex items-center gap-0.5 font-semibold">
                      <Lock class="h-2.5 w-2.5" />
                      {l("يحجب القيد", "Blocks Enrollment")}
                    </Badge>
                  {/if}
                </div>
              </div>
            </div>

            {#if isChecked}
              <CheckCircle class="h-4 w-4 text-emerald-500 shrink-0 mt-0.5" />
            {/if}
          </div>

          <!-- Attachment Status or Upload Area -->
          <div class="ms-7 pt-2 border-t border-border/40 flex items-center justify-between gap-2">
            {#if doc.fileName}
              <div class="flex items-center gap-2 min-w-0">
                <div class="flex flex-col min-w-0">
                  <span class="text-xs font-semibold text-foreground truncate max-w-40 font-mono">
                    {doc.fileName}
                  </span>
                  {#if doc.fileSize}
                    <span class="text-[10px] text-muted-foreground font-mono">
                      {formatBytes(doc.fileSize)}
                    </span>
                  {/if}
                </div>
              </div>

              <div class="flex items-center gap-1">
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-7 px-2 text-xs text-muted-foreground hover:text-foreground gap-1 cursor-pointer"
                  onclick={() => triggerFileUpload(doc.id)}
                  disabled={isUploading}
                >
                  <ArrowUpTray class="w-3 h-3" />
                  <span>{l("استبدال", "Replace")}</span>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-7 w-7 p-0 text-muted-foreground hover:text-destructive cursor-pointer"
                  onclick={() => state.document.removeAttachment(doc.id)}
                >
                  <Trash class="w-3.5 h-3.5" />
                </Button>
              </div>
            {:else}
              <span class="text-[11px] text-muted-foreground">
                {l("لم يتم إرفاق ملف المستند بعد", "No document file uploaded")}
              </span>

              <Button
                variant="outline"
                size="sm"
                class="h-7 px-2.5 text-xs font-semibold gap-1.5 cursor-pointer bg-background hover:bg-muted/40"
                onclick={() => triggerFileUpload(doc.id)}
                disabled={isUploading}
              >
                {#if isUploading}
                  <ArrowPath class="w-3 h-3 animate-spin text-primary" />
                  <span>{l("جارٍ الرفع...", "Uploading...")}</span>
                {:else}
                  <ArrowUpTray class="w-3 h-3 text-primary" />
                  <span>{l("رفع إلى الأرشيف", "Upload to Archive")}</span>
                {/if}
              </Button>
            {/if}

            <!-- Hidden File Input -->
            <input
              bind:this={fileInputRefs[doc.id]}
              type="file"
              accept="image/*,application/pdf"
              class="hidden"
              onchange={(e) => handleFilePicked(doc.id, e)}
            />
          </div>
        </div>
      {/each}
      </div>
    {/if}
  </CardContent>
</Card>
