<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "@orbis/ui-svelte/card";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogAction,
  AlertDialogCancel,
} from "@orbis/ui-svelte/alert-dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  FileText,
  Plus,
  Search,
  Trash2,
  Edit,
  CheckCircle,
  Archive,
  AlertCircle,
  RefreshCw,
} from "lucide-svelte";

// State runes
let templates = $state<any[]>([]);
let loading = $state(true);
let searchQuery = $state("");
let filterEntityType = $state("all");
let filterDocType = $state("all");

// Dialog & Dialog State
let isCreateDialogOpen = $state(false);
let newTemplateName = $state("");
let newTemplateNameAr = $state("");
let newTemplateDesc = $state("");
let newTemplateEntityType = $state("enrollment");
let newTemplateDocType = $state("CERTIFICATE");
let newTemplatePageSize = $state("A4");
let newTemplateOrientation = $state("portrait");

// Alert Dialog delete state
let isDeleteDialogOpen = $state(false);
let templateToDeleteId = $state("");

// Dynamic parameters from route
const orgId: string = $derived(page.params.orgId ?? "");

// Load templates from database
async function loadTemplates() {
  if (!orgId) return;
  loading = true;
  try {
    const listParams: any = {};
    if (filterEntityType !== "all") listParams.entity_type = filterEntityType;
    if (filterDocType !== "all") listParams.document_type = filterDocType;
    if (searchQuery.trim() !== "") listParams.search = searchQuery.trim();

    const response = await commands.documentTemplates.getTemplates(
      orgId,
      listParams,
    );
    templates = response || [];
  } catch (e: unknown) {
    console.error("Failed to load templates:", e);
    const msg = (e as any)?.message || "Failed to load templates";
    toast.error(msg);
  } finally {
    loading = false;
  }
}

// React to query changes
$effect(() => {
  if (orgId) {
    loadTemplates();
  }
});

// Trigger load on mounting
onMount(() => {
  if (orgId) {
    loadTemplates();
  }
});

// Handle Template Creation
async function handleCreateTemplate() {
  if (!newTemplateName.trim()) {
    toast.error("Template name in English is required");
    return;
  }

  try {
    const defaultDocJson = JSON.stringify({
      version: "1.0",
      meta: {
        name: newTemplateName,
        nameAr: newTemplateNameAr || null,
        documentType: newTemplateDocType,
        entityType: newTemplateEntityType,
        locale: getLocale(),
        pageSetup: {
          size: newTemplatePageSize,
          orientation: newTemplateOrientation,
          margins: { top: 15, bottom: 15, left: 15, right: 15 },
          unit: "mm",
        },
      },
      variables: {},
      assets: {},
      root: {
        type: "Page",
        id: "page-root",
        children: [
          {
            type: "Frame",
            id: "content-root",
            props: {
              layout: "vertical",
              gap: 10,
            },
            children: [
              {
                type: "Text",
                id: "main-title",
                props: {
                  content: newTemplateNameAr || newTemplateName,
                  fontSize: 24,
                  fontFamily: "Tahoma",
                  fontWeight: "bold",
                  textAlign: "center",
                },
              },
            ],
          },
        ],
      },
    });

    const createRequest = {
      name: newTemplateName,
      name_ar: newTemplateNameAr || null,
      description: newTemplateDesc || null,
      description_ar: null,
      document_type: newTemplateDocType,
      entity_type: newTemplateEntityType,
      category_id: null,
      code: null,
      page_size: newTemplatePageSize,
      orientation: newTemplateOrientation,
      margins_json: JSON.stringify({
        top: 15,
        bottom: 15,
        left: 15,
        right: 15,
      }),
      document_json: defaultDocJson,
      locale: getLocale(),
      tags: null,
    };

    // User ID mock/context or fetch from auth
    const userId = "current-user";
    const response = await commands.documentTemplates.createTemplate(
      orgId,
      userId,
      createRequest,
    );

    toast.success("Template created successfully");
    isCreateDialogOpen = false;

    // Redirect to editor page
    goto(`/${orgId}/settings/templates/${response.id}`);
  } catch (e: unknown) {
    console.error("Failed to create template:", e);
    const msg = (e as any)?.message || "Failed to create template";
    toast.error(msg);
  }
}

// Trigger Delete
function confirmDelete(id: string) {
  templateToDeleteId = id;
  isDeleteDialogOpen = true;
}

async function handleDeleteTemplate() {
  if (!templateToDeleteId) return;

  try {
    await commands.documentTemplates.deleteTemplate(orgId, templateToDeleteId);
    toast.success("Template deleted successfully");
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to delete template:", e);
    const msg = (e as any)?.message || "Failed to delete template";
    toast.error(msg);
  } finally {
    isDeleteDialogOpen = false;
    templateToDeleteId = "";
  }
}

// Handle Template Publishing/Activation
async function publishTemplate(id: string) {
  try {
    const userId = "current-user";
    await commands.documentTemplates.publishTemplate(orgId, id, userId);
    toast.success(m.templates_publish_success());
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to publish template:", e);
    const msg = (e as any)?.message || "Failed to publish template";
    toast.error(msg);
  }
}

// Handle Template Archiving
async function archiveTemplate(id: string) {
  try {
    await commands.documentTemplates.archiveTemplate(orgId, id);
    toast.success(m.templates_archive_success());
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to archive template:", e);
    const msg = (e as any)?.message || "Failed to archive template";
    toast.error(msg);
  }
}

// Handle Template Restoring
async function restoreTemplate(id: string) {
  try {
    await commands.documentTemplates.restoreTemplate(orgId, id);
    toast.success(m.templates_restore_success());
    loadTemplates();
  } catch (e: unknown) {
    console.error("Failed to restore template:", e);
    const msg = (e as any)?.message || "Failed to restore template";
    toast.error(msg);
  }
}
</script>

<svelte:head>
  <title>{getLocale() === "ar" ? "قوالب المستندات" : "Document Templates"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    <!-- Header Action Bar -->
    <div class="flex items-center justify-between gap-4 flex-nowrap border-b border-border/60 pb-4">
      <div class="flex items-center gap-2">
        {#if !isLoading}
          <span class="text-[10px] px-2 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
            {getLocale() === "ar" ? `قوالب المستندات (${templates.length})` : `Templates (${templates.length})`}
          </span>
        {/if}
      </div>
      <div class="flex items-center gap-2 flex-nowrap shrink-0">
        <Button variant="lime" size="sm" onclick={() => (isCreateDialogOpen = true)}>
          <Plus size={16} class="me-1.5" />
          {m.templates_create_btn()}
        </Button>
      </div>
    </div>

    <!-- Filters and Search -->
    <div class="flex flex-col sm:flex-row gap-4 items-center justify-between bg-card border border-border p-4 rounded-xl">
      <div class="relative w-full sm:max-w-md">
        <Search class="absolute start-3 top-2.5 h-4 w-4 text-muted-foreground" />
        <Input
          type="text"
          placeholder="ابحث بالقوالب... / Search templates..."
          class="ps-9"
          bind:value={searchQuery}
          oninput={loadTemplates}
        />
      </div>

      <div class="flex flex-wrap items-center gap-3 w-full sm:w-auto">
        <div class="flex flex-col gap-1 text-start min-w-[130px]">
          <Select type="single" bind:value={filterEntityType} onValueChange={loadTemplates}>
            <SelectTrigger class="h-9">
              {({ all: "كل الكيانات (All)", enrollment: "تسجيل الطلاب (Enrollment)", payment: "سندات القبض (Payment)", exam: "أرقام الجلوس (Exam)", user: "بطاقات التعريف (User)" } as any)[filterEntityType] || "نوع السجل"}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">كل الكيانات (All)</SelectItem>
              <SelectItem value="enrollment">تسجيل الطلاب (Enrollment)</SelectItem>
              <SelectItem value="payment">سندات القبض (Payment)</SelectItem>
              <SelectItem value="exam">أرقام الجلوس (Exam)</SelectItem>
              <SelectItem value="user">بطاقات التعريف (User)</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-1 text-start min-w-[130px]">
          <Select type="single" bind:value={filterDocType} onValueChange={loadTemplates}>
            <SelectTrigger class="h-9">
              {({ all: "كل الأنواع (All)", CERTIFICATE: "شهادات (Certificate)", ID_CARD: "بطاقات (ID Card)", RECEIPT: "سندات (Receipt)", EXAM_CARD: "بطاقات جلوس (Seating)" } as any)[filterDocType] || "نوع المستند"}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">كل الأنواع (All)</SelectItem>
              <SelectItem value="CERTIFICATE">شهادات (Certificate)</SelectItem>
              <SelectItem value="ID_CARD">بطاقات (ID Card)</SelectItem>
              <SelectItem value="RECEIPT">سندات (Receipt)</SelectItem>
              <SelectItem value="EXAM_CARD">بطاقات جلوس (Seating)</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
    </div>

    <!-- Templates Grid -->
    {#if loading}
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        {#each Array(3) as _}
          <div class="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 animate-pulse">
            <div class="h-6 w-2/3 bg-muted rounded"></div>
            <div class="h-4 w-full bg-muted rounded"></div>
            <div class="h-8 w-1/2 bg-muted rounded mt-2"></div>
          </div>
        {/each}
      </div>
    {:else if templates.length === 0}
      <div class="flex flex-col items-center justify-center py-16 px-4 bg-card border border-border border-dashed rounded-xl gap-4">
        <div class="bg-muted p-4 rounded-full text-muted-foreground">
          <FileText size={32} />
        </div>
        <div class="flex flex-col gap-1">
          <h3 class="text-sm font-semibold text-foreground">{m.templates_empty()}</h3>
          <p class="text-xs text-muted-foreground">{m.templates_empty_desc()}</p>
        </div>
        <Button variant="outline" size="sm" onclick={() => (isCreateDialogOpen = true)}>
          <Plus size={14} class="me-1" />
          {m.templates_create_btn()}
        </Button>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-start">
        {#each templates as template (template.id)}
          <Card class="flex flex-col h-full hover:border-primary/40 transition-all duration-200">
            <CardHeader class="pb-3">
              <div class="flex items-start justify-between gap-2">
                <Badge variant={template.status === 'published' ? 'lime' : (template.status === 'archived' ? 'outline' : 'secondary')}>
                  {template.status === 'published' ? m.templates_status_active() : (template.status === 'archived' ? m.templates_status_archived() : m.templates_status_draft())}
                </Badge>
                <div class="text-[10px] text-muted-foreground">v{template.version}</div>
              </div>
              <CardTitle class="text-sm font-bold text-foreground mt-2">
                {getLocale() === 'ar' && template.name_ar ? template.name_ar : template.name}
              </CardTitle>
              <CardDescription class="text-xs text-muted-foreground line-clamp-2 mt-1">
                {template.description || m.templates_no_description()}
              </CardDescription>
            </CardHeader>
            
            <CardContent class="flex-grow pb-4 flex flex-col gap-2">
              <div class="flex items-center justify-between text-xs text-muted-foreground border-t border-border/40 pt-3">
                <span>{m.templates_label_doc_type()}</span>
                <span class="font-medium text-foreground">{template.document_type}</span>
              </div>
              <div class="flex items-center justify-between text-xs text-muted-foreground">
                <span>{m.templates_label_entity_type()}</span>
                <span class="font-medium text-foreground">{template.entity_type}</span>
              </div>
              <div class="flex items-center justify-between text-xs text-muted-foreground">
                <span>{m.templates_label_size()}</span>
                <span class="font-medium text-foreground">{template.page_size} ({template.orientation})</span>
              </div>
            </CardContent>

            <CardFooter class="border-t border-border/40 pt-3 flex justify-between gap-2 bg-muted/10 flex-nowrap overflow-x-auto">
              <div class="flex items-center gap-1 shrink-0">
                <Button
                  variant="destructive"
                  size="icon-xs"
                  class="h-8 w-8 rounded-lg cursor-pointer"
                  onclick={() => confirmDelete(template.id)}
                >
                  <Trash2 size={13} />
                </Button>
              </div>
              
              <div class="flex items-center gap-1.5 shrink-0">
                <Button
                  variant="outline"
                  size="xs"
                  class="cursor-pointer"
                  href={`/${orgId}/settings/templates/${template.id}`}
                >
                  <Edit size={13} class="me-1" />
                  {m.templates_action_edit()}
                </Button>

                {#if template.status === 'published'}
                  <Button
                    variant="outline"
                    size="xs"
                    class="cursor-pointer"
                    onclick={() => archiveTemplate(template.id)}
                  >
                    <Archive size={13} class="me-1" />
                    {m.templates_action_archive()}
                  </Button>
                {:else if template.status === 'archived'}
                  <Button
                    variant="outline"
                    size="xs"
                    class="cursor-pointer"
                    onclick={() => restoreTemplate(template.id)}
                  >
                    <RefreshCw size={13} class="me-1" />
                    {m.templates_action_restore()}
                  </Button>
                {:else}
                  <Button
                    variant="lime"
                    size="xs"
                    class="cursor-pointer"
                    onclick={() => publishTemplate(template.id)}
                  >
                    <CheckCircle size={13} class="me-1" />
                    {m.templates_action_activate()}
                  </Button>
                {/if}
              </div>
            </CardFooter>
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- CREATE DIALOG -->
<Dialog bind:open={isCreateDialogOpen}>
  <DialogContent class="sm:max-w-[480px] text-start">
    <DialogHeader>
      <DialogTitle>{m.templates_create_btn()}</DialogTitle>
      <DialogDescription>
        أدخل تفاصيل قالب المستندات الجديد لبدء تصميمه في المحرر.
      </DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-2">
        <Label for="name-en" class="text-xs font-semibold text-muted-foreground">اسم القالب بالإنجليزية (Name EN) *</Label>
        <Input id="name-en" bind:value={newTemplateName} placeholder="e.g. Student Certificate" />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="name-ar" class="text-xs font-semibold text-muted-foreground">اسم القالب بالعربية (Name AR)</Label>
        <Input id="name-ar" bind:value={newTemplateNameAr} placeholder="مثال: شهادة طالب" />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="desc" class="text-xs font-semibold text-muted-foreground">وصف القالب / Description</Label>
        <Input id="desc" bind:value={newTemplateDesc} placeholder="وصف موجز للقالب..." />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">نوع السجل / Entity</Label>
          <Select type="single" bind:value={newTemplateEntityType}>
            <SelectTrigger>
              {({ enrollment: "تسجيل الطلاب", payment: "سندات القبض", exam: "أرقام الجلوس", user: "بطاقات التعريف" } as any)[newTemplateEntityType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="enrollment">تسجيل الطلاب</SelectItem>
              <SelectItem value="payment">سندات القبض</SelectItem>
              <SelectItem value="exam">أرقام الجلوس</SelectItem>
              <SelectItem value="user">بطاقات التعريف</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">نوع المستند / Document</Label>
          <Select type="single" bind:value={newTemplateDocType}>
            <SelectTrigger>
              {({ CERTIFICATE: "شهادات (Certificate)", ID_CARD: "بطاقات (ID Card)", RECEIPT: "سندات (Receipt)", EXAM_CARD: "بطاقات جلوس (Seating)" } as any)[newTemplateDocType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="CERTIFICATE">شهادات (Certificate)</SelectItem>
              <SelectItem value="ID_CARD">بطاقات (ID Card)</SelectItem>
              <SelectItem value="RECEIPT">سندات (Receipt)</SelectItem>
              <SelectItem value="EXAM_CARD">بطاقات جلوس (Seating)</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">حجم الصفحة / Size</Label>
          <Select type="single" bind:value={newTemplatePageSize}>
            <SelectTrigger>
              {({ A4: "A4 (210 x 297 mm)", A5: "A5 (148 x 210 mm)", Letter: "Letter (8.5 x 11 in)", Custom: "Custom" } as any)[newTemplatePageSize]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="A4">A4 (210 x 297 mm)</SelectItem>
              <SelectItem value="A5">A5 (148 x 210 mm)</SelectItem>
              <SelectItem value="Letter">Letter (8.5 x 11 in)</SelectItem>
              <SelectItem value="Custom">Custom</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">الاتجاه / Orientation</Label>
          <Select type="single" bind:value={newTemplateOrientation}>
            <SelectTrigger>
              {({ portrait: "طولي (Portrait)", landscape: "عرضي (Landscape)" } as any)[newTemplateOrientation]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="portrait">طولي (Portrait)</SelectItem>
              <SelectItem value="landscape">عرضي (Landscape)</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
    </div>

    <DialogFooter class="gap-2 sm:gap-0 mt-4">
      <Button variant="outline" size="sm" onclick={() => (isCreateDialogOpen = false)}>إلغاء / Cancel</Button>
      <Button variant="lime" size="sm" onclick={handleCreateTemplate}>إنشاء ومتابعة / Create</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<!-- DELETE ALERT DIALOG -->
<AlertDialog bind:open={isDeleteDialogOpen}>
  <AlertDialogContent class="text-start">
    <AlertDialogHeader>
      <AlertDialogTitle class="flex items-center gap-2 text-destructive">
        <AlertCircle size={20} />
        <span>تأكيد الحذف / Confirm Delete</span>
      </AlertDialogTitle>
      <AlertDialogDescription>
        هل أنت متأكد من رغبتك في حذف هذا القالب نهائياً؟ لن تتمكن من استرجاع البيانات أو استخدام هذا القالب للطباعة مجدداً.
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter class="gap-2 sm:gap-0">
      <AlertDialogCancel>إلغاء / Cancel</AlertDialogCancel>
      <AlertDialogAction class="bg-destructive hover:bg-destructive/90 text-white cursor-pointer" onclick={handleDeleteTemplate}>
        حذف نهائي / Delete
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
