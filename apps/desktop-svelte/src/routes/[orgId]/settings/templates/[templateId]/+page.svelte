<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Textarea } from "@orbis/ui-svelte/textarea";
import { Badge } from "@orbis/ui-svelte/badge";
import { Card, CardContent } from "@orbis/ui-svelte/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/i18n-utils";
import {
  Save,
  RefreshCw,
  Plus,
  Trash2,
  ChevronRight,
  Type,
  Barcode,
  QrCode,
  Sliders,
  Play,
  ArrowLeft,
  Braces,
  Sparkles,
  Info,
  FolderOpen,
} from "lucide-svelte";

// Router parameters
const orgId = $derived(page.params.orgId);
const templateId = $derived(page.params.templateId);

// State Runes
let templateRecord = $state<any>(null);
let documentModel = $state<any>(null);
let schemas = $state<Record<string, any>>({});
let selectedNodeId = $state<string | null>(null);
let selectedNode = $state<any>(null);
let testContext = $state<string>(
  '{\n  "name": "عبد الرحمن",\n  "std_id": "99581-2026"\n}',
);
let previewPdfUrl = $state<string | null>(null);
let isPreviewLoading = $state(false);
let isSaving = $state(false);
let variablesSchema = $derived(schemas[templateRecord?.entity_type] || {});

// Fetch template & schema details
async function initEditor() {
  try {
    // 1. Fetch template record
    templateRecord = await commands.documentTemplates.getTemplate(
      orgId || "",
      templateId || "",
    );

    // 2. Parse inner document model
    documentModel = JSON.parse(templateRecord.document_json);

    // 3. Fetch variables schemas
    const schemasResponse = await commands.docforge.getDatasourceSchemas();
    schemas = schemasResponse || {};

    // 4. Setup initial test context mock based on entity type
    if (templateRecord.entity_type === "payment") {
      testContext = JSON.stringify(
        {
          amount: 2500.0,
          currency: "SAR",
          receiptNumber: "R-77401",
          payerName: "خالد الحربي",
          notes: "سداد الرسوم الدراسية للفصل الأول",
          paidAt: "2026-06-29",
        },
        null,
        2,
      );
    } else if (templateRecord.entity_type === "exam") {
      testContext = JSON.stringify(
        {
          seatingNumber: "A-402",
          sequenceNumber: 124,
          level: "الثالث",
          studentNameAr: "ياسر الهذلي",
          session: {
            name: "امتحانات نهاية العام",
            term: "الفصل الثاني",
            academicYear: "2025/2026",
          },
        },
        null,
        2,
      );
    } else if (templateRecord.entity_type === "user") {
      testContext = JSON.stringify(
        {
          fullName: "د. عبد الله الهاشمي",
          email: "a.hashemi@orbis.edu",
          phone: "+966501234567",
          nationality: "سعودي",
          nationalId: "1029384756",
          primaryRole: "عضو هيئة تدريس",
        },
        null,
        2,
      );
    }

    // 5. Generate first live preview
    generatePreview();
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to initialize editor";
    toast.error(msg);
  }
}

// Convert byte array to local Data URL
function uint8ArrayToBase64(bytes: number[]): string {
  let binary = "";
  const len = bytes.length;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

// Generate live PDF Preview
async function generatePreview() {
  if (!documentModel) return;
  isPreviewLoading = true;
  try {
    // Stringify document model with state updates
    const templateJson = JSON.stringify(documentModel);

    // Resolve PDF bytes
    const pdfBytes = await commands.docforge.previewDocumentTemplate(
      templateJson,
      testContext,
    );

    if (pdfBytes && pdfBytes.length > 0) {
      const base64 = uint8ArrayToBase64(pdfBytes);
      previewPdfUrl = `data:application/pdf;base64,${base64}`;
    }
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to generate preview";
    console.error(msg);
  } finally {
    isPreviewLoading = false;
  }
}

// Save Changes to DB
async function saveTemplate() {
  if (!templateRecord || !documentModel) return;
  isSaving = true;
  try {
    const updateRequest = {
      name: templateRecord.name,
      name_ar: templateRecord.name_ar,
      description: templateRecord.description,
      category_id: templateRecord.category_id,
      code: templateRecord.code,
      page_size: documentModel.meta.pageSetup.size,
      orientation: documentModel.meta.pageSetup.orientation,
      margins_json: JSON.stringify(documentModel.meta.pageSetup.margins),
      document_json: JSON.stringify(documentModel),
      locale: templateRecord.locale || "ar",
      tags: templateRecord.tags,
    };

    const userId = "current-user";
    await commands.documentTemplates.updateTemplate(
      orgId || "",
      templateId || "",
      userId || "",
      updateRequest,
    );
    toast.success("قالب المستندات تم حفظه بنجاح");
  } catch (e: unknown) {
    const msg = (e as any)?.message || "Failed to save template";
    toast.error(msg);
  } finally {
    isSaving = false;
  }
}

// Traverse tree to find node
function findNodeById(root: any, id: string): any {
  if (root.id === id) return root;
  if (root.children && Array.isArray(root.children)) {
    for (const child of root.children) {
      const found = findNodeById(child, id);
      if (found) return found;
    }
  }
  return null;
}

// Select a node in the tree and load its inspector fields
function selectNode(id: string) {
  selectedNodeId = id;
  if (documentModel) {
    selectedNode = findNodeById(documentModel.root, id);
  }
}

// Update specific property on selected node and refresh preview
function updateNodeProp(propName: string, value: any, nestedPath?: string) {
  if (!selectedNode) return;
  if (nestedPath === "props") {
    if (!selectedNode.props) selectedNode.props = {};
    selectedNode.props[propName] = value;
  } else {
    selectedNode[propName] = value;
  }

  // Force Svelte 5 state reactivity update
  documentModel = { ...documentModel };

  // Auto regenerate live preview
  generatePreview();
}

// Add a child element to the selected Frame/Page node
function addChildNode(
  type: "Text" | "Spacer" | "Separator" | "Barcode" | "QRCode",
) {
  if (!selectedNode) return;

  const newId = `${type.toLowerCase()}-${Date.now()}`;
  let newNode: any = { type, id: newId };

  if (type === "Text") {
    newNode.props = {
      content: "نص جديد / New Text {{variable}}",
      fontSize: 14,
      fontFamily: "Tahoma",
      textAlign: "right",
      direction: "rtl",
    };
  } else if (type === "Spacer") {
    newNode.props = {
      height: 10,
    };
  } else if (type === "Separator") {
    newNode.props = {
      direction: "horizontal",
      thickness: 1,
      color: "#cccccc",
    };
  } else if (type === "Barcode") {
    newNode.props = {
      data: "{{studentNumber}}",
      format: "Code128",
      height: 12,
      showText: true,
    };
  } else if (type === "QRCode") {
    newNode.props = {
      data: "{{id}}",
      size: 25,
      errorCorrection: "Medium",
    };
  }

  if (!selectedNode.children) selectedNode.children = [];
  selectedNode.children.push(newNode);

  documentModel = { ...documentModel };
  selectNode(newId);
  generatePreview();
  toast.success(`تم إضافة عنصر ${type} بنجاح`);
}

// Remove the selected node
function deleteSelectedNode() {
  if (!selectedNodeId || selectedNodeId === "page-root") {
    toast.error("Cannot delete root node");
    return;
  }

  function removeNode(parent: any, id: string): boolean {
    if (parent.children && Array.isArray(parent.children)) {
      const idx = parent.children.findIndex((c: any) => c.id === id);
      if (idx !== -1) {
        parent.children.splice(idx, 1);
        return true;
      }
      for (const child of parent.children) {
        if (removeNode(child, id)) return true;
      }
    }
    return false;
  }

  if (removeNode(documentModel.root, selectedNodeId)) {
    documentModel = { ...documentModel };
    selectedNodeId = null;
    selectedNode = null;
    generatePreview();
    toast.success("تم حذف العنصر");
  }
}

// Drag & Drop variables helper into current input
function insertVariable(varPath: string) {
  if (selectedNode && selectedNode.type === "Text") {
    const currentText = selectedNode.props.content || "";
    updateNodeProp("content", `${currentText} {{${varPath}}}`, "props");
    toast.success(`Variable {{${varPath}}} inserted`);
  } else {
    toast.info("يرجى اختيار عنصر نصي أولاً لإدراج المتغير");
  }
}

// Mounting lifecycle
onMount(() => {
  initEditor();
});
</script>

<div class="h-full w-full flex flex-col bg-background/40">
  
  <!-- Editor Header Action Bar -->
  <header class="h-14 border-b border-border bg-card/60 px-4 flex items-center justify-between gap-4 flex-nowrap select-none">
    <div class="flex items-center gap-3">
      <Button variant="ghost" size="icon-sm" href={`/${orgId}/settings/templates`} class="cursor-pointer">
        <ArrowLeft size={16} class="rtl:rotate-180" />
      </Button>
      <div class="flex flex-col text-start">
        <div class="flex items-center gap-2">
          <h1 class="text-sm font-bold text-foreground">
            {templateRecord ? l(templateRecord.name_ar, templateRecord.name) : m.settings_templates_loading()}
          </h1>
          {#if templateRecord}
            <Badge variant="outline" class="text-[10px] capitalize">{templateRecord.entity_type}</Badge>
          {/if}
        </div>
        <p class="text-[10px] text-muted-foreground">محرر القوالب المرئي / Visual Document Editor</p>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <Button variant="outline" size="sm" onclick={generatePreview} disabled={isPreviewLoading}>
        <RefreshCw size={14} class="me-1.5 {isPreviewLoading ? 'animate-spin' : ''}" />
        معاينة / Preview
      </Button>
      <Button variant="lime" size="sm" onclick={saveTemplate} disabled={isSaving}>
        <Save size={14} class="me-1.5" />
        {isSaving ? "جاري الحفظ..." : "حفظ التغييرات / Save"}
      </Button>
    </div>
  </header>

  <!-- Editor Main Split Panels -->
  {#if documentModel}
    <div class="flex-grow flex overflow-hidden w-full h-[calc(100vh-3.5rem)] text-xs select-none">
      
      <!-- Panel 1: Tree Hierarchy & Variables Picker -->
      <aside class="w-80 border-e border-border bg-card/10 flex flex-col overflow-y-auto scrollbar-hide shrink-0 text-start">
        <!-- Tree Section -->
        <div class="p-4 border-b border-border/60 flex flex-col gap-3">
          <div class="flex items-center justify-between">
            <h3 class="font-bold text-foreground flex items-center gap-1.5">
              <FolderOpen size={13} class="text-primary" />
              شجرة العناصر / Hierarchy
            </h3>
          </div>
          
          <!-- Element Tree -->
          <div class="flex flex-col gap-1 border border-border/40 p-2 rounded-lg bg-background/30 max-h-56 overflow-y-auto">
            <!-- Recursive Tree Rendering helper -->
            {#snippet treeNode(node: any, depth: number = 0)}
              <button
                type="button"
                class="w-full flex items-center gap-2 py-1.5 px-2 rounded hover:bg-accent text-start cursor-pointer transition-all
                  {selectedNodeId === node.id ? 'bg-primary/10 text-primary font-semibold' : 'text-muted-foreground'}"
                style="padding-inline-start: {depth * 12 + 8}px;"
                onclick={() => selectNode(node.id)}
              >
                {#if node.type === "Page"}
                  <Sparkles size={11} class="text-primary" />
                {:else if node.type === "Frame"}
                  <Info size={11} class="text-blue-500" />
                {:else if node.type === "Text"}
                  <Type size={11} class="text-green-500" />
                {:else if node.type === "Barcode"}
                  <Barcode size={11} class="text-amber-500" />
                {:else if node.type === "QRCode"}
                  <QrCode size={11} class="text-purple-500" />
                {:else}
                  <Sliders size={11} />
                {/if}
                <span class="truncate">{node.id}</span>
              </button>
              {#if node.children && Array.isArray(node.children)}
                {#each node.children as child (child.id)}
                  {@render treeNode(child, depth + 1)}
                {/each}
              {/if}
            {/snippet}

            {@render treeNode(documentModel.root)}
          </div>
        </div>

        <!-- Variables Auto-complete Picker Section -->
        <div class="p-4 flex-grow flex flex-col gap-3">
          <h3 class="font-bold text-foreground flex items-center gap-1.5 border-b border-border/40 pb-2">
            <Braces size={13} class="text-primary" />
            المتغيرات المتاحة / Variables
          </h3>
          <div class="flex flex-col gap-2 overflow-y-auto max-h-[350px] scrollbar-hide pr-1">
            {#if Object.keys(variablesSchema).length === 0}
              <div class="text-[10px] text-muted-foreground text-center py-4">لا توجد مخططات متغيرات لهذا الكيان.</div>
            {:else}
              {#each Object.entries(variablesSchema) as [key, spec_val]}
                {@const spec = spec_val as any}
                {#if spec.type === "object" && spec.properties}
                  <!-- Nested Objects like Student.fullName -->
                  <div class="flex flex-col gap-1 border border-border/30 rounded-lg p-2 bg-muted/10">
                    <span class="font-semibold text-foreground text-[10px] opacity-80">{spec.label || key}</span>
                    <div class="grid grid-cols-1 gap-1.5 mt-1.5 ps-2">
                      {#each Object.entries(spec.properties || {}) as [subKey, subSpec_val]}
                        {@const subSpec = subSpec_val as any}
                        <button
                          type="button"
                          class="flex items-center justify-between p-1.5 border border-border/40 rounded bg-background hover:border-primary/50 text-start cursor-pointer transition-all"
                          onclick={() => insertVariable(`${key}.${subKey}`)}
                        >
                          <span class="font-mono text-[9px] text-primary truncate">{key}.{subKey}</span>
                          <span class="text-[9px] text-muted-foreground truncate">{subSpec.label || subKey}</span>
                        </button>
                      {/each}
                    </div>
                  </div>
                {:else}
                  <!-- Simple Fields -->
                  <button
                    type="button"
                    class="flex items-center justify-between p-2 border border-border/40 rounded bg-background hover:border-primary/50 text-start cursor-pointer transition-all"
                    onclick={() => insertVariable(key)}
                  >
                    <span class="font-mono text-[9px] text-primary">{key}</span>
                    <span class="text-[9px] text-muted-foreground">{spec.label || key}</span>
                  </button>
                {/if}
              {/each}
            {/if}
          </div>
        </div>
      </aside>

      <!-- Panel 2: Interactive Live PDF Preview Canvas -->
      <main class="flex-grow bg-muted/30 p-4 flex flex-col gap-3 overflow-hidden">
        <!-- Canvas Header tools -->
        <div class="flex items-center justify-between bg-card border border-border px-3 py-2 rounded-lg">
          <div class="flex items-center gap-1.5 text-muted-foreground">
            <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
            <span class="text-[10px]">تخطيط فوري حقيقي / Instant Layout Engine</span>
          </div>
          
          <Badge variant="outline">{documentModel.meta.pageSetup.size} ({documentModel.meta.pageSetup.orientation})</Badge>
        </div>

        <!-- Canvas Body (renders Live generated PDF inside iframe) -->
        <div class="flex-grow bg-white border border-border rounded-xl shadow-inner relative overflow-hidden flex items-center justify-center">
          {#if isPreviewLoading}
            <div class="absolute inset-0 bg-background/60 backdrop-blur-xs flex flex-col items-center justify-center gap-2 z-10">
              <RefreshCw size={24} class="text-primary animate-spin" />
              <p class="text-xs font-semibold">جاري التخطيط وتوليد المعاينة...</p>
            </div>
          {/if}

          {#if previewPdfUrl}
            <iframe
              src={previewPdfUrl}
              title="Template Preview"
              class="w-full h-full border-none"
            ></iframe>
          {:else}
            <div class="text-muted-foreground flex flex-col items-center gap-2">
              <Info size={32} />
              <p>اضغط على "معاينة" لتوليد ملف الـ PDF</p>
            </div>
          {/if}
        </div>
      </main>

      <!-- Panel 3: Inspector Panel & Mock Data Context -->
      <aside class="w-80 border-s border-border bg-card/10 flex flex-col overflow-y-auto scrollbar-hide shrink-0 text-start">
        
        <!-- Tab: Node Inspector -->
        <div class="p-4 border-b border-border/60 flex flex-col gap-4">
          <h3 class="font-bold text-foreground flex items-center justify-between border-b border-border/40 pb-2">
            <span class="flex items-center gap-1.5">
              <Sliders size={13} class="text-primary" />
              خصائص العنصر / Properties
            </span>
            
            {#if selectedNodeId && selectedNodeId !== "page-root"}
              <Button
                variant="destructive"
                size="icon-xs"
                class="h-7 w-7 rounded-md cursor-pointer"
                onclick={deleteSelectedNode}
              >
                <Trash2 size={12} />
              </Button>
            {/if}
          </h3>

          {#if !selectedNode}
            <div class="text-[10px] text-muted-foreground text-center py-6">
              اختر عنصراً من شجرة العناصر لتعديل خصائصه هنا.
            </div>
          {:else}
            <div class="flex flex-col gap-3">
              <div class="flex items-center justify-between bg-muted/20 p-2 rounded-lg">
                <span class="font-semibold text-foreground text-[10px]">{selectedNode.type} Element</span>
                <Badge variant="secondary">{selectedNode.id}</Badge>
              </div>

              <!-- Page setup inspector if root selected -->
              {#if selectedNode.type === "Page"}
                <div class="flex flex-col gap-3">
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[10px] font-semibold text-muted-foreground">حجم الصفحة / Size</Label>
                    <Select type="single" value={documentModel.meta.pageSetup.size} onValueChange={(val: any) => {
                      documentModel.meta.pageSetup.size = val;
                      generatePreview();
                    }}>
                      <SelectTrigger class="h-8">
                        {documentModel.meta.pageSetup.size}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="A4">A4</SelectItem>
                        <SelectItem value="A5">A5</SelectItem>
                        <SelectItem value="Letter">Letter</SelectItem>
                        <SelectItem value="Custom">Custom</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[10px] font-semibold text-muted-foreground">الاتجاه / Orientation</Label>
                    <Select type="single" value={documentModel.meta.pageSetup.orientation} onValueChange={(val: any) => {
                      documentModel.meta.pageSetup.orientation = val;
                      generatePreview();
                    }}>
                      <SelectTrigger class="h-8">
                        {documentModel.meta.pageSetup.orientation === "portrait" ? "طولي / Portrait" : "عرضي / Landscape"}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="portrait">طولي / Portrait</SelectItem>
                        <SelectItem value="landscape">عرضي / Landscape</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  
                  <div class="flex flex-col gap-1.5 border-t border-border/40 pt-3">
                    <Label class="text-[10px] font-semibold text-muted-foreground">إضافة عنصر ابن / Add Child</Label>
                    <div class="grid grid-cols-2 gap-1.5 mt-1">
                      <Button variant="outline" size="xs" onclick={() => addChildNode("Text")}>
                        <Type size={11} class="me-1" /> Text
                      </Button>
                      <Button variant="outline" size="xs" onclick={() => addChildNode("Barcode")}>
                        <Barcode size={11} class="me-1" /> Barcode
                      </Button>
                      <Button variant="outline" size="xs" onclick={() => addChildNode("QRCode")}>
                        <QrCode size={11} class="me-1" /> QR Code
                      </Button>
                    </div>
                  </div>
                </div>

              <!-- Frame Node properties inspector -->
              {:else if selectedNode.type === "Frame"}
                <div class="flex flex-col gap-3">
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[10px] font-semibold text-muted-foreground">هيكل التخطيط / Layout</Label>
                    <Select type="single" value={selectedNode.props.layout || "flex"} onValueChange={(val: any) => updateNodeProp("layout", val, "props")}>
                      <SelectTrigger class="h-8">
                        {({ flex: "Flex Flow", grid: "Grid", absolute: "Absolute / Manual" } as any)[selectedNode.props.layout || "flex"]}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="flex">Flex Flow</SelectItem>
                        <SelectItem value="grid">Grid</SelectItem>
                        <SelectItem value="absolute">Absolute / Manual</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[10px] font-semibold text-muted-foreground">تباعد العناصر الفجوة / Gap (mm)</Label>
                    <Input
                      type="number"
                      value={selectedNode.props.gap || 0}
                      oninput={(e) => updateNodeProp("gap", parseFloat(e.currentTarget.value) || 0, "props")}
                      class="h-8 text-xs"
                    />
                  </div>

                  <div class="flex flex-col gap-1.5 border-t border-border/40 pt-3">
                    <Label class="text-[10px] font-semibold text-muted-foreground">إضافة عنصر ابن / Add Child</Label>
                    <div class="grid grid-cols-2 gap-1.5 mt-1">
                      <Button variant="outline" size="xs" onclick={() => addChildNode("Text")}>
                        <Type size={11} class="me-1" /> Text
                      </Button>
                      <Button variant="outline" size="xs" onclick={() => addChildNode("Barcode")}>
                        <Barcode size={11} class="me-1" /> Barcode
                      </Button>
                      <Button variant="outline" size="xs" onclick={() => addChildNode("QRCode")}>
                        <QrCode size={11} class="me-1" /> QR Code
                      </Button>
                    </div>
                  </div>
                </div>

              <!-- Text Node properties inspector -->
              {:else if selectedNode.type === "Text"}
                <div class="flex flex-col gap-3">
                  <div class="flex flex-col gap-1">
                    <Label class="text-[10px] font-semibold text-muted-foreground">محتوى النص / Text Content</Label>
                    <Textarea
                      value={selectedNode.props.content || ""}
                      oninput={(e) => updateNodeProp("content", e.currentTarget.value, "props")}
                      class="text-xs min-h-[60px]"
                    />
                  </div>
                  
                  <div class="grid grid-cols-2 gap-2">
                    <div class="flex flex-col gap-1">
                      <Label class="text-[10px] font-semibold text-muted-foreground">حجم الخط / Size (pt)</Label>
                      <Input
                        type="number"
                        value={selectedNode.props.fontSize || 12}
                        oninput={(e) => updateNodeProp("fontSize", parseFloat(e.currentTarget.value) || 12, "props")}
                        class="h-8 text-xs"
                      />
                    </div>
                    <div class="flex flex-col gap-1">
                      <Label class="text-[10px] font-semibold text-muted-foreground">محاذاة / Align</Label>
                      <Select type="single" value={selectedNode.props.textAlign || "right"} onValueChange={(val: any) => updateNodeProp("textAlign", val, "props")}>
                        <SelectTrigger class="h-8">
                          {({ right: "يمين / Right", center: "وسط / Center", left: "يسار / Left" } as any)[selectedNode.props.textAlign || "right"]}
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="right">يمين / Right</SelectItem>
                          <SelectItem value="center">وسط / Center</SelectItem>
                          <SelectItem value="left">يسار / Left</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                  </div>

                  <div class="grid grid-cols-2 gap-2">
                    <div class="flex flex-col gap-1">
                      <Label class="text-[10px] font-semibold text-muted-foreground">عائلة الخط / Family</Label>
                      <Select type="single" value={selectedNode.props.fontFamily || "Tahoma"} onValueChange={(val: any) => updateNodeProp("fontFamily", val, "props")}>
                        <SelectTrigger class="h-8">
                          {({ Tahoma: "Tahoma (عربي)", Rubik: "Rubik" } as any)[selectedNode.props.fontFamily || "Tahoma"]}
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="Tahoma">Tahoma (عربي)</SelectItem>
                          <SelectItem value="Rubik">Rubik</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                    <div class="flex flex-col gap-1">
                      <Label class="text-[10px] font-semibold text-muted-foreground">الاتجاه / Direction</Label>
                      <Select type="single" value={selectedNode.props.direction || "rtl"} onValueChange={(val: any) => updateNodeProp("direction", val, "props")}>
                        <SelectTrigger class="h-8">
                          {({ rtl: "يمين ليسار (RTL)", ltr: "يسار ليمين (LTR)" } as any)[selectedNode.props.direction || "rtl"]}
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="rtl">يمين ليسار (RTL)</SelectItem>
                          <SelectItem value="ltr">يسار ليمين (LTR)</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                  </div>

                  <div class="flex flex-col gap-1">
                    <Label class="text-[10px] font-semibold text-muted-foreground">لون الخط / Text Color</Label>
                    <Input
                      type="text"
                      value={selectedNode.props.color || "#000000"}
                      oninput={(e) => updateNodeProp("color", e.currentTarget.value, "props")}
                      class="h-8 text-xs font-mono"
                    />
                  </div>
                </div>

              <!-- Barcode Node properties inspector -->
              {:else if selectedNode.type === "Barcode"}
                <div class="flex flex-col gap-3">
                  <div class="flex flex-col gap-1">
                    <Label class="text-[10px] font-semibold text-muted-foreground">البيانات / Barcode Value</Label>
                    <Input
                      type="text"
                      value={selectedNode.props.data || ""}
                      oninput={(e) => updateNodeProp("data", e.currentTarget.value, "props")}
                      class="h-8 text-xs font-mono"
                    />
                  </div>
                  
                  <div class="flex flex-col gap-1">
                    <Label class="text-[10px] font-semibold text-muted-foreground">الصيغة / Format</Label>
                    <Select type="single" value={selectedNode.props.format || "Code128"} onValueChange={(val: any) => updateNodeProp("format", val, "props")}>
                      <SelectTrigger class="h-8">
                        {selectedNode.props.format || "Code128"}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="Code128">Code128</SelectItem>
                        <SelectItem value="EAN8">EAN8</SelectItem>
                        <SelectItem value="EAN13">EAN13</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  <div class="flex flex-col gap-1">
                    <Label class="text-[10px] font-semibold text-muted-foreground">الارتفاع / Height (mm)</Label>
                    <Input
                      type="number"
                      value={selectedNode.props.height || 15}
                      oninput={(e) => updateNodeProp("height", parseFloat(e.currentTarget.value) || 15, "props")}
                      class="h-8 text-xs"
                    />
                  </div>
                </div>

              <!-- QRCode Node properties inspector -->
              {:else if selectedNode.type === "QRCode"}
                <div class="flex flex-col gap-3">
                  <div class="flex flex-col gap-1">
                    <Label class="text-[10px] font-semibold text-muted-foreground">رابط / بيانات الـ QR Code</Label>
                    <Input
                      type="text"
                      value={selectedNode.props.data || ""}
                      oninput={(e) => updateNodeProp("data", e.currentTarget.value, "props")}
                      class="h-8 text-xs font-mono"
                    />
                  </div>

                  <div class="flex flex-col gap-1">
                    <Label class="text-[10px] font-semibold text-muted-foreground">الحجم / Size (mm)</Label>
                    <Input
                      type="number"
                      value={selectedNode.props.size || 25}
                      oninput={(e) => updateNodeProp("size", parseFloat(e.currentTarget.value) || 25, "props")}
                      class="h-8 text-xs"
                    />
                  </div>
                </div>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Tab: Live Test Data Context Simulator -->
        <div class="p-4 flex-grow flex flex-col gap-3 overflow-hidden">
          <h3 class="font-bold text-foreground flex items-center gap-1.5 border-b border-border/40 pb-2">
            <Braces size={13} class="text-primary" />
            بيانات تجريبية / Mock Context
          </h3>
          <p class="text-[9px] text-muted-foreground">قم بتعديل JSON المفرش ديناميكياً لتحديث بيانات المعاينة المباشرة.</p>
          <div class="flex-grow flex flex-col gap-2 overflow-hidden h-full">
            <Textarea
              bind:value={testContext}
              class="w-full flex-grow font-mono text-[9px] min-h-[160px] bg-muted/10 focus:bg-background border-border overflow-y-auto"
              placeholder="Context JSON..."
            />
            <Button variant="outline" size="xs" onclick={generatePreview} class="w-full mt-1 cursor-pointer">
              <Play size={10} class="me-1" />
              تحديث المعاينة / Apply Context
            </Button>
          </div>
        </div>
      </aside>

    </div>
  {:else}
    <div class="flex-grow flex flex-col items-center justify-center gap-2">
      <RefreshCw size={24} class="animate-spin text-primary" />
      <p class="text-xs text-muted-foreground">جاري تحميل بيانات القالب والمخططات...</p>
    </div>
  {/if}
</div>
