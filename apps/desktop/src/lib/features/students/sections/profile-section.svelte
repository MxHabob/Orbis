<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";

import type { StudentFormState } from "../form-state.svelte";
import { ClassSelect } from "$lib/features/academics";
import DateOfBirthField from "../components/date-of-birth-field.svelte";

let { state }: { state: StudentFormState } = $props();
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    {m.common_form_student_info()}
  </div>

  <div class="flex flex-col">
    <!-- Arabic Name -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
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
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
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

    {#if !state.isEditMode}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="studentPassword" class="text-xs font-semibold text-foreground">
            {m.common_user_form_password()}
            <span class="text-destructive">*</span>
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_user_form_password_placeholder()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0">
          <Input
            id="studentPassword"
            type="password"
            placeholder={m.common_user_form_password_placeholder()}
            class="h-9 w-full bg-background font-mono"
            bind:value={state.registrationForm.values.password}
            autocomplete="new-password"
          />
        </div>
      </div>
    {/if}

    <!-- National ID / Passport -->
    {#if state.activeFields.national_id}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="nationalId" class="text-xs font-semibold text-foreground">
            {m.common_form_profile_nid_label()} 
            {#if state.requiredFields.national_id}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_nid_desc()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex flex-col gap-1">
          <Input 
            id="nationalId"
            type="text" 
            placeholder="1000234900" 
            class="h-9 w-full bg-background font-mono" 
            bind:value={state.personal.nationalId}
            onblur={() => state.triggerDuplicateCheck("national_id", state.personal.nationalId)}
          />
          {#if state.duplicateWarnings.national_id}
            <span class="text-[10px] text-destructive font-semibold text-start">
              {state.duplicateWarnings.national_id}
            </span>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Passport Number -->
    {#if state.activeFields.passport_number}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
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
            class="h-9 w-full bg-background font-mono" 
            bind:value={state.personal.passportNumber}
          />
        </div>
      </div>
    {/if}

    <!-- Nationality -->
    {#if state.activeFields.nationality}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="nationality" class="text-xs font-semibold text-foreground">
            {m.common_form_nationality()}
            {#if state.requiredFields.nationality}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_nationality_desc()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0">
          <Input 
            id="nationality"
            type="text" 
            placeholder={m.common_form_nationality_placeholder()}
            class="h-9 w-full bg-background" 
            bind:value={state.personal.nationality}
          />
        </div>
      </div>
    {/if}

    <!-- Place of Birth -->
    {#if state.activeFields.place_of_birth}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
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
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
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
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
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
            class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.personal.sex === 'male' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
            onclick={() => state.personal.sex = 'male'}
          >
            <span>{m.common_form_male()}</span>
          </Button>
          <Button 
            variant="ghost"
            type="button"
            class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.personal.sex === 'female' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
            onclick={() => state.personal.sex = 'female'}
          >
            <span>{m.common_form_female()}</span>
          </Button>
        </div>
      </div>
    {/if}

    <!-- Requested Class -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="classSelect" class="text-xs font-semibold text-foreground flex items-center gap-1">
          {m.common_form_target_class()}
          <span class="text-destructive">*</span>
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_class_desc()}</span>
      </div>
      <div class="w-full md:w-120 shrink-0">
        <ClassSelect
          id="classSelect"
          bind:value={state.personal.classId}
          classes={state.classes}
        />
      </div>
    </div>

    <!-- Academic Student Number -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="studentNumber" class="text-xs font-semibold text-foreground">
          {m.common_form_student_number()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_number_desc()}</span>
      </div>
      <div class="w-full md:w-120 shrink-0">
        <Input 
          id="studentNumber"
          type="text" 
          placeholder={m.common_form_student_number_placeholder()} 
          class="h-9 w-full bg-background" 
          disabled={!state.isEditMode}
          bind:value={state.personal.studentNumber}
        />
      </div>
    </div>

    <!-- Student Phone -->
    {#if state.activeFields.phone}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="phone" class="text-xs font-semibold text-foreground">
            {m.common_form_student_phone()}
            {#if state.requiredFields.phone}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_phone_desc()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex flex-col gap-1">
          <Input 
            id="phone"
            type="tel" 
            placeholder={m.common_form_student_phone_placeholder()} 
            class="h-9 w-full bg-background font-mono text-left" 
            bind:value={state.personal.phone}
            onblur={() => state.triggerDuplicateCheck("phone", state.personal.phone)}
          />
          {#if state.duplicateWarnings.phone}
            <span class="text-[10px] text-destructive font-semibold text-start">
              {state.duplicateWarnings.phone}
            </span>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Student Email -->
    {#if state.activeFields.email}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="email" class="text-xs font-semibold text-foreground">
            {m.common_form_student_email()}
            {#if state.requiredFields.email}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_profile_email_desc()}</span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex flex-col gap-1">
          <Input 
            id="email"
            type="email" 
            placeholder="student@example.com" 
            class="h-9 w-full bg-background font-mono text-left" 
            bind:value={state.personal.email}
            onblur={() => state.triggerDuplicateCheck("email", state.personal.email)}
          />
          {#if state.duplicateWarnings.email}
            <span class="text-[10px] text-destructive font-semibold text-start">
              {state.duplicateWarnings.email}
            </span>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Blood Type -->
    {#if state.activeFields.blood_type}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="bloodType" class="text-xs font-semibold text-foreground">
            فصيلة الدم
            {#if state.requiredFields.blood_type}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">فصيلة دم الطالب للحالات الطبية والإسعافية</span>
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
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="emergencyContact" class="text-xs font-semibold text-foreground">
            جهة اتصال للطوارئ (الاسم والهاتف)
            {#if state.requiredFields.emergency_contact}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">اسم وتلفون الشخص المسؤول للتواصل في حالات الطوارئ</span>
        </div>
        <div class="w-full md:w-120 shrink-0">
          <Input 
            id="emergencyContact"
            type="text" 
            placeholder="الاسم - رقم الهاتف" 
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
            الملاحظات والبيانات الطبية
            {#if state.requiredFields.medical_notes}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">أي حساسية، أمراض مزمنة أو تعليمات صحية خاصة للطالب</span>
        </div>
        <div class="w-full md:w-120 shrink-0">
          <Input 
            id="medicalNotes"
            type="text" 
            placeholder="أدخل الملاحظات أو الحالة الطبية إن وجدت..." 
            class="h-9 w-full bg-background" 
            bind:value={state.personal.medicalNotes}
          />
        </div>
      </div>
    {/if}
  </div>
</div>
