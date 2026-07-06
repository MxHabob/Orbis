<!--
  @component GuardianForm
  Form for linking a user as a guardian to a student with relation, contact permissions, and payer status.
  Supports creating a new parent user inline if they do not exist.
-->
<script lang="ts">
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
import { commands } from "$lib/core/ipc";
import { authStore } from "$lib/stores";
import type { GuardianLink, JsonRecord } from "@orbis/desktop-client";
import { toast } from "svelte-sonner";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";

let {
  orgId,
  studentId,
  editData,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  studentId: string;
  editData?: GuardianLink | null;
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
let relation = $state("father");
let job = $state("");
let isPrimary = $state(false);
let isEmergencyContact = $state(false);
let isPayer = $state(false);
let paymentSharePercent = $state(0);

$effect(() => {
  guardianUserId = editData ? String(editData.guardian_id ?? "") : "";
  relation = editData
    ? String(editData.relationship ?? editData.relation ?? "father")
    : "father";
  job = editData
    ? String(
        (editData as JsonRecord).occupation ??
          (editData as JsonRecord).job ??
          "",
      )
    : "";
  isPrimary = editData
    ? Boolean(editData.is_primary === 1 || editData.is_primary === true)
    : false;
  isEmergencyContact = editData
    ? Boolean(
        editData.is_emergency_contact === 1 ||
          editData.is_emergency_contact === true,
      )
    : false;
  isPayer = editData
    ? Boolean(editData.is_payer === 1 || editData.is_payer === true)
    : false;
  paymentSharePercent = editData
    ? Number(editData.payment_share_percent ?? 0)
    : 0;

  if (editData && editData.guardian_id) {
    commands.users
      .getUser(String(editData.guardian_id))
      .then((u) => {
        if (u) {
          fullName = String(u.full_name ?? u.name_ar ?? "");
          phone = String(u.phone ?? u.phone_number ?? "");
          email = String(u.email ?? "");
          nationalId = String(u.national_id ?? "");
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

let searchQuery = $state("");

// Remote data
let allUsers = $state<JsonRecord[]>([]);
let isLoading = $state(false);
let isSaving = $state(false);

const relations = $derived([
  { value: "father", label: m.guardians_relation_father() },
  { value: "mother", label: m.guardians_relation_mother() },
  { value: "sibling", label: m.guardians_relation_sibling() },
  { value: "grandparent", label: m.guardians_relation_grandparent() },
  { value: "uncle", label: m.guardians_relation_uncle() },
  { value: "other", label: m.guardians_relation_other() },
]);

let studentLastName = $state("");

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

const selectedRelation = $derived(relations.find((r) => r.value === relation));

onMount(async () => {
  isLoading = true;
  try {
    const [users, enrollments] = await Promise.all([
      commands.users.getUsers(orgId, { role: "parent" }),
      commands.enrollments.getAll(orgId),
    ]);
    allUsers = users as unknown as JsonRecord[];

    const student = (enrollments as any[]).find(
      (s) => String(s.id) === String(studentId),
    );
    if (student) {
      const studentName = (student.full_name || student.name_ar || "").trim();
      const nameParts = studentName.split(/\s+/);
      if (nameParts.length > 0) {
        studentLastName = nameParts[nameParts.length - 1].toLowerCase();
      }
    }
  } catch (e) {
    console.error("Failed to load data for guardian linking:", e);
  } finally {
    isLoading = false;
  }
});

function selectUser(u: JsonRecord) {
  guardianUserId = String(u.id);
  fullName = String(u.full_name ?? u.name_ar ?? "");
  phone = String(u.phone ?? u.phone_number ?? "");
  email = String(u.email ?? "");
  nationalId = String(u.national_id ?? "");
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
  if (!fullName.trim()) {
    toast.error("يرجى إدخال الاسم الكامل لولي الأمر");
    return;
  }
  if (!phone.trim()) {
    toast.error("يرجى إدخال رقم الهاتف لولي الأمر");
    return;
  }
  if (!relation) {
    toast.error("يرجى تحديد صلة القرابة");
    return;
  }

  isSaving = true;
  try {
    const createdBy = authStore.user?.id || "";
    let finalGuardianUserId = guardianUserId;

    if (!finalGuardianUserId) {
      // Check if user already exists in db by phone
      const checkExisting = await commands.guardians.search(
        orgId,
        phone.trim(),
      );
      if (checkExisting && checkExisting.length > 0) {
        finalGuardianUserId = checkExisting[0].id;
        // Optionally update the existing user info
        await commands.users.updateUser(finalGuardianUserId, {
          full_name: fullName.trim(),
          name_ar: fullName.trim(),
          phone: phone.trim() || null,
          national_id: nationalId.trim() || null,
        });
      } else {
        // Create a new parent user
        const guardEmail = email.trim() || `parent.${Date.now()}@orbis.local`;
        const guardUser = await commands.users.createUser(orgId, createdBy, {
          email: guardEmail,
          password: "Orbis123456",
          full_name: fullName.trim(),
          name_ar: fullName.trim(),
          primary_role: "parent",
          phone: phone.trim() || null,
          sex: "male",
          national_id: nationalId.trim() || null,
          date_of_birth: null,
        });
        finalGuardianUserId = guardUser.id;
      }
    } else {
      // Update existing parent user details
      await commands.users.updateUser(finalGuardianUserId, {
        full_name: fullName.trim(),
        name_ar: fullName.trim(),
        phone: phone.trim() || null,
        national_id: nationalId.trim() || null,
      });
    }

    const relationAr =
      relation === "father"
        ? "أب"
        : relation === "mother"
          ? "أم"
          : relation === "uncle"
            ? "عم"
            : "آخر/قريب";

    // Construct request conforming exactly to CreateGuardianLinkRequest / UpdateGuardianLinkRequest
    const request: JsonRecord = {
      student_id: studentId,
      guardian_id: finalGuardianUserId,
      relationship: relation,
      relationship_ar: relationAr,
      is_primary: isPrimary ? 1 : 0,
      is_emergency_contact: isEmergencyContact ? 1 : 0,
      is_payer: isPayer ? 1 : 0,
      payment_share_percent: isPayer ? paymentSharePercent : 0,
      occupation: job.trim() || null,
      can_view_grades: 1,
      can_view_attendance: 1,
      can_view_behavior: 1,
      can_view_finance: isPayer ? 1 : 0,
      can_view_schedule: 1,
      can_receive_notifications: 1,
    };

    if (isEditMode && editData?.id) {
      await commands.guardians.updateLink(String(editData.id), request);
      toast.success(m.guardians_update_success());
    } else {
      await commands.guardians.createLink(request);
      toast.success(m.guardians_create_success());
    }

    onSuccess?.();
  } catch (e: unknown) {
    const errMsg = (e as any)?.message || String(e);
    toast.error(`فشل الحفظ: ${errMsg}`);
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
      {isEditMode ? m.guardians_sheet_edit_title() : m.guardians_sheet_create_title()}
    </Sheet.Title>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
    <!-- User Search + Select (only show in create mode) -->
    {#if !isEditMode}
      <div class="flex flex-col gap-1.5">
        <Label class="text-xs font-semibold text-muted-foreground">{m.form_guardian_search_label()}</Label>
        <Input
          type="text"
          placeholder={m.form_guardian_search_placeholder()}
          bind:value={searchQuery}
          class="h-8 text-xs mb-1"
        />
        
        {#if !guardianUserId}
          <div class="rounded-lg border border-border/40 bg-card/50 overflow-hidden">
            {#if searchQuery.trim().length === 0}
              {#if filteredUsers.length === 0}
                <div class="p-3 text-xs text-muted-foreground text-center">{m.guardians_sheet_no_results()}</div>
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
                  <div class="p-3 text-xs text-muted-foreground text-center">{m.guardians_sheet_no_results()}</div>
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
              ✓ {m.guardians_sheet_selected_label({ name: fullName })}
            </span>
            <Button
              type="button"
              variant="ghost"
              class="h-5 px-1.5 text-[10px] text-destructive hover:bg-destructive/10"
              onclick={clearSelectedUser}
            >
              {m.guardians_sheet_clear_selection()}
            </Button>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Full Name -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.form_guardian_name_label()} *</Label>
      <Input
        type="text"
        bind:value={fullName}
        placeholder="الاسم الكامل لولي الأمر"
        class="h-9 text-sm"
      />
    </div>

    <!-- Phone -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.form_guardian_phone_label()} *</Label>
      <Input
        type="text"
        bind:value={phone}
        placeholder="777000000"
        class="h-9 text-sm font-mono text-start"
        dir="ltr"
      />
    </div>

    <!-- Email -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">البريد الإلكتروني (اختياري)</Label>
      <Input
        type="email"
        bind:value={email}
        placeholder="example@orbis.local"
        class="h-9 text-sm text-start"
        dir="ltr"
      />
    </div>

    <!-- National ID -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.form_guardian_national_id_label()}</Label>
      <Input
        type="text"
        bind:value={nationalId}
        placeholder="رقم هوية ولي الأمر"
        class="h-9 text-sm font-mono"
      />
    </div>

    <!-- Relation -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.guardians_form_relation()} *</Label>
      <Select type="single" value={relation} onValueChange={(v) => (relation = v ?? "father")}>
        <SelectTrigger class="h-9 text-sm">
          {selectedRelation?.label ?? relation}
        </SelectTrigger>
        <SelectContent>
          {#each relations as r (r.value)}
            <SelectItem value={r.value}>{r.label}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Job -->
    <div class="flex flex-col gap-1.5">
      <Label class="text-xs font-semibold text-muted-foreground">{m.guardians_form_job()}</Label>
      <Input
        type="text"
        bind:value={job}
        placeholder="المهنة الحالية لولي الأمر"
        class="h-9 text-sm"
      />
    </div>

    <!-- Permissions and Financial Responsibility -->
    <div class="flex flex-col gap-3 p-4 rounded-lg bg-muted/20 border border-border/30">
      <p class="text-xs font-bold text-foreground">الصلاحيات والمسؤوليات</p>
      
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
        <div class="flex items-center gap-2">
          <Checkbox id="primary_chk" checked={isPrimary} onCheckedChange={(val) => isPrimary = !!val} />
          <Label for="primary_chk" class="text-xs cursor-pointer">{m.form_guardian_role_primary()}</Label>
        </div>

        <div class="flex items-center gap-2">
          <Checkbox id="emergency_chk" checked={isEmergencyContact} onCheckedChange={(val) => isEmergencyContact = !!val} />
          <Label for="emergency_chk" class="text-xs cursor-pointer">{m.form_guardian_role_emergency()}</Label>
        </div>

        <div class="flex items-center gap-2">
          <Checkbox id="payer_chk" checked={isPayer} onCheckedChange={(val) => isPayer = !!val} />
          <Label for="payer_chk" class="text-xs cursor-pointer">{m.form_guardian_role_payer()}</Label>
        </div>
      </div>

      {#if isPayer}
        <div class="flex items-center justify-start gap-2 pt-2 border-t border-border/40">
          <Label class="text-xs text-muted-foreground">{m.form_guardian_payment_share()}</Label>
          <div class="flex items-center gap-1">
            <Input type="number" min="0" max="100" class="h-8 w-16 text-center font-bold px-1 bg-background" bind:value={paymentSharePercent} />
            <span class="text-xs text-muted-foreground">%</span>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- Actions -->
  <div class="px-6 py-4 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
    <Button
      type="button"
      variant="outline"
      class="flex-1 h-9 text-sm"
      onclick={onCancel}
    >
      {m.guardians_form_cancel()}
    </Button>
    <Button
      type="submit"
      class="flex-1 h-9 text-sm font-semibold"
      disabled={isSaving || isLoading || !fullName || !phone || !relation}
    >
      {isSaving ? m.guardians_form_saving() : m.guardians_form_save()}
    </Button>
  </div>
</form>
