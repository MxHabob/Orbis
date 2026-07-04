<script lang="ts">
/**
 * PrintConfigPanel — Print settings panel for Multi-Up configuration.
 * Controls sheet size, orientation, gaps, cut lines, fill order, and margins.
 */
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Switch } from "@orbis/ui-svelte/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";
import { Separator } from "@orbis/ui-svelte/separator";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

import type { MultiUpConfig, SheetSize } from "@docforge/core";

let {
  config = $bindable(),
}: {
  config: MultiUpConfig;
} = $props();

const sheetSizes: { value: SheetSize; label: string }[] = [
  { value: "A4", label: "A4 (210×297mm)" },
  { value: "A3", label: "A3 (297×420mm)" },
  { value: "A5", label: "A5 (148×210mm)" },
  { value: "Letter", label: "Letter (216×279mm)" },
  { value: "Legal", label: "Legal (216×356mm)" },
];

function onSheetSizeChange(val: string) {
  if (val) config.sheetSize = val as SheetSize;
}

function onOrientationChange(val: string) {
  if (val) config.sheetOrientation = val as "portrait" | "landscape";
}

function onFillOrderChange(val: string) {
  if (val) config.fillOrder = val as "ltr-ttb" | "rtl-ttb";
}
</script>

<div class="flex flex-col gap-4 text-start">
  <!-- Sheet Size -->
  <div class="flex flex-col gap-1.5">
    <Label class="text-xs font-semibold text-muted-foreground">
      {m.print_multi_up_sheet_size()}
    </Label>
    <Select type="single" value={config.sheetSize} onValueChange={onSheetSizeChange}>
      <SelectTrigger class="h-8 text-xs">
        {sheetSizes.find(s => s.value === config.sheetSize)?.label ?? config.sheetSize}
      </SelectTrigger>
      <SelectContent>
        {#each sheetSizes as size (size.value)}
          <SelectItem value={size.value}>{size.label}</SelectItem>
        {/each}
      </SelectContent>
    </Select>
  </div>

  <!-- Orientation -->
  <div class="flex flex-col gap-1.5">
    <Label class="text-xs font-semibold text-muted-foreground">
      {m.print_multi_up_orientation()}
    </Label>
    <Select type="single" value={config.sheetOrientation} onValueChange={onOrientationChange}>
      <SelectTrigger class="h-8 text-xs">
        {config.sheetOrientation === "portrait" ? m.print_multi_up_portrait() : m.print_multi_up_landscape()}
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="portrait">{m.print_multi_up_portrait()}</SelectItem>
        <SelectItem value="landscape">{m.print_multi_up_landscape()}</SelectItem>
      </SelectContent>
    </Select>
  </div>

  <Separator />

  <!-- Gap Settings -->
  <div class="grid grid-cols-2 gap-3">
    <div class="flex flex-col gap-1.5">
      <Label class="text-[10px] text-muted-foreground">{m.print_multi_up_gap_horizontal()}</Label>
      <Input
        type="number"
        class="h-7 text-xs"
        min={0}
        max={20}
        step={0.5}
        bind:value={config.gapHorizontalMm}
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <Label class="text-[10px] text-muted-foreground">{m.print_multi_up_gap_vertical()}</Label>
      <Input
        type="number"
        class="h-7 text-xs"
        min={0}
        max={20}
        step={0.5}
        bind:value={config.gapVerticalMm}
      />
    </div>
  </div>

  <!-- Cut Lines -->
  <div class="flex items-center justify-between gap-2">
    <div class="flex flex-col gap-0.5">
      <Label class="text-xs font-semibold text-muted-foreground">
        {m.print_multi_up_cut_lines()}
      </Label>
      <span class="text-[9px] text-muted-foreground/70">
        {m.print_multi_up_cut_lines_desc()}
      </span>
    </div>
    <Switch
      checked={config.cutLineWidthMm > 0}
      onCheckedChange={(checked) => {
        config.cutLineWidthMm = checked ? 0.5 : 0;
      }}
    />
  </div>

  <!-- Advanced Settings (Accordion) -->
  <Accordion type="single" collapsible>
    <AccordionItem value="advanced">
      <AccordionTrigger class="text-xs font-medium text-muted-foreground py-2">
        {m.print_multi_up_advanced()}
      </AccordionTrigger>
      <AccordionContent>
        <div class="flex flex-col gap-3 pt-2">
          <!-- Fill Order -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] text-muted-foreground">{m.print_multi_up_fill_order()}</Label>
            <Select type="single" value={config.fillOrder} onValueChange={onFillOrderChange}>
              <SelectTrigger class="h-7 text-xs">
                {config.fillOrder === "rtl-ttb" ? m.print_multi_up_fill_rtl() : m.print_multi_up_fill_ltr()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="rtl-ttb">{m.print_multi_up_fill_rtl()}</SelectItem>
                <SelectItem value="ltr-ttb">{m.print_multi_up_fill_ltr()}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <!-- Center on sheet -->
          <div class="flex items-center justify-between gap-2">
            <Label class="text-[10px] text-muted-foreground">
              {m.print_multi_up_center_on_sheet()}
            </Label>
            <Switch bind:checked={config.centerOnSheet} />
          </div>

          <!-- Margins -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] text-muted-foreground">{m.print_multi_up_margins()}</Label>
            <div class="grid grid-cols-2 gap-2">
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground/70">{m.print_multi_up_margin_top()}</span>
                <Input type="number" class="h-6 text-[10px]" min={0} max={50} step={1} bind:value={config.sheetMargins.top} />
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground/70">{m.print_multi_up_margin_bottom()}</span>
                <Input type="number" class="h-6 text-[10px]" min={0} max={50} step={1} bind:value={config.sheetMargins.bottom} />
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground/70">{m.print_multi_up_margin_left()}</span>
                <Input type="number" class="h-6 text-[10px]" min={0} max={50} step={1} bind:value={config.sheetMargins.left} />
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground/70">{m.print_multi_up_margin_right()}</span>
                <Input type="number" class="h-6 text-[10px]" min={0} max={50} step={1} bind:value={config.sheetMargins.right} />
              </div>
            </div>
          </div>
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</div>
