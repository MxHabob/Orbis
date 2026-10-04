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
    toast.error(m.ui_copy_document_name_is_required_933c340a());
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

    toast.success(m.ui_copy_document_requirement_created_successfully_07533818());
    oncreated?.(created);
    open = false;
  } catch (err: any) {
    console.error("Failed to create document requirement:", err);
    toast.error(m.ui_copy_failed_to_create_document_requirement_081fa272());
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
              {m.ui_copy_new_admission_document_requirement_5f33626c()}
            </SheetTitle>
            <SheetDescription class="text-[11px] text-muted-foreground">
              {m.ui_copy_configure_document_prerequisites_mandatory_c9264d58()}
            </SheetDescription>
          </div>
        </div>
      </SheetHeader>

      <form onsubmit={handleSubmit} class="flex flex-col gap-5">
        <!-- Arabic Name -->
        <div class="flex flex-col gap-1.5">
          <Label for="docNameAr" class="text-xs font-semibold text-foreground flex items-center gap-1">
            {m.ui_copy_document_title_arabic_43c58298()}
            <span class="text-destructive">*</span>
          </Label>
          <Input
            id="docNameAr"
            placeholder={m.ui_copy_e_g_certified_birth_certificate_22f4f391()}
            bind:value={nameAr}
            class="h-9 text-xs"
            onblur={autoGenerateCode}
            autocomplete="off"
          />
        </div>

        <!-- English Name -->
        <div class="flex flex-col gap-1.5">
          <Label for="docNameEn" class="text-xs font-semibold text-foreground">
            {m.ui_copy_document_title_english_80382f1d()}
          </Label>
          <Input
            id="docNameEn"
            placeholder={m.settings_ui_document_requirement_example()}
            bind:value={nameEn}
            class="h-9 text-xs font-mono"
            onblur={autoGenerateCode}
            autocomplete="off"
          />
        </div>

        <!-- Code Identifier -->
        <div class="flex flex-col gap-1.5">
          <Label for="docCode" class="text-xs font-semibold text-foreground">
            {m.ui_copy_identifier_code_2a0602de()}
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
            {m.ui_copy_target_class_grade_level_057499df()}
          </Label>
          <ClassSelect
            bind:value={selectedClassId}
            showAllOption={true}
            allOptionLabel={m.ui_copy_all_classes_grade_levels_cd709249()}
            placeholder={m.ui_copy_select_class_grade_2d561044()}
          />
        </div>

        <!-- Student Condition -->
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-semibold text-foreground">
            {m.ui_copy_applicability_condition_edba6012()}
          </Label>
          <Select 
            type="single"
            value={condition}
            onValueChange={(v) => (condition = v || "all_students")}
          >
            <SelectTrigger class="h-9 text-xs text-start">
              {#if condition === "all_students"}
                {m.ui_copy_all_new_students_b14cd3e5()}
              {:else if condition === "national_only"}
                {m.ui_copy_nationals_citizens_only_9e9f080e()}
              {:else if condition === "foreign_only"}
                {m.ui_copy_foreign_expatriate_students_only_93492941()}
              {:else if condition === "transfer_students"}
                {m.ui_copy_transfer_students_only_bf6acf94()}
              {:else}
                {condition}
              {/if}
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="all_students" class="text-xs">{m.ui_copy_all_new_students_b14cd3e5()}</SelectItem>
              <SelectItem value="national_only" class="text-xs">{m.ui_copy_nationals_citizens_only_9e9f080e()}</SelectItem>
              <SelectItem value="foreign_only" class="text-xs">{m.ui_copy_foreign_expatriate_students_only_93492941()}</SelectItem>
              <SelectItem value="transfer_students" class="text-xs">{m.ui_copy_transfer_students_only_bf6acf94()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Policy Toggles -->
        <div class="flex flex-col gap-3 p-3.5 rounded-lg bg-muted/30 border border-border/60">
          <div class="flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-foreground">{m.ui_copy_mandatory_requirement_9a00fd7c()}</span>
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_required_for_admission_completion_016fc0af()}</span>
            </div>
            <Switch bind:checked={isMandatory} />
          </div>

          <div class="border-t border-border/40 my-0.5"></div>

          <div class="flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-bold text-amber-500 flex items-center gap-1">
                <Lock class="w-3 h-3" />
                {m.ui_copy_block_final_enrollment_08364e7c()}
              </span>
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_prevents_final_enrollment_confirmation_unt_f324bf22()}</span>
            </div>
            <Switch bind:checked={blocksEnrollment} />
          </div>
        </div>

        <!-- Description / Instructions -->
        <div class="flex flex-col gap-1.5">
          <Label for="docDesc" class="text-xs font-semibold text-foreground">
            {m.ui_copy_submission_guidelines_optional_2fc61745()}
          </Label>
          <Textarea
            id="docDesc"
            placeholder={m.ui_copy_e_g_must_be_officially_certified_2cb2dcd7()}
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
        <span>{isSubmitting ? m.common_state_saving() : m.ui_copy_create_requirement_d83dc34e()}</span>
      </Button>
    </SheetFooter>
  </SheetContent>
</Sheet>
