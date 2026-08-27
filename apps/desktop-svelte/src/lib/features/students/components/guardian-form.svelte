<!--
  @component GuardianForm
  Form for linking a user as a guardian to a student with relationship, contact permissions, and payer status.
  Supports creating a new parent user inline if they do not exist.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import * as Sheet from "@orbis/ui-svelte/sheet";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { authStore } from "$lib/stores";
import type { JsonRecord } from "@orbis/sdk-core";
import { toast } from "svelte-sonner";
import { guardianFormInputSchema } from "@orbis/schemas";

import { createUsersQuery } from "$lib/features/settings/queries";
import { createEnrollmentsQuery } from "$lib/features/students/queries";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { debounce } from "$lib/shared/utils/debounce";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";

let {
  orgId,
  studentId,
  editData,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  studentId: string;
  editData?: any;
  onSuccess?: () => void;
  onCancel?: () => void;
} = $props();

const isEditMode = $derived(!!editData);

// Form state
let guardianUserId = $state("");
let fullName = $state("");
let phone = $state("");
let email = $state("");
let nationalId = $state("");
let relationship = $state("father");
let job = $state("");
let isPrimary = $state(false);
let isEmergencyContact = $state(false);
let isPayer = $state(false);
let paymentSharePercent = $state(0);

$effect(() => {
  guardianUserId = editData ? String(editData.guardian_id ?? "") : "";
  relationship = editData
    ? String(editData.relationship ?? editData.relationship ?? "father")
    : "father";
  job = editData
    ? String(
        (editData as unknown as JsonRecord).occupation ??
          (editData as unknown as JsonRecord).job ??
          "",
      )
    : "";
  isPrimary = editData
    ? Boolean(editData.is_primary === 1 || editData.is_primary === true)
    : false;
  isEmergencyContact = editData
    ? Boolean(editData.is_primary === 1 || editData.is_primary === true)
    : false;
  isPayer = editData
    ? Boolean(editData.is_primary === 1 || editData.is_primary === true)
    : false;
  paymentSharePercent = editData ? Number(editData.is_primary ?? 0) : 0;

  if (editData && editData.guardian_id) {
    commands.iam
      .getUser(String(editData.guardian_id))
      .then((u) => {
        if (u) {
          const user = u as unknown as JsonRecord;
          fullName = String(user.full_name ?? "");
          phone = String(user.phone ?? "");
          email = String(u.email ?? "");
          nationalId = String(user.national_id ?? "");
        }
      })
      .catch((e) => {
        console.error("Failed to load guardian details:", e);
      });
  } else {
    fullName = "";
    phone = "";
    email = "";
    nationalId = "";
  }
});

let localSearch = $state("");
let searchQuery = $state("");
const updateSearch = debounce((q: string) => {
  searchQuery = q;
}, 150);

// TanStack Queries
const usersQuery = createUsersQuery(() => orgId);
const enrollmentsQuery = createEnrollmentsQuery(() => orgId);

const allUsers = $derived(
  ((usersQuery.data as JsonRecord[]) ?? []).filter(
    (u) => String(u.primary_role ?? u.role) === "parent",
  ),
);
const enrollments = $derived((enrollmentsQuery.data as any[]) ?? []);

const isLoading = $derived(
  usersQuery.isLoading || enrollmentsQuery.isLoading,
);
let isSaving = $state(false);

const relationships = $derived([
  { value: "father", label: m.common_guardians_relationship_father() },
  { value: "mother", label: m.common_guardians_relationship_mother() },
  { value: "sibling", label: m.common_guardians_relationship_other() },
  { value: "grandparent", label: m.common_guardians_relationship_other() },
  { value: "uncle", label: m.common_guardians_relationship_other() },
  { value: "other", label: m.common_guardians_relationship_other() },
]);

const studentLastName = $derived.by(() => {
  const student = enrollments.find((s) => String(s.id) === String(studentId));
  if (!student) return "";
  const studentName = (student.full_name || student.name_ar || "").trim();
  const nameParts = studentName.split(/\s+/);
  return nameParts.length > 0
    ? nameParts[nameParts.length - 1].toLowerCase()
    : "";
});

const filteredUsers = $derived(
  allUsers
    .filter((u) => {
      const q = searchQuery.toLowerCase();
      if (!q) return true;
      const name = String(u.full_name ?? u.name_ar ?? "").toLowerCase();
      const emailVal = String(u.email ?? "").toLowerCase();
      const phoneVal = String(u.phone ?? u.phone_number ?? "").toLowerCase();
      return name.includes(q) || emailVal.includes(q) || phoneVal.includes(q);
    })
    .sort((a, b) => {
      if (!studentLastName) return 0;

      const nameA = String(a.full_name ?? a.name_ar ?? "").toLowerCase();
      const nameB = String(b.full_name ?? b.name_ar ?? "").toLowerCase();

      const partsA = nameA.trim().split(/\s+/);
      const partsB = nameB.trim().split(/\s+/);

      const lastNameA = partsA.length > 0 ? partsA[partsA.length - 1] : "";
      const lastNameB = partsB.length > 0 ? partsB[partsB.length - 1] : "";

      const matchA = lastNameA === studentLastName;
      const matchB = lastNameB === studentLastName;

      if (matchA && !matchB) return -1;
      if (!matchA && matchB) return 1;
      return 0;
    }),
);

const selectedRelation = $derived(
  relationships.find((r) => r.value === relationship),
);

function selectUser(u: JsonRecord) {
  guardianUserId = String(u.id);
  fullName = String(u.full_name ?? u.name_ar ?? "");
  phone = String(u.phone ?? u.phone_number ?? "");
  email = String(u.email ?? "");
  nationalId = String(u.national_id ?? "");
  localSearch = "";
  searchQuery = ""; // Close search results
}

function clearSelectedUser() {
  guardianUserId = "";
  fullName = "";
  phone = "";
  email = "";
  nationalId = "";
}

async function handleSubmit() {
  const validation = guardianFormInputSchema.safeParse({ fullName, phone, email, nationalId, relationship, paymentSharePercent });
  if (!validation.success) {
    toast.error(m.common_validation_guardian_name_required());
    return;
  }

  const { fullName: fullNameClean, phone: phoneClean, email: emailClean, nationalId: nationalIdClean } = validation.data;

  isSaving = true;
  try {
    const createdBy = authStore.user?.id || "";
    let finalGuardianUserId = guardianUserId;

    if (!finalGuardianUserId) {
      // Check if user already exists in db by phone
      const checkExisting = await commands.academic.search(phoneClean);
      if (checkExisting && checkExisting.length > 0) {
        finalGuardianUserId = String(
          checkExisting[0].guardian_id ?? checkExisting[0].id,
        );
        // Optionally update the existing user info
        await commands.iam.updateUser(finalGuardianUserId, {
          full_name: fullNameClean,
          phone: phoneClean || null,
        });
      } else {
        // Generate a secure random password for initial account setup
        const tempPassword = crypto.randomUUID().replace(/-/g, "").slice(0, 16);
        const guardEmail = emailClean || `parent.${Date.now()}@orbis.local`;

        // Create a new parent user
        const guardUser = await commands.iam.createUser({
          email: guardEmail,
          password_hash: tempPassword,
          full_name: fullNameClean,
          primary_role: "parent",
          phone: phoneClean || null,
        });
        finalGuardianUserId = guardUser.id;
      }
    } else {
      // Update existing parent user details
      await commands.iam.updateUser(finalGuardianUserId, {
        full_name: fullNameClean,
        phone: phoneClean || null,
      });
    }

    const relationshipAr =
      relationship === "father"
        ? "أب"
        : relationship === "mother"
          ? "أم"
          : relationship === "uncle"
            ? "عم"
            : "آخر/قريب";

    // Construct request conforming exactly to CreateGuardianLinkRequest / UpdateGuardianLinkRequest
    const request: JsonRecord = {
      student_id: studentId,
      relationship: relationship,
      relationship_ar: relationshipAr,
      is_primary: isPrimary,
      is_authorized_pickup: false,
      occupation: job.trim() || null,
      can_view_grades: true,
      can_view_attendance: true,
      can_view_behavior: true,
      can_view_finance: isPayer,
      can_view_schedule: true,
      can_receive_notifications: true,
    };

    if (isEditMode && editData?.id) {
      await commands.academic.updateGuardian(String(editData.id), request);
      toast.success(m.common_guardians_update_success());
    } else {
      // Create link requires guardian_id. Let's make sure it is sent correctly.
      const requestWithGuardian = {
        ...request,
        guardian_id: finalGuardianUserId,
      };
      await commands.academic.createGuardian(requestWithGuardian as any);
      toast.success(m.common_guardians_create_success());
    }

    onSuccess?.();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSaving = false;
  }
}
</script>

<form
  class="flex flex-col h-full overflow-hidden gap-0"
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
>
  <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0 text-start">
    <Sheet.Title class="text-base font-bold text-foreground">
      {isEditMode ? m.common_action_edit() : m.common_guardians_add()}
    </Sheet.Title>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
    {#if isLoading}
      <div class="flex flex-col gap-5 animate-pulse">
        <div class="flex flex-col gap-1.5">
          <Skeleton class="h-3.5 w-24" />
          <Skeleton class="h-8 w-full rounded-md" />
        </div>
        <div class="flex flex-col gap-1.5">
          <Skeleton class="h-3.5 w-20" />
          <Skeleton class="h-9 w-full rounded-md" />
        </div>
        <div class="flex flex-col gap-1.5">
          <Skeleton class="h-3.5 w-28" />
          <Skeleton class="h-9 w-full rounded-md" />
        </div>
        <div class="flex flex-col gap-1.5">
          <Skeleton class="h-3.5 w-16" />
          <Skeleton class="h-9 w-full rounded-md" />
        </div>
      </div>
    {:else}
      <Accordion type="multiple" value={["contact", "permissions"]} class="flex flex-col gap-4">
        <AccordionItem value="contact" class="rounded-lg">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_word_fullname()}</AccordionTrigger>
          <AccordionContent>
      <!-- User Search + Select (only show in create mode) -->
      {#if !isEditMode}
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-semibold text-muted-foreground">{m.common_form_guardian_search_label()}</Label>
          <Input
            type="text"
            placeholder={m.common_form_guardian_search_placeholder()}
            bind:value={localSearch}
            oninput={() => updateSearch(localSearch)}
            class="h-8 text-xs mb-1"
          />
          
          {#if !guardianUserId}
            <div class="rounded-lg border border-border/40 bg-card/50 overflow-hidden">
              {#if searchQuery.trim().length === 0}
                {#if filteredUsers.length === 0}
                  <div class="p-3 text-xs text-muted-foreground text-center">{m.common_switcher_no_results()}</div>
                {:else}
                  {#each filteredUsers.slice(0, 3) as u (u.id)}
                    <button
                      type="button"
                      class="w-full text-start px-3 py-2 text-xs hover:bg-accent/60 transition-colors flex items-center justify-between border-b border-border/30 last:border-b-0
                        {guardianUserId === String(u.id) ? 'bg-primary/10 text-primary font-semibold' : 'text-foreground'}"
                      onclick={() => selectUser(u)}
                    >
                      <span>{String(u.full_name ?? u.email ?? u.id)}</span>
                      {#if u.phone}
                        <span class="text-[10px] text-muted-foreground font-mono">{String(u.phone)}</span>
                      {/if}
                    </button>
                  {/each}
                {/if}
              {:else}
                <!-- Search query active - show scrollable list -->
                <div class="max-h-40 overflow-y-auto">
                  {#if filteredUsers.length === 0}
                    <div class="p-3 text-xs text-muted-foreground text-center">{m.common_switcher_no_results()}</div>
                  {:else}
                    {#each filteredUsers.slice(0, 20) as u (u.id)}
                      <button
                        type="button"
                        class="w-full text-start px-3 py-2 text-xs hover:bg-accent/60 transition-colors flex items-center justify-between border-b border-border/30 last:border-b-0
                          {guardianUserId === String(u.id) ? 'bg-primary/10 text-primary font-semibold' : 'text-foreground'}"
                        onclick={() => selectUser(u)}
                      >
                        <span>{String(u.full_name ?? u.email ?? u.id)}</span>
                        {#if u.phone}
                          <span class="text-[10px] text-muted-foreground font-mono">{String(u.phone)}</span>
                        {/if}
                      </button>
                    {/each}
                  {/if}
                </div>
              {/if}
            </div>
          {/if}

          {#if guardianUserId}
            <div class="flex items-center justify-between p-2 rounded bg-primary/10 border border-primary/20 text-xs mt-1">
              <span class="text-primary font-medium">
                ✓ {m.common_guardians_sheet_selected_label({ name: fullName })}
              </span>
              <Button
                type="button"
                variant="ghost"
                class="h-5 px-1.5 text-[10px] text-destructive hover:bg-destructive/10"
                onclick={clearSelectedUser}
              >
                {m.common_guardians_sheet_clear_selection()}
              </Button>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Full Name -->
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.common_word_fullname()} *</Label>
        <Input
          type="text"
          bind:value={fullName}
          placeholder="الاسم الكامل لولي الأمر"
          class="h-9 text-sm"
        />
      </div>

      <!-- Phone -->
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.common_word_phone()} *</Label>
        <Input
          type="text"
          bind:value={phone}
          placeholder="777000000"
          class="h-9 text-sm font-mono text-start"
        />
      </div>

      <!-- Email -->
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.students_guardian_email_optional()}</Label>
        <Input
          type="email"
          bind:value={email}
          placeholder="example@orbis.local"
          class="h-9 text-sm text-start"
        />
      </div>

      <!-- National ID -->
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.common_form_guardian_national_id_label()}</Label>
        <Input
          type="text"
          bind:value={nationalId}
          placeholder="رقم هوية ولي الأمر"
          class="h-9 text-sm font-mono"
        />
      </div>

      <!-- Relation -->
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.common_bulk_col_guardian_rel()} *</Label>
        <Select type="single" value={relationship} onValueChange={(v: string | undefined) => (relationship = v ?? "father")}>
          <SelectTrigger class="h-9 text-sm">
            {selectedRelation?.label ?? relationship}
          </SelectTrigger>
          <SelectContent>
            {#each relationships as r (r.value)}
              <SelectItem value={r.value}>{r.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Job -->
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.common_guardians_form_job()}</Label>
        <Input
          type="text"
          bind:value={job}
          placeholder="المهنة الحالية لولي الأمر"
          class="h-9 text-sm"
        />
      </div>

          </AccordionContent>
        </AccordionItem>

        <AccordionItem value="permissions" class="rounded-lg">
          <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.students_guardian_permissions()}</AccordionTrigger>
          <AccordionContent>

      <!-- Permissions and Financial Responsibility -->
      <div class="flex flex-col gap-3 p-4 rounded-lg bg-muted/20 border border-border/30">
        <p class="text-xs font-bold text-foreground">{m.students_guardian_permissions()}</p>
        
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
          <div class="flex items-center gap-2">
            <Checkbox id="primary_chk" checked={isPrimary} onCheckedChange={(val: boolean | "indeterminate") => isPrimary = val === true} />
            <Label for="primary_chk" class="text-xs cursor-pointer">{m.common_form_guardian_role_primary()}</Label>
          </div>

          <div class="flex items-center gap-2">
            <Checkbox id="emergency_chk" checked={isEmergencyContact} onCheckedChange={(val: boolean | "indeterminate") => isEmergencyContact = val === true} />
            <Label for="emergency_chk" class="text-xs cursor-pointer">{m.common_form_guardian_role_emergency()}</Label>
          </div>

          <div class="flex items-center gap-2">
            <Checkbox id="payer_chk" checked={isPayer} onCheckedChange={(val: boolean | "indeterminate") => isPayer = val === true} />
            <Label for="payer_chk" class="text-xs cursor-pointer">{m.common_form_guardian_role_payer()}</Label>
          </div>
        </div>

        {#if isPayer}
          <div class="flex items-center justify-start gap-2 pt-2 border-t border-border/40">
            <Label class="text-xs text-muted-foreground">{m.common_form_guardian_payment_share()}</Label>
            <div class="flex items-center gap-1">
              <Input type="number" min="0" max="100" class="h-8 w-16 text-center font-bold px-1 bg-background" bind:value={paymentSharePercent} />
              <span class="text-xs text-muted-foreground">%</span>
            </div>
          </div>
        {/if}
      </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    {/if}
  </div>

  <!-- Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
    <Button
      type="button"
        variant="ghost"
      class="flex-1 h-9 text-sm"
      onclick={onCancel}
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      class="flex-1 h-9 text-sm font-semibold"
      disabled={isSaving || isLoading || !fullName || !phone || !relationship}
    >
      {isSaving ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
