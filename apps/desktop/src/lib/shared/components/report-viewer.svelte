<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * ReportViewer — A C# .NET style native printing component.
 * Connects to api.reports.generate, polls status, and fetches PDF bytes.
 * Provides a WYSIWYG preview and triggers the OS native print dialog.
 */
import { api } from "$lib/api/client";
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
import { Printer, RefreshCw, FileText, Download, CheckCircle2, XCircle } from "@lucide/svelte";

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

function uint8ArrayToBase64(bytes: Uint8Array): string {
  let binary = "";
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

async function generateReport() {
  if (!templateId) return;

  isGenerating = true;
  pdfUrl = null;
  errorMessage = null;
  progress = 10;
  statusText = "Initializing report generation...";

  try {
    // 1. Trigger Generation
    const { jobId } = await api.reports.generate({
      templateId,
      format: "pdf",
      parameters,
    });
    
    currentJobId = jobId;
    progress = 30;
    statusText = "Generating document...";

    // 2. Poll Status
    const pollingDeadline = Date.now() + 15 * 60 * 1000; // 15 mins max
    while (currentJobId === jobId) {
      if (Date.now() >= pollingDeadline) {
        throw new Error("Report generation timed out.");
      }

      const { status } = await api.reports.getJobStatus(jobId);
      
      if (status === "completed") {
        progress = 80;
        statusText = "Fetching document...";
        break;
      } else if (status === "failed") {
        throw new Error("Report generation failed on the server.");
      }

      await new Promise((r) => setTimeout(r, 1000));
    }

    if (currentJobId !== jobId) return; // Dialog was closed or reset

    // 3. Fetch PDF Bytes
    const pdfBytes = await api.reports.downloadFile(jobId);
    
    if (pdfBytes && pdfBytes.length > 0) {
      const base64 = uint8ArrayToBase64(pdfBytes);
      pdfUrl = `data:application/pdf;base64,${base64}`;
      progress = 100;
      statusText = "Ready to print";
    } else {
        throw new Error("Received empty document.");
    }
  } catch (e: any) {
    errorMessage = e.message || "An error occurred while generating the report.";
    toast.error(errorMessage);
  } finally {
    isGenerating = false;
  }
}

// React to dialog open
$effect(() => {
  if (open && templateId && !pdfUrl && !isGenerating) {
    generateReport();
  }
  
  if (!open) {
      currentJobId = null; // Cancel polling
  }
});

function handleNativePrint() {
  if (!pdfUrl) return;
  
  // Find the iframe
  const iframe = document.getElementById("report-viewer-iframe") as HTMLIFrameElement;
  if (iframe && iframe.contentWindow) {
    // Calling print() on the iframe contentWindow triggers the OS Native Print Dialog
    iframe.contentWindow.print();
  } else {
    toast.error("Unable to access print frame.");
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
        <FileText class="text-primary h-5 w-5" />
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
            <RefreshCw size={24} class="animate-spin text-primary" />
          </div>
        </div>
      {:else if errorMessage}
        <div class="flex flex-col items-center justify-center gap-4 text-destructive p-8 text-center">
          <XCircle size={48} />
          <h3 class="text-lg font-bold">Generation Failed</h3>
          <p class="text-sm opacity-80 max-w-sm">{errorMessage}</p>
          <Button variant="outline" onclick={generateReport} class="mt-4">
            <RefreshCw size={14} class="me-2" />
            Retry
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
          <FileText size={48} class="opacity-20" />
          <p class="text-sm">No report generated</p>
        </div>
      {/if}
    </div>

    <!-- Footer Controls -->
    <DialogFooter class="shrink-0 p-4 border-t border-border bg-card flex items-center sm:justify-between">
      <div class="flex items-center gap-2 hidden sm:flex">
        {#if pdfUrl}
            <div class="flex items-center gap-1.5 text-emerald-600 dark:text-emerald-400 text-xs font-semibold px-2 py-1 bg-emerald-500/10 rounded-md">
                <CheckCircle2 size={14} />
                <span>Ready</span>
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
          Download PDF
        </Button>

        <Button 
            variant="default" 
            onclick={handleNativePrint}
            disabled={!pdfUrl || isGenerating}
            class="flex-1 sm:flex-none font-bold"
        >
          <Printer size={16} class="me-2" />
          Print Natively
        </Button>
      </div>
    </DialogFooter>
  </DialogContent>
</Dialog>
