<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
/**
 * ReportViewer — A C# .NET style native printing component.
 * Connects to api.generateReport, polls status, and fetches PDF bytes.
 * Provides a WYSIWYG preview and triggers the OS native print dialog.
 */
import { api, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import { Progress } from "@orbis/ui/progress";
import { toast } from "@orbis/ui";
import {
  CheckCircle,
  DocumentPage,
  Download,
  Refresh,
  XMark,
  Printer,
} from "@orbis/icons";

let {
  open = $bindable(false),
  templateId,
  parameters = {},
  reportTitle = m.common_page_title_reports(),
}: {
  open: boolean;
  templateId: string;
  parameters?: Record<string, any>;
  reportTitle?: string;
} = $props();

let isGenerating = $state(false);
let progress = $state(0);
let statusText = $state("");
let pdfUrl = $state<string | null>(null);
let errorMessage = $state<string | null>(null);
let currentJobId = $state<string | null>(null);
let startedTemplateId = "";

function uint8ArrayToBase64(bytes: Uint8Array): string {
  let binary = "";
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

async function generateReport() {
  if (!templateId || isGenerating) return;

  isGenerating = true;
  pdfUrl = null;
  errorMessage = null;
  progress = 10;
  statusText = m.shared_ui_initializing_report_generation();

  try {
    // 1. Trigger Generation
    const jobId = await api.generateReport(templateId, "pdf", parameters);

    currentJobId = jobId;
    progress = 30;
    statusText = m.shared_ui_generating_document();

    // 2. Poll Status
    const pollingDeadline = Date.now() + 15 * 60 * 1000; // 15 mins max
    while (currentJobId === jobId) {
      if (Date.now() >= pollingDeadline) {
        throw new Error(m.shared_ui_report_generation_timed_out());
      }

      const jobStatus = await api.getReportJobStatus(jobId);
      if (currentJobId !== jobId) return;

      if (
        jobStatus.status === "completed" ||
        jobStatus.status === "Completed"
      ) {
        progress = 80;
        statusText = m.shared_ui_fetching_document();
        break;
      } else if (
        jobStatus.status === "failed" ||
        jobStatus.status === "Failed"
      ) {
        throw new Error(
          jobStatus.error_message ||
            m.shared_ui_report_generation_server_failed(),
        );
      }

      await new Promise((r) => setTimeout(r, 1000));
    }

    if (currentJobId !== jobId) return; // Dialog was closed or reset

    // 3. Fetch PDF Bytes
    const pdfBytes = await api.downloadReportFile(jobId);

    if (pdfBytes && pdfBytes.length > 0) {
      const isPdf =
        pdfBytes.length >= 4 &&
        pdfBytes[0] === 0x25 &&
        pdfBytes[1] === 0x50 &&
        pdfBytes[2] === 0x44 &&
        pdfBytes[3] === 0x46;
      const mime = isPdf ? "application/pdf" : "text/html;charset=utf-8";
      const base64 = uint8ArrayToBase64(pdfBytes);
      pdfUrl = `data:${mime};base64,${base64}`;
      progress = 100;
      statusText = m.shared_ui_ready_to_print();
    } else {
      throw new Error(m.shared_ui_empty_document_received());
    }
  } catch (e: any) {
    errorMessage =
      extractErrorMessage(e) || m.shared_ui_report_generation_failed();
    toast.error(errorMessage);
  } finally {
    isGenerating = false;
  }
}

// React to dialog open
$effect(() => {
  if (!open) {
    startedTemplateId = "";
    currentJobId = null; // Cancel polling
  } else if (templateId && startedTemplateId !== templateId) {
    startedTemplateId = templateId;
    void generateReport();
  }
});

function handleNativePrint() {
  if (!pdfUrl) return;

  // Find the iframe
  const iframe = document.getElementById(
    "report-viewer-iframe",
  ) as HTMLIFrameElement;
  if (iframe && iframe.contentWindow) {
    // Calling print() on the iframe contentWindow triggers the OS Native Print Dialog
    iframe.contentWindow.print();
  } else {
    toast.error(m.shared_ui_print_frame_unavailable());
  }
}

function handleDownload() {
  if (!pdfUrl) return;
  const link = document.createElement("a");
  link.href = pdfUrl;
  link.download = `${reportTitle.replace(/\s+/g, "_")}_${Date.now()}.pdf`;
  link.click();
}
</script>

<Dialog bind:open={open}>
  <DialogContent class="sm:max-w-5xl h-[90vh] flex flex-col overflow-hidden p-0 gap-0 border-none bg-background shadow-2xl rounded-xl">
    <!-- Header -->
    <DialogHeader class="shrink-0 p-4 border-b border-border bg-muted/30">
      <DialogTitle class="flex items-center gap-2 text-lg">
        <DocumentPage class="text-primary h-5 w-5" />
        <span>{reportTitle}</span>
      </DialogTitle>
    </DialogHeader>

    <!-- Main Content Area -->
    <div class="flex-1 overflow-hidden relative bg-zinc-100 dark:bg-zinc-900 flex items-center justify-center">
      {#if isGenerating || (progress > 0 && progress < 100 && !errorMessage)}
        <div class="flex flex-col items-center justify-center gap-6 w-full max-w-md p-8 bg-card rounded-2xl shadow-sm border border-border">
          <div class="flex items-center justify-between w-full text-sm font-medium text-muted-foreground">
            <span>{statusText}</span>
            <span class="font-bold text-foreground">{progress}%</span>
          </div>
          <Progress value={progress} class="h-2.5 w-full" />
          <div class="flex items-center gap-2 mt-2">
            <LoadingIndicator class="text-primary"  size={24} />
          </div>
        </div>
      {:else if errorMessage}
        <div class="flex flex-col items-center justify-center gap-4 text-destructive p-8 text-center">
          <XMark size={48} />
          <h3 class="text-lg font-bold">{m.shared_ui_generation_failed()}</h3>
          <p class="text-sm opacity-80 max-w-sm">{errorMessage}</p>
          <Button variant="outline" onclick={generateReport} class="mt-4">
            <Refresh size={14} class="me-2" />
            {m.common_retry()}
          </Button>
        </div>
      {:else if pdfUrl}
        <iframe
          id="report-viewer-iframe"
          src={pdfUrl}
          title={reportTitle}
          class="w-full h-full border-none shadow-inner bg-transparent"
        ></iframe>
      {:else}
        <div class="text-muted-foreground flex flex-col items-center gap-3">
          <DocumentPage size={48} class="opacity-20" />
          <p class="text-sm">{m.shared_ui_no_report_generated()}</p>
        </div>
      {/if}
    </div>

    <!-- Footer Controls -->
    <DialogFooter class="shrink-0 p-4 border-t border-border bg-card flex items-center sm:justify-between">
      <div class="flex items-center gap-2 hidden sm:flex">
        {#if pdfUrl}
            <div class="flex items-center gap-1.5 text-emerald-600 dark:text-emerald-400 text-xs font-semibold px-2 py-1 bg-emerald-500/10 rounded-md">
                <CheckCircle size={14} />
                <span>{m.shared_ui_ready()}</span>
            </div>
        {/if}
      </div>
      
      <div class="flex items-center gap-3 w-full sm:w-auto">
        <Button variant="outline" class="flex-1 sm:flex-none" onclick={() => (open = false)}>
          {m.common_action_close()}
        </Button>
        
        <Button 
            variant="secondary" 
            onclick={handleDownload}
            disabled={!pdfUrl || isGenerating}
            class="flex-1 sm:flex-none"
        >
          <Download size={16} class="me-2" />
          {m.shared_ui_download_pdf()}
        </Button>

        <Button 
            variant="default" 
            onclick={handleNativePrint}
            disabled={!pdfUrl || isGenerating}
            class="flex-1 sm:flex-none font-bold"
        >
          <Printer size={16} class="me-2" />
          {m.shared_ui_print_natively()}
        </Button>
      </div>
    </DialogFooter>
  </DialogContent>
</Dialog>
