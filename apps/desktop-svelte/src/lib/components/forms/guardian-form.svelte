<!--
  @component GuardianForm
  Form for linking a user as a guardian to a student with relation and payer status.
  Fixes API type mismatches by formatting boolean flags as integer values (1 / 0) and correcting property names.
-->
<script lang="ts">
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
import { commands } from "$lib/services/ipc";
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
let guardianUserId = $state(editData ? String(editData.guardian_id ?? "") : "");
let relation = $state(
  editData ? String(editData.relationship ?? editData.relation ?? "father") : "father",
);
let isPayer = $state(editData ? Boolean(editData.is_payer === 1 || editData.is_payer === true) : false);
let job = $state(editData ? String((editData as JsonRecord).occupation ?? (editData as JsonRecord).job ?? "") : "");
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

const filteredUsers = $derived(
  allUsers.filter((u) => {
    const q = searchQuery.toLowerCase();
    if (!q) return true;
    const name = String(u.full_name ?? "").toLowerCase();
    const email = String(u.email ?? "").toLowerCase();
    return name.includes(q) || email.includes(q);
  }),
);

const selectedUser = $derived(
  allUsers.find((u) => String(u.id) === guardianUserId),
);

const selectedRelation = $derived(relations.find((r) => r.value === relation));

onMount(async () => {
  isLoading = true;
  try {
    const users = await commands.users.getUsers(orgId);
    allUsers = users as unknown as JsonRecord[];
  } catch (e) {
    console.error("Failed to load users", e);
  } finally {
    isLoading = false;
  }
});

async function handleSubmit() {
  if (!guardianUserId || !relation) {
    toast.error("يرجى اختيار المستخدم وتحديد صلة القرابة");
    return;
  }

  isSaving = true;
  try {
    const relationAr = relation === "father" ? "أب" : relation === "mother" ? "أم" : "قريب";
    
    // Construct request conforming exactly to CreateGuardianLinkRequest / UpdateGuardianLinkRequest
    const request: JsonRecord = {
      student_id: studentId,
      guardian_id: guardianUserId,
      relationship: relation,
      relationship_ar: relationAr,
      is_primary: relation === "father" ? 1 : 0,
      is_emergency_contact: relation === "father" ? 1 : 0,
      is_payer: isPayer ? 1 : 0, // Cast boolean to i32
      occupation: job.trim() || null, // Map to occupation instead of job
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
  class="flex flex-col gap-5"
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
>
  <!-- User Search + Select -->
  <div class="flex flex-col gap-1.5">
    <Label class="text-xs font-semibold text-muted-foreground">{m.guardians_form_user()} *</Label>
    <Input
      type="text"
      placeholder={m.guardians_form_select_user()}
      bind:value={searchQuery}
      class="h-8 text-xs mb-1"
    />
    <div class="max-h-40 overflow-y-auto rounded-lg border border-border/40 bg-card/50">
      {#if filteredUsers.length === 0}
        <div class="p-3 text-xs text-muted-foreground text-center">لا توجد نتائج</div>
      {:else}
        {#each filteredUsers.slice(0, 20) as u (u.id)}
          <button
            type="button"
            class="w-full text-start px-3 py-2 text-xs hover:bg-accent/60 transition-colors flex items-center justify-between
              {guardianUserId === String(u.id) ? 'bg-primary/10 text-primary font-semibold' : 'text-foreground'}"
            onclick={() => (guardianUserId = String(u.id))}
          >
            <span>{String(u.full_name ?? u.email ?? u.id)}</span>
            {#if u.email}
              <span class="text-[10px] text-muted-foreground font-mono">{String(u.email)}</span>
            {/if}
          </button>
        {/each}
      {/if}
    </div>
    {#if selectedUser}
      <p class="text-[10px] text-primary font-medium">
        ✓ تم اختيار: {String(selectedUser.full_name ?? selectedUser.email)}
      </p>
    {/if}
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
      placeholder="مثال: مهندس، معلم، موظف..."
      class="h-9 text-sm"
    />
  </div>

  <!-- Is Payer -->
  <div class="flex items-center justify-between p-3 rounded-lg bg-muted/20 border border-border/30">
    <div>
      <p class="text-xs font-semibold text-foreground">{m.guardians_form_is_payer()}</p>
      <p class="text-[10px] text-muted-foreground">سيتمكن من استلام فواتير الرسوم</p>
    </div>
    <Switch
      checked={isPayer}
      onCheckedChange={(checked) => isPayer = checked}
    />
  </div>

  <!-- Actions -->
  <div class="flex gap-2 pt-2 border-t border-border/40">
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
      disabled={isSaving || isLoading || !guardianUserId || !relation}
    >
      {isSaving ? m.guardians_form_saving() : m.guardians_form_save()}
    </Button>
  </div>
</form>
