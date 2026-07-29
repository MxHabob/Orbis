<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

import { getLocale } from "$lib/paraglide/runtime";
import type { StudentFormState } from "../form-state.svelte";
import { ClassSelect } from "$lib/features/academics";

let { state }: { state: StudentFormState } = $props();

const currentYear = new Date().getFullYear();
const yearsList = Array.from({ length: 47 }, (_, i) => currentYear - i);
const daysList = Array.from({ length: 31 }, (_, i) => String(i + 1));
const monthsList = $derived([
  { value: "1", label: m.common_form_month_1() },
  { value: "2", label: m.common_form_month_2() },
  { value: "3", label: m.common_form_month_3() },
  { value: "4", label: m.common_form_month_4() },
  { value: "5", label: m.common_form_month_5() },
  { value: "6", label: m.common_form_month_6() },
  { value: "7", label: m.common_form_month_7() },
  { value: "8", label: m.common_form_month_8() },
  { value: "9", label: m.common_form_month_9() },
  { value: "10", label: m.common_form_month_10() },
  { value: "11", label: m.common_form_month_11() },
  { value: "12", label: m.common_form_month_12() },
]);
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
      <div class="w-full md:w-[480px] shrink-0">
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
      <div class="w-full md:w-[480px] shrink-0">
        <Input 
          id="nameEn"
          type="text" 
          placeholder={m.common_form_full_name_en_placeholder()} 
          class="h-9 w-full bg-background font-mono text-left" 
          dir="ltr"
          bind:value={state.personal.nameEn}
          autocomplete="off"
        />
      </div>
    </div>

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
        <div class="w-full md:w-[480px] shrink-0 flex flex-col gap-1">
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
            رقم جواز السفر
            {#if state.requiredFields.passport_number}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">رقم جواز السفر الخاص بالطلبة الوافدين أو الدوليين</span>
        </div>
        <div class="w-full md:w-[480px] shrink-0">
          <Input 
            id="passportNumber"
            type="text" 
            placeholder="A12345678" 
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
            الجنسية
            {#if state.requiredFields.nationality}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">جنسية الطالب الرسمية المدونة بالوثائق</span>
        </div>
        <div class="w-full md:w-[480px] shrink-0">
          <Input 
            id="nationality"
            type="text" 
            placeholder="مثال: يمني، سعودي، مصري..." 
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
            مكان الميلاد
            {#if state.requiredFields.place_of_birth}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">المدينة أو المحافظة المدونة في شهادة الميلاد</span>
        </div>
        <div class="w-full md:w-[480px] shrink-0">
          <Input 
            id="placeOfBirth"
            type="text" 
            placeholder="أدخل مكان الميلاد..." 
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
        <div class="w-full md:w-[480px] shrink-0 grid grid-cols-3 gap-2" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
          <!-- Day -->
          <Select bind:value={state.personal.birthDay} type="single">
            <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between bg-background px-2" id="birthDaySelect">
              <span class="truncate">{state.personal.birthDay || m.common_form_day()}</span>
            </SelectTrigger>
            <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
              {#each daysList as dy}
                <SelectItem value={dy} class="text-xs text-start justify-start cursor-pointer">
                  {dy}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>

          <!-- Month -->
          <Select bind:value={state.personal.birthMonth} type="single">
            <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between bg-background px-2" id="birthMonthSelect">
              <span class="truncate">{monthsList.find(m => m.value === state.personal.birthMonth)?.label || m.common_form_month()}</span>
            </SelectTrigger>
            <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
              {#each monthsList as mo}
                <SelectItem value={mo.value} class="text-xs text-start justify-start cursor-pointer">
                  {mo.label}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>

          <!-- Year -->
          <Select bind:value={state.personal.birthYear} type="single">
            <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between bg-background px-2" id="birthYearSelect">
              <span class="truncate">{state.personal.birthYear || m.common_form_year()}</span>
            </SelectTrigger>
            <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
              {#each yearsList as yr}
                <SelectItem value={String(yr)} class="text-xs text-start justify-start cursor-pointer">
                  {yr}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
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
        <div class="w-full md:w-[480px] shrink-0 flex gap-2 items-center">
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
      <div class="w-full md:w-[480px] shrink-0">
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
      <div class="w-full md:w-[480px] shrink-0">
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
        <div class="w-full md:w-[480px] shrink-0 flex flex-col gap-1">
          <Input 
            id="phone"
            type="tel" 
            placeholder={m.common_form_student_phone_placeholder()} 
            class="h-9 w-full bg-background font-mono text-left" 
            dir="ltr"
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
        <div class="w-full md:w-[480px] shrink-0 flex flex-col gap-1">
          <Input 
            id="email"
            type="email" 
            placeholder="student@example.com" 
            class="h-9 w-full bg-background font-mono text-left" 
            dir="ltr"
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
        <div class="w-full md:w-[480px] shrink-0">
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
        <div class="w-full md:w-[480px] shrink-0">
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
        <div class="w-full md:w-[480px] shrink-0">
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
