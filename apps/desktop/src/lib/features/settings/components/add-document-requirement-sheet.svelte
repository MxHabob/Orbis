<!--
  @component AddDocumentRequirementSheet
  Slide-over Sheet for creating and configuring admission document requirements.
  Dynamically maps requirements to real database grade levels and classes with zero hardcoding.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Textarea } from "@orbis/ui/textarea";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
  SheetFooter,
} from "@orbis/ui/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { DocumentCheck, Plus, Lock, Sparkles, BuildingOffice } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { api, type ClassDto, type DocumentRequirementDto } from "$lib/api";
import { ClassSelect } from "$lib/features/academics";

let {
  open = $bindable(false),
  oncreated,
}: {
  open: boolean;
  oncreated?: (req: DocumentRequirementDto) => void;
} = $props();

// Form State
let nameAr = $state("");
let nameEn = $state("");
let code = $state("");
let selectedClassId = $state<string>("all");
let condition = $state("all_students"); // all_students | national_only | foreign_only | transfer_students
let isMandatory = $state(true);
let blocksEnrollment = $state(true);
let description = $state("");
let isSubmitting = $state(false);

// Reset form when sheet opens
$effect(() => {
  if (open) {
    nameAr = "";
    nameEn = "";
    code = "";
    selectedClassId = "all";
    condition = "all_students";
    isMandatory = true;
    blocksEnrollment = true;
    description = "";
  }
});

// Auto-generate code from English or Arabic name
function autoGenerateCode() {
  if (!code && (nameEn || nameAr)) {
    code = (nameEn || nameAr)
      .trim()
      .toLowerCase()
      .replace(/\s+/g, "_")
      .replace(/[^a-z0-9_]/g, "") || `doc_${Date.now()}`;
  }
}

async function handleSubmit(e?: Event) {
  e?.preventDefault();
  if (!nameAr.trim() && !nameEn.trim()) {
    toast.error(l("الرجاء إدخال اسم المستند المطلوب", "Document name is required"));
    return;
  }

  const generatedCode = (code.trim() || nameEn.trim() || nameAr.trim())
    .toLowerCase()
    .replace(/\s+/g, "_")
    .replace(/[^a-z0-9_]/g, "") || `doc_${Date.now()}`;

  isSubmitting = true;
  try {
    const created = await api.createDocumentRequirement({
      code: generatedCode,
      name_ar: nameAr.trim() || nameEn.trim(),
      name_en: nameEn.trim() || nameAr.trim(),
      is_mandatory: isMandatory,
      blocks_enrollment: blocksEnrollment,
      target_type: selectedClassId !== "all" ? "grade_level" : "global",
      target_id: selectedClassId !== "all" ? selectedClassId : undefined,
      condition,
    });

    toast.success(l("تم إنشاء متطلب المستند بنجاح", "Document requirement created successfully"));
    oncreated?.(created);
    open = false;
  } catch (err: any) {
    console.error("Failed to create document requirement:", err);
    toast.error(l("فشل إنشاء متطلب المستند", "Failed to create document requirement"));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="w-full sm:max-w-lg p-0 flex flex-col justify-between bg-background overflow-y-auto">
    <div class="p-6 flex flex-col gap-6 text-start">
      <SheetHeader class="border-b border-border/40 pb-4">
        <div class="flex items-center gap-2">
          <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
            <DocumentCheck class="w-4 h-4" />
          </div>
          <div class="flex flex-col text-start">
            <SheetTitle class="text-sm font-bold text-foreground">
              {l("إضافة متطلب مستند أو وثيقة قبول جديدة", "New Admission Document Requirement")}
            </SheetTitle>
            <SheetDescription class="text-[11px] text-muted-foreground">
              {l("تخصيص قواعد الوثائق المطلوبة وإلزاميتها وحجب التسجيل حسب الصف الدراسي", "Configure document prerequisites, mandatory rules, and enrollment blockers")}
            </SheetDescription>
          </div>
        </div>
      </SheetHeader>

      <form onsubmit={handleSubmit} class="flex flex-col gap-5">
        <!-- Arabic Name -->
        <div class="flex flex-col gap-1.5">
          <Label for="docNameAr" class="text-xs font-semibold text-foreground flex items-center gap-1">
            {l("اسم المستند (بالعربية)", "Document Title (Arabic)")}
            <span class="text-destructive">*</span>
          </Label>
          <Input
            id="docNameAr"
            placeholder={l("مثال: شهادة الميلاد الأصلية المعتمدة", "e.g. Certified Birth Certificate")}
            bind:value={nameAr}
            class="h-9 text-xs"
            onblur={autoGenerateCode}
            autocomplete="off"
          />
        </div>

        <!-- English Name -->
        <div class="flex flex-col gap-1.5">
          <Label for="docNameEn" class="text-xs font-semibold text-foreground">
            {l("اسم المستند (بالإنجليزية)", "Document Title (English)")}
          </Label>
          <Input
            id="docNameEn"
            placeholder="e.g. Certified Birth Certificate"
            bind:value={nameEn}
            class="h-9 text-xs font-mono"
            onblur={autoGenerateCode}
            autocomplete="off"
          />
        </div>

        <!-- Code Identifier -->
        <div class="flex flex-col gap-1.5">
          <Label for="docCode" class="text-xs font-semibold text-foreground">
            {l("الرمز التعريفي الفريد (Code)", "Identifier Code")}
          </Label>
          <Input
            id="docCode"
            placeholder="e.g. birth_certificate"
            bind:value={code}
            class="h-9 text-xs font-mono lowercase"
            autocomplete="off"
          />
        </div>

        <!-- Target Class / Grade Level via pre-built ClassSelect -->
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-semibold text-foreground">
            {l("الصف أو المرحلة التعليمية المستهدفة", "Target Class / Grade Level")}
          </Label>
          <ClassSelect
            bind:value={selectedClassId}
            showAllOption={true}
            allOptionLabel={l("كافة الصفوف والمراحل الدراسية", "All Classes & Grade Levels")}
            placeholder={l("اختر الصف أو المرحلة...", "Select Class / Grade...")}
          />
        </div>

        <!-- Student Condition -->
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-semibold text-foreground">
            {l("شرط انطباق المتطلب على الطالب", "Applicability Condition")}
          </Label>
          <Select 
            type="single"
            value={condition}
            onValueChange={(v) => (condition = v || "all_students")}
          >
            <SelectTrigger class="h-9 text-xs text-start">
              {#if condition === "all_students"}
                {l("كافة الطلاب الجدد دون استثناء", "All New Students")}
              {:else if condition === "national_only"}
                {l("الطلاب الوطنيون (المواطنون فقط)", "Nationals / Citizens Only")}
              {:else if condition === "foreign_only"}
                {l("الطلاب الوافدون / الأجانب فقط", "Foreign / Expatriate Students Only")}
              {:else if condition === "transfer_students"}
                {l("الطلاب المحولون من مدارس أخرى فقط", "Transfer Students Only")}
              {:else}
                {condition}
              {/if}
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="all_students" class="text-xs">{l("كافة الطلاب الجدد دون استثناء", "All New Students")}</SelectItem>
              <SelectItem value="national_only" class="text-xs">{l("الطلاب الوطنيون (المواطنون فقط)", "Nationals / Citizens Only")}</SelectItem>
              <SelectItem value="foreign_only" class="text-xs">{l("الطلاب الوافدون / الأجانب فقط", "Foreign / Expatriate Students Only")}</SelectItem>
              <SelectItem value="transfer_students" class="text-xs">{l("الطلاب المحولون من مدارس أخرى فقط", "Transfer Students Only")}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Policy Toggles -->
        <div class="flex flex-col gap-3 p-3.5 rounded-lg bg-muted/30 border border-border/60">
          <div class="flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">{l("متطلب إلزامي للقبول", "Mandatory Requirement")}</span>
              <span class="text-[10px] text-muted-foreground">{l("يلزم الطالب بتقديم الوثيقة لاستكمال إجراءات القبول", "Required for admission completion")}</span>
            </div>
            <Switch bind:checked={isMandatory} />
          </div>

          <div class="border-t border-border/40 my-0.5"></div>

          <div class="flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-amber-500 flex items-center gap-1">
                <Lock class="w-3 h-3" />
                {l("حجب اعتماد القيد حتى رفع الوثيقة", "Block Final Enrollment")}
              </span>
              <span class="text-[10px] text-muted-foreground">{l("يمنع تسكين الطالب واعتماد ملفه نهائياً حتى يتم رفع الوثيقة", "Prevents final enrollment confirmation until uploaded")}</span>
            </div>
            <Switch bind:checked={blocksEnrollment} />
          </div>
        </div>

        <!-- Description / Instructions -->
        <div class="flex flex-col gap-1.5">
          <Label for="docDesc" class="text-xs font-semibold text-foreground">
            {l("ملاحظات وتعليمات تقديم الوثيقة (اختياري)", "Submission Guidelines (Optional)")}
          </Label>
          <Textarea
            id="docDesc"
            placeholder={l("مثال: يجب أن تكون الشهادة مصدقة من وزارة التربية والتعليم...", "e.g. Must be officially certified...")}
            bind:value={description}
            class="h-18 text-xs resize-none"
          />
        </div>
      </form>
    </div>

    <!-- Sheet Footer Actions -->
    <SheetFooter class="p-6 border-t border-border/40 bg-muted/20 flex flex-row items-center justify-end gap-2">
      <Button
        variant="outline"
        size="sm"
        class="h-9 px-4 text-xs font-semibold cursor-pointer"
        onclick={() => (open = false)}
        disabled={isSubmitting}
      >
        {m.common_action_cancel()}
      </Button>

      <Button
        variant="lime"
        size="sm"
        class="h-9 px-5 text-xs font-bold gap-1.5 cursor-pointer"
        onclick={handleSubmit}
        disabled={isSubmitting}
      >
        <Plus class="w-4 h-4" />
        <span>{isSubmitting ? m.common_state_saving() : l("حفظ وإضافة المتطلب", "Create Requirement")}</span>
      </Button>
    </SheetFooter>
  </SheetContent>
</Sheet>
