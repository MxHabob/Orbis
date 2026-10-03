<script lang="ts">
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { ArrowPath, DocumentChartBar, Download } from "@orbis/icons";
import { api } from "$lib/api";
import { toast } from "@orbis/ui";

import * as m from "$lib/paraglide/messages";

interface Props {
  open: boolean;
  definitionId: string;
  filters: any;
  onOpenChange: (val: boolean) => void;
}

let {
  open = $bindable(),
  definitionId,
  filters,
  onOpenChange,
}: Props = $props();

let selectedFormat = $state("csv");
let isExporting = $state(false);

async function handleExport() {
  if (!definitionId) return;
  try {
    isExporting = true;
    const jobId = await api.generateReport(
      definitionId,
      selectedFormat,
      filters || {},
    );
    const result = await api.downloadReportFile(jobId);

    // result is the byte array, we need to download it
    const blob = new Blob([new Uint8Array(result)], {
      type: "text/csv",
    });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `report_${definitionId}_${new Date().toISOString()}.${selectedFormat}`;
    a.click();
    window.URL.revokeObjectURL(url);

    toast.success(m.ui_copy_report_exported_successfully_566e1620());
    onOpenChange(false);
  } catch (e: any) {
    toast.error(e.message || "Export failed");
  } finally {
    isExporting = false;
  }
}
</script>

<Dialog bind:open {onOpenChange}>
  <DialogContent class="sm:max-w-106.25">
    <DialogHeader>
      <DialogTitle>{m.common_action_export()}</DialogTitle>
      <DialogDescription>
        {m.ui_copy_choose_the_format_to_export_your_report_112df733()}
      </DialogDescription>
    </DialogHeader>

    <div class="grid grid-cols-1 gap-4 py-4">
      <button
        class="flex flex-col items-center justify-center gap-2 p-4 rounded-xl border-2 transition-all hover:bg-muted {selectedFormat === 'csv' ? 'border-primary bg-primary/5 text-primary' : 'border-transparent bg-muted/50'}"
        onclick={() => selectedFormat = "csv"}
      >
        <DocumentChartBar size={32} aria-hidden="true" />
        <span class="text-sm font-medium">CSV</span>
      </button>
    </div>

    <DialogFooter>
      <Button variant="outline" onclick={() => onOpenChange(false)} disabled={isExporting}>
        {m.common_action_cancel()}
      </Button>
      <Button onclick={handleExport} disabled={isExporting}>
        {#if isExporting}
          <ArrowPath class="mr-2 h-4 w-4 animate-spin" />
        {:else}
          <Download class="mr-2 h-4 w-4" />
        {/if}
        {m.common_action_export()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
