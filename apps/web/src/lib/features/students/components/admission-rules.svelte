<script lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import type { AcademicPeriodDto } from "$lib/api";
import type { StudentFormState } from "../form-state.svelte";

let {
  state,
  period,
}: { state: StudentFormState; period: AcademicPeriodDto | null } = $props();
</script>

<Accordion
  type="single"
  class="rounded-xl border border-border bg-card px-4 sm:px-6"
>
  <AccordionItem value="admission-rules" class="border-0">
    <AccordionTrigger class="py-4 text-sm font-semibold hover:no-underline">
      {m.student_registration_policy_title()}
    </AccordionTrigger>
    <AccordionContent>
      <div class="flex flex-col divide-y divide-border">
        <section class="pb-4">
          <h3 class="mb-3 text-xs font-semibold text-foreground">
            {m.student_registration_enrollment_rules()}
          </h3>
          <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">{m.student_registration_active_period()}</p>
              <p class="truncate text-sm font-medium">
                {l(period?.name_ar, period?.name) || "—"}
              </p>
            </div>
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_enable_registration_portal_dbaf8a8e()}
              </p>
              <p class="text-sm font-medium">
                {state.admissionPolicy.registrationEnabled
                  ? m.student_registration_policy_enabled()
                  : m.student_registration_policy_disabled()}
              </p>
            </div>
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_require_academic_registrar_admin_approval_b147c1b8()}
              </p>
              <p class="text-sm font-medium">
                {state.admissionPolicy.requiresApproval
                  ? m.ui_copy_under_review_46b36239()
                  : m.ui_copy_approved_39578cc4()}
              </p>
            </div>
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_allow_late_enrollment_06114ddb()}
              </p>
              <p class="text-sm font-medium">
                {state.admissionPolicy.allowLateEnrollment
                  ? m.student_registration_policy_enabled()
                  : m.student_registration_policy_disabled()}
              </p>
            </div>
          </div>
        </section>

        <section class="py-4">
          <h3 class="mb-3 text-xs font-semibold text-foreground">
            {m.student_registration_capacity_rules()}
          </h3>
          <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_default_class_capacity_limit_700d9cac()}
              </p>
              <p class="text-sm font-medium">
                {state.selectedClass?.capacity ?? state.admissionPolicy.maxClassCapacity}
              </p>
            </div>
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_enable_waitlist_on_capacity_189ed812()}
              </p>
              <p class="text-sm font-medium">
                {state.admissionPolicy.allowWaitlist
                  ? m.student_registration_policy_enabled()
                  : m.student_registration_policy_disabled()}
              </p>
            </div>
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_sibling_priority_enrollment_b6d49ba5()}
              </p>
              <p class="text-sm font-medium">
                {state.admissionPolicy.siblingPriorityEnabled
                  ? m.student_registration_policy_enabled()
                  : m.student_registration_policy_disabled()}
              </p>
            </div>
          </div>
        </section>

        <section class="py-4">
          <h3 class="mb-3 text-xs font-semibold text-foreground">
            {m.student_registration_identity_rules()}
          </h3>
          <div class="grid gap-4 sm:grid-cols-2">
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_allow_independent_enrollment_no_guardian_7ce5a41b()}
              </p>
              <p class="text-sm font-medium">
                {state.admissionPolicy.allowNoGuardian
                  ? m.bulk_entry_workbook_optional()
                  : m.common_validation_guardian_name_required()}
              </p>
            </div>
            <div class="min-w-0">
              <p class="text-xs text-muted-foreground">
                {m.ui_copy_national_id_c94d5a1a()}
              </p>
              <p class="text-sm font-medium">
                {state.admissionPolicy.allowDuplicateNationalId
                  ? m.student_registration_duplicate_nid_allowed()
                  : m.student_registration_duplicate_nid_blocked()}
              </p>
            </div>
          </div>
        </section>

        <section class="pt-4">
          <h3 class="mb-3 text-xs font-semibold text-foreground">
            {m.student_registration_finance_rules()}
          </h3>
          <div>
            <p class="text-xs text-muted-foreground">
              {m.ui_copy_auto_assign_tuition_fee_template_on_enroll_5ed6c3b1()}
            </p>
            <p class="text-sm font-medium">
              {state.admissionPolicy.autoAssignFees
                ? m.student_registration_policy_enabled()
                : m.student_registration_policy_disabled()}
            </p>
          </div>
        </section>
      </div>
    </AccordionContent>
  </AccordionItem>
</Accordion>
