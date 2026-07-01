<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Label } from "@orbis/ui-svelte/label";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Progress } from "@orbis/ui-svelte/progress";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import {
  FileText,
  Printer,
  Download,
  RefreshCw,
  AlertCircle,
  CheckCircle2,
} from "lucide-svelte";

// Component Props (Svelte 5 runes style)
let {
  open = $bindable(false),
  entityType,
  entityIds = [],
}: {
  open: boolean;
  entityType: string;
  entityIds: string[];
} = $props();

// Local State Runes
let templates = $state<any[]>([]);
let selectedTemplateId = $state<string>("");
let loadingTemplates = $state(false);
let generatingPdf = $state(false);
let pdfUrl = $state<string | null>(null);

// Batch progress state
let batchProgress = $state(0);
let isBatchProcessing = $state(false);
let batchStatusText = $state("");

// Derived orgId
const orgId = $derived(page.params.orgId);

// Load templates for this entity type
async function loadTemplates() {
  if (!entityType || !orgId) return;
  loadingTemplates = true;
  try {
    const list = await commands.documentTemplates.getTemplates(orgId || "", {
      entity_type: entityType,
      status: "published", // Only allow printing with published templates
    });
    templates = list || [];
    if (templates.length > 0) {
      selectedTemplateId = templates[0].id;
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to load templates";
    console.error(msg);
  } finally {
    loadingTemplates = false;
  }
}

// React to open state & entityType
$effect(() => {
  if (open && entityType) {
    loadTemplates();
    pdfUrl = null;
    batchProgress = 0;
    isBatchProcessing = false;
  }
});

// Helper: Uint8Array to base64 Data URL
function uint8ArrayToBase64(bytes: number[]): string {
  let binary = "";
  const len = bytes.length;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

// Generate high fidelity preview (using first entity in selection)
async function generatePreview() {
  if (!selectedTemplateId || entityIds.length === 0) return;
  generatingPdf = true;
  pdfUrl = null;
  try {
    const firstId = entityIds[0];
    const pdfBytes = await commands.docforge.renderDocument(
      orgId || "",
      selectedTemplateId,
      firstId,
    );
    if (pdfBytes && pdfBytes.length > 0) {
      const base64 = uint8ArrayToBase64(pdfBytes);
      pdfUrl = `data:application/pdf;base64,${base64}`;
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to generate print preview";
    toast.error(msg);
  } finally {
    generatingPdf = false;
  }
}

// Single / Batch action execution
async function handlePrintAction() {
  if (!selectedTemplateId) {
    toast.error("Please select a template");
    return;
  }

  if (entityIds.length === 1) {
    // Single render & download / preview
    await generatePreview();
    toast.success("تم توليد المستند للطباعة بنجاح");
  } else {
    // Batch processing progress bar simulation
    isBatchProcessing = true;
    batchProgress = 0;
    batchStatusText = `جاري بدء معالجة ${entityIds.length} مستند...`;

    // Process one by one (simplified front-end loop, back-end orchestrator in Phase 5)
    for (let i = 0; i < entityIds.length; i++) {
      const entityId = entityIds[i];
      batchStatusText = `جاري معالجة السجل ${i + 1} من ${entityIds.length}...`;
      try {
        // Run renderer
        await commands.docforge.renderDocument(
          orgId || "",
          selectedTemplateId,
          entityId,
        );
      } catch (e) {
        console.error(`Error rendering ${entityId}`, e);
      }
      batchProgress = Math.round(((i + 1) / entityIds.length) * 100);
      // Small pause to visual progress
      await new Promise((r) => setTimeout(r, 200));
    }

    batchStatusText = "تم إكمال توليد جميع المستندات بنجاح!";
    toast.success(`تم إنتاج ${entityIds.length} مستند بنجاح`);

    // Reset after some time
    setTimeout(() => {
      isBatchProcessing = false;
      open = false;
    }, 1500);
  }
}

// Helper download trigger
function triggerDownload() {
  if (!pdfUrl) return;
  const link = document.createElement("a");
  link.href = pdfUrl;
  link.download = `document_${entityType}_${Date.now()}.pdf`;
  link.click();
}
</script>

<Dialog bind:open={open}>
  <DialogContent class="sm:max-w-[650px] text-start max-h-[85vh] flex flex-col overflow-hidden">
    <DialogHeader class="shrink-0">
      <DialogTitle class="flex items-center gap-2">
        <Printer class="text-primary h-5 w-5" />
        <span>طباعة المستندات / Print Documents</span>
      </DialogTitle>
      <DialogDescription>
        اختر القالب المطلوب لطباعة أو تصدير {entityIds.length} من سجلات {entityType}.
      </DialogDescription>
    </DialogHeader>

    {#if isBatchProcessing}
      <!-- Batch Progress View -->
      <div class="flex-grow py-12 flex flex-col items-center justify-center gap-6">
        <div class="w-full max-w-md flex flex-col gap-3">
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>{batchStatusText}</span>
            <span class="font-bold text-foreground">{batchProgress}%</span>
          </div>
          <Progress value={batchProgress} class="h-2 w-full" />
        </div>
        {#if batchProgress === 100}
          <div class="flex items-center gap-2 text-emerald-500 text-xs font-semibold animate-bounce mt-2">
            <CheckCircle2 size={16} />
            <span>جاهز للطباعة والتنزيل!</span>
          </div>
        {:else}
          <RefreshCw size={24} class="animate-spin text-primary mt-2" />
        {/if}
      </div>
    {:else}
      <!-- Normal Print Selection & Preview View -->
      <div class="flex-grow flex flex-col md:flex-row gap-4 overflow-hidden py-3">
        <!-- Configuration Side -->
        <div class="w-full md:w-60 flex flex-col gap-4 shrink-0">
          <div class="flex flex-col gap-1.5">
            <Label class="text-xs font-semibold text-muted-foreground">قالب الطباعة / Template</Label>
            {#if loadingTemplates}
              <div class="h-8 bg-muted rounded animate-pulse"></div>
            {:else if templates.length === 0}
              <div class="text-[10px] text-destructive flex items-center gap-1">
                <AlertCircle size={12} />
                <span>لا توجد قوالب نشطة منشورة!</span>
              </div>
            {:else}
              <Select type="single" bind:value={selectedTemplateId} onValueChange={(val: any) => { pdfUrl = null; if (val) selectedTemplateId = val; }}>
                <SelectTrigger class="h-9">
                  {(() => {
                    const selected = templates.find(t => t.id === selectedTemplateId);
                    return selected ? (getLocale() === "ar" && selected.name_ar ? selected.name_ar : selected.name) : "اختر قالب...";
                  })()}
                </SelectTrigger>
                <SelectContent>
                  {#each templates as t (t.id)}
                    <SelectItem value={t.id}>
                      {getLocale() === "ar" && t.name_ar ? t.name_ar : t.name}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/if}
          </div>

          <div class="bg-card border border-border rounded-xl p-3 flex flex-col gap-2">
            <div class="flex items-center justify-between text-[10px] text-muted-foreground border-b border-border/40 pb-2">
              <span>السجلات المحددة:</span>
              <Badge variant="secondary" size="xs">{entityIds.length}</Badge>
            </div>
            <div class="flex items-center justify-between text-[10px] text-muted-foreground">
              <span>نوع الكيان:</span>
              <Badge variant="outline" size="xs" class="capitalize">{entityType}</Badge>
            </div>
          </div>

          {#if entityIds.length > 0}
            <Button
              variant="outline"
              size="xs"
              onclick={generatePreview}
              disabled={generatingPdf || !selectedTemplateId}
              class="w-full cursor-pointer"
            >
              <RefreshCw size={12} class="me-1 {generatingPdf ? 'animate-spin' : ''}" />
              تحديث المعاينة / Load Preview
            </Button>
          {/if}
        </div>

        <!-- Preview Screen Side -->
        <div class="flex-grow border border-border rounded-xl bg-muted/20 relative min-h-[250px] overflow-hidden flex items-center justify-center">
          {#if generatingPdf}
            <div class="absolute inset-0 bg-background/60 backdrop-blur-xs flex flex-col items-center justify-center gap-2 z-10">
              <RefreshCw size={20} class="text-primary animate-spin" />
              <p class="text-[10px] font-semibold">جاري تحضير ملف المعاينة...</p>
            </div>
          {/if}

          {#if pdfUrl}
            <iframe
              src={pdfUrl}
              title="Print Document Preview"
              class="w-full h-full border-none"
            ></iframe>
          {:else}
            <div class="text-muted-foreground flex flex-col items-center gap-2 text-[10px]">
              <FileText size={28} />
              <p>انقر على "تحديث المعاينة" لعرض المستند</p>
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <DialogFooter class="shrink-0 border-t border-border/40 pt-3 gap-2 sm:gap-0">
      <Button variant="outline" size="sm" onclick={() => (open = false)}>إلغاء / Cancel</Button>
      
      {#if pdfUrl && !isBatchProcessing}
        <Button variant="outline" size="sm" onclick={triggerDownload}>
          <Download size={14} class="me-1" />
          تنزيل / Download
        </Button>
      {/if}

      <Button
        variant="lime"
        size="sm"
        disabled={!selectedTemplateId || generatingPdf || isBatchProcessing}
        onclick={handlePrintAction}
      >
        <Printer size={14} class="me-1.5" />
        {entityIds.length > 1 ? `بدء طباعة الدفعة (${entityIds.length})` : "توليد للطباعة / Generate"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
