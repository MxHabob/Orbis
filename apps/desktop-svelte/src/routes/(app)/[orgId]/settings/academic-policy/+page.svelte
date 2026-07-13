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
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@orbis/ui-svelte/card";
import { Separator } from "@orbis/ui-svelte/separator";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  Settings,
  ShieldAlert,
  Award,
  Calendar,
  Loader2,
  Save,
  Cpu,
} from "lucide-svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);

const queryClient = useQueryClient();
const policyQuery = createOrgAcademicPolicyQuery(() => orgId);

// Form states
let isSaving = $state(false);
let form = $state({
  default_grading_system: "percentage",
  default_passing_grade: 50,
  default_max_grade: 100,
  enrollment_auto_close: false,
  allow_late_enrollment: false,
  late_enrollment_approval: false,
  grade_entry_auto_close: false,
  allow_grade_edit_after_close: false,
  grade_edit_approval: false,
  autopilot_mode: "suggested",
  autopilot_enabled: false,
  auto_open_enrollment: false,
  auto_close_enrollment: false,
  auto_open_grade_entry: false,
});

// Sync database record to local form state
$effect(() => {
  if ($policyQuery.data) {
    const d = $policyQuery.data as any; // Type workaround
    form = {
      default_grading_system: d.default_grading_system || "percentage",
      default_passing_grade: d.default_passing_grade ?? 50,
      default_max_grade: d.default_max_grade ?? 100,
      enrollment_auto_close: d.enrollment_auto_close === 1,
      allow_late_enrollment: d.allow_late_enrollment === 1,
      late_enrollment_approval: d.late_enrollment_approval === 1,
      grade_entry_auto_close: d.grade_entry_auto_close === 1,
      allow_grade_edit_after_close: d.allow_grade_edit_after_close === 1,
      grade_edit_approval: d.grade_edit_approval === 1,
      autopilot_mode: d.autopilot_mode || "suggested",
      autopilot_enabled: d.autopilot_enabled === 1,
      auto_open_enrollment: d.auto_open_enrollment === 1,
      auto_close_enrollment: d.auto_close_enrollment === 1,
      auto_open_grade_entry: d.auto_open_grade_entry === 1,
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
      enrollment_auto_close: form.enrollment_auto_close ? 1 : 0,
      allow_late_enrollment: form.allow_late_enrollment ? 1 : 0,
      late_enrollment_approval: form.late_enrollment_approval ? 1 : 0,
      grade_entry_auto_close: form.grade_entry_auto_close ? 1 : 0,
      allow_grade_edit_after_close: form.allow_grade_edit_after_close ? 1 : 0,
      grade_edit_approval: form.grade_edit_approval ? 1 : 0,
      autopilot_mode: form.autopilot_mode,
      autopilot_enabled: form.autopilot_enabled ? 1 : 0,
      auto_open_enrollment: form.auto_open_enrollment ? 1 : 0,
      auto_close_enrollment: form.auto_close_enrollment ? 1 : 0,
      auto_open_grade_entry: form.auto_open_grade_entry ? 1 : 0,
      settings: null,
    };

    await commands.platform.updateOrgPolicy(orgId, payload);

    toast.success(
      m.settings_general_save_success
        ? m.settings_general_save_success()
        : "Settings saved successfully",
    );
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
        Retry
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
        <!-- AutoPilot Configuration Card -->
        <Card>
          <CardHeader>
            <div class="flex items-center gap-2">
              <Cpu class="size-4.5 text-primary" />
              <CardTitle class="text-sm">
                {m.autopilot_popover_config_title ? m.autopilot_popover_config_title() : "AutoPilot Configuration"}
              </CardTitle>
            </div>
            <CardDescription class="text-[11px]">
              Configure automated academic execution rules and operational mode.
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-col gap-4">
            <!-- Enabled Status -->
            <div class="flex items-center justify-between py-2 border-b border-border/40">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  Enable AutoPilot
                </span>
                <span class="text-[10px] text-muted-foreground">Toggle all background lifecycle automations.</span>
              </div>
              <Switch bind:checked={form.autopilot_enabled} disabled={isSaving} />
            </div>

            <!-- Mode Selection -->
            {#if form.autopilot_enabled}
              <div class="flex flex-col gap-2 py-2 border-b border-border/40">
                <Label class="text-xs text-muted-foreground">
                  {m.policy_field_exec_mode ? m.policy_field_exec_mode() : "Execution Mode"}
                </Label>
                <Select
                  type="single"
                  value={form.autopilot_mode}
                  onValueChange={(v) => { if (v) form.autopilot_mode = v; }}
                >
                  <SelectTrigger class="text-xs capitalize h-9">
                    {form.autopilot_mode === 'automatic' 
                      ? (m.autopilot_popover_mode_auto ? m.autopilot_popover_mode_auto() : "Automatic") 
                      : form.autopilot_mode === 'suggested' 
                        ? (m.autopilot_popover_mode_suggested ? m.autopilot_popover_mode_suggested() : "Suggested") 
                        : (m.autopilot_popover_mode_manual ? m.autopilot_popover_mode_manual() : "Manual")}
                  </SelectTrigger>
                  <SelectContent class="text-start text-xs">
                    <SelectItem value="automatic">{m.autopilot_popover_mode_auto()}</SelectItem>
                    <SelectItem value="suggested">{m.autopilot_popover_mode_suggested()}</SelectItem>
                    <SelectItem value="manual">{m.autopilot_popover_mode_manual()}</SelectItem>
                  </SelectContent>
                </Select>
              </div>


            {/if}
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
                <span class="text-[10px] text-muted-foreground">Permit registrations after active period enrollment closed.</span>
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
              <Switch bind:checked={form.late_enrollment_approval} disabled={isSaving || !form.allow_late_enrollment} />
            </div>

            <!-- Auto Close on Capacity -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_auto_close_capacity ? m.policy_field_auto_close_capacity() : "Auto Close on Capacity"}
                </span>
                <span class="text-[10px] text-muted-foreground">Automatically lock sections when max capacity is reached.</span>
              </div>
              <Switch bind:checked={form.enrollment_auto_close} disabled={isSaving} />
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
              <Switch bind:checked={form.grade_entry_auto_close} disabled={isSaving} />
            </div>

            <!-- Allow Grade Modification After Close -->
            <div class="flex items-center justify-between py-2 border-b border-border/40">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_allow_mod_after_close ? m.policy_field_allow_mod_after_close() : "Allow Modifications After Close"}
                </span>
                <span class="text-[10px] text-muted-foreground">Permit changes to grade sheets after period has closed.</span>
              </div>
              <Switch bind:checked={form.allow_grade_edit_after_close} disabled={isSaving} />
            </div>

            <!-- Grade Modification Requires Approval -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.policy_field_mod_approval ? m.policy_field_mod_approval() : "Modifications Require Approval"}
                </span>
                <span class="text-[10px] text-muted-foreground">Force all grade modifications to pass through approvals.</span>
              </div>
              <Switch bind:checked={form.grade_edit_approval} disabled={isSaving || !form.allow_grade_edit_after_close} />
            </div>
          </CardContent>
        </Card>
      </div>
    </form>
  {/if}
</div>
