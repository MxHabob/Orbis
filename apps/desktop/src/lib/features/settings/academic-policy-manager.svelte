<!--
@component AcademicPolicyManager
Manages organization-wide academic, grading, enrollment, and evaluation policies.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { untrack } from "svelte";
import {
  createOrgAcademicPolicyQuery,
  useUpdateAcademicPolicy,
} from "$lib/features/academics/queries-policy";
import { api, extractErrorMessage, type OrganizationSettingsDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { BookOpen, Lock, ReceiptPercent, Scale, UserBadgeCheck } from "@orbis/icons";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

let { orgId }: { orgId: string } = $props();

const policyQuery = createOrgAcademicPolicyQuery(() => orgId);
const updateMutation = useUpdateAcademicPolicy();

// Form states
let isSaving = $state(false);
let initialSnapshot = $state("");
let rawSettings = $state<OrganizationSettingsDto | null>(null);

let form = $state({
  default_grading_system: "percentage",
  default_passing_grade: 50,
  default_max_grade: 100,
  min_attendance_rate: 75,
  enrollment_auto_close: true,
  allow_late_enrollment: false,
  late_enrollment_approval: true,
  grade_entry_auto_close: true,
  allow_grade_edit_after_close: false,
  grade_edit_approval: true,
  auto_graduate_final_level: false,
  exclude_withdrawn: true,
});

// Financial Policy State
let currency = $state("YER");
let taxRate = $state(0.15);
let invoiceDueDays = $state(14);
let lateFeeType = $state<string>("fixed");
let lateFeeValue = $state(0.0);
let allowPartialPayments = $state(false);

// Sync database record and org finance settings to local form state
$effect(() => {
  if (policyQuery.data) {
    const d = policyQuery.data as any;
    form = {
      default_grading_system: d.default_grading_system || "percentage",
      default_passing_grade:
        d.default_passing_score ?? d.default_passing_grade ?? 50,
      default_max_grade: d.default_max_score ?? d.default_max_grade ?? 100,
      min_attendance_rate: d.min_attendance_rate ?? 75,
      enrollment_auto_close: d.enrollment_auto_close === 1,
      allow_late_enrollment: d.allow_late_enrollment === 1,
      late_enrollment_approval: d.late_enrollment_approval === 1,
      grade_entry_auto_close: d.grade_entry_auto_close === 1,
      allow_grade_edit_after_close: d.allow_grade_edit_after_close === 1,
      grade_edit_approval: d.grade_edit_approval === 1,
      auto_graduate_final_level: d.auto_graduate_final_level === 1,
      exclude_withdrawn: d.exclude_withdrawn !== 0,
    };

    api.getSettings().then((s) => {
      if (s) {
        rawSettings = s;
        if (s.finance) {
          currency = s.finance.currency || "YER";
          taxRate = s.finance.tax_rate ?? 0.15;
          invoiceDueDays = s.finance.invoice_due_days ?? 14;
          lateFeeType = s.finance.late_fee_type || "fixed";
          lateFeeValue = s.finance.late_fee_value ?? 0.0;
          allowPartialPayments = Boolean(s.finance.allow_partial_payments);
        }
      }
      initialSnapshot = untrack(() =>
        JSON.stringify({
          ...form,
          finance: {
            taxRate,
            invoiceDueDays,
            lateFeeType,
            lateFeeValue,
            allowPartialPayments,
          },
        }),
      );
    }).catch(() => {
      initialSnapshot = untrack(() =>
        JSON.stringify({
          ...form,
          finance: {
            taxRate,
            invoiceDueDays,
            lateFeeType,
            lateFeeValue,
            allowPartialPayments,
          },
        }),
      );
    });
  }
});

const isDirty = $derived(
  initialSnapshot !== "" &&
    JSON.stringify({
      ...form,
      finance: {
        taxRate,
        invoiceDueDays,
        lateFeeType,
        lateFeeValue,
        allowPartialPayments,
      },
    }) !== initialSnapshot,
);

function handleReset() {
  if (!initialSnapshot) return;
  const parsed = JSON.parse(initialSnapshot);
  form = {
    default_grading_system: parsed.default_grading_system,
    default_passing_grade: parsed.default_passing_grade,
    default_max_grade: parsed.default_max_grade,
    min_attendance_rate: parsed.min_attendance_rate,
    enrollment_auto_close: parsed.enrollment_auto_close,
    allow_late_enrollment: parsed.allow_late_enrollment,
    late_enrollment_approval: parsed.late_enrollment_approval,
    grade_entry_auto_close: parsed.grade_entry_auto_close,
    allow_grade_edit_after_close: parsed.allow_grade_edit_after_close,
    grade_edit_approval: parsed.grade_edit_approval,
    auto_graduate_final_level: parsed.auto_graduate_final_level,
    exclude_withdrawn: parsed.exclude_withdrawn,
  };
  if (parsed.finance) {
    taxRate = parsed.finance.taxRate;
    invoiceDueDays = parsed.finance.invoiceDueDays;
    lateFeeType = parsed.finance.lateFeeType;
    lateFeeValue = parsed.finance.lateFeeValue;
    allowPartialPayments = parsed.finance.allowPartialPayments;
  }
}

async function handleSave(e?: Event) {
  e?.preventDefault();
  isSaving = true;
  try {
    const payload = {
      default_grading_system: form.default_grading_system,
      default_passing_score: Number(form.default_passing_grade),
      default_max_score: Number(form.default_max_grade),
      min_attendance_rate: Number(form.min_attendance_rate),
      enrollment_auto_close: form.enrollment_auto_close ? 1 : 0,
      allow_late_enrollment: form.allow_late_enrollment ? 1 : 0,
      late_enrollment_approval: form.late_enrollment_approval ? 1 : 0,
      grade_entry_auto_close: form.grade_entry_auto_close ? 1 : 0,
      allow_grade_edit_after_close: form.allow_grade_edit_after_close ? 1 : 0,
      grade_edit_approval: form.grade_edit_approval ? 1 : 0,
      auto_graduate_final_level: form.auto_graduate_final_level ? 1 : 0,
      exclude_withdrawn: form.exclude_withdrawn ? 1 : 0,
    };

    await $updateMutation.mutateAsync({
      orgId,
      policy: payload,
    });

    const currentSettings = rawSettings || await api.getSettings().catch(() => null);
    if (currentSettings) {
      const updatedSettings = {
        ...currentSettings,
        finance: {
          ...currentSettings.finance,
          currency: currentSettings.finance?.currency || currency || "YER",
          tax_rate: Number(taxRate),
          invoice_due_days: Number(invoiceDueDays),
          late_fee_type: lateFeeType,
          late_fee_value: Number(lateFeeValue),
          allow_partial_payments: Boolean(allowPartialPayments),
        },
      };
      const res = await api.updateSettings(updatedSettings);
      rawSettings = res;
    }

    initialSnapshot = JSON.stringify({
      ...form,
      finance: {
        taxRate,
        invoiceDueDays,
        lateFeeType,
        lateFeeValue,
        allowPartialPayments,
      },
    });
    toast.success(
      m.settings_policy_save_success(),
    );
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSaving = false;
  }
}
</script>

<div
  class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide"
>
  {#if policyQuery.isLoading}
    <div class="w-full">
      <SettingsFormSkeleton />
    </div>
  {:else if policyQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>
        {m.settings_policy_load_error()}
      </p>
      <Button
        variant="outline"
        class="mt-4 cursor-pointer"
        onclick={() => policyQuery.refetch()}
      >
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <form onsubmit={handleSave} class="flex flex-col gap-6 w-full max-w-6xl">
      <!-- Top Action Bar -->
      <div class="flex items-center justify-between gap-3 w-full select-none pb-2 border-b border-border">
        <div class="flex flex-col text-start">
          <h1 class="text-sm font-semibold text-foreground">{m.settings_ui_policy_page_title()}</h1>
          <p class="text-xs text-muted-foreground mt-0.5">{m.settings_ui_policy_page_description()}</p>
        </div>
      </div>

      <!-- Two-Column Settings Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- 1. Grading & Evaluation Defaults Section -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Scale size={14} class="text-primary" />
              <CardTitle>{m.settings_ui_policy_grading_title()}</CardTitle>
            </div>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-5 text-start">
            <!-- Grading System -->
            <div class="flex flex-col gap-1.5 text-start">
              <Label class="text-xs font-semibold">{m.settings_ui_policy_default_grading_system()}</Label>
              <Select
                type="single"
                value={form.default_grading_system}
                onValueChange={(v) => { if (v) form.default_grading_system = v; }}
              >
                <SelectTrigger class="text-xs h-9 w-full">
                  {form.default_grading_system === "percentage" ? m.settings_ui_policy_scale_percentage() : form.default_grading_system === "gpa_4_0" ? m.settings_ui_policy_scale_gpa() : m.settings_ui_policy_scale_pass_fail()}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="percentage">{m.settings_ui_policy_scale_percentage()}</SelectItem>
                  <SelectItem value="gpa_4_0">{m.settings_ui_policy_scale_gpa()}</SelectItem>
                  <SelectItem value="pass_fail">{m.settings_ui_policy_scale_pass_fail()}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <!-- Scores Grid -->
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.settings_ui_policy_default_passing_grade()}</Label>
                <Input
                  type="number"
                  step="0.5"
                  bind:value={form.default_passing_grade}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.settings_ui_policy_default_max_grade()}</Label>
                <Input
                  type="number"
                  step="1"
                  bind:value={form.default_max_grade}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
              </div>
            </div>

            <!-- Min Attendance Rate -->
            <div class="flex flex-col gap-1.5 text-start pt-1">
              <Label class="text-xs font-semibold">{m.settings_ui_policy_min_attendance()}</Label>
              <Input
                type="number"
                min="0"
                max="100"
                bind:value={form.min_attendance_rate}
                class="text-xs font-mono h-9"
                disabled={isSaving}
              />
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_policy_min_attendance_description()}</span>
            </div>
          </CardContent>
        </Card>

        <!-- 2. Billing & Financial Policies Section -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <ReceiptPercent size={14} class="text-primary" />
              <CardTitle>{m.settings_ui_policy_finance_title()}</CardTitle>
            </div>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-4 text-start">
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.settings_ui_policy_default_tax_rate()}</Label>
                <Input
                  type="number"
                  step="0.01"
                  min="0"
                  max="1"
                  value={taxRate}
                  oninput={(e) => taxRate = parseFloat(e.currentTarget.value) || 0}
                  disabled={isSaving}
                  class="text-xs font-mono h-9"
                />
                <span class="text-[10px] text-muted-foreground">{m.settings_ui_policy_tax_rate_example()}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.settings_ui_policy_invoice_due_days()}</Label>
                <Input
                  type="number"
                  min="1"
                  max="365"
                  bind:value={invoiceDueDays}
                  disabled={isSaving}
                  class="text-xs font-mono h-9"
                />
                <span class="text-[10px] text-muted-foreground">{m.settings_ui_policy_payment_deadline_hint()}</span>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4 pt-1">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.settings_ui_policy_late_fee_type()}</Label>
                <Select
                  type="single"
                  value={lateFeeType}
                  onValueChange={(v) => { if (v) lateFeeType = v; }}
                  disabled={isSaving}
                >
                  <SelectTrigger class="text-xs h-9 w-full">
                    {lateFeeType === "percentage" ? m.settings_ui_policy_fee_percentage() : m.settings_ui_policy_fee_fixed()}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="fixed">{m.settings_ui_policy_fee_fixed()}</SelectItem>
                    <SelectItem value="percentage">{m.settings_ui_policy_fee_percentage()}</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.settings_ui_policy_late_fee_value()}</Label>
                <Input
                  type="number"
                  step="0.01"
                  min="0"
                  value={lateFeeValue}
                  oninput={(e) => lateFeeValue = parseFloat(e.currentTarget.value) || 0}
                  disabled={isSaving}
                  class="text-xs font-mono h-9"
                />
              </div>
            </div>

            <div class="flex items-center justify-between py-2 border-t border-border mt-1">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.settings_ui_policy_partial_payments()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.settings_ui_policy_partial_payments_description()}
                </span>
              </div>
              <Switch
                checked={allowPartialPayments}
                onCheckedChange={(v) => (allowPartialPayments = v)}
                disabled={isSaving}
              />
            </div>
          </CardContent>
        </Card>

        <!-- 3. Grade Entry & Locking Policies Section -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Lock size={14} class="text-primary" />
              <CardTitle>{m.settings_ui_policy_grade_entry_title()}</CardTitle>
            </div>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-4 text-start">
            <!-- Grade Entry Auto Close -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.settings_ui_policy_grade_entry_auto_close()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.settings_ui_policy_grade_entry_auto_close_description()}
                </span>
              </div>
              <Switch
                checked={form.grade_entry_auto_close}
                onCheckedChange={(v) => (form.grade_entry_auto_close = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Allow Grade Edit After Close -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.settings_ui_policy_allow_grade_edit()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.settings_ui_policy_allow_grade_edit_description()}
                </span>
              </div>
              <Switch
                checked={form.allow_grade_edit_after_close}
                onCheckedChange={(v) => (form.allow_grade_edit_after_close = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Grade Edit Requires Approval -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.settings_ui_policy_grade_edit_approval()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.settings_ui_policy_grade_edit_approval_description()}
                </span>
              </div>
              <Switch
                checked={form.grade_edit_approval}
                onCheckedChange={(v) => (form.grade_edit_approval = v)}
                disabled={isSaving || !form.allow_grade_edit_after_close}
              />
            </div>
          </CardContent>
        </Card>

        <!-- 4. Promotion & Graduation Rules Section -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <BookOpen size={14} class="text-primary" />
              <CardTitle>{m.settings_ui_policy_promotion_title()}</CardTitle>
            </div>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-4 text-start">
            <!-- Auto Graduate Final Level -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.settings_ui_policy_auto_graduate()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.settings_ui_policy_auto_graduate_description()}
                </span>
              </div>
              <Switch
                checked={form.auto_graduate_final_level}
                onCheckedChange={(v) => (form.auto_graduate_final_level = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Exclude Withdrawn -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.settings_ui_policy_exclude_withdrawn()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.settings_ui_policy_exclude_withdrawn_description()}
                </span>
              </div>
              <Switch
                checked={form.exclude_withdrawn}
                onCheckedChange={(v) => (form.exclude_withdrawn = v)}
                disabled={isSaving}
              />
            </div>
          </CardContent>
        </Card>
      </div>
    </form>
    <SettingsBottomBar
      {isDirty}
      {isSaving}
      onsave={handleSave}
      onreset={handleReset}
    />
  {/if}
</div>
