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
  { value: "regular", label: m.operations_ui_study_mode_regular() },
  { value: "part_time", label: m.operations_ui_study_mode_part_time() },
  { value: "distance_learning", label: m.operations_ui_study_mode_distance() },
  { value: "homeschooled", label: m.operations_ui_study_mode_homeschooled() },
  { value: "auditor", label: m.operations_ui_study_mode_auditor() },
  { value: "visiting", label: m.operations_ui_study_mode_visiting() },
  { value: "exchange", label: m.operations_ui_study_mode_exchange() },
  { value: "special_needs", label: m.operations_ui_study_mode_special_needs() },
];
</script>

<Card class="w-full border-border/80 shadow-xs">
  <CardHeader class="py-3 px-6 border-b border-border/40 flex flex-row items-center justify-between">
    <div class="flex items-center gap-2">
      <User class="w-4 h-4 text-primary" />
      <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
        {m.ui_copy_academic_identity_personal_profile_5a3e9546()}
      </CardTitle>
    </div>
    <span class="text-[11px] text-muted-foreground">
      {m.ui_copy_fields_marked_with_are_required_df81f006()}
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
            <span class="text-xs font-bold text-foreground">{m.ui_copy_student_profile_photo_de522661()}</span>
            <span class="text-[11px] text-muted-foreground">{m.ui_copy_used_for_student_id_pass_and_official_reco_8e5547ff()}</span>
          </div>
        </div>

        <!-- Study Mode Selector -->
        <div class="flex flex-col gap-1.5 text-start w-full md:w-72">
          <Label class="text-xs font-semibold text-foreground">{m.ui_copy_study_mode_enrollment_track_2468575d()}</Label>
          <Select
            type="single"
            value={state.studyMode}
            onValueChange={(v) => { if (v) state.studyMode = v; }}
          >
            <SelectTrigger class="h-9 text-xs bg-background">
              {studyModes.find(m => m.value === state.studyMode) 
                ? studyModes.find(m => m.value === state.studyMode)!.label
                : state.studyMode}
            </SelectTrigger>
            <SelectContent class="text-start max-h-64 overflow-y-auto" align="end">
              {#each studyModes as mode (mode.value)}
                <SelectItem value={mode.value} class="text-xs">
                  {mode.label}
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
            class="h-9 w-full bg-background font-mono"
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
          />
          {#if state.selectedClass}
            <div class="flex items-center justify-between px-1">
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_capacity_ebf7f7c9()}</span>
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
              class="h-9 w-full bg-background font-mono {state.duplicateWarnings.national_id ? 'border-destructive focus-visible:ring-destructive' : ''}"
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
              class="h-9 w-full bg-background font-mono {state.duplicateWarnings.phone ? 'border-destructive focus-visible:ring-destructive' : ''}"
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
              class="h-9 w-full bg-background font-mono {state.duplicateWarnings.email ? 'border-destructive focus-visible:ring-destructive' : ''}"
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
              {m.employees_blood_type()}
              {#if state.requiredFields.blood_type}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.ui_copy_student_blood_type_for_medical_records_e7535748()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="bloodType"
              type="text" 
              placeholder={m.operations_ui_blood_type_example()}
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
              {m.ui_copy_emergency_contact_2ff8ef2f()}
              {#if state.requiredFields.emergency_contact}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.ui_copy_contact_person_name_and_phone_in_case_of_e_319c6951()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="emergencyContact"
              type="text" 
              placeholder={m.ui_copy_name_phone_14b9bb22()} 
              class="h-9 w-full bg-background" 
              bind:value={state.personal.emergencyContact}
            />
          </div>
        </div>
      {/if}

      <!-- Medical Notes / Conditions -->
      {#if state.activeFields.medical_notes}
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label for="medicalNotes" class="text-xs font-semibold text-foreground">
              {m.ui_copy_medical_notes_889f381c()}
              {#if state.requiredFields.medical_notes}<span class="text-destructive">*</span>{/if}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.ui_copy_any_allergies_chronic_conditions_or_specia_f9351b55()}</span>
          </div>
          <div class="w-full md:w-120 shrink-0">
            <Input 
              id="medicalNotes"
              type="text" 
              placeholder={m.ui_copy_enter_medical_notes_or_conditions_if_any_626e8645()} 
              class="h-9 w-full bg-background" 
              bind:value={state.personal.medicalNotes}
            />
          </div>
        </div>
      {/if}

      <!-- Special Needs Notes -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="specialNeedsNotes" class="text-xs font-semibold text-foreground">
            {m.ui_copy_special_needs_accommodations_4763ae09()}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.ui_copy_special_accommodations_or_accessibility_as_701c987f()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0">
          <Input 
            id="specialNeedsNotes"
            type="text" 
            placeholder={m.ui_copy_enter_special_needs_notes_if_any_0ef91688()} 
            class="h-9 w-full bg-background" 
            bind:value={state.personal.specialNeedsNotes}
          />
        </div>
      </div>
    </div>
  </CardContent>
</Card>
