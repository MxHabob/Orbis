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
import { l } from "$lib/core/i18n";

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
    errors.nameAr = l("اسم الفرع مطلوب", "Branch name is required");
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
      toast.success(l("تم تحديث بيانات الفرع بنجاح", "Branch updated successfully"));
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
      toast.success(l("تم إنشاء الفرع بنجاح", "Branch created successfully"));
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
      {isEdit ? l("تعديل بيانات الفرع", "Edit Branch") : l("إضافة فرع جديد", "Add Branch")}
    </SheetTitle>
    <SheetDescription class="text-xs text-muted-foreground">
      {isEdit ? l("تعديل معلومات وتفاصيل الفرع التشغيلي", "Update branch details and operational scope") : l("إدخال بيانات الفرع الجديد وتحديد مديره ونطاق البيانات", "Enter branch details, assign manager, and configure isolation")}
    </SheetDescription>
  </SheetHeader>

  <!-- Form Body -->
  <div class="flex-1 overflow-y-auto p-6 space-y-4 scrollbar-hide">
    <!-- Branch Name Arabic -->
    <FormField id="branch_name_ar" label={l("اسم الفرع (بالعربية)", "Branch Name (Arabic)")} error={errors.nameAr} required>
      {#snippet children()}
      <Input
        id="branch_name_ar"
        type="text"
        bind:value={nameAr}
        placeholder={l("مثال: الفرع الرئيسي - الرياض", "e.g. Main Branch - Riyadh")}
        disabled={isSubmitting}
        class="h-9 text-sm"
        required
      />
      {/snippet}
    </FormField>

    <!-- Branch Name English -->
    <FormField id="branch_name_en" label={l("اسم الفرع (بالإنجليزية)", "Branch Name (English)")}>
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
      <FormField id="branch_code" label={l("كود الفرع", "Branch Code")}>
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

      <FormField id="branch_capacity" label={l("السعة الطلابية", "Capacity")}>
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
    <FormField id="branch_head" label={l("مدير الفرع / المشرف الإداري", "Branch Manager")}>
      {#snippet children()}
      <EmployeeSelect
        bind:value={headId}
        {orgId}
        placeholder={l("اختر مدير الفرع من الكادر الوظيفي...", "Select branch manager from employees...")}
        disabled={isSubmitting}
      />
      {/snippet}
    </FormField>

    <!-- Data Isolation Selector -->
    <FormField id="branch_isolation" label={l("نطاق عزل البيانات", "Data Isolation Scope")} required>
      {#snippet children()}
      <Select type="single" value={dataIsolation} onValueChange={(v) => (dataIsolation = v ?? "shared")} disabled={isSubmitting}>
        <SelectTrigger class="h-9 text-sm w-full">
          {dataIsolation === "isolated" ? l("معزول بالكامل (بيانات منفصلة للفرع)", "Strict Isolated") : dataIsolation === "read_only" ? l("قراءة فقط للمركز", "Read-Only") : l("مشترك مع باقي الفروع", "Shared")}
        </SelectTrigger>
        <SelectContent class="text-sm">
          <SelectItem value="shared">{l("مشترك مع باقي الفروع", "Shared")}</SelectItem>
          <SelectItem value="isolated">{l("معزول بالكامل (بيانات منفصلة للفرع)", "Strict Isolated")}</SelectItem>
          <SelectItem value="read_only">{l("قراءة فقط للمركز", "Read-Only")}</SelectItem>
        </SelectContent>
      </Select>
      {/snippet}
    </FormField>

    <!-- Location details Grid -->
    <div class="grid grid-cols-3 gap-3 pt-1">
      <FormField id="branch_building" label={l("المبنى", "Building")}>
        {#snippet children()}
        <Input
          id="branch_building"
          type="text"
          bind:value={building}
          placeholder={l("المبنى A", "Hall A")}
          disabled={isSubmitting}
          class="h-9 text-sm"
        />
        {/snippet}
      </FormField>

      <FormField id="branch_floor" label={l("الطابق", "Floor")}>
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

      <FormField id="branch_room" label={l("المكتب", "Office")}>
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
        {l("تفعيل الفرع للعمليات الأكاديمية والإدارية", "Activate branch for academic operations")}
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
