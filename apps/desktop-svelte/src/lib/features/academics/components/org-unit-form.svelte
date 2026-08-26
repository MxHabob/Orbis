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
  { value: "branch", label: "فرع / مجمع تعليمي (Branch / Campus)", icon: School },
  { value: "stage", label: "مرحلة دراسية (Educational Stage)", icon: Layers },
  { value: "section", label: "قسم / مسار بنين أو بنات (Section / Track)", icon: Users },
  { value: "college", label: "كلية / معهد عالي (College / Faculty)", icon: GraduationCap },
  { value: "department", label: "قسم أكاديمي أو إداري (Department)", icon: Briefcase },
  { value: "program", label: "برنامج / مسار تدريبي (Program / Track)", icon: BookOpen },
  { value: "building", label: "مبنى (Building)", icon: Building2 },
  { value: "facility", label: "قاعة / مختبر / معمل (Hall / Lab / Facility)", icon: DoorOpen },
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
    toast.error("يرجى إدخال اسم الوحدة التنظيمية");
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
      toast.success("تم تحديث بيانات الوحدة التنظيمية بنجاح");
    } else {
      await commands.organization.createOrgUnit(payload);
      toast.success("تم إنشاء الوحدة التنظيمية بنجاح");
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

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0 bg-background text-start">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-base font-bold text-foreground flex items-center gap-2">
      <Building2 size={18} class="text-primary" />
      {isEdit ? "تعديل الوحدة التنظيمية" : "إضافة وحدة تنظيمية جديدة"}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <Accordion type="multiple" value={["structure", "access", "location"]} class="flex flex-col gap-4">
      <AccordionItem value="structure" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">الهيكل التنظيمي</AccordionTrigger>
        <AccordionContent>
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="unit_name_ar" class="text-xs font-semibold">
          الاسم بالعربية <span class="text-destructive">*</span>
        </Label>
        <Input
          id="unit_name_ar"
          type="text"
          bind:value={nameAr}
          placeholder="مثال: المرحلة الثانوية - بنين"
          required
          disabled={isSubmitting}
          class="text-xs h-9"
        />
      </div>

      <div class="flex flex-col gap-1.5 text-start">
        <Label for="unit_name_en" class="text-xs font-semibold">
          الاسم بالإنجليزية
        </Label>
        <Input
          id="unit_name_en"
          type="text"
          bind:value={name}
          placeholder="e.g. High School - Boys"
          disabled={isSubmitting}
          class="text-xs h-9"
        />
      </div>
    </div>

    <!-- Type & Parent -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Unit Type -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">نوع الوحدة التنظيمية</Label>
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
        <Label class="text-xs font-semibold">الوحدة الأم (المستوى الأعلى)</Label>
        <Select type="single" value={parentId} onValueChange={(v) => (parentId = v ?? "")}>
          <SelectTrigger class="h-9 text-xs w-full">
            {unitsList.find(u => String(u.id) === parentId) ? `${unitsList.find(u => String(u.id) === parentId).name_ar || unitsList.find(u => String(u.id) === parentId).name}` : "— جذر المؤسسة (Root Unit) —"}
          </SelectTrigger>
          <SelectContent class="text-xs">
            <SelectItem value="">— جذر المؤسسة (Root Unit) —</SelectItem>
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
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">نطاق البيانات والصلاحيات</AccordionTrigger>
        <AccordionContent>

    <!-- Data Isolation -->
    <div class="flex flex-col gap-1.5 text-start border-t border-border/30 pt-3">
      <Label class="text-xs font-semibold">نطاق عزل البيانات والصلاحيات (Data Isolation)</Label>
      <Select type="single" value={dataIsolation} onValueChange={(v) => { if (v) dataIsolation = v; }}>
        <SelectTrigger class="h-9 text-xs w-full">
          {dataIsolation === "isolated" ? "معزول (Isolated) - لا يرى موظفوه إلا بيانات هذا الفرع" : dataIsolation === "read_only" ? "اطلاع فقط (Read Only)" : "مشترك (Shared) - بيانات مدمجة مع الإدارة المركزية"}
        </SelectTrigger>
        <SelectContent class="text-xs">
          <SelectItem value="shared">مشترك (Shared) - بيانات مدمجة مع الإدارة المركزية</SelectItem>
          <SelectItem value="isolated">معزول (Isolated) - لا يرى موظفوه إلا بيانات هذا الفرع</SelectItem>
          <SelectItem value="read_only">اطلاع فقط (Read Only) - صلاحيات تصفح فقط</SelectItem>
        </SelectContent>
      </Select>
    </div>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="location" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">الموقع والطاقة الاستيعابية</AccordionTrigger>
        <AccordionContent>

    <!-- Location & Capacity (Optional / Conditional) -->
    <div class="border-t border-border/30 pt-3 flex flex-col gap-3">
      <span class="text-xs font-semibold text-muted-foreground">الموقع المكاني والطاقة الاستيعابية</span>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_bld" class="text-[11px]">المبنى</Label>
          <Input id="unit_bld" type="text" bind:value={building} placeholder="مثال: المبنى A" disabled={isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_floor" class="text-[11px]">الطابق</Label>
          <Input id="unit_floor" type="number" bind:value={floor} placeholder="1" disabled={isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_room" class="text-[11px]">رقم الغرفة/القاعة</Label>
          <Input id="unit_room" type="text" bind:value={roomNumber} placeholder="102" disabled={isSubmitting} class="text-xs h-8" />
        </div>
        <div class="flex flex-col gap-1 text-start">
          <Label for="unit_cap" class="text-[11px]">الطاقة الاستيعابية</Label>
          <Input id="unit_cap" type="number" bind:value={capacity} placeholder="35" disabled={isSubmitting} class="text-xs h-8" />
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
      إلغاء
    </Button>
    <Button
      type="submit"
      variant="default"
      size="sm"
      class="font-semibold px-6 text-xs cursor-pointer"
      disabled={isSubmitting}
    >
      {isSubmitting ? "جاري الحفظ..." : isEdit ? "تحديث الوحدة" : "إنشاء الوحدة"}
    </Button>
  </div>
</form>
