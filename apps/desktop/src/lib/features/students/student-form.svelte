<!--
  @component StudentForm
  Next-Generation 2026 ERP Student Registration & Admission Command Center.
  Unified single-scroll command canvas with sticky live ledger, identity sidebar, and SettingsBottomBar.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount, untrack } from "svelte";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  ArrowLeft,
  ChevronDown,
  EllipsisHorizontal,
  FloppyDisk,
  Check,
  User,
  DocumentCheck,
  Tag,
  Home,
  CheckCircle,
  Clock,
  Lock,
  Printer,
  Sparkles,
  UserBadgeCheck,
} from "@orbis/icons";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@orbis/ui/dropdown-menu";

import { l, formatCurrency } from "$lib/core/i18n";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { StudentFormState } from "./form-state.svelte";
import { createClassesQuery } from "$lib/features/academics";
import { useQueryClient } from "@tanstack/svelte-query";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

// Sections
import ProfileSection from "./sections/profile-section.svelte";
import FamilySection from "./sections/family-section.svelte";
import LocationSection from "./sections/location-section.svelte";
import FinancialsSection from "./sections/financials-section.svelte";
import DocumentsSection from "./sections/documents-section.svelte";

let { studentId, orgId }: { studentId?: string; orgId: string } = $props();

const state = untrack(() => new StudentFormState(orgId || "", studentId));
const queryClient = useQueryClient();

async function handleSave(status?: string) {
  if (status) state.registrationStatus = status;
  await state.handleSave(queryClient);
}

// Fetch classes via TanStack Query
const classesQuery = createClassesQuery(() => orgId);

onMount(async () => {
  if (orgId) {
    state.orgId = orgId;
  }
  if (studentId) {
    state.studentId = studentId;
    state.isEditMode = true;
  }
  await state.init();
});

// Computed Document Compliance
const documentStats = $derived.by(() => {
  const total = state.document.fulfillments.length || state.document.requirements.length || 3;
  const fulfilled = state.document.fulfillments.filter(f => f.is_fulfilled).length || state.document.submittedDocs.length;
  const isComplete = fulfilled >= total && total > 0;
  return { total, fulfilled, isComplete };
});

const isFormDirty = $derived.by(() => {
  return (
    !!state.personal.nameAr ||
    !!state.personal.nameEn ||
    !!state.personal.classId ||
    !!state.personal.nationalId ||
    state.document.submittedDocs.length > 0 ||
    Object.keys(state.document.attachments).length > 0
  );
});

function handleReset() {
  state.personal.nameAr = "";
  state.personal.nameEn = "";
  state.personal.nationalId = "";
  state.personal.phone = "";
  state.personal.email = "";
  state.personal.classId = "";
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide text-foreground text-start pb-28">
  <div class="max-w-7xl mx-auto p-6 flex flex-col gap-6">
    
    <!-- Top Command Bar Header -->
    <div class="sticky top-0 z-30 flex items-center justify-between gap-4 flex-wrap shrink-0 border-b border-border/60 bg-background/95 py-3 -mx-6 px-6 backdrop-blur-md select-none">
      <div class="flex items-center gap-3 min-w-0">
        <Button
          variant="secondary"
          size="icon-sm"
          class="shrink-0 cursor-pointer"
          onclick={() => goto(localizeHref(`/${orgId}/students`))}
          aria-label={m.common_action_cancel()}
        >
          <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
        </Button>

        <div class="flex flex-col text-start">
          <div class="flex items-center gap-2">
            <h1 class="text-base font-bold text-foreground truncate max-w-sm">
              {state.personal.nameAr || state.personal.nameEn || (state.isEditMode ? l("تعديل بيانات الطالب", "Edit Student") : l("تسجيل طالب جديد", "New Student Admission"))}
            </h1>
            <Badge 
              variant={state.registrationStatus === "approved" ? "lime" : state.registrationStatus === "pending_review" ? "warning" : "secondary"}
              class="text-[10px] font-bold uppercase tracking-wider"
            >
              {#if state.registrationStatus === "approved"}
                {l("معتمد / مسجل", "Approved")}
              {:else if state.registrationStatus === "pending_review"}
                {l("قيد المراجعة", "Under Review")}
              {:else if state.registrationStatus === "withdrawn"}
                {l("منسحب", "Withdrawn")}
              {:else if state.registrationStatus === "graduated"}
                {l("متخرج", "Graduated")}
              {:else}
                {l("مسودة تسجيل", "Draft")}
              {/if}
            </Badge>
          </div>
          <span class="text-[11px] text-muted-foreground font-mono">
            {state.personal.studentNumber || (state.isEditMode ? "STU" : l("الرقم الأكاديمي سيولد تلقائياً", "Auto-Generated Academic ID"))}
          </span>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <!-- Print Template Action -->
        {#if state.availableTemplates.length > 0}
          <DropdownMenu>
            <DropdownMenuTrigger>
              <Button variant="outline" size="sm" class="h-8 text-xs font-semibold gap-1.5 cursor-pointer">
                <Printer class="h-3.5 w-3.5 text-muted-foreground" />
                <span class="truncate max-w-36">
                  {state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name_ar || 
                   state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name || 
                   l("طباعة الاستمارة", "Print Form")}
                </span>
                <ChevronDown class="h-3 w-3 text-muted-foreground" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="p-1 min-w-48 text-start" align="end">
              {#each state.availableTemplates as tpl}
                <DropdownMenuItem 
                  class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center justify-between gap-1.5" 
                  onclick={async () => { 
                    state.selectedTemplateId = tpl.id; 
                    await state.handleDownloadRegistrationForm(); 
                  }}
                >
                  <span>{tpl.name_ar || tpl.name}</span>
                  {#if state.selectedTemplateId === tpl.id}
                    <Check class="text-primary h-3.5 w-3.5" />
                  {/if}
                </DropdownMenuItem>
              {/each}
            </DropdownMenuContent>
          </DropdownMenu>
        {/if}
      </div>
    </div>

    <!-- Main 2-Column Responsive Layout -->
    <div class="grid grid-cols-1 xl:grid-cols-12 gap-6 items-start">
      
      <!-- Primary Form Sections Column (8 Columns) -->
      <div class="xl:col-span-8 flex flex-col gap-6">
        
        <!-- Section 1: Academic Identity & Personal Profile -->
        <ProfileSection {state} />

        <!-- Section 2: Family & Legal Guardianship -->
        <FamilySection {state} />

        <!-- Section 3: Geolocation & Educational Background -->
        <LocationSection {state} />

        <!-- Section 4: Financial Architecture & Tuition Ledger -->
        <FinancialsSection {state} />

        <!-- Section 5: Document Verification Hub -->
        <DocumentsSection {state} />
      </div>

      <!-- Contextual Live Command Sidebar (4 Columns) -->
      <div class="xl:col-span-4 flex flex-col gap-6 sticky top-16">
        
        <!-- Student ID & Admission Card Preview -->
        <Card class="w-full overflow-hidden border-border/80 bg-card/60 backdrop-blur-sm shadow-xs">
          <CardHeader class="py-3 px-5 border-b border-border/40 bg-muted/20 flex flex-row items-center justify-between">
            <div class="flex items-center gap-2">
              <User class="w-4 h-4 text-primary" />
              <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
                {l("بطاقة قيد الطالب الأكاديمية", "Student ID Pass Preview")}
              </CardTitle>
            </div>
            <Badge variant="outline" class="text-[9px] font-mono px-1.5 py-0">
              LIVE PREVIEW
            </Badge>
          </CardHeader>
          
          <CardContent class="p-5 flex flex-col gap-4 text-start">
            <div class="flex items-center gap-4">
              <div class="w-14 h-14 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary font-bold text-xl shrink-0 uppercase">
                {state.personal.nameAr ? state.personal.nameAr.slice(0, 1) : (state.personal.nameEn ? state.personal.nameEn.slice(0, 1) : "STU")}
              </div>
              <div class="flex flex-col min-w-0">
                <span class="text-sm font-bold text-foreground truncate">
                  {state.personal.nameAr || state.personal.nameEn || l("اسم الطالب الجديد", "New Student Name")}
                </span>
                <span class="text-xs font-mono text-muted-foreground">
                  {state.personal.nameEn || state.personal.nameAr || "Student Name (En)"}
                </span>
                <div class="flex items-center gap-1.5 mt-1">
                  <Badge variant="lime" class="text-[9px] font-mono font-bold px-1.5 py-0">
                    {state.personal.studentNumber || "STU-AUTO"}
                  </Badge>
                  <span class="text-[10px] text-muted-foreground capitalize">
                    {#if state.studyMode === "regular"}
                      {l("انتظام كلي", "Regular")}
                    {:else if state.studyMode === "distance_learning"}
                      {l("تعليم عن بعد", "Distance Learning")}
                    {:else if state.studyMode === "part_time"}
                      {l("انتظام جزئي", "Part-Time")}
                    {:else if state.studyMode === "homeschooled"}
                      {l("تعليم منزلي", "Homeschooled")}
                    {:else if state.studyMode === "auditor"}
                      {l("طالب مستمع", "Auditor")}
                    {:else if state.studyMode === "visiting"}
                      {l("طالب زائر", "Visiting")}
                    {:else if state.studyMode === "exchange"}
                      {l("تبادل طلابي", "Exchange")}
                    {:else if state.studyMode === "special_needs"}
                      {l("تربية خاصة", "Special Needs")}
                    {:else}
                      {state.studyMode}
                    {/if}
                  </span>
                </div>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-2 pt-2 border-t border-border/40 text-[11px]">
              <div class="flex flex-col gap-0.5">
                <span class="text-[10px] text-muted-foreground">{m.common_form_target_class()}:</span>
                <span class="font-semibold text-foreground truncate">{state.selectedClassName}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[10px] text-muted-foreground">{l("الجنسية", "Nationality")}:</span>
                <span class="font-semibold text-foreground truncate">{state.personal.nationality || "—"}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[10px] text-muted-foreground">{l("تاريخ الميلاد", "Date of Birth")}:</span>
                <span class="font-mono text-foreground">{state.personal.birthDate || "—"}</span>
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[10px] text-muted-foreground">{l("الهوية الوطنية", "National ID")}:</span>
                <span class="font-mono text-foreground truncate">{state.personal.nationalId || "—"}</span>
              </div>
            </div>
          </CardContent>
        </Card>

        <!-- Live Tuition & Financial Ledger Widget -->
        <Card class="w-full border-border/80 bg-card/60 backdrop-blur-sm shadow-xs">
          <CardHeader class="py-3 px-5 border-b border-border/40 bg-muted/20 flex flex-row items-center justify-between">
            <div class="flex items-center gap-2">
              <Tag class="w-4 h-4 text-primary" />
              <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
                {l("الملخص المالي ومقاصة الرسوم", "Tuition & Fees Ledger")}
              </CardTitle>
            </div>
            <Badge variant="outline" class="font-mono text-[9px]">
              {state.currency}
            </Badge>
          </CardHeader>

          <CardContent class="p-5 flex flex-col gap-3 text-start">
            <div class="flex justify-between items-center text-xs">
              <span class="text-muted-foreground">{m.common_form_summary_mandatory()}:</span>
              <span class="font-bold text-foreground font-mono">{formatCurrency(state.mandatoryTotal, state.currency)}</span>
            </div>

            <div class="flex justify-between items-center text-xs">
              <span class="text-muted-foreground">{m.common_form_summary_optional()}:</span>
              <span class="font-bold text-foreground font-mono">+{formatCurrency(state.optionalTotal, state.currency)}</span>
            </div>

            <div class="flex justify-between items-center text-xs">
              <span class="text-muted-foreground">{m.common_form_summary_discount()}:</span>
              <span class="font-bold text-destructive font-mono">-{formatCurrency(state.discountTotal, state.currency)}</span>
            </div>

            <div class="pt-2 border-t border-border flex justify-between items-center">
              <span class="text-xs font-bold text-foreground">{m.common_form_summary_net()}:</span>
              <span class="text-base font-black text-primary font-mono">{formatCurrency(state.netTotal, state.currency)}</span>
            </div>
          </CardContent>
        </Card>

        <!-- Document Compliance & Readiness Card -->
        <Card class="w-full border-border/80 bg-card/60 backdrop-blur-sm shadow-xs">
          <CardHeader class="py-3 px-5 border-b border-border/40 bg-muted/20 flex flex-row items-center justify-between">
            <div class="flex items-center gap-2">
              <DocumentCheck class="w-4 h-4 text-primary" />
              <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
                {l("جاهزية ملف المستندات", "Document Readiness")}
              </CardTitle>
            </div>
            <Badge variant={documentStats.isComplete ? "lime" : "outline"} class="text-[9px] font-mono">
              {documentStats.fulfilled} / {documentStats.total}
            </Badge>
          </CardHeader>

          <CardContent class="p-5 flex flex-col gap-2.5 text-start">
            <div class="flex items-center justify-between text-xs">
              <span class="text-muted-foreground">{l("اكتمال الملفات المطلوبة:", "Fulfillment:")}</span>
              <span class="font-bold text-foreground">{Math.round((documentStats.fulfilled / Math.max(1, documentStats.total)) * 100)}%</span>
            </div>
            <div class="w-full h-2 rounded-full bg-muted/60 overflow-hidden">
              <div 
                class="h-full rounded-full transition-all duration-300 {documentStats.isComplete ? 'bg-emerald-500' : 'bg-primary'}"
                style="width: {Math.min(100, Math.round((documentStats.fulfilled / Math.max(1, documentStats.total)) * 100))}%"
              ></div>
            </div>
            {#if !documentStats.isComplete}
              <span class="text-[10px] text-amber-500 font-medium">
                {l("توجد مستندات مطلوبة لم تستوف بعد", "Required admission documents pending upload")}
              </span>
            {:else}
              <span class="text-[10px] text-emerald-500 font-medium flex items-center gap-1">
                <CheckCircle class="w-3 h-3" />
                {l("كافة متطلبات القبول مستوفاة", "All admission requirements fulfilled")}
              </span>
            {/if}
          </CardContent>
        </Card>

      </div>
    </div>

  </div>

  <!-- Floating Student Admission Command Bar -->
  <div
    class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 w-[calc(100%-2rem)] max-w-4xl pointer-events-auto select-none"
  >
    <div class="bg-card/95 backdrop-blur-md border border-border/80 shadow-2xl rounded-xl p-3 flex flex-col sm:flex-row items-center justify-between gap-3">
      <!-- Left: Status & Context Indicator -->
      <div class="flex items-center gap-2.5 text-start w-full sm:w-auto">
        <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
          <UserBadgeCheck class="w-4 h-4" />
        </div>
        <div class="flex flex-col min-w-0">
          <div class="flex items-center gap-1.5">
            <span class="text-xs font-bold text-foreground truncate">
              {state.personal.nameAr || state.personal.nameEn || l("استمارة تسجيل طالب جديد", "Student Admission Application")}
            </span>
            <Badge variant="outline" class="text-[9px] font-mono uppercase px-1.5 py-0">
              {state.registrationStatus}
            </Badge>
          </div>
          <span class="text-[10px] text-muted-foreground truncate">
            {state.selectedClassName} • {state.personal.studentNumber || (state.isEditMode ? "STU" : l("الرقم الأكاديمي سيولد تلقائياً", "Auto-Generated Academic ID"))}
          </span>
        </div>
      </div>

      <!-- Right: Action Workflow Buttons -->
      <div class="flex items-center gap-2 flex-wrap justify-end w-full sm:w-auto shrink-0">
        <!-- Discard / Cancel Button -->
        <Button
          type="button"
          variant="ghost"
          size="sm"
          class="h-8 px-3 text-xs font-semibold text-muted-foreground hover:text-foreground cursor-pointer"
          onclick={() => goto(localizeHref(`/${orgId}/students`))}
          disabled={state.isSaving}
        >
          {m.common_action_cancel()}
        </Button>

        {#if isFormDirty}
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="h-8 px-2.5 text-xs text-muted-foreground hover:text-destructive cursor-pointer"
            onclick={handleReset}
            disabled={state.isSaving}
          >
            {l("مسح الحقول", "Clear")}
          </Button>
        {/if}

        <!-- Save as Draft Button -->
        <Button
          type="button"
          variant="secondary"
          size="sm"
          class="h-8 px-3 text-xs font-semibold gap-1.5 cursor-pointer bg-muted/60 hover:bg-muted text-foreground"
          onclick={() => handleSave("draft")}
          disabled={state.isSaving}
        >
          <span class="w-2 h-2 rounded-full bg-muted-foreground/60"></span>
          <span>{l("حفظ كمسودة", "Save Draft")}</span>
        </Button>

        <!-- Submit for Academic Approval Button -->
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="h-8 px-3 text-xs font-bold gap-1.5 cursor-pointer border-amber-500/40 text-amber-500 hover:bg-amber-500/10"
          onclick={() => handleSave("pending_review")}
          disabled={state.isSaving}
        >
          <Clock class="w-3.5 h-3.5" />
          <span>{l("إرسال للمراجعة والاعتماد", "Submit for Approval")}</span>
        </Button>

        <!-- Direct Enrollment & Save Button -->
        <Button
          type="button"
          variant="lime"
          size="sm"
          class="h-8 px-4 text-xs font-bold gap-1.5 shadow-xs cursor-pointer"
          onclick={() => handleSave("approved")}
          disabled={state.isSaving}
        >
          {#if state.isSaving}
            <FloppyDisk class="w-3.5 h-3.5 animate-spin" />
            <span>{m.common_state_saving()}</span>
          {:else}
            <CheckCircle class="w-3.5 h-3.5" />
            <span>{state.isEditMode ? m.settings_action_save_changes() : l("اعتماد وقيد مباشر", "Save & Direct Enroll")}</span>
          {/if}
        </Button>
      </div>

    </div>
  </div>
</div>
