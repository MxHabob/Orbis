<!--
@component AcademicPolicySettingsPage
Manages organization-wide academic, grading, enrollment, and automation policies.
Uses TanStack Query to load current policy and updates it via Rust backend command.
-->
<script lang="ts">
import { onMount } from "svelte";
import { useQueryClient } from "@tanstack/svelte-query";
import { createOrgAcademicPolicyQuery } from "$lib/features/academics/queries-policy";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Switch } from "@orbis/ui-svelte/switch";
import {
  Select, SelectTrigger, SelectContent, SelectItem,
} from "@orbis/ui-svelte/select";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@orbis/ui-svelte/card";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  Settings, ShieldAlert, Award, Calendar, Loader2, Save,
} from "lucide-svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);

const queryClient = useQueryClient();
const policyQuery = createOrgAcademicPolicyQuery(orgId);

// Form state
let isSaving = $state(false);
let form = $state({
  default_grading_system: "percentage",
  default_passing_grade: 50,
  default_max_grade: 100,
  enrollment_auto_close_on_capacity: false,
  allow_late_enrollment: false,
  late_enrollment_requires_approval: false,
  grade_entry_auto_close_on_deadline: false,
  allow_grade_modification_after_close: false,
  grade_modification_requires_approval: false,
  default_event_execution_mode: "suggested",
});

// Sync database record to local form state
$effect(() => {
  if ($policyQuery.data) {
    const d = $policyQuery.data;
    form = {
      default_grading_system: d.default_grading_system || "percentage",
      default_passing_grade: d.default_passing_grade ?? 50,
      default_max_grade: d.default_max_grade ?? 100,
      enrollment_auto_close_on_capacity: d.enrollment_auto_close_on_capacity === 1,
      allow_late_enrollment: d.allow_late_enrollment === 1,
      late_enrollment_requires_approval: d.late_enrollment_requires_approval === 1,
      grade_entry_auto_close_on_deadline: d.grade_entry_auto_close_on_deadline === 1,
      allow_grade_modification_after_close: d.allow_grade_modification_after_close === 1,
      grade_modification_requires_approval: d.grade_modification_requires_approval === 1,
      default_event_execution_mode: d.default_event_execution_mode || "suggested",
    };
  }
});

async function handleSave(e: Event) {
  e.preventDefault();
  isSaving = true;
  try {
    const payload = {
      default_grading_system: form.default_grading_system,
      default_passing_grade: form.default_passing_grade,
      default_max_grade: form.default_max_grade,
      enrollment_auto_close_on_capacity: form.enrollment_auto_close_on_capacity ? 1 : 0,
      allow_late_enrollment: form.allow_late_enrollment ? 1 : 0,
      late_enrollment_requires_approval: form.late_enrollment_requires_approval ? 1 : 0,
      grade_entry_auto_close_on_deadline: form.grade_entry_auto_close_on_deadline ? 1 : 0,
      allow_grade_modification_after_close: form.allow_grade_modification_after_close ? 1 : 0,
      grade_modification_requires_approval: form.grade_modification_requires_approval ? 1 : 0,
      default_event_execution_mode: form.default_event_execution_mode,
      settings: null,
    };

    await commands.calendar.updatePolicy(orgId, payload);
    toast.success(m.settings_general_save_success ? m.settings_general_save_success() : "Settings saved successfully");
    queryClient.invalidateQueries({ queryKey: ["org-academic-policy", orgId] });
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSaving = false;
  }
}
</script>

<svelte:head>
  <title>
    {m.settings_academic_policy_title ? m.settings_academic_policy_title() : "Academic Policy & Automation"} | Orbis
  </title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  {#if $policyQuery.isLoading}
    <div class="max-w-4xl mx-auto p-6">
      <SettingsFormSkeleton />
    </div>
  {:else if $policyQuery.error}
    <div class="max-w-4xl mx-auto p-6 text-center text-destructive">
      <p>{m.settings_policy_load_error ? m.settings_policy_load_error() : "Failed to load academic policies"}</p>
      <Button variant="outline" class="mt-4" onclick={() => $policyQuery.refetch()}>
        {m.retry ? m.retry() : "Retry"}
      </Button>
    </div>
  {:else}
    <form onsubmit={handleSave} class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
      <!-- Header row -->
      <div class="flex items-center justify-between flex-nowrap shrink-0 border-b border-border/40 pb-5">
        <div class="flex items-center gap-3">
          <div class="flex items-center justify-center size-10 rounded-xl bg-primary/10">
            <Settings class="size-5 text-primary" />
          </div>
          <div>
            <h1 class="text-lg font-bold text-foreground">
              {m.settings_academic_policy_title ? m.settings_academic_policy_title() : "Academic Policy & Automation"}
            </h1>
            <p class="text-xs text-muted-foreground">
              {m.settings_academic_policy_desc ? m.settings_academic_policy_desc() : "Configure organization-wide rules for calendar automation, enrollment, and grading."}
            </p>
          </div>
        </div>
        <Button
          type="submit"
          variant="lime"
          class="font-semibold text-xs h-9 px-6"
          disabled={isSaving}
        >
          {#if isSaving}
            <Loader2 class="size-3.5 animate-spin me-1.5" />
          {/if}
          <Save class="size-3.5 me-1.5" />
          {m.settings_general_save ? m.settings_general_save() : "Save Settings"}
        </Button>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Automation & Calendar Card -->
        <Card>
          <CardHeader>
            <div class="flex items-center gap-2">
              <Calendar class="size-4.5 text-primary" />
              <CardTitle class="text-sm">
                {m.policy_card_automation ? m.policy_card_automation() : "Calendar & Automation"}
              </CardTitle>
            </div>
            <CardDescription class="text-[11px]">
              {m.policy_card_automation_desc ? m.policy_card_automation_desc() : "Define how calendar events trigger system phases."}
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-col gap-4">
            <div class="flex flex-col gap-2">
              <Label class="text-xs text-muted-foreground">
                {m.policy_field_exec_mode ? m.policy_field_exec_mode() : "Default Event Execution Mode"}
              </Label>
              <Select
                type="single"
                value={form.default_event_execution_mode}
                onValueChange={(v) => { if (v) form.default_event_execution_mode = v; }}
              >
                <SelectTrigger class="text-xs capitalize h-9">
                  {form.default_event_execution_mode}
                </SelectTrigger>
                <SelectContent class="text-start">
                  <SelectItem value="manual">
                    <span class="font-medium text-xs">Manual</span>
                    <p class="text-[10px] text-muted-foreground">Admins must execute events manually.</p>
                  </SelectItem>
                  <SelectItem value="suggested">
                    <span class="font-medium text-xs">Suggested</span>
                    <p class="text-[10px] text-muted-foreground">Events suggest execution on date; triggers approval request.</p>
                  </SelectItem>
                  <SelectItem value="automatic">
                    <span class="font-medium text-xs">Automatic</span>
                    <p class="text-[10px] text-muted-foreground">Events execute automatically when target date is reached.</p>
                  </SelectItem>
                </SelectContent>
              </Select>
            </div>
          </CardContent>
        </Card>

        <!-- Enrollment Policies Card -->
        <Card>
          <CardHeader>
            <div class="flex items-center gap-2">
              <ShieldAlert class="size-4.5 text-primary" />
              <CardTitle class="text-sm">
                {m.policy_card_enrollment ? m.policy_card_enrollment() : "Enrollment Rules"}
              </CardTitle>
            </div>
            <CardDescription class="text-[11px]">
              {m.policy_card_enrollment_desc ? m.policy_card_enrollment_desc() : "Establish controls for student registrations."}
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-col gap-4">
            <!-- Allow Late Enrollment -->
            <div class="flex items-center justify-between py-2 border-b border-border/40">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_allow_late ? m.policy_field_allow_late() : "Allow Late Enrollment"}
                </span>
                <span class="text-[10px] text-muted-foreground">Permit registrations after active cycle enrollment closed.</span>
              </div>
              <Switch bind:checked={form.allow_late_enrollment} disabled={isSaving} />
            </div>

            <!-- Late Enrollment Requires Approval -->
            <div class="flex items-center justify-between py-2 border-b border-border/40">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_late_approval ? m.policy_field_late_approval() : "Late Enrollment Requires Approval"}
                </span>
                <span class="text-[10px] text-muted-foreground">Route late enrollment requests to approval inbox.</span>
              </div>
              <Switch bind:checked={form.late_enrollment_requires_approval} disabled={isSaving || !form.allow_late_enrollment} />
            </div>

            <!-- Auto Close on Capacity -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_auto_close_capacity ? m.policy_field_auto_close_capacity() : "Auto Close on Capacity"}
                </span>
                <span class="text-[10px] text-muted-foreground">Automatically lock sections when max capacity is reached.</span>
              </div>
              <Switch bind:checked={form.enrollment_auto_close_on_capacity} disabled={isSaving} />
            </div>
          </CardContent>
        </Card>

        <!-- Grading Defaults Card -->
        <Card>
          <CardHeader>
            <div class="flex items-center gap-2">
              <Award class="size-4.5 text-primary" />
              <CardTitle class="text-sm">
                {m.policy_card_grading ? m.policy_card_grading() : "Assessment & Grading"}
              </CardTitle>
            </div>
            <CardDescription class="text-[11px]">
              {m.policy_card_grading_desc ? m.policy_card_grading_desc() : "Define passing thresholds and modification behaviors."}
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-col gap-5">
            <!-- Grading System -->
            <div class="flex flex-col gap-2">
              <Label class="text-xs text-muted-foreground">
                {m.policy_field_grading_system ? m.policy_field_grading_system() : "Default Grading System"}
              </Label>
              <Select
                type="single"
                value={form.default_grading_system}
                onValueChange={(v) => { if (v) form.default_grading_system = v; }}
              >
                <SelectTrigger class="text-xs capitalize h-9">
                  {form.default_grading_system}
                </SelectTrigger>
                <SelectContent class="text-start">
                  <SelectItem value="percentage">Percentage (0 - 100)</SelectItem>
                  <SelectItem value="letters">GPA Letter Grades (A, B, C, D, F)</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <!-- Default Passing Grade -->
              <div class="flex flex-col gap-2">
                <Label class="text-xs text-muted-foreground">
                  {m.policy_field_passing_grade ? m.policy_field_passing_grade() : "Passing Score"}
                </Label>
                <Input type="number" min="0" max={form.default_max_grade} bind:value={form.default_passing_grade} disabled={isSaving} />
              </div>

              <!-- Default Max Grade -->
              <div class="flex flex-col gap-2">
                <Label class="text-xs text-muted-foreground">
                  {m.policy_field_max_grade ? m.policy_field_max_grade() : "Max Score"}
                </Label>
                <Input type="number" min={form.default_passing_grade} bind:value={form.default_max_grade} disabled={isSaving} />
              </div>
            </div>

            <Separator class="bg-border/40" />

            <!-- Auto Close on Deadline -->
            <div class="flex items-center justify-between py-2 border-b border-border/40">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_close_deadline ? m.policy_field_close_deadline() : "Auto Close on Deadline"}
                </span>
                <span class="text-[10px] text-muted-foreground">Disable teacher grade modification once deadline passes.</span>
              </div>
              <Switch bind:checked={form.grade_entry_auto_close_on_deadline} disabled={isSaving} />
            </div>

            <!-- Allow Grade Modification After Close -->
            <div class="flex items-center justify-between py-2 border-b border-border/40">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_allow_mod_after_close ? m.policy_field_allow_mod_after_close() : "Allow Modifications After Close"}
                </span>
                <span class="text-[10px] text-muted-foreground">Permit changes to grade sheets after period has closed.</span>
              </div>
              <Switch bind:checked={form.allow_grade_modification_after_close} disabled={isSaving} />
            </div>

            <!-- Grade Modification Requires Approval -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_mod_approval ? m.policy_field_mod_approval() : "Modifications Require Approval"}
                </span>
                <span class="text-[10px] text-muted-foreground">Force all grade modifications to pass through approvals.</span>
              </div>
              <Switch bind:checked={form.grade_modification_requires_approval} disabled={isSaving || !form.allow_grade_modification_after_close} />
            </div>
          </CardContent>
        </Card>
      </div>
    </form>
  {/if}
</div>
