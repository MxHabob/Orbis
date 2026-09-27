<!--
  @component GuardianForm
  Form for assigning/linking an existing person/user as a guardian to a student.
  Uses FormController with TanStack Form & Zod schema validation as single source of truth.
  Strictly references canonical guardian identity and executes api.assignGuardian directly.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import * as Sheet from "@orbis/ui/sheet";
import {
  api,
  type AssignGuardianRequest,
  type GuardianDto,
  type UserDto,
} from "$lib/api";
import { toast } from "@orbis/ui";
import {
  guardianFormInputSchema,
  type GuardianFormInput,
} from "@orbis/schemas";
import { FormField } from "@orbis/ui";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { createUsersQuery } from "$lib/features/settings/queries";
import { debounce } from "$lib/shared/utils/debounce";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";

let {
  orgId,
  studentId,
  editData = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  studentId: string;
  editData?: GuardianDto | null;
  onSuccess?: () => void;
  onCancel?: () => void;
} = $props();

const isEditMode = $derived(!!editData);

// Single Source of Truth Form Controller
const form = new FormController<GuardianFormInput>({
  initialValues: {
    guardianUserId: editData?.guardian_id ?? "",
    fullName: "",
    phone: "",
    email: null,
    nationalId: null,
    relationship: editData?.relationship ?? "father",
    relationshipAr: null,
    occupation: null,
    isPrimary: editData?.is_primary ?? false,
    isEmergencyContact: editData?.is_emergency_contact ?? true,
    isAuthorizedPickup: editData?.is_authorized_pickup ?? false,
    isPayer: editData?.is_payer ?? false,
    paymentSharePercent: 0,
  },
  schema: guardianFormInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

// Populate guardian details when editing
$effect(() => {
  if (editData && editData.guardian_id) {
    form.setField("guardianUserId", editData.guardian_id);
    form.setField("relationship", editData.relationship ?? "father");
    form.setField("isPrimary", Boolean(editData.is_primary));
    form.setField("isEmergencyContact", Boolean(editData.is_emergency_contact));
    form.setField("isAuthorizedPickup", Boolean(editData.is_authorized_pickup));
    form.setField("isPayer", Boolean(editData.is_payer));

    api
      .getUser(editData.guardian_id)
      .then((user: UserDto) => {
        if (user) {
          form.setField("fullName", user.full_name ?? "");
          form.setField("phone", user.phone ?? "");
          form.setField("email", user.email ?? null);
          form.setField("nationalId", user.national_id ?? null);
        }
      })
      .catch((e: unknown) => {
        console.error("Failed to load guardian user details:", e);
      });
  }
});

// Search functionality for selecting existing parent/contact users in organization
let localSearch = $state("");
let searchQuery = $state("");
const updateSearch = debounce((q: string) => {
  searchQuery = q.trim();
}, 150);

const usersQuery = createUsersQuery(() => orgId);
const parentUsers = $derived(
  (usersQuery.data ?? []).filter((u: UserDto) =>
    (u.roles ?? []).includes("parent") || u.status === "active",
  ),
);

const filteredUsers = $derived(
  !searchQuery
    ? []
    : parentUsers.filter((u: UserDto) => {
        const q = searchQuery.toLowerCase();
        const name = (u.full_name ?? u.full_name_ar ?? "").toLowerCase();
        const emailVal = (u.email ?? "").toLowerCase();
        const phoneVal = (u.phone ?? "").toLowerCase();
        return (
          name.includes(q) || emailVal.includes(q) || phoneVal.includes(q)
        );
      }),
);

const relationships = $derived([
  { value: "father", label: m.common_guardians_relationship_father() },
  { value: "mother", label: m.common_guardians_relationship_mother() },
  { value: "sibling", label: m.common_guardians_relationship_other() },
  { value: "grandparent", label: m.common_guardians_relationship_other() },
  { value: "uncle", label: m.common_guardians_relationship_other() },
  { value: "other", label: m.common_guardians_relationship_other() },
]);

const selectedRelation = $derived(
  relationships.find((r) => r.value === form.values.relationship),
);

function selectUser(u: UserDto) {
  form.setField("guardianUserId", u.id);
  form.setField("fullName", u.full_name ?? u.full_name_ar ?? "");
  form.setField("phone", u.phone ?? "");
  form.setField("email", u.email ?? null);
  form.setField("nationalId", u.national_id ?? null);
  localSearch = "";
  searchQuery = "";
}

function clearSelectedUser() {
  form.setField("guardianUserId", "");
  form.setField("fullName", "");
  form.setField("phone", "");
  form.setField("email", null);
  form.setField("nationalId", null);
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  const success = await form.submit(async (values: GuardianFormInput) => {
    const relationshipAr =
      values.relationship === "father"
        ? "أب"
        : values.relationship === "mother"
          ? "أم"
          : values.relationship === "uncle"
            ? "عم"
            : "آخر/قريب";

    const request: AssignGuardianRequest = {
      student_id: studentId,
      guardian_id: values.guardianUserId,
      relationship: values.relationship,
      relationship_ar: relationshipAr,
      is_primary: values.isPrimary,
      is_authorized_pickup: values.isAuthorizedPickup,
      is_emergency_contact: values.isEmergencyContact,
      is_payer: values.isPayer,
      payment_share_percent: values.isPayer ? values.paymentSharePercent : 0,
    };

    await api.assignGuardian(request);
  });

  if (success) {
    toast.success(
      isEditMode
        ? m.common_guardians_update_success()
        : m.common_guardians_create_success(),
    );
    form.reset();
    onSuccess?.();
  } else if (form.formError) {
    toast.error(form.formError);
  } else if (Object.keys(form.errors).length > 0) {
    toast.error(fieldError("guardianUserId") || m.common_validation_guardian_name_required());
  }
}
</script>

<form
  class="flex flex-col h-full overflow-hidden gap-0"
  onsubmit={handleSubmit}
>
  <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0 text-start">
    <Sheet.Title class="text-base font-bold text-foreground">
      {isEditMode ? m.common_action_edit() : m.common_guardians_add()}
    </Sheet.Title>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
    <Accordion type="multiple" value={["contact"]} class="flex flex-col gap-5">
      <AccordionItem value="contact" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_word_fullname()}</AccordionTrigger>
        <AccordionContent class="pt-6 space-y-6">
          <!-- User Search + Select (in create mode) -->
          {#if !isEditMode}
            <div class="flex flex-col gap-3">
              <Label class="text-xs font-semibold text-muted-foreground">{m.common_form_guardian_search_label()}</Label>
              <Input
                type="text"
                placeholder={m.common_form_guardian_search_placeholder()}
                bind:value={localSearch}
                oninput={() => updateSearch(localSearch)}
                class="h-10 text-sm"
              />
              
              {#if !form.values.guardianUserId && searchQuery.length > 0}
                <div class="rounded-lg border border-border/40 bg-card/50 overflow-hidden max-h-40 overflow-y-auto">
                  {#if filteredUsers.length === 0}
                    <div class="p-3 text-xs text-muted-foreground text-center">{m.common_switcher_no_results()}</div>
                  {:else}
                    {#each filteredUsers.slice(0, 10) as u (u.id)}
                      <button
                        type="button"
                        class="w-full text-start px-3 py-2 text-xs hover:bg-accent/60 transition-colors flex items-center justify-between border-b border-border/30 last:border-b-0"
                        onclick={() => selectUser(u)}
                      >
                        <span class="font-medium">{u.full_name || u.email || u.id}</span>
                        {#if u.phone}
                          <span class="text-[10px] text-muted-foreground font-mono">{u.phone}</span>
                        {/if}
                      </button>
                    {/each}
                  {/if}
                </div>
              {/if}

              {#if form.values.guardianUserId}
                <div class="flex items-center justify-between p-2 rounded bg-primary/10 border border-primary/20 text-xs">
                  <span class="text-primary font-medium">
                    ✓ {m.common_guardians_sheet_selected_label({ name: form.values.fullName || form.values.guardianUserId })}
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

          <!-- Guardian Identity Details (Read-only verified demographic info) -->
          {#if form.values.guardianUserId}
            <div class="p-3.5 rounded-lg border border-border/60 bg-muted/20 space-y-2 text-xs">
              <div class="flex justify-between">
                <span class="text-muted-foreground">{m.common_word_fullname()}:</span>
                <span class="font-semibold text-foreground">{form.values.fullName || "—"}</span>
              </div>
              <div class="flex justify-between font-mono">
                <span class="text-muted-foreground">{m.common_word_phone()}:</span>
                <span class="text-foreground">{form.values.phone || "—"}</span>
              </div>
              {#if form.values.email}
                <div class="flex justify-between font-mono">
                  <span class="text-muted-foreground">{m.common_word_email()}:</span>
                  <span class="text-foreground">{form.values.email}</span>
                </div>
              {/if}
              {#if form.values.nationalId}
                <div class="flex justify-between font-mono">
                  <span class="text-muted-foreground">{m.common_form_guardian_national_id_label()}:</span>
                  <span class="text-foreground">{form.values.nationalId}</span>
                </div>
              {/if}
            </div>
          {:else}
            <div class="p-4 rounded-lg border border-dashed border-border/60 text-center text-xs text-muted-foreground">
              يرجى البحث عن ولي الأمر واختياره لربطه بالطالب
            </div>
          {/if}

          <!-- Relation -->
          <FormField id="guardian-relationship" label={m.common_bulk_col_guardian_rel()} error={fieldError("relationship")} required>
            {#snippet children()}
              <Select
                type="single"
                value={form.values.relationship}
                onValueChange={(v: string | undefined) => {
                  if (v) form.setField("relationship", v);
                }}
              >
                <SelectTrigger id="guardian-relationship" class="h-10 text-sm" aria-invalid={Boolean(fieldError("relationship"))}>
                  {selectedRelation?.label ?? form.values.relationship}
                </SelectTrigger>
                <SelectContent>
                  {#each relationships as r (r.value)}
                    <SelectItem value={r.value}>{r.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/snippet}
          </FormField>
        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="permissions" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.students_guardian_permissions()}</AccordionTrigger>
        <AccordionContent class="pt-6 space-y-6">
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <div class="flex items-center gap-2">
              <Checkbox
                id="primary_chk"
                checked={form.values.isPrimary}
                onCheckedChange={(val: boolean | "indeterminate") => form.setField("isPrimary", val === true)}
              />
              <Label for="primary_chk" class="text-xs cursor-pointer">{m.common_form_guardian_role_primary()}</Label>
            </div>

            <div class="flex items-center gap-2">
              <Checkbox
                id="emergency_chk"
                checked={form.values.isEmergencyContact}
                onCheckedChange={(val: boolean | "indeterminate") => form.setField("isEmergencyContact", val === true)}
              />
              <Label for="emergency_chk" class="text-xs cursor-pointer">{m.common_form_guardian_role_emergency()}</Label>
            </div>

            <div class="flex items-center gap-2">
              <Checkbox
                id="pickup_chk"
                checked={form.values.isAuthorizedPickup}
                onCheckedChange={(val: boolean | "indeterminate") => form.setField("isAuthorizedPickup", val === true)}
              />
              <Label for="pickup_chk" class="text-xs cursor-pointer">مخول بالاستلام</Label>
            </div>

            <div class="flex items-center gap-2">
              <Checkbox
                id="payer_chk"
                checked={form.values.isPayer}
                onCheckedChange={(val: boolean | "indeterminate") => form.setField("isPayer", val === true)}
              />
              <Label for="payer_chk" class="text-xs cursor-pointer">{m.common_form_guardian_role_payer()}</Label>
            </div>
          </div>

          {#if form.values.isPayer}
            <div class="flex items-center justify-start gap-2 pt-2 border-t border-border/40">
              <Label class="text-xs text-muted-foreground">{m.common_form_guardian_payment_share()}</Label>
              <div class="flex items-center gap-1">
                <Input
                  type="number"
                  min="0"
                  max="100"
                  class="h-8 w-16 text-center font-bold px-1 bg-background"
                  bind:value={form.values.paymentSharePercent}
                />
                <span class="text-xs text-muted-foreground">%</span>
              </div>
            </div>
          {/if}
        </AccordionContent>
      </AccordionItem>
    </Accordion>
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
      disabled={form.isSubmitting || !form.values.guardianUserId || !form.values.relationship}
    >
      {form.isSubmitting ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
