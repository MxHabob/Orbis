<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { orgUnitInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { SheetHeader, SheetTitle } from "@orbis/ui-svelte/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import {
  Building2,
  School,
  GraduationCap,
  Layers,
  DoorOpen,
  Briefcase,
  ShieldCheck,
  MapPin,
  Users,
  BookOpen,
} from "@lucide/svelte";
import { onMount, untrack } from "svelte";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";

let {
  orgId,
  unit = null,
  defaultParentId = "",
  onSuccess,
  onCancel,
}: {
  orgId: string;
  unit?: any;
  defaultParentId?: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!unit);

const initUnit = untrack(() => unit);
const initParentId = untrack(() => defaultParentId);

// Form state
let unitsList = $state<any[]>([]);
let staffList = $state<any[]>([]);

const form = new FormController({
  initialValues: {
    name: initUnit?.name || "",
    name_ar: initUnit?.name_ar || "",
    code: initUnit?.code || "",
    unit_type: initUnit?.unit_type || "stage",
    parent_id: initUnit?.parent_id || initParentId || "",
    head_id: initUnit?.head_id || "",
    capacity: initUnit?.capacity ?? null,
    room_number: initUnit?.room_number || "",
    floor: initUnit?.floor ?? null,
    building: initUnit?.building || "",
    data_isolation: initUnit?.data_isolation || "shared",
  },
  schema: orgUnitInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

const unitTypeOptions = [
  { value: "branch", label: m.common_org_unit_branch(), icon: School },
  { value: "stage", label: m.common_org_unit_stage(), icon: Layers },
  { value: "section", label: m.common_org_unit_section(), icon: Users },
  { value: "college", label: m.common_org_unit_college(), icon: GraduationCap },
  { value: "department", label: m.common_org_unit_department(), icon: Briefcase },
  { value: "program", label: m.common_org_unit_program(), icon: BookOpen },
  { value: "building", label: m.common_org_unit_building(), icon: Building2 },
  { value: "facility", label: m.common_org_unit_facility(), icon: DoorOpen },
];

async function loadReferenceData() {
  if (!orgId) return;
  try {
    const [allUnits, allUsers] = await Promise.all([
      commands.organization.listOrgUnits().catch(() => []),
      commands.iam.listUsers().catch(() => []),
    ]);

    unitsList = (allUnits || []).filter(
      (u: any) => !isEdit || String(u.id) !== String(unit?.id),
    );

    staffList = allUsers || [];
  } catch (e) {
    console.error("Failed to load reference data for org unit:", e);
  }
}

onMount(() => {
  loadReferenceData();
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (values) => {
    const payload = {
      ...values,
      name: values.name.trim() || values.name_ar.trim(),
      name_ar: values.name_ar.trim() || values.name.trim(),
      parent_id: values.parent_id || undefined,
      head_id: values.head_id || undefined,
    };
    if (isEdit && unit) {
      await commands.organization.updateOrgUnit(unit.id, payload);
      toast.success(m.common_org_unit_update_success());
    } else {
      await commands.organization.createOrgUnit(payload);
      toast.success(m.common_org_unit_create_success());
    }
  });
  if (submitted) onSuccess();
  else if (form.formError) toast.error(form.formError);
  else if (Object.keys(form.errors).length > 0) toast.error(m.common_org_unit_error_name_required());
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0 text-start">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-base font-bold text-foreground flex items-center gap-2">
      <Building2 size={18} class="text-primary" />
      {isEdit ? m.common_org_unit_edit_title() : m.common_org_unit_add_title()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <Accordion type="multiple" value={["structure", "access", "location"]} class="flex flex-col gap-4">
      <AccordionItem value="structure" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_org_unit_structure()}</AccordionTrigger>
        <AccordionContent>
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="unit_name_ar" class="text-xs font-semibold">
          {m.common_org_unit_name_ar()} <span class="text-destructive">*</span>
        </Label>
        <Input
          id="unit_name_ar"
          type="text"
          bind:value={form.values.name_ar}
          placeholder={m.common_org_unit_name_ar_placeholder()}
          required
          disabled={form.isSubmitting}
          class="text-xs h-9"
        />
      </div>

      <div class="flex flex-col gap-1.5 text-start">
        <Label for="unit_name_en" class="text-xs font-semibold">
          {m.common_org_unit_name_en()}
        </Label>
        <Input
          id="unit_name_en"
          type="text"
          bind:value={form.values.name}
          placeholder={m.common_org_unit_name_en_placeholder()}
          disabled={form.isSubmitting}
          class="text-xs h-9"
        />
      </div>
    </div>

    <!-- Type & Parent -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Unit Type -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">{m.common_org_unit_type()}</Label>
        <Select type="single" value={form.values.unit_type} onValueChange={(v) => { if (v) form.values.unit_type = v; }}>
          <SelectTrigger class="h-9 text-xs w-full">
            {unitTypeOptions.find(o => o.value === form.values.unit_type)?.label || form.values.unit_type}
          </SelectTrigger>
          <SelectContent class="text-xs">
            {#each unitTypeOptions as opt}
              <SelectItem value={opt.value}>
                <div class="flex items-center gap-2">
                  <opt.icon size={13} class="text-primary" />
                  <span>{opt.label}</span>
                </div>
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Parent Unit -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">{m.common_org_unit_parent()}</Label>
        <Select type="single" value={form.values.parent_id} onValueChange={(v) => (form.values.parent_id = v ?? "")}>
          <SelectTrigger class="h-9 text-xs w-full">
            {unitsList.find(u => String(u.id) === form.values.parent_id) ? `${unitsList.find(u => String(u.id) === form.values.parent_id).name_ar || unitsList.find(u => String(u.id) === form.values.parent_id).name}` : m.common_org_unit_root()}
          </SelectTrigger>
          <SelectContent class="text-xs">
            <SelectItem value="">{m.common_org_unit_root()}</SelectItem>
            {#each unitsList as u}
              <SelectItem value={String(u.id)}>
                {u.name_ar || u.name} ({u.unit_type})
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Code & Head of Unit -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="unit_code" class="text-xs font-semibold">{m.common_org_unit_code()}</Label>
        <Input
          id="unit_code"
          type="text"
          bind:value={form.values.code}
          placeholder={m.common_org_unit_code()}
          disabled={form.isSubmitting}
          class="text-xs font-mono h-9"
        />
      </div>

      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">{m.common_org_unit_head()}</Label>
        <Select type="single" value={form.values.head_id} onValueChange={(v) => (form.values.head_id = v ?? "")}>
          <SelectTrigger class="h-9 text-xs w-full">
            {staffList.find(s => String(s.id) === headId) ? (staffList.find(s => String(s.id) === headId).name_ar || staffList.find(s => String(s.id) === headId).full_name) : m.common_org_unit_none()}
          </SelectTrigger>
          <SelectContent class="text-xs">
            <SelectItem value="">{m.common_org_unit_none()}</SelectItem>
            {#each staffList as s}
              <SelectItem value={String(s.id)}>
                {s.name_ar || s.full_name || s.username}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="access" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_org_unit_access()}</AccordionTrigger>
        <AccordionContent>

    <!-- Data Isolation -->
    <div class="flex flex-col gap-1.5 text-start border-t border-border/30 pt-3">
      <Label class="text-xs font-semibold">{m.common_org_unit_isolation()}</Label>
      <Select type="single" value={form.values.data_isolation} onValueChange={(v) => { if (v) form.values.data_isolation = v; }}>
        <SelectTrigger class="h-9 text-xs w-full">
          {dataIsolation === "isolated" ? m.common_org_unit_isolated() : dataIsolation === "read_only" ? m.common_org_unit_read_only() : m.common_org_unit_shared()}
        </SelectTrigger>
        <SelectContent class="text-xs">
          <SelectItem value="shared">{m.common_org_unit_shared()}</SelectItem>
          <SelectItem value="isolated">{m.common_org_unit_isolated()}</SelectItem>
          <SelectItem value="read_only">{m.common_org_unit_read_only()}</SelectItem>
        </SelectContent>
      </Select>
    </div>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="location" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_org_unit_location()}</AccordionTrigger>
        <AccordionContent>

    <!-- Location & Capacity (Optional / Conditional) -->
    <div class="border-t border-border/30 pt-3 flex flex-col gap-3">
      <span class="text-xs font-semibold text-muted-foreground">{m.common_org_unit_location_details()}</span>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_bld" class="text-[11px]">{m.common_org_unit_building()}</Label>
          <Input id="unit_bld" type="text" bind:value={form.values.building} placeholder={m.common_org_unit_building_placeholder()} disabled={form.isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_floor" class="text-[11px]">{m.common_rooms_label_floor()}</Label>
          <Input id="unit_floor" type="number" bind:value={form.values.floor} placeholder="1" disabled={form.isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_room" class="text-[11px]">{m.common_org_unit_room()}</Label>
          <Input id="unit_room" type="text" bind:value={form.values.room_number} placeholder={m.common_org_unit_room_placeholder()} disabled={form.isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_cap" class="text-[11px]">{m.common_org_unit_capacity()}</Label>
          <Input id="unit_cap" type="number" bind:value={form.values.capacity} placeholder={m.common_org_unit_capacity_placeholder()} disabled={form.isSubmitting} class="text-xs h-8" />
        </div>
      </div>
    </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </div>

  <!-- Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex items-center justify-end gap-2 shrink-0 bg-muted/20">
    <Button
      type="button"
      variant="ghost"
      size="sm"
      onclick={onCancel}
      disabled={form.isSubmitting}
      class="text-xs cursor-pointer"
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="default"
      size="sm"
      class="font-semibold px-6 text-xs cursor-pointer"
      disabled={form.isSubmitting}
    >
      {form.isSubmitting ? m.common_state_saving() : isEdit ? m.common_org_unit_update() : m.common_org_unit_create()}
    </Button>
  </div>
</form>
