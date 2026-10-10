<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetFooter,
} from "@orbis/ui/sheet";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import type { AcademicPeriodDto, ClassDto, ReportDefinitionDto } from "$lib/api";

interface Props {
  open: boolean;
  selectedDefinition: ReportDefinitionDto | null;
  periods: AcademicPeriodDto[];
  classes: ClassDto[];
  selectedFormat: "PDF" | "HTML";
  selectedPeriodId: string;
  selectedClassId: string;
  dateFrom: string;
  dateTo: string;
  isGenerating: boolean;
  jobStatus: string;
  onExecute: () => void;
}

let {
  open = $bindable(false),
  selectedDefinition,
  periods,
  classes,
  selectedFormat = $bindable<"PDF" | "HTML">("PDF"),
  selectedPeriodId = $bindable(""),
  selectedClassId = $bindable(""),
  dateFrom = $bindable(""),
  dateTo = $bindable(""),
  isGenerating,
  jobStatus,
  onExecute,
}: Props = $props();
</script>

<Sheet bind:open>
  {#if selectedDefinition}
    <SheetContent side="end" class="flex h-full w-full flex-col gap-0 overflow-hidden p-0 text-start sm:max-w-md">
      <SheetHeader class="shrink-0 border-b border-border/40 px-6 py-4 text-start">
        <SheetTitle class="text-base font-bold leading-snug text-foreground">
          {m.operations_ui_generate_report_named({ name: selectedDefinition.name_ar || selectedDefinition.name })}
        </SheetTitle>
      </SheetHeader>

      <div class="min-h-0 flex-1 overflow-y-auto p-6 scrollbar-hide">
        <div class="space-y-5">
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{m.operations_ui_academic_term()}</Label>
            <Select value={selectedPeriodId} onValueChange={(v) => (selectedPeriodId = v)}>
              <SelectTrigger class="h-9 text-xs">
                <span>
                  {periods.find((p) => p.id === selectedPeriodId)
                    ? l(periods.find((p) => p.id === selectedPeriodId)?.name_ar, periods.find((p) => p.id === selectedPeriodId)?.name)
                    : m.operations_ui_select_academic_term()}
                </span>
              </SelectTrigger>
              <SelectContent>
                {#each periods as p (p.id)}
                  <SelectItem value={p.id}>
                    {l(p.name_ar, p.name) || p.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>

          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{m.operations_ui_filter_by_class_optional()}</Label>
            <Select value={selectedClassId} onValueChange={(v) => (selectedClassId = v)}>
              <SelectTrigger class="h-9 text-xs">
                <span>
                  {classes.find((c) => String(c.id) === selectedClassId)
                    ? l(classes.find((c) => String(c.id) === selectedClassId)?.name_ar, classes.find((c) => String(c.id) === selectedClassId)?.name)
                    : m.operations_ui_all_classes_sections()}
                </span>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">{m.operations_ui_all_classes_sections()}</SelectItem>
                {#each classes as c (c.id)}
                  <SelectItem value={String(c.id)}>
                    {l(c.name_ar, c.name) || c.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <div class="space-y-1.5">
              <Label class="text-xs font-semibold">{m.operations_ui_date_from()}</Label>
              <Input type="date" bind:value={dateFrom} class="h-9 text-xs" />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-semibold">{m.operations_ui_date_to()}</Label>
              <Input type="date" bind:value={dateTo} class="h-9 text-xs" />
            </div>
          </div>

          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{m.operations_ui_file_format()}</Label>
            <div class="grid grid-cols-2 gap-2">
              <Button
                type="button"
                variant={selectedFormat === "PDF" ? "default" : "outline"}
                onclick={() => (selectedFormat = "PDF")}
                aria-pressed={selectedFormat === "PDF"}
                class="h-auto min-h-10 w-full whitespace-normal px-3 py-2 text-xs font-semibold"
              >
                {m.operations_ui_print_ready_pdf()}
              </Button>
              <Button
                type="button"
                variant={selectedFormat === "HTML" ? "default" : "outline"}
                onclick={() => (selectedFormat = "HTML")}
                aria-pressed={selectedFormat === "HTML"}
                class="h-auto min-h-10 w-full whitespace-normal px-3 py-2 text-xs font-semibold"
              >
                {m.operations_ui_interactive_html()}
              </Button>
            </div>
          </div>

          {#if isGenerating}
            <div class="rounded-lg border border-primary/20 bg-primary/5 p-4">
              <div class="text-xs text-primary font-semibold">{jobStatus}</div>
            </div>
          {/if}
        </div>
      </div>

      <SheetFooter class="mt-0 shrink-0 flex-row items-center justify-end gap-2 border-t border-border/40 bg-muted/20 px-6 py-3">
        <Button
          variant="outline"
          size="sm"
          onclick={() => (open = false)}
          disabled={isGenerating}
          class="h-9 px-4 text-xs font-semibold"
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          variant="default"
          size="sm"
          onclick={onExecute}
          disabled={isGenerating}
          class="h-9 px-4 text-xs font-semibold cursor-pointer"
        >
          {isGenerating ? m.operations_ui_report_generating() : m.operations_ui_start_generate_preview()}
        </Button>
      </SheetFooter>
    </SheetContent>
  {/if}
</Sheet>
