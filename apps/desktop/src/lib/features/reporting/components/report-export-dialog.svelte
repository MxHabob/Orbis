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
  import { Table, Download, Loader2 } from "@lucide/svelte";
  import { commands } from "@orbis/sdk-desktop";
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
    onOpenChange
  } = $props<Props>();

  let isExporting = $state(false);
  // svc-reporting currently exposes CSV only. Keep the UI aligned with the
  // active backend path until PDF/XLSX are implemented there.
  let selectedFormat = $state<"csv">("csv");

  async function handleExport() {
    if (!definitionId) return;
    try {
      isExporting = true;
      const result = await commands.reporting.exportReport(definitionId, filters, selectedFormat, null);
      
      // result is the byte array, we need to download it
      const blob = new Blob([new Uint8Array(result)], {
        type:
          "text/csv",
      });
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `report_${definitionId}_${new Date().toISOString()}.${selectedFormat}`;
      a.click();
      window.URL.revokeObjectURL(url);
      
      toast.success(m.common_success());
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
      <DialogTitle>{m.common_export()}</DialogTitle>
      <DialogDescription>
        Choose the format to export your report.
      </DialogDescription>
    </DialogHeader>

    <div class="grid grid-cols-1 gap-4 py-4">
      <button
        class="flex flex-col items-center justify-center gap-2 p-4 rounded-xl border-2 transition-all hover:bg-muted {selectedFormat === 'csv' ? 'border-primary bg-primary/5 text-primary' : 'border-transparent bg-muted/50'}"
        onclick={() => selectedFormat = "csv"}
      >
        <Table size={32} aria-hidden="true" />
        <span class="text-sm font-medium">CSV</span>
      </button>
    </div>

    <DialogFooter>
      <Button variant="outline" onclick={() => onOpenChange(false)} disabled={isExporting}>
        {m.common_cancel()}
      </Button>
      <Button onclick={handleExport} disabled={isExporting}>
        {#if isExporting}
          <Loader2 class="mr-2 h-4 w-4 animate-spin" />
        {:else}
          <Download class="mr-2 h-4 w-4" />
        {/if}
        {m.common_export()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
