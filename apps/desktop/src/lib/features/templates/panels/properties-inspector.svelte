<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { EditorStore } from "../stores/editor-store.svelte";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Separator } from "@orbis/ui/separator";
import { ScrollArea } from "@orbis/ui/scroll-area";
import { Checkbox } from "@orbis/ui/checkbox";
import { Textarea } from "@orbis/ui/textarea";
import { toast } from "@orbis/ui";
import ColorPicker from "../shared/color-picker.svelte";
import PropertySelect from "../shared/property-select.svelte";
import {
  AdjustmentsHorizontal,
  ArrowUpTray,
  DocumentPage,
  Plus,
  Trash,
  XMark,
} from "@orbis/icons";
import {
  useUploadFile,
  getDocumentSource,
} from "$lib/features/documents/queries";

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
    ? "إعدادات الجدول"
    : node?.type === "Text"
      ? "خصائص النص"
      : node?.type === "Shape"
        ? "خصائص الشكل"
        : "بيانات العنصر",
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
  const uploadToast = toast.loading("جارٍ رفع الصورة واستبدالها...", {
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
            title: "تعذر استبدال الصورة: لم يُرجع الخادم معرّف الملف.",
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
            title: "تم استبدال الصورة بنجاح.",
            variant: "success",
            duration: 4000,
          });
        } catch (error) {
          uploadToast.update({
            title: error instanceof Error ? error.message : "تعذر تحديث الصورة في المحرر.",
            variant: "error",
            duration: 5000,
          });
          console.error("Failed to replace template image:", error);
        }
      },
      onError: (error) => {
        uploadToast.update({
          title: error instanceof Error ? error.message : "تعذر رفع الصورة.",
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

<aside class="properties-inspector" style:width={`${width}px`} aria-label="خصائص المحرر">
  <Button
    variant="ghost"
    size="icon-xs"
    class="resize-handle"
    type="button"
    aria-label="تغيير عرض لوحة الخصائص"
    title="اسحب لتغيير العرض"
    onpointerdown={startResize}
    onkeydown={resizeWithKeyboard}
  />
  <div class="inspector-header">
    <div class="flex min-w-0 items-center gap-2">
      <AdjustmentsHorizontal class="size-4 shrink-0 text-muted-foreground" />
      <h2 class="inspector-title">{node ? "خصائص العنصر" : "خصائص الصفحة"}</h2>
    </div>
    <div class="flex shrink-0 items-center gap-1">
      {#if node}
        <Badge variant="outline" class="inspector-type">{node.type}</Badge>
      {/if}
      {#if onclose}
        <Button variant="ghost" size="icon-sm" aria-label="إخفاء الخصائص" onclick={onclose}>
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
            <span>{pageSetup.size} · {pageSetup.orientation === "landscape" ? "أفقي" : "عمودي"}</span>
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
            إعداد الصفحة
          </Button>

          <p class="text-xs leading-5 text-muted-foreground">حدد عنصرًا لعرض خصائصه.</p>
        </div>
      {:else}
        <div class="selected-layer">
          <span class="selected-layer-name" title={node.name || node.id}>{node.name || node.id}</span>
        </div>

        <details class="property-section" open>
          <summary><span>عام</span></summary>
          <div class="property-section-body space-y-3">
            <div class="space-y-1">
              <Label class="property-label">اسم الطبقة</Label>
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
                  aria-label="إظهار العنصر"
                />
                <span>إظهار العنصر</span>
              </div>
              <div class="flex items-center gap-2 text-xs">
                <Checkbox
                  checked={node.locked === true}
                  onCheckedChange={(checked) => updateProp("locked", checked === true)}
                  aria-label="قفل العنصر"
                />
                <span>قفل العنصر</span>
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1">
                <Label class="property-label">الشفافية</Label>
                <Input type="number" min="0" max="1" step="0.05" value={node.opacity ?? 1} onchange={(e) => updateProp("opacity", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">الدوران°</Label>
                <Input type="number" step="1" value={node.rotation ?? 0} onchange={(e) => updateProp("rotation", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
            </div>
          </div>
        </details>

        <details class="property-section" open>
          <summary>
            <span>الموضع والحجم</span>
            <span class="section-unit">ملم</span>
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
              <span class="property-label">العرض</span>
              <Input
                type="text"
                value={pos.width ?? "100%"}
                onchange={(e) => updatePosition("width", (e.target as HTMLInputElement).value)}
                class="h-8 border-border/80 bg-background text-xs font-mono"
              />
            </div>
            <div class="space-y-1">
              <span class="property-label">الارتفاع</span>
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
              <Label class="text-xs font-semibold text-foreground">الجدول</Label>
              <Button
                variant="ghost"
                size="xs"
                onclick={applyYemeniCertificateColumnsPreset}
                class="text-[11px] h-7 text-primary cursor-pointer hover:bg-primary/10"
                title="تطبيق هيكل أعمدة شهادة الدرجات اليمنية المعتمدة"
              >
                نموذج الشهادة المعتمدة
              </Button>
            </div>

            <details class="inspector-subsection" open>
              <summary>البيانات والعرض</summary>
              <div class="inspector-subsection-body">
            <!-- Data Source Field -->
            <div class="space-y-1">
              <Label class="text-[11px] text-muted-foreground">مصدر البيانات</Label>
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
                  aria-label="إظهار الترويسة"
                />
                <span class="text-[11px]">إظهار الترويسة</span>
              </div>

              <div class="flex items-center gap-1.5 cursor-pointer">
                <Checkbox
                  checked={nodeProps.showTotals === true}
                  onCheckedChange={(checked) => updateProp("showTotals", checked === true)}
                  aria-label="صف المجموع العام"
                />
                <span class="text-[11px]">صف المجموع العام</span>
              </div>

              <div class="flex items-center gap-1.5 cursor-pointer">
                <Checkbox
                  checked={nodeProps.showGrid !== false}
                  onCheckedChange={(checked) => updateProp("showGrid", checked === true)}
                  aria-label="خطوط الشبكة"
                />
                <span class="text-[11px]">خطوط الشبكة</span>
              </div>

              <div class="flex items-center gap-1.5 cursor-pointer">
                <Checkbox
                  checked={nodeProps.showBorder !== false}
                  onCheckedChange={(checked) => updateProp("showBorder", checked === true)}
                  aria-label="إطار خارجي"
                />
                <span class="text-[11px]">إطار خارجي</span>
              </div>
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
              </div>
            </details>

            <Separator />

            <!-- Columns Management -->
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <Label class="text-xs font-semibold text-foreground">الأعمدة</Label>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={addColumn}
                  class="h-8 text-xs gap-1.5 cursor-pointer"
                >
                  <Plus class="size-3.5" />
                  إضافة عمود
                </Button>
              </div>

              <details class="inspector-subsection">
                <summary>عرض الأعمدة <span class="text-muted-foreground">({columns.length})</span></summary>
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
                        <span class="text-[10px] text-muted-foreground">العنوان العربي</span>
                        <Input
                          type="text"
                          value={col.headerAr || ""}
                          placeholder="المادة"
                          onchange={(e) => updateColumn(idx, "headerAr", (e.target as HTMLInputElement).value)}
                          class="h-8 text-xs"
                        />
                      </div>
                      <div>
                        <span class="text-[10px] text-muted-foreground">العنوان الإنجليزي</span>
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
                        <span class="text-[10px] text-muted-foreground">الحقل</span>
                        <Input
                          type="text"
                          value={col.field || col.key || ""}
                          placeholder="e.g. max_score"
                          onchange={(e) => updateColumn(idx, "field", (e.target as HTMLInputElement).value)}
                          class="h-8 text-xs font-mono"
                        />
                      </div>
                      <div>
                        <span class="text-[10px] text-muted-foreground">العرض</span>
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
                        <span class="text-[10px] text-muted-foreground">المحاذاة</span>
                        <PropertySelect
                          value={col.alignment || "center"}
                          options={[
                            { value: "center", label: "وسط" },
                            { value: "right", label: "يمين" },
                            { value: "left", label: "يسار" },
                          ]}
                          onchange={(value) => updateColumn(idx, "alignment", value)}
                        />
                      </div>
                      <div>
                        <span class="text-[10px] text-muted-foreground">المجموع</span>
                        <PropertySelect
                          value={col.totalType || "none"}
                          options={[
                            { value: "none", label: "بدون" },
                            { value: "sum", label: "مجموع" },
                            { value: "avg", label: "متوسط" },
                            { value: "count", label: "عدد" },
                            { value: "label", label: "تسمية" },
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
              <summary>التنسيق</summary>
              <div class="inspector-subsection-body">

              <div class="space-y-1">
                <span class="text-[11px] text-muted-foreground">خلفية الترويسة</span>
                <ColorPicker
                  value={nodeProps.style?.headerBgColor || "#1e3a8a"}
                  onchange={(c) => updateTableStyle("headerBgColor", c)}
                />
              </div>

              <div class="space-y-1">
                <span class="text-[11px] text-muted-foreground">لون النص</span>
                <ColorPicker
                  value={nodeProps.style?.headerTextColor || "#ffffff"}
                  onchange={(c) => updateTableStyle("headerTextColor", c)}
                />
              </div>

              <div class="space-y-1">
                <span class="text-[11px] text-muted-foreground">لون الحدود</span>
                <ColorPicker
                  value={nodeProps.style?.borderColor || "#475569"}
                  onchange={(c) => updateTableStyle("borderColor", c)}
                />
              </div>

              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <span class="text-[11px] text-muted-foreground">ارتفاع الصف</span>
                  <Input
                    type="number"
                    step="1"
                    value={nodeProps.style?.rowHeight ?? 20}
                    onchange={(e) => updateTableStyle("rowHeight", parseFloat((e.target as HTMLInputElement).value) || 20)}
                    class="h-7 text-xs"
                  />
                </div>
                <div>
                  <span class="text-[11px] text-muted-foreground">حجم خط العنوان</span>
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
            <Label class="text-xs font-semibold text-foreground">المحتوى</Label>
            <Textarea
              value={nodeProps.content || ""}
              oninput={(e) => updateProp("content", (e.target as HTMLTextAreaElement).value)}
              rows={4}
              class="text-xs font-mono"
              placeholder={"اكتب النص أو التعبير (مثل: {{student_name}})"}
            />

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
                <PropertySelect
                  value={nodeProps.fontFamily || "Amiri"}
                  class="h-7"
                  options={[
                    { value: "Amiri", label: "Amiri (عربي)" },
                    { value: "Rubik", label: "Rubik" },
                    { value: "Tahoma", label: "Tahoma" },
                    { value: "Arial", label: "Arial" },
                  ]}
                  onchange={(value) => updateProp("fontFamily", value)}
                />
              </div>
            </div>

            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">لون النص:</span>
              <ColorPicker
                value={nodeProps.color || "#000000"}
                onchange={(c) => updateProp("color", c)}
              />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1">
                <Label class="property-label">سماكة الخط</Label>
                <PropertySelect
                  value={nodeProps.fontWeight || "normal"}
                  options={[
                    { value: "light", label: "خفيف" },
                    { value: "normal", label: "عادي" },
                    { value: "bold", label: "عريض" },
                  ]}
                  onchange={(value) => updateProp("fontWeight", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">محاذاة النص</Label>
                <PropertySelect
                  value={nodeProps.textAlign || "start"}
                  options={[
                    { value: "start", label: "البداية" },
                    { value: "center", label: "الوسط" },
                    { value: "end", label: "النهاية" },
                    { value: "justify", label: "ضبط" },
                  ]}
                  onchange={(value) => updateProp("textAlign", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">اتجاه النص</Label>
                <PropertySelect
                  value={nodeProps.direction || "auto"}
                  options={[
                    { value: "auto", label: "تلقائي" },
                    { value: "rtl", label: "يمين إلى يسار" },
                    { value: "ltr", label: "يسار إلى يمين" },
                  ]}
                  onchange={(value) => updateProp("direction", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">نمط الخط</Label>
                <PropertySelect
                  value={nodeProps.fontStyle || "normal"}
                  options={[
                    { value: "normal", label: "عادي" },
                    { value: "italic", label: "مائل" },
                  ]}
                  onchange={(value) => updateProp("fontStyle", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">زخرفة النص</Label>
                <PropertySelect
                  value={nodeProps.textDecoration || "none"}
                  options={[
                    { value: "none", label: "بدون" },
                    { value: "underline", label: "تسطير" },
                    { value: "line-through", label: "يتوسطه خط" },
                  ]}
                  onchange={(value) => updateProp("textDecoration", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">حالة الأحرف</Label>
                <PropertySelect
                  value={nodeProps.textTransform || "none"}
                  options={[
                    { value: "none", label: "كما هي" },
                    { value: "uppercase", label: "كبيرة" },
                    { value: "lowercase", label: "صغيرة" },
                  ]}
                  onchange={(value) => updateProp("textTransform", value)}
                />
              </div>
              <div class="space-y-1">
                <Label class="property-label">ارتفاع السطر</Label>
                <Input type="number" min="0.5" step="0.1" value={nodeProps.lineHeight ?? 1.2} onchange={(e) => updateProp("lineHeight", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">تباعد الأحرف</Label>
                <Input type="number" step="0.1" value={nodeProps.letterSpacing ?? 0} onchange={(e) => updateProp("letterSpacing", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">أقصى عدد أسطر</Label>
                <Input type="number" min="1" step="1" value={nodeProps.maxLines ?? ""} onchange={(e) => updateProp("maxLines", Number((e.target as HTMLInputElement).value) || undefined)} class="h-8 text-xs" />
              </div>
              <div class="space-y-1">
                <Label class="property-label">تجاوز المحتوى</Label>
                <PropertySelect
                  value={nodeProps.overflow || "visible"}
                  options={[
                    { value: "visible", label: "ظاهر" },
                    { value: "hidden", label: "إخفاء" },
                    { value: "ellipsis", label: "علامة الحذف" },
                  ]}
                  onchange={(value) => updateProp("overflow", value)}
                />
              </div>
            </div>
            <div class="space-y-1">
              <Label class="property-label">لون الخلفية</Label>
              <ColorPicker value={nodeProps.backgroundColor || "#ffffff"} onchange={(c) => updateProp("backgroundColor", c)} />
            </div>
          </div>

        {:else if node.type === "Image"}
          <div class="space-y-3">
            <div class="flex items-center justify-between gap-3">
              <div class="min-w-0">
                <Label class="property-label">الصورة</Label>
                <p class="text-[10px] text-muted-foreground">
                  {nodeProps.assetRef ? "صورة مرتبطة بمكتبة الوسائط" : "صورة مخصصة"}
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
                {imageUploadMutation.isPending ? "جارٍ الاستبدال..." : "استبدال"}
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
              <Label class="property-label">تعبير مصدر الصورة</Label>
              <Input value={nodeProps.expression || ""} placeholder={"{{image_url}}"} onchange={(e) => updateProp("expression", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">طريقة الملاءمة</Label>
                <PropertySelect
                  value={nodeProps.fit || "contain"}
                  options={[
                    { value: "contain", label: "احتواء" },
                    { value: "cover", label: "ملء" },
                    { value: "fill", label: "تمديد" },
                    { value: "none", label: "أصلية" },
                  ]}
                  onchange={(value) => updateProp("fit", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">طبقة الصورة</Label>
                <PropertySelect
                  value={nodeProps.layer || "foreground"}
                  options={[
                    { value: "foreground", label: "أمامية" },
                    { value: "background", label: "خلفية" },
                  ]}
                  onchange={(value) => updateProp("layer", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">انحناء الزوايا</Label><Input type="number" min="0" step="0.5" value={nodeProps.borderRadius ?? 0} onchange={(e) => updateProp("borderRadius", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">سمك الإطار</Label><Input type="number" min="0" step="0.25" value={nodeProps.borderWidth ?? 0} onchange={(e) => updateProp("borderWidth", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            </div>
            <div class="space-y-1"><Label class="property-label">لون الإطار</Label><ColorPicker value={nodeProps.borderColor || "#000000"} onchange={(c) => updateProp("borderColor", c)} /></div>
          </div>

        <!-- ══════════════ SHAPE NODE INSPECTOR ══════════════ -->
        {:else if node.type === "Shape"}
          <div class="space-y-3">
            <Label class="text-xs font-semibold text-foreground">التعبئة والحدود</Label>
            <div class="space-y-1">
              <Label class="property-label">نوع الشكل</Label>
              <PropertySelect
                value={nodeProps.shapeType || nodeProps.shape || "rectangle"}
                options={[
                  { value: "rectangle", label: "مستطيل" },
                  { value: "rounded-rect", label: "مستطيل بزوايا مستديرة" },
                  { value: "circle", label: "دائرة" },
                  { value: "ellipse", label: "قطع ناقص" },
                  { value: "line", label: "خط" },
                ]}
                onchange={(value) => updateProp("shapeType", value)}
              />
            </div>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">لون التعبئة (Fill):</span>
              <ColorPicker
                value={nodeProps.fill || nodeProps.fillColor || "#ffffff"}
                onchange={(c) => updateProp("fill", c)}
              />
            </div>

            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">لون الإطار (Stroke):</span>
              <ColorPicker
                value={nodeProps.stroke || nodeProps.strokeColor || "#000000"}
                onchange={(c) => updateProp("stroke", c)}
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
            <Label class="text-xs font-semibold text-foreground">بيانات الرمز</Label>
            <div class="space-y-1">
              <span class="text-[10px] text-muted-foreground">البيانات / الرابط:</span>
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
                <div class="space-y-1"><Label class="property-label">الحجم</Label><Input type="number" min="5" value={nodeProps.size ?? pos.width ?? 30} onchange={(e) => updateProp("size", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">تصحيح الخطأ</Label>
                  <PropertySelect
                    value={nodeProps.errorCorrection || "M"}
                    options={[
                      { value: "L", label: "منخفض" },
                      { value: "M", label: "متوسط" },
                      { value: "Q", label: "جيد" },
                      { value: "H", label: "عالٍ" },
                    ]}
                    onchange={(value) => updateProp("errorCorrection", value)}
                  />
                </div>
                <div class="space-y-1"><Label class="property-label">منطقة هادئة</Label><Input type="number" min="0" value={nodeProps.quietZone ?? 4} onchange={(e) => updateProp("quietZone", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">حجم الشعار %</Label><Input type="number" min="0" max="40" value={nodeProps.logoSize ?? 20} onchange={(e) => updateProp("logoSize", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              </div>
              <div class="space-y-1"><Label class="property-label">مرجع الشعار</Label><Input value={nodeProps.logoAssetRef || ""} placeholder="معرّف ملف من الوسائط" onchange={(e) => updateProp("logoAssetRef", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            {:else}
              <div class="grid grid-cols-2 gap-2">
                <div class="space-y-1"><Label class="property-label">الصيغة</Label>
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
                <div class="space-y-1"><Label class="property-label">ارتفاع الرمز</Label><Input type="number" min="5" value={nodeProps.height ?? pos.height ?? 20} onchange={(e) => updateProp("height", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">عرض الوحدة</Label><Input type="number" min="0.1" step="0.1" value={nodeProps.moduleWidth ?? 0.3} onchange={(e) => updateProp("moduleWidth", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">حجم نص الرمز</Label><Input type="number" min="1" value={nodeProps.textFontSize ?? 8} onchange={(e) => updateProp("textFontSize", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              </div>
              <div class="flex items-center gap-2 text-xs">
                <Checkbox
                  checked={nodeProps.showText !== false}
                  onCheckedChange={(checked) => updateProp("showText", checked === true)}
                  aria-label="إظهار البيانات أسفل الرمز"
                />
                <span>إظهار البيانات أسفل الرمز</span>
              </div>
            {/if}
            <div class="grid grid-cols-2 gap-3">
              <div class="space-y-1"><Label class="property-label">لون الرمز</Label><ColorPicker value={nodeProps.color || "#000000"} onchange={(c) => updateProp("color", c)} /></div>
              <div class="space-y-1"><Label class="property-label">لون الخلفية</Label><ColorPicker value={nodeProps.backgroundColor || "#ffffff"} onchange={(c) => updateProp("backgroundColor", c)} /></div>
            </div>
          </div>
        {:else if node.type === "Separator"}
          <div class="grid grid-cols-2 gap-2">
            <div class="space-y-1">
              <Label class="property-label">الاتجاه</Label>
              <PropertySelect
                value={nodeProps.direction || "horizontal"}
                options={[
                  { value: "horizontal", label: "أفقي" },
                  { value: "vertical", label: "عمودي" },
                ]}
                onchange={(value) => updateProp("direction", value)}
              />
            </div>
            <div class="space-y-1">
              <Label class="property-label">النمط</Label>
              <PropertySelect
                value={nodeProps.style || "solid"}
                options={[
                  { value: "solid", label: "متصل" },
                  { value: "dashed", label: "متقطع" },
                  { value: "dotted", label: "منقط" },
                ]}
                onchange={(value) => updateProp("style", value)}
              />
            </div>
            <div class="space-y-1"><Label class="property-label">السماكة</Label><Input type="number" min="0.1" step="0.1" value={nodeProps.thickness ?? 1} onchange={(e) => updateProp("thickness", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            <div class="space-y-1"><Label class="property-label">الطول</Label><Input type="number" min="1" value={nodeProps.length ?? pos.width} onchange={(e) => updateProp("length", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            <div class="col-span-2 space-y-1"><Label class="property-label">اللون</Label><ColorPicker value={nodeProps.color || "#cbd5e1"} onchange={(c) => updateProp("color", c)} /></div>
          </div>
        {:else if node.type === "Spacer"}
          <div class="grid grid-cols-2 gap-2">
            <div class="space-y-1"><Label class="property-label">العرض</Label><Input type="number" min="0" value={nodeProps.width ?? pos.width} onchange={(e) => updateProp("width", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            <div class="space-y-1"><Label class="property-label">الارتفاع</Label><Input type="number" min="0" value={nodeProps.height ?? pos.height} onchange={(e) => updateProp("height", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
          </div>
        {:else if node.type === "Frame" || node.type === "Grid"}
          <div class="space-y-3">
            {#if node.type === "Frame"}
              <div class="grid grid-cols-2 gap-2">
                <div class="space-y-1">
                  <Label class="property-label">اتجاه التخطيط</Label>
                  <PropertySelect
                    value={nodeProps.layout || "vertical"}
                    options={[
                      { value: "vertical", label: "عمودي" },
                      { value: "horizontal", label: "أفقي" },
                    ]}
                    onchange={(value) => updateProp("layout", value)}
                  />
                </div>
                <div class="space-y-1">
                  <Label class="property-label">المحاذاة</Label>
                  <PropertySelect
                    value={nodeProps.alignment || "start"}
                    options={[
                      { value: "start", label: "بداية" },
                      { value: "center", label: "وسط" },
                      { value: "end", label: "نهاية" },
                      { value: "space-between", label: "توزيع" },
                      { value: "space-around", label: "مسافات متساوية" },
                    ]}
                    onchange={(value) => updateProp("alignment", value)}
                  />
                </div>
                <div class="space-y-1">
                  <Label class="property-label">محاذاة العناصر</Label>
                  <PropertySelect
                    value={nodeProps.crossAlignment || "stretch"}
                    options={[
                      { value: "start", label: "بداية" },
                      { value: "center", label: "وسط" },
                      { value: "end", label: "نهاية" },
                      { value: "stretch", label: "تمديد" },
                    ]}
                    onchange={(value) => updateProp("crossAlignment", value)}
                  />
                </div>
                <div class="flex items-center gap-2 text-xs">
                  <Checkbox
                    checked={nodeProps.wrap === true}
                    onCheckedChange={(checked) => updateProp("wrap", checked === true)}
                    aria-label="التفاف العناصر"
                  />
                  <span>التفاف العناصر</span>
                </div>
              </div>
            {:else}
              <div class="space-y-1"><Label class="property-label">الأعمدة (مثل 1fr, 2fr)</Label><Input value={(nodeProps.columns || []).join(", ")} onchange={(e) => updateProp("columns", (e.target as HTMLInputElement).value.split(",").map((part) => part.trim()).filter(Boolean))} class="h-8 text-xs font-mono" /></div>
              <div class="space-y-1"><Label class="property-label">الصفوف (اختياري)</Label><Input value={(nodeProps.rows || []).join(", ")} onchange={(e) => updateProp("rows", (e.target as HTMLInputElement).value.split(",").map((part) => part.trim()).filter(Boolean))} class="h-8 text-xs font-mono" /></div>
            {/if}
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">المسافة بين العناصر</Label><Input type="number" min="0" value={nodeProps.gap ?? 0} onchange={(e) => updateProp("gap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">سماكة الإطار</Label><Input type="number" min="0" value={nodeProps.borderWidth ?? 0} onchange={(e) => updateProp("borderWidth", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              {#if node.type === "Grid"}
                <div class="space-y-1"><Label class="property-label">فجوة الأعمدة</Label><Input type="number" min="0" value={nodeProps.columnGap ?? nodeProps.gap ?? 0} onchange={(e) => updateProp("columnGap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
                <div class="space-y-1"><Label class="property-label">فجوة الصفوف</Label><Input type="number" min="0" value={nodeProps.rowGap ?? nodeProps.gap ?? 0} onchange={(e) => updateProp("rowGap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              {/if}
              <div class="space-y-1"><Label class="property-label">الحشو الداخلي</Label><Input type="number" min="0" value={typeof nodeProps.padding === "number" ? nodeProps.padding : nodeProps.padding?.top ?? 0} onchange={(e) => { const value = Number((e.target as HTMLInputElement).value); updateProp("padding", { top: value, right: value, bottom: value, left: value }); }} class="h-8 text-xs" /></div>
            </div>
            <div class="grid grid-cols-2 gap-3">
              <div class="space-y-1"><Label class="property-label">لون الخلفية</Label><ColorPicker value={nodeProps.backgroundColor || "#ffffff"} onchange={(c) => updateProp("backgroundColor", c)} /></div>
              <div class="space-y-1"><Label class="property-label">لون الإطار</Label><ColorPicker value={nodeProps.borderColor || "#cbd5e1"} onchange={(c) => updateProp("borderColor", c)} /></div>
            </div>
          </div>
        {:else if node.type === "Repeater"}
          <div class="space-y-3">
            <div class="space-y-1"><Label class="property-label">مصدر التكرار</Label><Input value={nodeProps.dataSource || ""} onchange={(e) => updateProp("dataSource", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">اسم العنصر</Label><Input value={nodeProps.itemVariable || "item"} onchange={(e) => updateProp("itemVariable", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">اسم الفهرس</Label><Input value={nodeProps.indexVariable || "index"} onchange={(e) => updateProp("indexVariable", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1">
                <Label class="property-label">اتجاه التكرار</Label>
                <PropertySelect
                  value={nodeProps.layout || "vertical"}
                  options={[
                    { value: "vertical", label: "عمودي" },
                    { value: "horizontal", label: "أفقي" },
                    { value: "grid", label: "شبكة" },
                  ]}
                  onchange={(value) => updateProp("layout", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">عدد الأعمدة</Label><Input type="number" min="1" value={nodeProps.columns ?? 2} onchange={(e) => updateProp("columns", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">ترتيب حسب</Label><Input value={nodeProps.sortBy || ""} onchange={(e) => updateProp("sortBy", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1">
                <Label class="property-label">اتجاه الترتيب</Label>
                <PropertySelect
                  value={nodeProps.sortDirection || "asc"}
                  options={[
                    { value: "asc", label: "تصاعدي" },
                    { value: "desc", label: "تنازلي" },
                  ]}
                  onchange={(value) => updateProp("sortDirection", value)}
                />
              </div>
              <div class="space-y-1"><Label class="property-label">المسافة</Label><Input type="number" min="0" value={nodeProps.gap ?? 0} onchange={(e) => updateProp("gap", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">الحد الأقصى</Label><Input type="number" min="1" value={nodeProps.limit ?? ""} onchange={(e) => updateProp("limit", Number((e.target as HTMLInputElement).value) || undefined)} class="h-8 text-xs" /></div>
            </div>
            <div class="space-y-1"><Label class="property-label">شرط التصفية</Label><Input value={nodeProps.filter || ""} onchange={(e) => updateProp("filter", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            <div class="space-y-1"><Label class="property-label">رسالة القائمة الفارغة</Label><Input value={nodeProps.emptyMessage || ""} onchange={(e) => updateProp("emptyMessage", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
          </div>
        {:else if node.type === "Conditional"}
          <div class="space-y-1">
            <Label class="property-label">شرط العرض</Label>
            <Textarea
              value={nodeProps.condition || ""}
              oninput={(e) => updateProp("condition", (e.target as HTMLTextAreaElement).value)}
              rows={3}
              class={compactTextareaClass}
            />
          </div>
        {:else if node.type === "ComponentRef"}
          <div class="space-y-1"><Label class="property-label">معرّف المكوّن</Label><Input value={nodeProps.componentId || ""} onchange={(e) => updateProp("componentId", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
        {:else if node.type === "Header" || node.type === "Footer"}
          <div class="space-y-3">
            <div class="space-y-1"><Label class="property-label">الارتفاع</Label><Input type="number" min="1" value={nodeProps.height ?? 20} onchange={(e) => updateProp("height", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            {#if node.type === "Footer"}
              <div class="flex items-center gap-2 text-xs">
                <Checkbox
                  checked={nodeProps.showPageNumber === true}
                  onCheckedChange={(checked) => updateProp("showPageNumber", checked === true)}
                  aria-label="إظهار رقم الصفحة"
                />
                <span>إظهار رقم الصفحة</span>
              </div>
              <div class="space-y-1"><Label class="property-label">تنسيق رقم الصفحة</Label><Input value={nodeProps.pageNumberFormat || "{{page}}/{{total}}"} onchange={(e) => updateProp("pageNumberFormat", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            {/if}
          </div>
        {:else if node.type === "Watermark"}
          <div class="space-y-3">
            <div class="space-y-1">
              <Label class="property-label">نوع العلامة</Label>
              <PropertySelect
                value={nodeProps.watermarkType || "text"}
                options={[
                  { value: "text", label: "نص" },
                  { value: "image", label: "صورة" },
                ]}
                onchange={(value) => updateProp("watermarkType", value)}
              />
            </div>
            {#if nodeProps.watermarkType !== "image"}
              <div class="space-y-1"><Label class="property-label">نص العلامة</Label><Input value={nodeProps.text || ""} onchange={(e) => updateProp("text", (e.target as HTMLInputElement).value)} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">لون النص</Label><ColorPicker value={nodeProps.color || "#94a3b8"} onchange={(c) => updateProp("color", c)} /></div>
              <div class="space-y-1"><Label class="property-label">حجم الخط</Label><Input type="number" min="1" value={nodeProps.fontSize ?? 36} onchange={(e) => updateProp("fontSize", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            {:else}
              <div class="space-y-1"><Label class="property-label">معرّف الصورة</Label><Input value={nodeProps.assetRef || ""} onchange={(e) => updateProp("assetRef", (e.target as HTMLInputElement).value)} class="h-8 text-xs font-mono" /></div>
            {/if}
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1"><Label class="property-label">الشفافية</Label><Input type="number" min="0" max="1" step="0.05" value={nodeProps.opacity ?? 0.2} onchange={(e) => updateProp("opacity", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
              <div class="space-y-1"><Label class="property-label">الدوران°</Label><Input type="number" value={nodeProps.rotation ?? 0} onchange={(e) => updateProp("rotation", Number((e.target as HTMLInputElement).value))} class="h-8 text-xs" /></div>
            </div>
          </div>
        {:else if node.type === "PageBreak"}
          <p class="text-xs leading-5 text-muted-foreground">يفرض هذا العنصر بدء صفحة جديدة عند الطباعة.</p>
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
            حذف العنصر
          </Button>
          <Button
            variant="outline"
            size="sm"
            onclick={() => store.duplicateSelected()}
            class="grow"
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
