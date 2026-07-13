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
        <span class="text-[11px] text-muted-foreground leading-relaxed">الاسم الكامل للطالب باللغة العربية كما هو وارد في الوثائق الرسمية</span>
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
        <span class="text-[11px] text-muted-foreground leading-relaxed">الاسم الكامل للطالب باللغة الإنجليزية</span>
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
            رقم الهوية الوطنية / جواز السفر 
            {#if state.requiredFields.national_id}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">رقم الهوية أو جواز سفر الطالب المكون من 10 خانات</span>
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

    <!-- Birth Date -->
    {#if state.activeFields.birth_date}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label class="text-xs font-semibold text-foreground">
            {m.common_bulk_col_dob()}
            {#if state.requiredFields.birth_date}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">تاريخ الميلاد الرسمي للطالب (اليوم / الشهر / السنة)</span>
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
          <span class="text-[11px] text-muted-foreground leading-relaxed">تحديد جنس الطالب</span>
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
        <span class="text-[11px] text-muted-foreground leading-relaxed">الصف أو المستوى الدراسي الذي سيتم تسجيل الطالب فيه</span>
      </div>
      <div class="w-full md:w-[480px] shrink-0">
        <Select bind:value={state.personal.classId} type="single">
          <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between bg-background" id="classSelect">
            <span>{state.selectedClassName}</span>
          </SelectTrigger>
          <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
            {#each state.classes as cls (cls.id)}
              <SelectItem value={String(cls.id)} class="text-xs text-start justify-start cursor-pointer">
                {cls.name || cls.name_ar}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Academic Student Number -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="studentNumber" class="text-xs font-semibold text-foreground">
          {m.common_form_student_number()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">الرقم الأكاديمي المميز للطالب (متاح فقط في وضع التعديل)</span>
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
          <span class="text-[11px] text-muted-foreground leading-relaxed">رقم هاتف الطالب للتواصل المباشر</span>
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
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="email" class="text-xs font-semibold text-foreground">
            {m.common_form_student_email()}
            {#if state.requiredFields.email}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">البريد الإلكتروني الأساسي للطالب المستخدم لتسجيل الدخول والتنبيهات</span>
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
  </div>
</div>
