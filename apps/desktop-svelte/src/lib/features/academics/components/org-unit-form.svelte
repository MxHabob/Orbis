<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
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
let isSubmitting = $state(false);
let unitsList = $state<any[]>([]);
let staffList = $state<any[]>([]);

let name = $state(initUnit?.name || "");
let nameAr = $state(initUnit?.name_ar || "");
let code = $state(initUnit?.code || "");
let unitType = $state(initUnit?.unit_type || "stage");
let parentId = $state(initUnit?.parent_id || initParentId || "");
let capacity = $state<number | null>(
  initUnit?.capacity ? Number(initUnit.capacity) : null,
);
let roomNumber = $state(initUnit?.room_number || "");
let floor = $state<number | null>(
  initUnit?.floor ? Number(initUnit.floor) : null,
);
let building = $state(initUnit?.building || "");
let headId = $state(initUnit?.head_id || "");
let dataIsolation = $state(initUnit?.data_isolation || "shared");

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

  if (!name.trim() && !nameAr.trim()) {
    toast.error(m.common_org_unit_error_name_required());
    return;
  }

  isSubmitting = true;
  try {
    const payload = {
      name: name.trim() || nameAr.trim(),
      name_ar: nameAr.trim() || name.trim(),
      code: code.trim() || undefined,
      unit_type: unitType,
      parent_id: parentId || undefined,
      head_id: headId || undefined,
      capacity: capacity ? Number(capacity) : undefined,
      room_number: roomNumber.trim() || undefined,
      floor: floor !== null ? Number(floor) : undefined,
      building: building.trim() || undefined,
      data_isolation: dataIsolation,
    };

    if (isEdit && unit) {
      await commands.organization.updateOrgUnit(unit.id, payload);
      toast.success(m.common_org_unit_update_success());
    } else {
      await commands.organization.createOrgUnit(payload);
      toast.success(m.common_org_unit_create_success());
    }

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  } finally {
    isSubmitting = false;
  }
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
          bind:value={nameAr}
          placeholder={m.common_org_unit_name_ar_placeholder()}
          required
          disabled={isSubmitting}
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
          bind:value={name}
          placeholder={m.common_org_unit_name_en_placeholder()}
          disabled={isSubmitting}
          class="text-xs h-9"
        />
      </div>
    </div>

    <!-- Type & Parent -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Unit Type -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">{m.common_org_unit_type()}</Label>
        <Select type="single" value={unitType} onValueChange={(v) => { if (v) unitType = v; }}>
          <SelectTrigger class="h-9 text-xs w-full">
            {unitTypeOptions.find(o => o.value === unitType)?.label || unitType}
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
        <Select type="single" value={parentId} onValueChange={(v) => (parentId = v ?? "")}>
          <SelectTrigger class="h-9 text-xs w-full">
            {unitsList.find(u => String(u.id) === parentId) ? `${unitsList.find(u => String(u.id) === parentId).name_ar || unitsList.find(u => String(u.id) === parentId).name}` : m.common_org_unit_root()}
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
        <Label for="unit_code" class="text-xs font-semibold">الرمز / الكود التعريفي</Label>
        <Input
          id="unit_code"
          type="text"
          bind:value={code}
          placeholder="مثال: HIGH-BOYS أو SEC-01"
          disabled={isSubmitting}
          class="text-xs font-mono h-9"
        />
      </div>

      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">مدير / رئيس الوحدة</Label>
        <Select type="single" value={headId} onValueChange={(v) => (headId = v ?? "")}>
          <SelectTrigger class="h-9 text-xs w-full">
            {staffList.find(s => String(s.id) === headId) ? (staffList.find(s => String(s.id) === headId).name_ar || staffList.find(s => String(s.id) === headId).full_name) : "— بدون تحديد —"}
          </SelectTrigger>
          <SelectContent class="text-xs">
            <SelectItem value="">— بدون تحديد —</SelectItem>
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
      <Select type="single" value={dataIsolation} onValueChange={(v) => { if (v) dataIsolation = v; }}>
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
          <Input id="unit_bld" type="text" bind:value={building} placeholder={m.common_org_unit_building_placeholder()} disabled={isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_floor" class="text-[11px]">{m.common_rooms_label_floor()}</Label>
          <Input id="unit_floor" type="number" bind:value={floor} placeholder="1" disabled={isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_room" class="text-[11px]">{m.common_org_unit_room()}</Label>
          <Input id="unit_room" type="text" bind:value={roomNumber} placeholder={m.common_org_unit_room_placeholder()} disabled={isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_cap" class="text-[11px]">{m.common_org_unit_capacity()}</Label>
          <Input id="unit_cap" type="number" bind:value={capacity} placeholder={m.common_org_unit_capacity_placeholder()} disabled={isSubmitting} class="text-xs h-8" />
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
      disabled={isSubmitting}
      class="text-xs cursor-pointer"
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="default"
      size="sm"
      class="font-semibold px-6 text-xs cursor-pointer"
      disabled={isSubmitting}
    >
      {isSubmitting ? m.common_state_saving() : isEdit ? m.common_org_unit_update() : m.common_org_unit_create()}
    </Button>
  </div>
</form>
