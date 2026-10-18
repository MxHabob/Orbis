<script lang="ts">
import {
    getDocumentSource,
    useUploadFile,
} from "$lib/features/documents/queries";
import * as m from "$lib/paraglide/messages";
import {
    AdjustmentsHorizontal,
    ArrowUpTray,
    DocumentPage,
    Plus,
    Trash,
    XMark,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { ScrollArea } from "@orbis/ui/scroll-area";
import { Separator } from "@orbis/ui/separator";
import { Textarea } from "@orbis/ui/textarea";
import ColorPicker from "../shared/color-picker.svelte";
import PropertySelect from "../shared/property-select.svelte";
import type { EditorStore } from "../stores/editor-store.svelte";

let {
  store,
  onclose,
  width = $bindable(300),
}: {
  store: EditorStore;
  onclose?: () => void;
  width?: number;
} = $props();

const node = $derived(store.selectedNode);
let imageFileInput: HTMLInputElement | undefined = $state();
const imageUploadMutation = useUploadFile();
const compactTextareaClass = "text-xs";
const pageSetup = $derived(store.document.meta.pageSetup);
const nodeSectionTitle = $derived(
  node?.type === "Table"
    ? m.template_inspector_table_settings()
    : node?.type === "Text"
      ? m.template_inspector_text_properties()
      : node?.type === "Shape"
        ? m.template_inspector_shape_properties()
        : m.template_inspector_element_details(),
);
const nodeProps = $derived(
  node && "props" in node ? ((node as any).props || {}) : {}
);
const pos = $derived(
  nodeProps.position || (node as any)?.position || { x: 0, y: 0, width: 50, height: 20 }
);

function updateProp(key: string, value: unknown) {
  if (!node) return;
  store.updateSelectedProps({ [key]: value });
}

function triggerImageReplace() {
  imageFileInput?.click();
}

function handleImageFileSelect(event: Event) {
  const fileInput = event.currentTarget as HTMLInputElement;
  const file = fileInput.files?.[0];
  const targetNodeId = node?.id;
  if (!file || !targetNodeId) return;
  const uploadToast = toast.loading(m.template_inspector_replacing_image(), {
    duration: 0,
  });

  imageUploadMutation.mutate(
    {
      file,
      name: file.name,
      entity_type: "template",
      entity_id: store.document.meta.id || "shared",
      category: "image",
      visibility: "private",
    },
    {
      onSuccess: (uploaded) => {
        if (!uploaded?.id) {
          uploadToast.update({
            title: m.template_inspector_replace_missing_id(),
            variant: "error",
            duration: 5000,
          });
          return;
        }

        try {
          store.addAsset(uploaded.id, "image");
          store.updateNodeProps(targetNodeId, {
            props: {
              assetRef: uploaded.id,
              src: getDocumentSource(uploaded),
            },
          });
          uploadToast.update({
            title: m.template_inspector_replace_success(),
            variant: "success",
            duration: 4000,
          });
        } catch (error) {
          uploadToast.update({
            title: error instanceof Error ? error.message : m.template_inspector_update_failed(),
            variant: "error",
            duration: 5000,
          });
          console.error("Failed to replace template image:", error);
        }
      },
      onError: (error) => {
        uploadToast.update({
          title: error instanceof Error ? error.message : m.template_inspector_image_upload_failed(),
          variant: "error",
          duration: 5000,
        });
      },
      onSettled: () => {
        fileInput.value = "";
      },
    },
  );
}

function updateTableStyle(key: string, value: unknown) {
  if (!node) return;
  const currentStyle = nodeProps.style || {};
  store.updateSelectedProps({
    style: {
      ...currentStyle,
      [key]: value,
    },
  });
}

function updatePosition(key: string, val: number | string) {
  if (!node) return;
  const currentPos = { ...pos, [key]: val };
  if ("props" in node) {
    store.updateSelectedProps({ props: { position: currentPos } });
  } else {
    store.updateSelectedProps({ position: currentPos });
  }
}

function startResize(event: PointerEvent) {
  event.preventDefault();
  const startX = event.clientX;
  const startWidth = width;
  const direction = getComputedStyle(document.documentElement).direction;
  const handleMove = (moveEvent: PointerEvent) => {
    const delta = direction === "rtl"
      ? moveEvent.clientX - startX
      : startX - moveEvent.clientX;
    width = Math.min(400, Math.max(260, startWidth + delta));
  };
  const stopResize = () => {
    window.removeEventListener("pointermove", handleMove);
    window.removeEventListener("pointerup", stopResize);
    window.removeEventListener("pointercancel", stopResize);
  };
  window.addEventListener("pointermove", handleMove);
  window.addEventListener("pointerup", stopResize, { once: true });
  window.addEventListener("pointercancel", stopResize, { once: true });
}

function resizeWithKeyboard(event: KeyboardEvent) {
  if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") return;
  event.preventDefault();
  const direction = getComputedStyle(document.documentElement).direction;
  const expands = direction === "rtl" ? event.key === "ArrowRight" : event.key === "ArrowLeft";
  width = Math.min(400, Math.max(260, width + (expands ? 16 : -16)));
}

// ── Table Column Helpers ─────────────────────────────────────
const columns = $derived<any[]>(nodeProps.columns || []);

function addColumn() {
  const newCol = {
    id: `col_${Date.now()}`,
    header: "New Column",
    headerAr: "عمود جديد",
    field: "field",
    width: "auto",
    alignment: "center",
    totalType: "none",
    isVisible: true,
  };
  updateProp("columns", [...columns, newCol]);
}

function removeColumn(index: number) {
  const next = [...columns];
  next.splice(index, 1);
  updateProp("columns", next);
}

function updateColumn(index: number, key: string, val: unknown) {
  const next = [...columns];
  next[index] = { ...next[index], [key]: val };
  updateProp("columns", next);
}

function applyYemeniCertificateColumnsPreset() {
  const preset = [
    { id: "seq", header: "#", headerAr: "م", field: "seq", width: "6%", alignment: "center", totalType: "label", isVisible: true },
    { id: "subject_name", header: "Subject", headerAr: "المادة الدراسية", field: "subject_name", width: "26%", alignment: "right", totalType: "none", isVisible: true },
    { id: "max_score", header: "Max", headerAr: "النهاية الكبرى", field: "max_score", width: "10%", alignment: "center", totalType: "sum", isVisible: true },
    { id: "min_score", header: "Min", headerAr: "النهاية الصغرى", field: "min_score", width: "10%", alignment: "center", totalType: "sum", isVisible: true },
    { id: "t1_coursework", header: "CW 1", headerAr: "محصلة ف1", field: "t1_coursework", width: "10%", alignment: "center", totalType: "sum", isVisible: true },
    { id: "t1_exam", header: "Exam 1", headerAr: "اختبار ف1", field: "t1_exam", width: "10%", alignment: "center", totalType: "sum", isVisible: true },
    { id: "annual_total", header: "Total", headerAr: "المجموع السنوي", field: "annual_total", width: "12%", alignment: "center", totalType: "sum", isVisible: true },
    { id: "appreciation", header: "Grade", headerAr: "التقدير", field: "appreciation", width: "9%", alignment: "center", totalType: "none", isVisible: true },
    { id: "status", header: "Result", headerAr: "النتيجة", field: "status", width: "7%", alignment: "center", totalType: "none", isVisible: true },
  ];
  updateProp("columns", preset);
  updateProp("dataSource", "subjects");
  updateProp("showHeader", true);
  updateProp("showTotals", true);
  updateProp("showGrid", true);
  updateProp("showBorder", true);
  updateProp("direction", "rtl");
  updateProp("style", {
    headerBgColor: "#1e3a8a",
    headerTextColor: "#ffffff",
    headerFontSize: 9,
    headerFontWeight: "bold",
    rowBgColor: "#ffffff",
    altRowBgColor: "#f8fafc",
    borderColor: "#475569",
    borderWidth: 0.75,
    fontSize: 8.5,
    fontFamily: "Amiri",
    rowHeight: 20,
    direction: "rtl",
  });
}
</script>

<aside class="properties-inspector" style:width={`${width}px`} aria-label={m.template_inspector_editor_properties()}>
  <Button
    variant="ghost"
    size="icon-xs"
    class="resize-handle"
    type="button"
    aria-label={m.template_inspector_resize_panel()}
    title={m.template_inspector_resize_hint()}
    onpointerdown={startResize}
    onkeydown={resizeWithKeyboard}
  />
  <div class="inspector-header">
    <div class="flex min-w-0 items-center gap-2">
      <AdjustmentsHorizontal class="size-4 shrink-0 text-muted-foreground" />
      <h2 class="inspector-title">{node ? m.template_inspector_element_properties() : m.template_inspector_page_properties()}</h2>
    </div>
    <div class="flex shrink-0 items-center gap-1">
      {#if node}
        <Badge variant="outline" class="inspector-type">{node.type}</Badge>
      {/if}
      {#if onclose}
        <Button variant="ghost" size="icon-sm" aria-label={m.template_inspector_hide_properties()} onclick={onclose}>
          <XMark class="size-4" />
        </Button>
      {/if}
    </div>
  </div>

  <ScrollArea class="inspector-scroll">
    <div class="inspector-body">
      {#if !node}
        <div class="page-inspector">
          <div class="page-summary">
            <DocumentPage class="size-4 text-muted-foreground" />
            <span>{pageSetup.size} · {pageSetup.orientation === "landscape" ? m.template_orientation_horz() : m.template_orientation_vert()}</span>
          </div>

          <Button
            variant="outline"
            size="sm"
            class="w-full"
            onclick={() => {
              onclose?.();
              store.activeRailPanel = "page-setup";
            }}
          >
            <AdjustmentsHorizontal class="size-4" />
            {m.template_inspector_page_setup()}
          </Button>

          <p class="text-xs leading-5 text-muted-foreground">{m.template_inspector_select_element_hint()}</p>
        </div>
      {:else}
        <div class="selected-layer">
          <span class="selected-layer-name" title={node.name || node.id}>{node.name || node.id}</span>
        </div>

        <details class="property-section" open>
          <summary><span>{m.template_inspector_general()}</span></summary>
          <div class="property-section-body space-y-3">
            <div class="space-y-1">
              <Label class="property-label">{m.template_inspector_layer_name()}</Label>
              <Input
                value={node.name || ""}
                placeholder={node.type}
                onchange={(e) => updateProp("name", (e.target as HTMLInputElement).value)}
                class="h-8 text-xs"
              />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="flex items-center gap-2 text-xs">
                <Checkbox
                  checked={node.visible !== false}
                  onCheckedChange={(checked) => updateProp("visible", checked === true)}
                  aria-label={m.template_inspector_show_element()}
                />
                <span>{m.template_inspector_show_element()}</span>
              </div>
              <div class="flex items-center gap-2 text-xs">
                <Checkbox
                  checked={node.locked === true}
                  onCheckedChange={(checked) => updateProp("locked", checked === true)}
                  aria-label={m.template_inspector_lock_element()}
                />
                <span>{m.template_inspector_lock_element()}</span>
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_opacity()}</Label>
                <Input type="number" min="0" max="1" step="0.05" value={node.opacity ?? 1} onchange={(e) => updateProp("opacity", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_rotation()}</Label>
                <Input type="number" step="1" value={node.rotation ?? 0} onchange={(e) => updateProp("rotation", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
            </div>
          </div>
        </details>

        <details class="property-section" open>
          <summary>
            <span>{m.template_inspector_position_size()}</span>
            <span class="section-unit">{m.template_inspector_unit_mm()}</span>
          </summary>
          <div class="property-section-body">
          <div class="grid grid-cols-2 gap-2 text-xs">
            <div class="space-y-1">
              <span class="property-label">X</span>
              <Input
                type="number"
                step="0.5"
                value={pos.x ?? 0}
                onchange={(e) => updatePosition("x", parseFloat((e.target as HTMLInputElement).value) || 0)}
                class="h-8 border-border/80 bg-background text-xs font-mono"
              />
            </div>
            <div class="space-y-1">
              <span class="property-label">Y</span>
              <Input
                type="number"
                step="0.5"
                value={pos.y ?? 0}
                onchange={(e) => updatePosition("y", parseFloat((e.target as HTMLInputElement).value) || 0)}
                class="h-8 border-border/80 bg-background text-xs font-mono"
              />
            </div>
            <div class="space-y-1">
              <span class="property-label">{m.template_inspector_width()}</span>
              <Input
                type="text"
                value={pos.width ?? "100%"}
                onchange={(e) => updatePosition("width", (e.target as HTMLInputElement).value)}
                class="h-8 border-border/80 bg-background text-xs font-mono"
              />
            </div>
            <div class="space-y-1">
              <span class="property-label">{m.template_inspector_height()}</span>
              <Input
                type="text"
                value={pos.height ?? "auto"}
                onchange={(e) => updatePosition("height", (e.target as HTMLInputElement).value)}
                class="h-8 border-border/80 bg-background text-xs font-mono"
              />
            </div>
          </div>
          </div>
        </details>

        <details class="property-section" open>
          <summary>
            <span>{nodeSectionTitle}</span>
          </summary>
          <div class="property-section-body">
        {#if node.type === "Table"}
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <Label class="text-xs font-semibold text-foreground">{m.template_inspector_table()}</Label>
              <Button
                variant="ghost"
                size="xs"
                onclick={applyYemeniCertificateColumnsPreset}
                class="text-[11px] h-7 text-primary cursor-pointer hover:bg-primary/10"
                title={m.template_inspector_yemeni_certificate_preset_title()}
              >
                {m.template_inspector_approved_certificate_preset()}
              </Button>
            </div>

            <details class="inspector-subsection" open>
              <summary>{m.template_inspector_data_display()}</summary>
              <div class="inspector-subsection-body">
            <!-- Data Source Field -->
            <div class="space-y-1">
              <Label class="text-[11px] text-muted-foreground">{m.template_inspector_data_source()}</Label>
              <Input
                type="text"
                placeholder="e.g. subjects or rows"
                value={nodeProps.dataSource || "subjects"}
                onchange={(e) => updateProp("dataSource", (e.target as HTMLInputElement).value)}
                class="h-7 text-xs font-mono"
              />
            </div>

            <!-- Direction & Display Toggles -->
            <div class="grid grid-cols-2 gap-2 text-xs pt-1">
              <div class="flex items-center gap-1.5 cursor-pointer">
                <Checkbox
                  checked={nodeProps.showHeader !== false}
                  onCheckedChange={(checked) => updateProp("showHeader", checked === true)}
                  aria-label={m.template_inspector_show_header()}
                />
                <span class="text-[11px]">{m.template_inspector_show_header()}</span>
              </div>

              <div class="flex items-center gap-1.5 cursor-pointer">
                <Checkbox
                  checked={nodeProps.showTotals === true}
                  onCheckedChange={(checked) => updateProp("showTotals", checked === true)}
                  aria-label={m.template_inspector_grand_total_row()}
                />
                <span class="text-[11px]">{m.template_inspector_grand_total_row()}</span>
              </div>

              <div class="flex items-center gap-1.5 cursor-pointer">
                <Checkbox
                  checked={nodeProps.showGrid !== false}
                  onCheckedChange={(checked) => updateProp("showGrid", checked === true)}
                  aria-label={m.template_inspector_grid_lines()}
                />
                <span class="text-[11px]">{m.template_inspector_grid_lines()}</span>
              </div>

              <div class="flex items-center gap-1.5 cursor-pointer">
                <Checkbox
                  checked={nodeProps.showBorder !== false}
                  onCheckedChange={(checked) => updateProp("showBorder", checked === true)}
                  aria-label={m.template_inspector_outer_border()}
                />
                <span class="text-[11px]">{m.template_inspector_outer_border()}</span>
              </div>
            </div>

            <!-- Direction -->
            <div class="space-y-1 pt-1">
              <Label class="text-[11px] text-muted-foreground">{m.template_inspector_table_direction()}</Label>
              <div class="flex gap-2">
                <Button
                  variant={nodeProps.direction === "rtl" ? "default" : "outline"}
                  size="xs"
                  onclick={() => updateProp("direction", "rtl")}
                  class="grow h-7 text-xs cursor-pointer"
                >
                  {m.template_inspector_rtl()}
                </Button>
                <Button
                  variant={nodeProps.direction === "ltr" ? "default" : "outline"}
                  size="xs"
                  onclick={() => updateProp("direction", "ltr")}
                  class="grow h-7 text-xs cursor-pointer"
                >
                  {m.template_inspector_ltr()}
                </Button>
              </div>
            </div>
              </div>
            </details>

            <Separator />

            <!-- Columns Management -->
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <Label class="text-xs font-semibold text-foreground">{m.template_inspector_columns()}</Label>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={addColumn}
                  class="h-8 text-xs gap-1.5 cursor-pointer"
                >
                  <Plus class="size-3.5" />
                  {m.template_inspector_add_column()}
                </Button>
              </div>

              <details class="inspector-subsection">
                <summary>{m.template_inspector_column_display()} <span class="text-muted-foreground">({columns.length})</span></summary>
                <div class="inspector-subsection-body">
              <div class="space-y-2 max-h-72 overflow-y-auto pr-1">
                {#each columns as col, idx (col.id || idx)}
                  <div class="p-3 rounded-md border border-border bg-muted/20 space-y-2.5 text-xs">
                    <div class="flex items-center justify-between gap-1">
                      <span class="font-semibold text-foreground text-xs">#{idx + 1} {col.headerAr || col.header}</span>
                      <Button
                        variant="ghost"
                        size="icon"
                        onclick={() => removeColumn(idx)}
                        class="size-6 text-destructive hover:bg-destructive/10 cursor-pointer"
                      >
                        <Trash class="size-3" />
                      </Button>
                    </div>

                    <div class="grid grid-cols-2 gap-1.5">
                      <div>
                        <span class="text-[10px] text-muted-foreground">{m.template_inspector_arabic_heading()}</span>
                        <Input
                          type="text"
                          value={col.headerAr || ""}
                          placeholder={m.template_inspector_subject()}
                          onchange={(e) => updateColumn(idx, "headerAr", (e.target as HTMLInputElement).value)}
                          class="h-8 text-xs"
                        />
                      </div>
                      <div>
                        <span class="text-[10px] text-muted-foreground">{m.template_inspector_english_heading()}</span>
                        <Input
                          type="text"
                          value={col.header || ""}
                          placeholder="Subject"
                          onchange={(e) => updateColumn(idx, "header", (e.target as HTMLInputElement).value)}
                          class="h-8 text-xs"
                        />
                      </div>
                    </div>

                    <div class="grid grid-cols-2 gap-1.5">
                      <div>
                        <span class="text-[10px] text-muted-foreground">{m.template_inspector_field()}</span>
                        <Input
                          type="text"
                          value={col.field || col.key || ""}
                          placeholder="e.g. max_score"
                          onchange={(e) => updateColumn(idx, "field", (e.target as HTMLInputElement).value)}
                          class="h-8 text-xs font-mono"
                        />
                      </div>
                      <div>
                        <span class="text-[10px] text-muted-foreground">{m.template_inspector_width()}</span>
                        <Input
                          type="text"
                          value={col.width || "auto"}
                          placeholder="10% or auto"
                          onchange={(e) => updateColumn(idx, "width", (e.target as HTMLInputElement).value)}
                          class="h-8 text-xs font-mono"
                        />
                      </div>
                    </div>

                    <div class="grid grid-cols-2 gap-1.5">
                      <div>
                        <span class="text-[10px] text-muted-foreground">{m.template_inspector_alignment()}</span>
                        <PropertySelect
                          value={col.alignment || "center"}
                          options={[
                            { value: "center", label: m.template_inspector_center() },
                            { value: "right", label: m.template_inspector_right() },
                            { value: "left", label: m.template_inspector_left() },
                          ]}
                          onchange={(value) => updateColumn(idx, "alignment", value)}
                        />
                      </div>
                      <div>
                        <span class="text-[10px] text-muted-foreground">{m.template_inspector_total()}</span>
                        <PropertySelect
                          value={col.totalType || "none"}
                          options={[
                            { value: "none", label: m.template_inspector_none() },
                            { value: "sum", label: m.template_inspector_sum() },
                            { value: "avg", label: m.template_inspector_average() },
                            { value: "count", label: m.template_inspector_count() },
                            { value: "label", label: m.template_inspector_label() },
                          ]}
                          onchange={(value) => updateColumn(idx, "totalType", value)}
                        />
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
                </div>
              </details>
            </div>

            <Separator />

            <!-- Table Styling -->
            <details class="inspector-subsection">
              <summary>{m.template_inspector_formatting()}</summary>
              <div class="inspector-subsection-body">

              <div class="space-y-1">
                <span class="text-[11px] text-muted-foreground">{m.template_inspector_header_background()}</span>
                <ColorPicker
                  value={nodeProps.style?.headerBgColor || "#1e3a8a"}
                  onchange={(c) => updateTableStyle("headerBgColor", c)}
                />
              </div>

              <div class="space-y-1">
                <span class="text-[11px] text-muted-foreground">{m.template_inspector_text_color()}</span>
                <ColorPicker
                  value={nodeProps.style?.headerTextColor || "#ffffff"}
                  onchange={(c) => updateTableStyle("headerTextColor", c)}
                />
              </div>

              <div class="space-y-1">
                <span class="text-[11px] text-muted-foreground">{m.template_inspector_border_color()}</span>
                <ColorPicker
                  value={nodeProps.style?.borderColor || "#475569"}
                  onchange={(c) => updateTableStyle("borderColor", c)}
                />
              </div>

              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <span class="text-[11px] text-muted-foreground">{m.template_inspector_row_height()}</span>
                  <Input
                    type="number"
                    step="1"
                    value={nodeProps.style?.rowHeight ?? 20}
                    onchange={(e) => updateTableStyle("rowHeight", parseFloat((e.target as HTMLInputElement).value) || 20)}
                    class="h-7 text-xs"
                  />
                </div>
                <div>
                  <span class="text-[11px] text-muted-foreground">{m.template_inspector_heading_font_size()}</span>
                  <Input
                    type="number"
                    step="0.5"
                    value={nodeProps.style?.headerFontSize ?? 9}
                    onchange={(e) => updateTableStyle("headerFontSize", parseFloat((e.target as HTMLInputElement).value) || 9)}
                    class="h-7 text-xs"
                  />
                </div>
                </div>
              </div>
              </details>
            </div>

        <!-- ══════════════ TEXT NODE INSPECTOR ══════════════ -->
        {:else if node.type === "Text"}
          <div class="space-y-3">
            <Label class="text-xs font-semibold text-foreground">{m.template_inspector_content()}</Label>
            <Textarea
              value={nodeProps.content || ""}
              oninput={(e) => updateProp("content", (e.target as HTMLTextAreaElement).value)}
              rows={4}
              class="text-xs font-mono"
              placeholder={m.template_inspector_text_expression_placeholder()}
            />

            <div class="grid grid-cols-2 gap-2 text-xs">
              <div>
                <span class="text-[10px] text-muted-foreground">{m.template_inspector_font_size_label()}</span>
                <Input
                  type="number"
                  step="0.5"
                  value={nodeProps.fontSize ?? 10}
                  onchange={(e) => updateProp("fontSize", parseFloat((e.target as HTMLInputElement).value) || 10)}
                  class="h-7 text-xs"
                />
              </div>
              <div>
                <span class="text-[10px] text-muted-foreground">{m.template_inspector_font_family_label()}</span>
                <PropertySelect
                  value={nodeProps.fontFamily || "Amiri"}
                  class="h-7"
                  options={[
                    { value: "Amiri", label: m.template_inspector_amiri_arabic() },
                    { value: "Rubik", label: "Rubik" },
                    { value: "Tahoma", label: "Tahoma" },
                    { value: "Arial", label: "Arial" },
                  ]}
                  onchange={(value) => updateProp("fontFamily", value)}
                />
              </div>
            </div>

            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">{m.template_inspector_text_color_label()}</span>
              <ColorPicker
                value={nodeProps.color || "#000000"}
                onchange={(c) => updateProp("color", c)}
              />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_font_weight()}</Label>
                <PropertySelect
                  value={nodeProps.fontWeight || "normal"}
                  options={[
                    { value: "light", label: m.template_inspector_light() },
                    { value: "normal", label: m.template_inspector_normal() },
                    { value: "bold", label: m.template_inspector_bold() },
                  ]}
                  onchange={(value) => updateProp("fontWeight", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_text_alignment()}</Label>
                <PropertySelect
                  value={nodeProps.textAlign || "start"}
                  options={[
                    { value: "start", label: m.template_inspector_beginning() },
                    { value: "center", label: m.template_inspector_center() },
                    { value: "end", label: m.template_inspector_end() },
                    { value: "justify", label: m.template_inspector_justify() },
                  ]}
                  onchange={(value) => updateProp("textAlign", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_text_direction()}</Label>
                <PropertySelect
                  value={nodeProps.direction || "auto"}
                  options={[
                    { value: "auto", label: m.template_inspector_automatic() },
                    { value: "rtl", label: m.template_inspector_right_to_left() },
                    { value: "ltr", label: m.template_inspector_left_to_right() },
                  ]}
                  onchange={(value) => updateProp("direction", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_font_style()}</Label>
                <PropertySelect
                  value={nodeProps.fontStyle || "normal"}
                  options={[
                    { value: "normal", label: m.template_inspector_normal() },
                    { value: "italic", label: m.template_inspector_italic() },
                  ]}
                  onchange={(value) => updateProp("fontStyle", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_text_decoration()}</Label>
                <PropertySelect
                  value={nodeProps.textDecoration || "none"}
                  options={[
                    { value: "none", label: m.template_inspector_none() },
                    { value: "underline", label: m.template_inspector_underline() },
                    { value: "line-through", label: m.template_inspector_line_through() },
                  ]}
                  onchange={(value) => updateProp("textDecoration", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_letter_case()}</Label>
                <PropertySelect
                  value={nodeProps.textTransform || "none"}
                  options={[
                    { value: "none", label: m.template_inspector_unchanged_case() },
                    { value: "uppercase", label: m.template_inspector_uppercase() },
                    { value: "lowercase", label: m.template_inspector_lowercase() },
                  ]}
                  onchange={(value) => updateProp("textTransform", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_line_height()}</Label>
                <Input type="number" min="0.5" step="0.1" value={nodeProps.lineHeight ?? 1.2} onchange={(e) => updateProp("lineHeight", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_letter_spacing()}</Label>
                <Input type="number" step="0.1" value={nodeProps.letterSpacing ?? 0} onchange={(e) => updateProp("letterSpacing", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_max_lines()}</Label>
                <Input type="number" min="1" step="1" value={nodeProps.maxLines ?? ""} onchange={(e) => updateProp("maxLines", Number((e.target as HTMLInputElement).value) || undefined)} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_content_overflow()}</Label>
                <PropertySelect
                  value={nodeProps.overflow || "visible"}
                  options={[
                    { value: "visible", label: m.template_inspector_visible() },
                    { value: "hidden", label: m.template_inspector_hide() },
                    { value: "ellipsis", label: m.template_inspector_ellipsis() },
                  ]}
                  onchange={(value) => updateProp("overflow", value)}
                />
              </div>
            </div>
            <div class="space-y-1">
              <Label class="property-label">{m.template_inspector_background_color()}</Label>
              <ColorPicker value={nodeProps.backgroundColor || "#ffffff"} onchange={(c) => updateProp("backgroundColor", c)} />
            </div>
          </div>

        {:else if node.type === "Image"}
          <div class="space-y-3">
            <div class="flex items-center justify-between gap-3">
              <div class="min-w-0">
                <Label class="property-label">{m.template_inspector_image()}</Label>
                <p class="text-[10px] text-muted-foreground">
                  {nodeProps.assetRef ? m.template_inspector_image_media_library() : m.template_inspector_custom_image()}
                </p>
              </div>
              <Button
                variant="outline"
                size="sm"
                class="shrink-0"
                onclick={triggerImageReplace}
                disabled={imageUploadMutation.isPending}
              >
                <ArrowUpTray size={14} />
                {imageUploadMutation.isPending ? m.template_inspector_replacing_image() : m.template_inspector_replace_image()}
              </Button>
              <input
                type="file"
                bind:this={imageFileInput}
                class="hidden"
                accept="image/*"
                onchange={handleImageFileSelect}
              />
            </div>
            <div class="space-y-1">
              <Label class="property-label">{m.template_inspector_image_source_expression()}</Label>
              <Input value={nodeProps.expression || ""} placeholder={"{{image_url}}"} onchange={(e) => updateProp("expression", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_image_fit()}</Label>
                <PropertySelect
                  value={nodeProps.fit || "contain"}
                  options={[
                    { value: "contain", label: m.template_inspector_fit_contain() },
                    { value: "cover", label: m.template_inspector_fit_cover() },
                    { value: "fill", label: m.template_inspector_fit_fill() },
                    { value: "none", label: m.template_inspector_fit_original() },
                  ]}
                  onchange={(value) => updateProp("fit", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_image_layer()}</Label>
                <PropertySelect
                  value={nodeProps.layer || "foreground"}
                  options={[
                    { value: "foreground", label: m.template_inspector_foreground() },
                    { value: "background", label: m.template_inspector_background() },
                  ]}
                  onchange={(value) => updateProp("layer", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_corner_radius()}</Label><Input type="number" min="0" step="0.5" value={nodeProps.borderRadius ?? 0} onchange={(e) => updateProp("borderRadius", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_border_width()}</Label><Input type="number" min="0" step="0.25" value={nodeProps.borderWidth ?? 0} onchange={(e) => updateProp("borderWidth", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            </div>
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_border_color()}</Label><ColorPicker value={nodeProps.borderColor || "#000000"} onchange={(c) => updateProp("borderColor", c)} /></div>
          </div>

        <!-- ══════════════ SHAPE NODE INSPECTOR ══════════════ -->
        {:else if node.type === "Shape"}
          <div class="space-y-3">
            <Label class="text-xs font-semibold text-foreground">{m.template_inspector_fill_and_border()}</Label>
            <div class="space-y-1">
              <Label class="property-label">{m.template_inspector_shape_type()}</Label>
              <PropertySelect
                value={nodeProps.shapeType || nodeProps.shape || "rectangle"}
                options={[
                  { value: "rectangle", label: m.template_inspector_shape_rectangle() },
                  { value: "rounded-rect", label: m.template_inspector_shape_rounded_rectangle() },
                  { value: "circle", label: m.template_inspector_shape_circle() },
                  { value: "ellipse", label: m.template_inspector_shape_ellipse() },
                  { value: "line", label: m.template_inspector_shape_line() },
                ]}
                onchange={(value) => updateProp("shapeType", value)}
              />
            </div>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">{m.template_inspector_fill_color()}</span>
              <ColorPicker
                value={nodeProps.fill || nodeProps.fillColor || "#ffffff"}
                onchange={(c) => updateProp("fill", c)}
              />
            </div>

            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">{m.template_inspector_stroke_color()}</span>
              <ColorPicker
                value={nodeProps.stroke || nodeProps.strokeColor || "#000000"}
                onchange={(c) => updateProp("stroke", c)}
              />
            </div>

            <div class="grid grid-cols-2 gap-2 text-xs">
              <div>
                <span class="text-[10px] text-muted-foreground">{m.template_inspector_border_width_label()}</span>
                <Input
                  type="number"
                  step="0.25"
                  value={nodeProps.strokeWidth ?? 1}
                  onchange={(e) => updateProp("strokeWidth", parseFloat((e.target as HTMLInputElement).value) || 0)}
                  class="h-7 text-xs"
                />
              </div>
              <div>
                <span class="text-[10px] text-muted-foreground">{m.template_inspector_corner_radius_label()}</span>
                <Input
                  type="number"
                  step="0.5"
                  value={nodeProps.borderRadius ?? 0}
                  onchange={(e) => updateProp("borderRadius", parseFloat((e.target as HTMLInputElement).value) || 0)}
                  class="h-7 text-xs"
                />
              </div>
            </div>
          </div>

        <!-- ══════════════ QR CODE / BARCODE INSPECTOR ══════════════ -->
        {:else if node.type === "QRCode" || node.type === "Barcode"}
          <div class="space-y-3">
            <Label class="text-xs font-semibold text-foreground">{m.template_inspector_code_data()}</Label>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">{m.template_inspector_data_or_link()}</span>
              <Input
                type="text"
                value={nodeProps.data || ""}
                placeholder={"https://... or {{student_id}}"}
                onchange={(e) => updateProp("data", (e.target as HTMLInputElement).value)}
                class="h-7 text-xs font-mono"
              />
            </div>
            {#if node.type === "QRCode"}
              <div class="grid grid-cols-2 gap-2">
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_size()}</Label><Input type="number" min="5" value={nodeProps.size ?? pos.width ?? 30} onchange={(e) => updateProp("size", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_error_correction()}</Label>
                  <PropertySelect
                    value={nodeProps.errorCorrection || "M"}
                    options={[
                      { value: "L", label: m.template_inspector_low() },
                      { value: "M", label: m.template_inspector_medium() },
                      { value: "Q", label: m.template_inspector_good() },
                      { value: "H", label: m.template_inspector_high() },
                    ]}
                    onchange={(value) => updateProp("errorCorrection", value)}
                  />
                </div>
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_quiet_zone()}</Label><Input type="number" min="0" value={nodeProps.quietZone ?? 4} onchange={(e) => updateProp("quietZone", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_logo_size_percent()}</Label><Input type="number" min="0" max="40" value={nodeProps.logoSize ?? 20} onchange={(e) => updateProp("logoSize", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              </div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_logo_reference()}</Label><Input value={nodeProps.logoAssetRef || ""} placeholder={m.template_inspector_media_file_id_placeholder()} onchange={(e) => updateProp("logoAssetRef", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            {:else}
              <div class="grid grid-cols-2 gap-2">
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_format()}</Label>
                  <PropertySelect
                    value={nodeProps.format || "code128"}
                    options={[
                      { value: "code128", label: "Code 128" },
                      { value: "code39", label: "Code 39" },
                      { value: "ean13", label: "EAN-13" },
                      { value: "ean8", label: "EAN-8" },
                      { value: "upc_a", label: "UPC-A" },
                      { value: "itf", label: "ITF" },
                      { value: "codabar", label: "Codabar" },
                    ]}
                    onchange={(value) => updateProp("format", value)}
                  />
                </div>
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_symbol_height()}</Label><Input type="number" min="5" value={nodeProps.height ?? pos.height ?? 20} onchange={(e) => updateProp("height", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_module_width()}</Label><Input type="number" min="0.1" step="0.1" value={nodeProps.moduleWidth ?? 0.3} onchange={(e) => updateProp("moduleWidth", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_code_text_size()}</Label><Input type="number" min="1" value={nodeProps.textFontSize ?? 8} onchange={(e) => updateProp("textFontSize", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              </div>
              <div class="flex items-center gap-2 text-xs">
                <Checkbox
                  checked={nodeProps.showText !== false}
                  onCheckedChange={(checked) => updateProp("showText", checked === true)}
                  aria-label={m.template_inspector_show_data_below_symbol()}
                />
                <span>{m.template_inspector_show_data_below_symbol()}</span>
              </div>
            {/if}
            <div class="grid grid-cols-2 gap-3">
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_symbol_color()}</Label><ColorPicker value={nodeProps.color || "#000000"} onchange={(c) => updateProp("color", c)} /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_background_color()}</Label><ColorPicker value={nodeProps.backgroundColor || "#ffffff"} onchange={(c) => updateProp("backgroundColor", c)} /></div>
            </div>
          </div>
        {:else if node.type === "Separator"}
          <div class="grid grid-cols-2 gap-2">
            <div class="space-y-1">
              <Label class="property-label">{m.template_inspector_direction()}</Label>
              <PropertySelect
                value={nodeProps.direction || "horizontal"}
                options={[
                  { value: "horizontal", label: m.template_orientation_horz() },
                  { value: "vertical", label: m.template_orientation_vert() },
                ]}
                onchange={(value) => updateProp("direction", value)}
              />
            </div>
            <div class="space-y-1">
              <Label class="property-label">{m.template_inspector_style()}</Label>
              <PropertySelect
                value={nodeProps.style || "solid"}
                options={[
                  { value: "solid", label: m.template_inspector_solid() },
                  { value: "dashed", label: m.template_inspector_dashed() },
                  { value: "dotted", label: m.template_inspector_dotted() },
                ]}
                onchange={(value) => updateProp("style", value)}
              />
            </div>
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_thickness()}</Label><Input type="number" min="0.1" step="0.1" value={nodeProps.thickness ?? 1} onchange={(e) => updateProp("thickness", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_length()}</Label><Input type="number" min="1" value={nodeProps.length ?? pos.width} onchange={(e) => updateProp("length", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            <div class="col-span-2 space-y-1"><Label class="property-label">{m.template_inspector_color()}</Label><ColorPicker value={nodeProps.color || "#cbd5e1"} onchange={(c) => updateProp("color", c)} /></div>
          </div>
        {:else if node.type === "Spacer"}
          <div class="grid grid-cols-2 gap-2">
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_width()}</Label><Input type="number" min="0" value={nodeProps.width ?? pos.width} onchange={(e) => updateProp("width", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_height()}</Label><Input type="number" min="0" value={nodeProps.height ?? pos.height} onchange={(e) => updateProp("height", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
          </div>
        {:else if node.type === "Frame" || node.type === "Grid"}
          <div class="space-y-3">
            {#if node.type === "Frame"}
              <div class="grid grid-cols-2 gap-2">
                <div class="space-y-1">
                  <Label class="property-label">{m.template_inspector_layout_direction()}</Label>
                  <PropertySelect
                    value={nodeProps.layout || "vertical"}
                    options={[
                      { value: "vertical", label: m.template_orientation_vert() },
                      { value: "horizontal", label: m.template_orientation_horz() },
                    ]}
                    onchange={(value) => updateProp("layout", value)}
                  />
                </div>
                <div class="space-y-1">
                  <Label class="property-label">{m.template_inspector_alignment()}</Label>
                  <PropertySelect
                    value={nodeProps.alignment || "start"}
                    options={[
                      { value: "start", label: m.template_inspector_start() },
                      { value: "center", label: m.template_inspector_center() },
                      { value: "end", label: m.template_inspector_end() },
                      { value: "space-between", label: m.template_inspector_distribute() },
                      { value: "space-around", label: m.template_inspector_equal_spacing() },
                    ]}
                    onchange={(value) => updateProp("alignment", value)}
                  />
                </div>
                <div class="space-y-1">
                  <Label class="property-label">{m.template_inspector_align_items()}</Label>
                  <PropertySelect
                    value={nodeProps.crossAlignment || "stretch"}
                    options={[
                      { value: "start", label: m.template_inspector_start() },
                      { value: "center", label: m.template_inspector_center() },
                      { value: "end", label: m.template_inspector_end() },
                      { value: "stretch", label: m.template_inspector_stretch() },
                    ]}
                    onchange={(value) => updateProp("crossAlignment", value)}
                  />
                </div>
                <div class="flex items-center gap-2 text-xs">
                  <Checkbox
                    checked={nodeProps.wrap === true}
                    onCheckedChange={(checked) => updateProp("wrap", checked === true)}
                    aria-label={m.template_inspector_wrap_items()}
                  />
                  <span>{m.template_inspector_wrap_items()}</span>
                </div>
              </div>
            {:else}
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_columns_example()}</Label><Input value={(nodeProps.columns || []).join(", ")} onchange={(e) => updateProp("columns", (e.target as HTMLInputElement).value.split(",").map((part) => part.trim()).filter(Boolean))} class="h-8 text-xs font-mono" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_optional_rows()}</Label><Input value={(nodeProps.rows || []).join(", ")} onchange={(e) => updateProp("rows", (e.target as HTMLInputElement).value.split(",").map((part) => part.trim()).filter(Boolean))} class="h-8 text-xs font-mono" /></div>
            {/if}
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_item_spacing()}</Label><Input type="number" min="0" value={nodeProps.gap ?? 0} onchange={(e) => updateProp("gap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_border_width()}</Label><Input type="number" min="0" value={nodeProps.borderWidth ?? 0} onchange={(e) => updateProp("borderWidth", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              {#if node.type === "Grid"}
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_column_gap()}</Label><Input type="number" min="0" value={nodeProps.columnGap ?? nodeProps.gap ?? 0} onchange={(e) => updateProp("columnGap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">{m.template_inspector_row_gap()}</Label><Input type="number" min="0" value={nodeProps.rowGap ?? nodeProps.gap ?? 0} onchange={(e) => updateProp("rowGap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              {/if}
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_inner_padding()}</Label><Input type="number" min="0" value={typeof nodeProps.padding === "number" ? nodeProps.padding : nodeProps.padding?.top ?? 0} onchange={(e) => { const value = Number((e.target as HTMLInputElement).value); updateProp("padding", { top: value, right: value, bottom: value, left: value }); }} class="h-8 text-xs" /></div>
            </div>
            <div class="grid grid-cols-2 gap-3">
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_background_color()}</Label><ColorPicker value={nodeProps.backgroundColor || "#ffffff"} onchange={(c) => updateProp("backgroundColor", c)} /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_border_color()}</Label><ColorPicker value={nodeProps.borderColor || "#cbd5e1"} onchange={(c) => updateProp("borderColor", c)} /></div>
            </div>
          </div>
        {:else if node.type === "Repeater"}
          <div class="space-y-3">
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_repeat_source()}</Label><Input value={nodeProps.dataSource || ""} onchange={(e) => updateProp("dataSource", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_item_name()}</Label><Input value={nodeProps.itemVariable || "item"} onchange={(e) => updateProp("itemVariable", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_index_name()}</Label><Input value={nodeProps.indexVariable || "index"} onchange={(e) => updateProp("indexVariable", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_repeat_direction()}</Label>
                <PropertySelect
                  value={nodeProps.layout || "vertical"}
                  options={[
                    { value: "vertical", label: m.template_orientation_vert() },
                    { value: "horizontal", label: m.template_orientation_horz() },
                    { value: "grid", label: m.template_inspector_grid() },
                  ]}
                  onchange={(value) => updateProp("layout", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_column_count()}</Label><Input type="number" min="1" value={nodeProps.columns ?? 2} onchange={(e) => updateProp("columns", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_sort_by()}</Label><Input value={nodeProps.sortBy || ""} onchange={(e) => updateProp("sortBy", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1">
                <Label class="property-label">{m.template_inspector_sort_direction()}</Label>
                <PropertySelect
                  value={nodeProps.sortDirection || "asc"}
                  options={[
                    { value: "asc", label: m.template_inspector_ascending() },
                    { value: "desc", label: m.template_inspector_descending() },
                  ]}
                  onchange={(value) => updateProp("sortDirection", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_spacing()}</Label><Input type="number" min="0" value={nodeProps.gap ?? 0} onchange={(e) => updateProp("gap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_maximum_limit()}</Label><Input type="number" min="1" value={nodeProps.limit ?? ""} onchange={(e) => updateProp("limit", Number((e.target as HTMLInputElement).value) || undefined)} class="h-8 text-xs" /></div>
            </div>
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_filter_condition()}</Label><Input value={nodeProps.filter || ""} onchange={(e) => updateProp("filter", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_empty_list_message()}</Label><Input value={nodeProps.emptyMessage || ""} onchange={(e) => updateProp("emptyMessage", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
          </div>
        {:else if node.type === "Conditional"}
          <div class="space-y-1">
            <Label class="property-label">{m.template_inspector_display_condition()}</Label>
            <Textarea
              value={nodeProps.condition || ""}
              oninput={(e) => updateProp("condition", (e.target as HTMLTextAreaElement).value)}
              rows={3}
              class={compactTextareaClass}
            />
          </div>
        {:else if node.type === "ComponentRef"}
          <div class="space-y-1"><Label class="property-label">{m.template_inspector_component_id()}</Label><Input value={nodeProps.componentId || ""} onchange={(e) => updateProp("componentId", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
        {:else if node.type === "Header" || node.type === "Footer"}
          <div class="space-y-3">
            <div class="space-y-1"><Label class="property-label">{m.template_inspector_height()}</Label><Input type="number" min="1" value={nodeProps.height ?? 20} onchange={(e) => updateProp("height", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            {#if node.type === "Footer"}
              <div class="flex items-center gap-2 text-xs">
                <Checkbox
                  checked={nodeProps.showPageNumber === true}
                  onCheckedChange={(checked) => updateProp("showPageNumber", checked === true)}
                  aria-label={m.template_inspector_show_page_number()}
                />
                <span>{m.template_inspector_show_page_number()}</span>
              </div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_page_number_format()}</Label><Input value={nodeProps.pageNumberFormat || "{{page}}/{{total}}"} onchange={(e) => updateProp("pageNumberFormat", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            {/if}
          </div>
        {:else if node.type === "Watermark"}
          <div class="space-y-3">
            <div class="space-y-1">
              <Label class="property-label">{m.template_inspector_watermark_type()}</Label>
              <PropertySelect
                value={nodeProps.watermarkType || "text"}
                options={[
                  { value: "text", label: m.template_element_text() },
                  { value: "image", label: m.template_element_image() },
                ]}
                onchange={(value) => updateProp("watermarkType", value)}
              />
            </div>
            {#if nodeProps.watermarkType !== "image"}
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_watermark_text()}</Label><Input value={nodeProps.text || ""} onchange={(e) => updateProp("text", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_text_color()}</Label><ColorPicker value={nodeProps.color || "#94a3b8"} onchange={(c) => updateProp("color", c)} /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_font_size()}</Label><Input type="number" min="1" value={nodeProps.fontSize ?? 36} onchange={(e) => updateProp("fontSize", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            {:else}
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_image_id()}</Label><Input value={nodeProps.assetRef || ""} onchange={(e) => updateProp("assetRef", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            {/if}
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_opacity()}</Label><Input type="number" min="0" max="1" step="0.05" value={nodeProps.opacity ?? 0.2} onchange={(e) => updateProp("opacity", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">{m.template_inspector_rotation()}</Label><Input type="number" value={nodeProps.rotation ?? 0} onchange={(e) => updateProp("rotation", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            </div>
          </div>
        {:else if node.type === "PageBreak"}
          <p class="text-xs leading-5 text-muted-foreground">{m.template_inspector_page_break_print_hint()}</p>
        {/if}

          </div>
        </details>

        <!-- Actions -->
        <div class="inspector-actions">
          <Button
            variant="destructive"
            size="sm"
            onclick={() => store.deleteSelected()}
            class="grow"
          >
            <Trash class="size-4" />
            {m.template_inspector_delete_element()}
          </Button>
          <Button
            variant="outline"
            size="sm"
            onclick={() => store.duplicateSelected()}
            class="grow"
          >
            {m.template_inspector_duplicate_element()}
          </Button>
        </div>
      {/if}
    </div>
  </ScrollArea>
</aside>

<style>
  .properties-inspector {
    position: relative;
    width: 300px;
    height: 100%;
    max-width: min(440px, 40vw);
    background: var(--color-card);
    border-inline-start: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    flex: 0 0 auto;
    z-index: 10;
  }

  :global(.resize-handle) {
    position: absolute;
    z-index: 2;
    inset-block: 0;
    inset-inline-start: -3px;
    width: 6px;
    border: 0;
    background: transparent;
    cursor: col-resize;
    touch-action: none;
  }

  :global(.resize-handle)::after {
    position: absolute;
    inset-block: 0;
    inset-inline-start: 2px;
    width: 1px;
    background: transparent;
    content: "";
    transition: background 0.15s ease;
  }

  :global(.resize-handle:hover)::after,
  :global(.resize-handle:focus-visible)::after {
    background: var(--color-primary);
  }

  .inspector-header {
    min-height: 52px;
    padding: 8px 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .inspector-title {
    margin: 0;
    color: var(--color-foreground);
    font-size: 12px;
    font-weight: 700;
  }

  :global(.inspector-type) {
    max-width: 90px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 10px;
  }

  :global(.inspector-scroll) {
    flex-grow: 1;
    overflow-y: auto;
  }

  .inspector-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 10px 12px;
  }

  .page-inspector {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .page-summary {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 36px;
    color: var(--color-muted-foreground);
    font-size: 11px;
  }

  .selected-layer {
    display: flex;
    align-items: center;
    gap: 7px;
    min-width: 0;
    padding: 1px 0 3px;
  }

  .selected-layer-name {
    overflow: hidden;
    color: var(--color-foreground);
    font-size: 11px;
    font-weight: 500;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .property-section {
    overflow: hidden;
    border-bottom: 1px solid var(--color-border);
  }

  .property-section > summary {
    display: flex;
    min-height: 38px;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 0 2px;
    color: var(--color-foreground);
    cursor: pointer;
    font-size: 12px;
    font-weight: 600;
    list-style: none;
    user-select: none;
  }

  .property-section > summary::-webkit-details-marker {
    display: none;
  }

  .property-section > summary::after,
  .inspector-subsection > summary::after {
    width: 7px;
    height: 7px;
    flex: 0 0 auto;
    border-inline-end: 1px solid var(--color-muted-foreground);
    border-bottom: 1px solid var(--color-muted-foreground);
    content: "";
    transform: rotate(45deg) translateY(-2px);
    transition: transform 0.15s ease;
  }

  .property-section:not([open]) > summary::after,
  .inspector-subsection:not([open]) > summary::after {
    transform: rotate(135deg);
  }

  .section-unit {
    color: var(--color-muted-foreground);
    font-size: 10px;
    font-weight: 500;
  }

  .property-section-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 4px 0 12px;
  }

  .property-label {
    color: var(--color-muted-foreground);
    font-size: 11px;
  }

  .inspector-subsection {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding-inline: 10px;
  }

  .inspector-subsection > summary {
    display: flex;
    min-height: 38px;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    color: var(--color-foreground);
    cursor: pointer;
    font-size: 11px;
    font-weight: 500;
    list-style: none;
  }

  .inspector-subsection > summary::-webkit-details-marker {
    display: none;
  }

  .inspector-subsection-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding-bottom: 10px;
  }

  .inspector-actions {
    display: flex;
    gap: 6px;
    padding-top: 4px;
  }
</style>
