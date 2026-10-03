<!--
@component BranchForm
Form for creating or editing an organizational branch / campus.
Follows the standard Orbis Settings Sheet design system and shared UI components.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, type OrgUnitDto, extractErrorMessage } from "$lib/api";
import { FormField } from "@orbis/ui";
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
import {
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui/sheet";
import { EmployeeSelect } from "$lib/features/employees";
import { toast } from "@orbis/ui";


let {
  orgId,
  branch = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  branch?: OrgUnitDto | null;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!branch);

// Form Fields
let nameAr = $state(branch?.name_ar || "");
let nameEn = $state(branch?.name || "");
let code = $state(branch?.code || "");
let headId = $state<string | null>(branch?.head_id || null);
let dataIsolation = $state<string>(branch?.data_isolation || "shared");
let capacity = $state<number | null>(branch?.capacity ?? null);
let building = $state(branch?.building || "");
let floor = $state<number | null>(branch?.floor ?? null);
let roomNumber = $state(branch?.room_number || "");
let isActive = $state(branch ? branch.is_active : true);

let isSubmitting = $state(false);
let errors = $state<Record<string, string>>({});

function validate(): boolean {
  errors = {};
  if (!nameAr.trim() && !nameEn.trim()) {
    errors.nameAr = m.ui_copy_branch_name_is_required_6a82e12f();
  }
  return Object.keys(errors).length === 0;
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!validate()) return;

  isSubmitting = true;
  try {
    const finalNameAr = nameAr.trim() || nameEn.trim();
    const finalNameEn = nameEn.trim() || nameAr.trim();

    if (isEdit && branch) {
      await api.updateOrgUnit(branch.id, {
        name: finalNameEn,
        name_ar: finalNameAr,
        code: code.trim() || undefined,
        unit_type: "branch",
        head_id: headId.trim() || undefined,
        capacity: capacity ?? undefined,
        building: building.trim() || undefined,
        floor: floor ?? undefined,
        room_number: roomNumber.trim() || undefined,
        data_isolation: dataIsolation,
        is_active: isActive,
      });
      toast.success(m.ui_copy_branch_updated_successfully_b1250bdb());
    } else {
      await api.createOrgUnit({
        name: finalNameEn,
        name_ar: finalNameAr,
        code: code.trim() || undefined,
        unit_type: "branch",
        head_id: headId.trim() || undefined,
        capacity: capacity ?? undefined,
        building: building.trim() || undefined,
        floor: floor ?? undefined,
        room_number: roomNumber.trim() || undefined,
        data_isolation: dataIsolation,
        is_active: isActive,
      });
      toast.success(m.ui_copy_branch_created_successfully_7eab8b34());
    }
    onSuccess();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden text-start">
  <!-- Sheet Header -->
  <SheetHeader class="px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-base font-bold text-foreground">
      {isEdit ? m.ui_copy_edit_branch_8fab4955() : m.ui_copy_add_branch_bbb91ff4()}
    </SheetTitle>
    <SheetDescription class="text-xs text-muted-foreground">
      {isEdit ? m.ui_copy_update_branch_details_and_operational_scop_638ab3ef() : m.ui_copy_enter_branch_details_assign_manager_and_co_56814293()}
    </SheetDescription>
  </SheetHeader>

  <!-- Form Body -->
  <div class="flex-1 overflow-y-auto p-6 space-y-4 scrollbar-hide">
    <!-- Branch Name Arabic -->
    <FormField id="branch_name_ar" label={m.ui_copy_branch_name_arabic_008219c2()} error={errors.nameAr} required>
      {#snippet children()}
      <Input
        id="branch_name_ar"
        type="text"
        bind:value={nameAr}
        placeholder={m.ui_copy_e_g_main_branch_riyadh_7bbe0d3f()}
        disabled={isSubmitting}
        class="h-9 text-sm"
        required
      />
      {/snippet}
    </FormField>

    <!-- Branch Name English -->
    <FormField id="branch_name_en" label={m.ui_copy_branch_name_english_533d70f3()}>
      {#snippet children()}
      <Input
        id="branch_name_en"
        type="text"
        bind:value={nameEn}
        placeholder="e.g. Riyadh Main Campus"
        disabled={isSubmitting}
        class="h-9 text-sm"
      />
      {/snippet}
    </FormField>

    <!-- Code & Capacity Grid -->
    <div class="grid grid-cols-2 gap-4">
      <FormField id="branch_code" label={m.ui_copy_branch_code_4ed70f95()}>
        {#snippet children()}
        <Input
          id="branch_code"
          type="text"
          bind:value={code}
          placeholder="BR-01"
          disabled={isSubmitting}
          class="h-9 text-sm font-mono uppercase"
        />
        {/snippet}
      </FormField>

      <FormField id="branch_capacity" label={m.ui_copy_capacity_6432d6d2()}>
        {#snippet children()}
        <Input
          id="branch_capacity"
          type="number"
          min="0"
          bind:value={capacity}
          placeholder="500"
          disabled={isSubmitting}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>
    </div>

    <!-- Branch Manager Selector (Using EmployeeSelect) -->
    <FormField id="branch_head" label={m.ui_copy_branch_manager_f6ad9d5a()}>
      {#snippet children()}
      <EmployeeSelect
        bind:value={headId}
        {orgId}
        placeholder={m.ui_copy_select_branch_manager_from_employees_859e6d36()}
        disabled={isSubmitting}
      />
      {/snippet}
    </FormField>

    <!-- Data Isolation Selector -->
    <FormField id="branch_isolation" label={m.ui_copy_data_isolation_scope_a497a98b()} required>
      {#snippet children()}
      <Select type="single" value={dataIsolation} onValueChange={(v) => (dataIsolation = v ?? "shared")} disabled={isSubmitting}>
        <SelectTrigger class="h-9 text-sm w-full">
          {dataIsolation === "isolated" ? m.ui_copy_strict_isolated_05b9e70c() : dataIsolation === "read_only" ? m.ui_copy_read_only_23a7a348() : m.ui_copy_shared_9692d50d()}
        </SelectTrigger>
        <SelectContent class="text-sm">
          <SelectItem value="shared">{m.ui_copy_shared_9692d50d()}</SelectItem>
          <SelectItem value="isolated">{m.ui_copy_strict_isolated_05b9e70c()}</SelectItem>
          <SelectItem value="read_only">{m.ui_copy_read_only_23a7a348()}</SelectItem>
        </SelectContent>
      </Select>
      {/snippet}
    </FormField>

    <!-- Location details Grid -->
    <div class="grid grid-cols-3 gap-3 pt-1">
      <FormField id="branch_building" label={m.common_rooms_label_building()}>
        {#snippet children()}
        <Input
          id="branch_building"
          type="text"
          bind:value={building}
          placeholder={m.ui_copy_hall_a_394d4980()}
          disabled={isSubmitting}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>

      <FormField id="branch_floor" label={m.common_rooms_label_floor()}>
        {#snippet children()}
        <Input
          id="branch_floor"
          type="number"
          bind:value={floor}
          placeholder="1"
          disabled={isSubmitting}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>

      <FormField id="branch_room" label={m.ui_copy_office_2a64e5ec()}>
        {#snippet children()}
        <Input
          id="branch_room"
          type="text"
          bind:value={roomNumber}
          placeholder="101"
          disabled={isSubmitting}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>
    </div>

    <!-- Active status checkbox -->
    <div class="flex items-center gap-2 pt-2">
      <Checkbox
        id="branch_is_active"
        checked={isActive}
        onCheckedChange={(val) => (isActive = !!val)}
        disabled={isSubmitting}
      />
      <Label for="branch_is_active" class="text-xs font-medium cursor-pointer">
        {m.ui_copy_activate_branch_for_academic_operations_b394f284()}
      </Label>
    </div>
  </div>

  <!-- Sheet Footer Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={isSubmitting}
      class="flex-1 h-9 text-sm cursor-pointer"
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="default"
      disabled={isSubmitting}
      class="flex-1 h-9 text-sm font-semibold cursor-pointer"
    >
      {isSubmitting ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
