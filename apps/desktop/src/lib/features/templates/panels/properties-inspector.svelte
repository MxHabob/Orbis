<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { EditorStore } from "../stores/editor-store.svelte";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Separator } from "@orbis/ui/separator";
import { ScrollArea } from "@orbis/ui/scroll-area";
import ColorPicker from "../shared/color-picker.svelte";
import {
  AdjustmentsHorizontal,
  Bars3BottomLeft,
  ChevronDown,
  ChevronRight,
  Code,
  Hashtag,
  Plus,
  Trash,
} from "@orbis/icons";

let {
  store,
  onclose,
}: {
  store: EditorStore;
  onclose?: () => void;
} = $props();

const node = $derived(store.selectedNode);
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
  if ("props" in node && (node as any).props?.position) {
    store.updateSelectedProps({ position: currentPos });
  } else {
    store.moveNodes([{ id: node.id, x: currentPos.x, y: currentPos.y }]);
  }
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

<aside class="properties-inspector" aria-label="Properties Inspector">
  <div class="inspector-header">
    <div class="flex items-center gap-2">
      <AdjustmentsHorizontal class="size-4 text-primary" />
      <span class="text-xs font-bold text-foreground">خصائص العنصر | Properties</span>
    </div>
    {#if node}
      <Badge variant="outline" class="text-[10px] uppercase font-mono px-1.5 py-0">
        {node.type}
      </Badge>
    {/if}
  </div>

  <ScrollArea class="inspector-scroll">
    <div class="p-3.5 space-y-4">
      {#if !node}
        <!-- No element selected → Page properties summary -->
        <div class="text-center py-6 text-muted-foreground space-y-2">
          <p class="text-xs">لم يتم تحديد أي عنصر</p>
          <p class="text-[11px] text-muted-foreground/80">انقر على أي عنصر في مساحة العمل لتعديل خصائصه وبياناته.</p>
        </div>
      {:else}
        <!-- Element Meta & Position -->
        <div class="space-y-2">
          <Label class="text-[11px] font-semibold text-muted-foreground">الموضع والحجم (ملم)</Label>
          <div class="grid grid-cols-2 gap-2 text-xs">
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">X:</span>
              <Input
                type="number"
                step="0.5"
                value={pos.x ?? 0}
                onchange={(e) => updatePosition("x", parseFloat((e.target as HTMLInputElement).value) || 0)}
                class="h-7 text-xs font-mono"
              />
            </div>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">Y:</span>
              <Input
                type="number"
                step="0.5"
                value={pos.y ?? 0}
                onchange={(e) => updatePosition("y", parseFloat((e.target as HTMLInputElement).value) || 0)}
                class="h-7 text-xs font-mono"
              />
            </div>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">العرض:</span>
              <Input
                type="text"
                value={pos.width ?? "100%"}
                onchange={(e) => updatePosition("width", (e.target as HTMLInputElement).value)}
                class="h-7 text-xs font-mono"
              />
            </div>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">الارتفاع:</span>
              <Input
                type="text"
                value={pos.height ?? "auto"}
                onchange={(e) => updatePosition("height", (e.target as HTMLInputElement).value)}
                class="h-7 text-xs font-mono"
              />
            </div>
          </div>
        </div>

        <Separator />

        <!-- ══════════════ TABLE NODE INSPECTOR ══════════════ -->
        {#if node.type === "Table"}
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <Label class="text-xs font-bold text-foreground">بيانات الجدول | Data Binding</Label>
              <Button
                variant="ghost"
                size="xs"
                onclick={applyYemeniCertificateColumnsPreset}
                class="text-[10px] h-6 text-primary cursor-pointer hover:bg-primary/10"
                title="تطبيق هيكل أعمدة شهادة الدرجات اليمنية المعتمدة"
              >
                نموذج الشهادة المعتمدة
              </Button>
            </div>

            <!-- Data Source Field -->
            <div class="space-y-1">
              <Label class="text-[11px] text-muted-foreground">مصدر البيانات (Collection Path):</Label>
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
              <label class="flex items-center gap-1.5 cursor-pointer">
                <input
                  type="checkbox"
                  checked={nodeProps.showHeader !== false}
                  onchange={(e) => updateProp("showHeader", (e.target as HTMLInputElement).checked)}
                  class="rounded text-primary focus:ring-primary size-3.5"
                />
                <span class="text-[11px]">إظهار الترويسة</span>
              </label>

              <label class="flex items-center gap-1.5 cursor-pointer">
                <input
                  type="checkbox"
                  checked={nodeProps.showTotals === true}
                  onchange={(e) => updateProp("showTotals", (e.target as HTMLInputElement).checked)}
                  class="rounded text-primary focus:ring-primary size-3.5"
                />
                <span class="text-[11px]">صف المجموع العام</span>
              </label>

              <label class="flex items-center gap-1.5 cursor-pointer">
                <input
                  type="checkbox"
                  checked={nodeProps.showGrid !== false}
                  onchange={(e) => updateProp("showGrid", (e.target as HTMLInputElement).checked)}
                  class="rounded text-primary focus:ring-primary size-3.5"
                />
                <span class="text-[11px]">خطوط الشبكة</span>
              </label>

              <label class="flex items-center gap-1.5 cursor-pointer">
                <input
                  type="checkbox"
                  checked={nodeProps.showBorder !== false}
                  onchange={(e) => updateProp("showBorder", (e.target as HTMLInputElement).checked)}
                  class="rounded text-primary focus:ring-primary size-3.5"
                />
                <span class="text-[11px]">إطار خارجي</span>
              </label>
            </div>

            <!-- Direction -->
            <div class="space-y-1 pt-1">
              <Label class="text-[11px] text-muted-foreground">اتجاه الجدول:</Label>
              <div class="flex gap-2">
                <Button
                  variant={nodeProps.direction === "rtl" ? "default" : "outline"}
                  size="xs"
                  onclick={() => updateProp("direction", "rtl")}
                  class="grow h-7 text-xs cursor-pointer"
                >
                  RTL (من اليمين)
                </Button>
                <Button
                  variant={nodeProps.direction === "ltr" ? "default" : "outline"}
                  size="xs"
                  onclick={() => updateProp("direction", "ltr")}
                  class="grow h-7 text-xs cursor-pointer"
                >
                  LTR (من اليسار)
                </Button>
              </div>
            </div>

            <Separator />

            <!-- Columns Management -->
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <Label class="text-xs font-bold text-foreground">أعمدة الجدول ({columns.length})</Label>
                <Button
                  variant="outline"
                  size="xs"
                  onclick={addColumn}
                  class="h-6 text-[10px] gap-1 cursor-pointer"
                >
                  <Plus class="size-3" />
                  إضافة عمود
                </Button>
              </div>

              <div class="space-y-2 max-h-72 overflow-y-auto pr-1">
                {#each columns as col, idx (col.id || idx)}
                  <div class="p-2.5 rounded-md border border-border bg-card/60 space-y-2 text-xs">
                    <div class="flex items-center justify-between gap-1">
                      <span class="font-bold text-primary text-[11px]">#{idx + 1} {col.headerAr || col.header}</span>
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
                        <span class="text-[9px] text-muted-foreground">العنوان العربي:</span>
                        <Input
                          type="text"
                          value={col.headerAr || ""}
                          placeholder="المادة"
                          onchange={(e) => updateColumn(idx, "headerAr", (e.target as HTMLInputElement).value)}
                          class="h-6 text-[11px]"
                        />
                      </div>
                      <div>
                        <span class="text-[9px] text-muted-foreground">العنوان الإنجليزي:</span>
                        <Input
                          type="text"
                          value={col.header || ""}
                          placeholder="Subject"
                          onchange={(e) => updateColumn(idx, "header", (e.target as HTMLInputElement).value)}
                          class="h-6 text-[11px]"
                        />
                      </div>
                    </div>

                    <div class="grid grid-cols-2 gap-1.5">
                      <div>
                        <span class="text-[9px] text-muted-foreground">الحقل (Field Key):</span>
                        <Input
                          type="text"
                          value={col.field || col.key || ""}
                          placeholder="e.g. max_score"
                          onchange={(e) => updateColumn(idx, "field", (e.target as HTMLInputElement).value)}
                          class="h-6 text-[11px] font-mono"
                        />
                      </div>
                      <div>
                        <span class="text-[9px] text-muted-foreground">العرض:</span>
                        <Input
                          type="text"
                          value={col.width || "auto"}
                          placeholder="10% or auto"
                          onchange={(e) => updateColumn(idx, "width", (e.target as HTMLInputElement).value)}
                          class="h-6 text-[11px] font-mono"
                        />
                      </div>
                    </div>

                    <div class="grid grid-cols-2 gap-1.5">
                      <div>
                        <span class="text-[9px] text-muted-foreground">المحاذاة:</span>
                        <select
                          value={col.alignment || "center"}
                          onchange={(e) => updateColumn(idx, "alignment", (e.target as HTMLSelectElement).value)}
                          class="w-full h-6 rounded border border-input bg-background px-1.5 text-[10px]"
                        >
                          <option value="center">وسط (Center)</option>
                          <option value="right">يمين (Right)</option>
                          <option value="left">يسار (Left)</option>
                        </select>
                      </div>
                      <div>
                        <span class="text-[9px] text-muted-foreground">المجموع (Total):</span>
                        <select
                          value={col.totalType || "none"}
                          onchange={(e) => updateColumn(idx, "totalType", (e.target as HTMLSelectElement).value)}
                          class="w-full h-6 rounded border border-input bg-background px-1.5 text-[10px]"
                        >
                          <option value="none">بدون (None)</option>
                          <option value="sum">مجموع (Sum)</option>
                          <option value="avg">متوسط (Avg)</option>
                          <option value="count">عدد (Count)</option>
                          <option value="label">تسمية (Label)</option>
                        </select>
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            </div>

            <Separator />

            <!-- Table Styling -->
            <div class="space-y-2">
              <Label class="text-xs font-bold text-foreground">تنسيق وألوان الجدول | Styling</Label>

              <div class="space-y-1">
                <span class="text-[10px] text-muted-foreground">لون خلفية الترويسة:</span>
                <ColorPicker
                  value={nodeProps.style?.headerBgColor || "#1e3a8a"}
                  onchange={(c) => updateTableStyle("headerBgColor", c)}
                />
              </div>

              <div class="space-y-1">
                <span class="text-[10px] text-muted-foreground">لون خط الترويسة:</span>
                <ColorPicker
                  value={nodeProps.style?.headerTextColor || "#ffffff"}
                  onchange={(c) => updateTableStyle("headerTextColor", c)}
                />
              </div>

              <div class="space-y-1">
                <span class="text-[10px] text-muted-foreground">لون الإطار والخطوط:</span>
                <ColorPicker
                  value={nodeProps.style?.borderColor || "#475569"}
                  onchange={(c) => updateTableStyle("borderColor", c)}
                />
              </div>

              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <span class="text-[10px] text-muted-foreground">ارتفاع الصف:</span>
                  <Input
                    type="number"
                    step="1"
                    value={nodeProps.style?.rowHeight ?? 20}
                    onchange={(e) => updateTableStyle("rowHeight", parseFloat((e.target as HTMLInputElement).value) || 20)}
                    class="h-7 text-xs"
                  />
                </div>
                <div>
                  <span class="text-[10px] text-muted-foreground">حجم خط الترويسة:</span>
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
          </div>

        <!-- ══════════════ TEXT NODE INSPECTOR ══════════════ -->
        {:else if node.type === "Text"}
          <div class="space-y-3">
            <Label class="text-xs font-bold text-foreground">محتوى النص | Text Content</Label>
            <textarea
              value={nodeProps.content || ""}
              oninput={(e) => updateProp("content", (e.target as HTMLTextAreaElement).value)}
              rows="4"
              class="w-full rounded-md border border-input bg-background p-2 text-xs font-mono focus:ring-1 focus:ring-primary"
              placeholder="اكتب النص أو التعبير (مثل: {{student_name}})"
            ></textarea>

            <div class="grid grid-cols-2 gap-2 text-xs">
              <div>
                <span class="text-[10px] text-muted-foreground">حجم الخط:</span>
                <Input
                  type="number"
                  step="0.5"
                  value={nodeProps.fontSize ?? 10}
                  onchange={(e) => updateProp("fontSize", parseFloat((e.target as HTMLInputElement).value) || 10)}
                  class="h-7 text-xs"
                />
              </div>
              <div>
                <span class="text-[10px] text-muted-foreground">نوع الخط:</span>
                <select
                  value={nodeProps.fontFamily || "Amiri"}
                  onchange={(e) => updateProp("fontFamily", (e.target as HTMLSelectElement).value)}
                  class="w-full h-7 rounded border border-input bg-background px-1.5 text-xs"
                >
                  <option value="Amiri">Amiri (عربي)</option>
                  <option value="Rubik">Rubik</option>
                  <option value="Tahoma">Tahoma</option>
                  <option value="Arial">Arial</option>
                </select>
              </div>
            </div>

            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">لون النص:</span>
              <ColorPicker
                value={nodeProps.color || "#000000"}
                onchange={(c) => updateProp("color", c)}
              />
            </div>
          </div>

        <!-- ══════════════ SHAPE NODE INSPECTOR ══════════════ -->
        {:else if node.type === "Shape"}
          <div class="space-y-3">
            <Label class="text-xs font-bold text-foreground">خصائص الشكل | Shape Properties</Label>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">لون التعبئة (Fill):</span>
              <ColorPicker
                value={nodeProps.fillColor || "#ffffff"}
                onchange={(c) => updateProp("fillColor", c)}
              />
            </div>

            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">لون الإطار (Stroke):</span>
              <ColorPicker
                value={nodeProps.strokeColor || "#000000"}
                onchange={(c) => updateProp("strokeColor", c)}
              />
            </div>

            <div class="grid grid-cols-2 gap-2 text-xs">
              <div>
                <span class="text-[10px] text-muted-foreground">سمك الإطار:</span>
                <Input
                  type="number"
                  step="0.25"
                  value={nodeProps.strokeWidth ?? 1}
                  onchange={(e) => updateProp("strokeWidth", parseFloat((e.target as HTMLInputElement).value) || 0)}
                  class="h-7 text-xs"
                />
              </div>
              <div>
                <span class="text-[10px] text-muted-foreground">انحناء الزوايا:</span>
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
            <Label class="text-xs font-bold text-foreground">بيانات الرمز | Code Data</Label>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">البيانات / الرابط:</span>
              <Input
                type="text"
                value={nodeProps.data || ""}
                placeholder="https://... or {{student_id}}"
                onchange={(e) => updateProp("data", (e.target as HTMLInputElement).value)}
                class="h-7 text-xs font-mono"
              />
            </div>
          </div>
        {/if}

        <Separator />

        <!-- Actions -->
        <div class="pt-2 flex gap-2">
          <Button
            variant="destructive"
            size="xs"
            onclick={() => store.deleteSelected()}
            class="grow h-7 text-xs gap-1 cursor-pointer"
          >
            <Trash class="size-3" />
            حذف العنصر
          </Button>
          <Button
            variant="outline"
            size="xs"
            onclick={() => store.duplicateSelected()}
            class="grow h-7 text-xs cursor-pointer"
          >
            تكرار
          </Button>
        </div>
      {/if}
    </div>
  </ScrollArea>
</aside>

<style>
  .properties-inspector {
    width: 290px;
    height: 100%;
    background: var(--color-card, #ffffff);
    border-inline-start: 1px solid var(--color-border, #e2e8f0);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    z-index: 20;
    flex-shrink: 0;
  }

  .inspector-header {
    height: 40px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--color-border, #e2e8f0);
    background: var(--color-muted, #f8fafc);
    flex-shrink: 0;
  }

  :global(.inspector-scroll) {
    flex-grow: 1;
    overflow-y: auto;
  }
</style>
