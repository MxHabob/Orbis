<!--
  @component ProfileSection
  Academic Identity & Student Personal Profile.
  Includes Avatar upload, NationalitySelect, CapacityIndicator with class select,
  Study Mode segmented control, and live authoritative duplicate checks.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { User, Sparkles, BookOpen } from "@orbis/icons";
import { l } from "$lib/core/i18n";

import type { StudentFormState } from "../form-state.svelte";
import { ClassSelect } from "$lib/features/academics";
import DateOfBirthField from "../components/date-of-birth-field.svelte";
import NationalitySelect from "$lib/shared/components/nationality-select.svelte";
import CapacityIndicator from "$lib/shared/components/capacity-indicator.svelte";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";

let { state }: { state: StudentFormState } = $props();

const studyModes = [
  { value: "regular", labelAr: "انتظام كلي (دراسة اعتيادية)", labelEn: "Regular Full-Time" },
  { value: "part_time", labelAr: "انتظام جزئي", labelEn: "Part-Time" },
  { value: "distance_learning", labelAr: "تعليم عن بعد / إلكتروني", labelEn: "Distance Learning" },
  { value: "homeschooled", labelAr: "تعليم منزلي", labelEn: "Homeschooled" },
  { value: "auditor", labelAr: "طالب مستمع", labelEn: "Auditor" },
  { value: "visiting", labelAr: "طالب زائر", labelEn: "Visiting Student" },
  { value: "exchange", labelAr: "تبادل طلابي", labelEn: "Student Exchange" },
  { value: "special_needs", labelAr: "تربية خاصة ورعاية تخصصية", labelEn: "Special Needs Education" },
];
</script>

<Card class="w-full border-border/80 shadow-xs">
  <CardHeader class="py-3 px-6 border-b border-border/40 flex flex-row items-center justify-between">
    <div class="flex items-center gap-2">
      <User class="w-4 h-4 text-primary" />
      <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
        {l("الهوية الأكاديمية والبيانات الشخصية", "Academic Identity & Personal Profile")}
      </CardTitle>
    </div>
    <span class="text-[11px] text-muted-foreground">
      {l("الحقول المميزة بـ (*) إلزامية", "Fields marked with (*) are required")}
    </span>
  </CardHeader>

  <CardContent class="p-0">
    <div class="flex flex-col divide-y divide-border">
      
      <!-- Student Avatar & Study Mode -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div class="flex items-center gap-4 text-start">
          <AvatarUpload
            bind:avatarUrl={state.avatarUrl}
            bind:file={state.avatarFile}
            deferred={true}
            entityId={state.studentId || "new-student"}
            fallbackText={state.personal.nameAr || state.personal.nameEn || "ST"}
            size={60}
          />
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-bold text-foreground">{l("الصورة الشخصية للطالب", "Student Profile Photo")}</span>
            <span class="text-[11px] text-muted-foreground">{l("تستخدم في البطاقة المدرسية وسجل القيد", "Used for student ID pass and official records")}</span>
          </div>
        </div>

        <!-- Study Mode Selector -->
        <div class="flex flex-col gap-1.5 text-start w-full md:w-72">
          <Label class="text-xs font-semibold text-foreground">{l("نوع ونمط الدراسة الأكاديمي", "Study Mode & Enrollment Track")}</Label>
          <Select
            type="single"
            value={state.studyMode}
            onValueChange={(v) => { if (v) state.studyMode = v; }}
          >
            <SelectTrigger class="h-9 text-xs bg-background">
              {studyModes.find(m => m.value === state.studyMode) 
                ? l(studyModes.find(m => m.value === state.studyMode)!.labelAr, studyModes.find(m => m.value === state.studyMode)!.labelEn)
                : state.studyMode}
            </SelectTrigger>
            <SelectContent class="text-start max-h-64 overflow-y-auto" align="end">
              {#each studyModes as mode (mode.value)}
                <SelectItem value={mode.value} class="text-xs">
                  {l(mode.labelAr, mode.labelEn)}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <!-- Arabic Name -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="nameAr" class="text-xs font-semibold text-foreground flex items-center gap-1">
            {m.common_form_full_name_ar()}
            <span class="text-destructive">*</span>
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_name_ar_desc()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0">
          <Input 
            id="nameAr"
            type="text" 
            placeholder={m.common_form_full_name_ar_placeholder()} 
            class="h-9 w-full bg-background" 
            bind:value={state.personal.nameAr}
            autocomplete="off"
          />
        </div>
      </div>

      <!-- English Name -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="nameEn" class="text-xs font-semibold text-foreground">
            {m.common_form_full_name_en()}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_name_en_desc()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0">
          <Input 
            id="nameEn"
            type="text" 
            placeholder={m.common_form_full_name_en_placeholder()} 
            class="h-9 w-full bg-background font-mono text-left" 
            bind:value={state.personal.nameEn}
            autocomplete="off"
          />
        </div>
      </div>

      <!-- Target Class & Section with CapacityIndicator -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="classSelect" class="text-xs font-semibold text-foreground flex items-center gap-1">
            {m.common_form_target_class()}
            <span class="text-destructive">*</span>
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_class_desc()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex flex-col gap-2">
          <ClassSelect
            id="classSelect"
            bind:value={state.personal.classId}
            classes={state.classes}
          />
          {#if state.selectedClass}
            <div class="flex items-center justify-between px-1">
              <span class="text-[10px] text-muted-foreground">{l("سعة الشعبة:", "Capacity:")}</span>
              <CapacityIndicator 
                current={state.selectedClass.student_count || 0} 
                max={state.selectedClass.capacity || 30} 
                size="sm"
                class="w-44"
              />
            </div>
          {/if}
        </div>
      </div>

      <!-- National ID / Civil Number -->
      {#if state.activeFields.national_id}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="nationalId" class="text-xs font-semibold text-foreground">
              {m.common_form_profile_nid_label()} 
              {#if state.requiredFields.national_id}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_nid_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0 flex flex-col gap-1.5">
            <Input 
              id="nationalId"
              type="text" 
              placeholder="1000234900" 
              class="h-9 w-full bg-background font-mono text-start {state.duplicateWarnings.national_id ? 'border-destructive focus-visible:ring-destructive' : ''}" 
              bind:value={state.personal.nationalId}
              oninput={() => state.triggerDuplicateCheck("national_id", state.personal.nationalId)}
            />
            {#if state.duplicateWarnings.national_id}
              <div class="flex items-center gap-1.5 text-[11px] text-destructive font-medium text-start">
                <span>{state.duplicateWarnings.national_id}</span>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- Passport Number -->
      {#if state.activeFields.passport_number}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="passportNumber" class="text-xs font-semibold text-foreground">
              {m.common_form_passport_number()}
              {#if state.requiredFields.passport_number}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_passport_number_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="passportNumber"
              type="text" 
              placeholder={m.common_form_passport_number_placeholder()} 
              class="h-9 w-full bg-background font-mono uppercase" 
              bind:value={state.personal.passportNumber}
            />
          </div>
        </div>
      {/if}

      <!-- Nationality (Shared Component) -->
      {#if state.activeFields.nationality}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="nationality" class="text-xs font-semibold text-foreground">
              {m.common_form_nationality()}
              {#if state.requiredFields.nationality}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_nationality_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <NationalitySelect
              bind:value={state.personal.nationality}
              placeholder={m.common_form_nationality_placeholder()}
            />
          </div>
        </div>
      {/if}

      <!-- Place of Birth -->
      {#if state.activeFields.place_of_birth}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="placeOfBirth" class="text-xs font-semibold text-foreground">
              {m.common_form_place_of_birth()}
              {#if state.requiredFields.place_of_birth}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_place_of_birth_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="placeOfBirth"
              type="text" 
              placeholder={m.common_form_place_of_birth_placeholder()} 
              class="h-9 w-full bg-background" 
              bind:value={state.personal.placeOfBirth}
            />
          </div>
        </div>
      {/if}

      <!-- Birth Date -->
      {#if state.activeFields.birth_date}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.common_bulk_col_dob()}
              {#if state.requiredFields.birth_date}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_dob_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <DateOfBirthField
              bind:day={state.personal.birthDay}
              bind:month={state.personal.birthMonth}
              bind:year={state.personal.birthYear}
              idPrefix="student-birth"
            />
          </div>
        </div>
      {/if}

      <!-- Gender -->
      {#if state.activeFields.sex}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.common_form_sex()}
              {#if state.requiredFields.sex}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_sex_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0 flex gap-2 items-center">
            <Button
              variant="ghost"
              type="button"
              class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.personal.sex === 'male' ? 'border-primary bg-primary/10 font-bold text-primary shadow-xs' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
              onclick={() => (state.personal.sex = 'male')}
            >
              <span>{m.common_form_male()}</span>
            </Button>
            <Button 
              variant="ghost" 
              type="button"
              class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.personal.sex === 'female' ? 'border-primary bg-primary/10 font-bold text-primary shadow-xs' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
              onclick={() => (state.personal.sex = 'female')}
            >
              <span>{m.common_form_female()}</span>
            </Button>
          </div>
        </div>
      {/if}

      <!-- Student Phone -->
      {#if state.activeFields.phone}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="phone" class="text-xs font-semibold text-foreground">
              {m.common_form_student_phone()}
              {#if state.requiredFields.phone}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_phone_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0 flex flex-col gap-1.5">
            <Input 
              id="phone"
              type="tel" 
              placeholder={m.common_form_student_phone_placeholder()} 
              class="h-9 w-full bg-background font-mono text-left {state.duplicateWarnings.phone ? 'border-destructive focus-visible:ring-destructive' : ''}" 
              bind:value={state.personal.phone}
              oninput={() => state.triggerDuplicateCheck("phone", state.personal.phone)}
            />
            {#if state.duplicateWarnings.phone}
              <div class="flex items-center gap-1.5 text-[11px] text-destructive font-medium text-start">
                <span>{state.duplicateWarnings.phone}</span>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- Student Email -->
      {#if state.activeFields.email}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="email" class="text-xs font-semibold text-foreground">
              {m.common_form_student_email()}
              {#if state.requiredFields.email}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_email_desc()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0 flex flex-col gap-1.5">
            <Input 
              id="email"
              type="email" 
              placeholder="student@example.com" 
              class="h-9 w-full bg-background font-mono text-left {state.duplicateWarnings.email ? 'border-destructive focus-visible:ring-destructive' : ''}" 
              bind:value={state.personal.email}
              oninput={() => state.triggerDuplicateCheck("email", state.personal.email)}
            />
            {#if state.duplicateWarnings.email}
              <div class="flex items-center gap-1.5 text-[11px] text-destructive font-medium text-start">
                <span>{state.duplicateWarnings.email}</span>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- Blood Type -->
      {#if state.activeFields.blood_type}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="bloodType" class="text-xs font-semibold text-foreground">
              {l("فصيلة الدم", "Blood Type")}
              {#if state.requiredFields.blood_type}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{l("فصيلة دم الطالب للحالات الطبية والإسعافية", "Student blood type for medical records")}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="bloodType"
              type="text" 
              placeholder="مثال: O+, A+, B-..." 
              class="h-9 w-full bg-background font-mono" 
              bind:value={state.personal.bloodType}
            />
          </div>
        </div>
      {/if}

      <!-- Emergency Contact -->
      {#if state.activeFields.emergency_contact}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="emergencyContact" class="text-xs font-semibold text-foreground">
              {l("جهة اتصال للطوارئ (الاسم والهاتف)", "Emergency Contact")}
              {#if state.requiredFields.emergency_contact}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{l("اسم وتلفون الشخص المسؤول للتواصل في حالات الطوارئ", "Contact person name and phone in case of emergency")}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="emergencyContact"
              type="text" 
              placeholder={l("الاسم - رقم الهاتف", "Name - Phone")} 
              class="h-9 w-full bg-background" 
              bind:value={state.personal.emergencyContact}
            />
          </div>
        </div>
      {/if}

      <!-- Medical Notes / Conditions -->
      {#if state.activeFields.medical_notes}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="medicalNotes" class="text-xs font-semibold text-foreground">
              {l("الملاحظات والبيانات الطبية", "Medical Notes")}
              {#if state.requiredFields.medical_notes}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{l("أي حساسية، أمراض مزمنة أو تعليمات صحية خاصة للطالب", "Any allergies, chronic conditions or special medical advice")}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="medicalNotes"
              type="text" 
              placeholder={l("أدخل الملاحظات أو الحالة الطبية إن وجدت...", "Enter medical notes or conditions if any...")} 
              class="h-9 w-full bg-background" 
              bind:value={state.personal.medicalNotes}
            />
          </div>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>
