<!--
  @component ProfileSection
  Academic Identity & Student Personal Profile.
  Student identity, contact information, and profile options.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { Checkbox } from "@orbis/ui/checkbox";
import { Button } from "@orbis/ui/button";
import { FormField } from "@orbis/ui";

import type { StudentFormState } from "../form-state.svelte";
import DateOfBirthField from "../components/date-of-birth-field.svelte";
import NationalitySelect from "$lib/shared/components/nationality-select.svelte";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import { orgContext } from "$lib/stores/org-context.svelte";

let { state }: { state: StudentFormState } = $props();

</script>

<section class="@container/profile w-full rounded-xl border border-border bg-card text-card-foreground">
  <div>
    <div class="flex flex-col divide-y divide-border">
      
      <div class="px-4 py-3 @sm/profile:px-5">
        <BilingualNameFields
          arId="nameAr"
          enId="nameEn"
          arLabel={m.student_registration_name_ar()}
          enLabel={m.student_registration_name_en()}
          bind:arValue={state.personal.nameAr}
          bind:enValue={state.personal.nameEn}
          arPlaceholder={m.common_form_full_name_ar_placeholder()}



          kind="person-name"
          requestSuggestion={suggestLocalTranslation}
          arRequired
          loadingMessage={m.bilingual_entry_loading()}
          failureMessage={m.bilingual_entry_failure()}
        />
      </div>

      <div class="grid gap-x-5 gap-y-3 px-4 py-4 empty:hidden @sm/profile:px-6 @min-[46rem]/profile:grid-cols-2">
        <!-- National ID / Civil Number -->
        {#if state.activeFields.national_id}
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="nationalId" class="text-xs font-semibold text-foreground">
                {m.common_form_profile_nid_label()}
                {#if state.requiredFields.national_id}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0 flex flex-col gap-1.5">
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
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="passportNumber" class="text-xs font-semibold text-foreground">
                {m.common_form_passport_number()}
                {#if state.requiredFields.passport_number}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0">
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
          <FormField id="nationality" label={m.common_form_nationality()} required={state.requiredFields.nationality}>
            <NationalitySelect
              id="nationality"
              bind:value={state.personal.nationality}
              required={state.requiredFields.nationality}
              preferredCountry={String(orgContext.metadata?.country_code ?? "")}
            />
          </FormField>
        {/if}

        <!-- Place of Birth -->
        {#if state.activeFields.place_of_birth}
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="placeOfBirth" class="text-xs font-semibold text-foreground">
                {m.common_form_place_of_birth()}
                {#if state.requiredFields.place_of_birth}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0">
              <Input
                id="placeOfBirth"
                type="text"
                    class="h-9 w-full bg-background"
                bind:value={state.personal.placeOfBirth}
              />
            </div>
          </div>
        {/if}

        <!-- Birth Date -->
        {#if state.activeFields.birth_date}
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">
                {m.common_bulk_col_dob()}
                {#if state.requiredFields.birth_date}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0">
              <DateOfBirthField
                bind:day={state.personal.birthDay}
                bind:month={state.personal.birthMonth}
                bind:year={state.personal.birthYear}
                idPrefix="student-birth"
                required={state.requiredFields.birth_date}
              />
            </div>
          </div>
        {/if}

        <!-- Gender -->
        {#if state.activeFields.sex}
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">
                {m.common_form_sex()}
                {#if state.requiredFields.sex}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0 flex gap-2 items-center">
              <Button
                variant="ghost"
                type="button"
                class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.personal.sex === 'male' ? 'border-primary bg-primary/10 font-bold text-primary shadow-xs' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                id="student-sex"
                aria-pressed={state.personal.sex === "male"}
                onclick={() => (state.personal.sex = 'male')}
              >
                <span>{m.common_form_male()}</span>
              </Button>
              <Button
                variant="ghost"
                type="button"
                class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.personal.sex === 'female' ? 'border-primary bg-primary/10 font-bold text-primary shadow-xs' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                aria-pressed={state.personal.sex === "female"}
                onclick={() => (state.personal.sex = 'female')}
              >
                <span>{m.common_form_female()}</span>
              </Button>
            </div>
          </div>
        {/if}

        <!-- Student Phone -->
        {#if state.activeFields.phone}
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="phone" class="text-xs font-semibold text-foreground">
                {m.common_form_student_phone()}
                {#if state.requiredFields.phone}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0 flex flex-col gap-1.5">
              <Input
                id="phone"
                type="tel"
                dir="ltr"
                aria-required={state.requiredFields.phone}
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
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="email" class="text-xs font-semibold text-foreground">
                {m.common_form_student_email()}
                {#if state.requiredFields.email}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0 flex flex-col gap-1.5">
              <Input
                id="email"
                type="email"
                dir="ltr"
                aria-required={state.requiredFields.email}
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
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="bloodType" class="text-xs font-semibold text-foreground">
                {m.employees_blood_type()}
                {#if state.requiredFields.blood_type}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0">
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
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="emergencyContact" class="text-xs font-semibold text-foreground">
                {m.ui_copy_emergency_contact_2ff8ef2f()}
                {#if state.requiredFields.emergency_contact}<span class="text-destructive">*</span>{/if}
              </Label>
            </div>
            <div class="w-full min-w-0">
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
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="medicalNotes" class="text-xs font-semibold text-foreground">
                {m.ui_copy_medical_notes_889f381c()}
                {#if state.requiredFields.medical_notes}<span class="text-destructive">*</span>{/if}
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.ui_copy_any_allergies_chronic_conditions_or_specia_f9351b55()}</span>
            </div>
            <div class="w-full min-w-0">
              <Input
                id="medicalNotes"
                type="text"
                class="h-9 w-full bg-background"
                bind:value={state.personal.medicalNotes}
              />
            </div>
          </div>
        {/if}

        <!-- Special Needs Notes -->
        {#if state.activeFields.special_needs_notes}
          <div class="flex min-w-0 flex-col gap-1.5">
            <div class="flex flex-col gap-1 text-start">
              <Label for="specialNeedsNotes" class="text-xs font-semibold text-foreground">
                {m.ui_copy_special_needs_accommodations_4763ae09()}
                {#if state.requiredFields.special_needs_notes}<span class="text-destructive">*</span>{/if}
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.ui_copy_special_accommodations_or_accessibility_as_701c987f()}</span>
            </div>
            <div class="w-full min-w-0">
              <Input
                id="specialNeedsNotes"
                type="text"
                class="h-9 w-full bg-background"
                bind:value={state.personal.specialNeedsNotes}
              />
            </div>
          </div>
        {/if}
      </div>

      <div class="px-4 py-3 @sm/profile:px-5">
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
          </div>
        </div>
      </div>
      {#if !state.isEditMode}
        <div class="px-4 py-3 @sm/profile:px-5 flex items-start gap-3">
          <Checkbox
            id="student-create-login-account"
            checked={state.createLoginAccount}
            onCheckedChange={(value) => (state.createLoginAccount = !!value)}
          />
          <label for="student-create-login-account" class="flex flex-col gap-1 text-start cursor-pointer">
            <span class="text-xs font-semibold text-foreground">{m.bulk_entry_employee_header_provision_user()}</span>
            <span class="text-[11px] text-muted-foreground">{m.students_create_login_account_description()}</span>
          </label>
        </div>
      {/if}
    </div>
  </div>
</section>
