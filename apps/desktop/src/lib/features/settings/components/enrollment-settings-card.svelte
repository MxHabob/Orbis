<!--
  @component EnrollmentSettingsCard
  Configures institutional enrollment rules and defaults.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Input } from "@orbis/ui/input";
import { UserBadgeCheck } from "@orbis/icons";


let {
  registrationEnabled = $bindable(true),
  requireAdminApproval = $bindable(true),
  allowWaitlist = $bindable(true),
  siblingPriorityEnabled = $bindable(true),
  maxClassSizeDefault = $bindable(25),
  allowLateEnrollment = $bindable(false),
  isSaving = false,
}: {
  registrationEnabled?: boolean;
  requireAdminApproval?: boolean;
  allowWaitlist?: boolean;
  siblingPriorityEnabled?: boolean;
  maxClassSizeDefault?: number;
  allowLateEnrollment?: boolean;
  isSaving?: boolean;
} = $props();
</script>

<Card class="border-border/80 shadow-xs">
  <CardHeader>
    <div class="flex items-center gap-3 text-start">
      <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
        <UserBadgeCheck class="w-4 h-4" />
      </div>
      <div class="flex flex-col">
        <CardTitle>{m.ui_copy_enrollment_admission_defaults_b40fabb5()}</CardTitle>
        <CardDescription>{m.ui_copy_manage_registration_portals_approval_workf_ac467977()}</CardDescription>
      </div>
    </div>
  </CardHeader>
  <CardContent class="flex flex-col gap-5">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{m.ui_copy_enable_registration_28657c4e()}</Label>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_allow_incoming_student_admission_requests_964650f3()}</span>
        </div>
        <Switch bind:checked={registrationEnabled} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{m.ui_copy_require_admin_approval_a0ef4ad5()}</Label>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_require_administrative_review_before_final_681fd709()}</span>
        </div>
        <Switch bind:checked={requireAdminApproval} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{m.ui_copy_allow_waitlist_843e0440()}</Label>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_place_students_on_waitlist_when_classes_re_d51491e7()}</span>
        </div>
        <Switch bind:checked={allowWaitlist} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{m.ui_copy_sibling_priority_05adfd05()}</Label>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_prioritize_siblings_of_already_enrolled_st_a50cde19()}</span>
        </div>
        <Switch bind:checked={siblingPriorityEnabled} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{m.common_policy_field_allow_late()}</Label>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_accept_enrollment_applications_after_perio_388f004a()}</span>
        </div>
        <Switch bind:checked={allowLateEnrollment} disabled={isSaving} />
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{m.ui_copy_default_class_capacity_7bb11790()}</Label>
        <Input
          type="number"
          min="1"
          max="200"
          bind:value={maxClassSizeDefault}
          disabled={isSaving}
          class="h-8 text-xs w-32"
        />
      </div>
    </div>
  </CardContent>
</Card>
