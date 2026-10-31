<!--
@component BranchDetailsView
Comprehensive inner view for an organizational branch / campus.
Shows live telemetry, classes, rooms, child sub-units, staff directory, and operational details.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import {
  api,
  type OrgUnitDto,
  type ClassDto,
  type RoomDto,
  type EmployeeDto,
  type UserDto,
  extractErrorMessage,
} from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import { toast } from "@orbis/ui";
import { ArrowRight, Pencil, Refresh, Plus } from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { isAr, formatDate, formatNumber } from "$lib/core/i18n";
import OrgUnitSheet from "$lib/features/academics/components/org-unit-sheet.svelte";
import RoomCreateSheet from "$lib/features/academics/components/room-create-sheet.svelte";

let {
  orgId,
  branch,
  onBack,
  onEdit,
}: {
  orgId: string;
  branch: OrgUnitDto;
  onBack: () => void;
  onEdit: (branch: OrgUnitDto) => void;
} = $props();

// Data State
let classes = $state<ClassDto[]>([]);
let rooms = $state<RoomDto[]>([]);
let employees = $state<EmployeeDto[]>([]);
let allUnits = $state<OrgUnitDto[]>([]);
let manager = $state<UserDto | null>(null);
let isLoading = $state(true);
let isRefreshing = $state(false);

// Active Tab
let activeTab = $state<"overview" | "classes" | "rooms" | "subunits" | "staff">(
  "overview",
);

// Sheets
let isSubUnitSheetOpen = $state(false);
let editingSubUnit = $state<OrgUnitDto | null>(null);
let isRoomSheetOpen = $state(false);

async function loadBranchData() {
  isLoading = true;
  try {
    const [classesRes, roomsRes, empRes, unitsRes, usersRes] =
      await Promise.all([
        api.listSections().catch(() => []),
        api.listRooms().catch(() => []),
        api.listEmployees().catch(() => []),
        api.listOrgUnits().catch(() => []),
        api.listUsers().catch(() => []),
      ]);

    // Sub-units belonging to this branch
    allUnits = Array.isArray(unitsRes) ? unitsRes : [];

    // Find all unit IDs within this branch hierarchy
    const branchUnitIds = new Set<string>([branch.id]);
    for (const u of allUnits) {
      if (u.parent_id === branch.id || (u.path && u.path.includes(branch.id))) {
        branchUnitIds.add(u.id);
      }
    }

    // Filter classes belonging to this branch or its stages
    classes = (Array.isArray(classesRes) ? classesRes : []).filter((c) =>
      branchUnitIds.has(c.unit_id),
    );

    // Filter rooms belonging to this branch
    rooms = (Array.isArray(roomsRes) ? roomsRes : []).filter(
      (r) =>
        r.org_unit_id === branch.id ||
        (r.building && r.building === branch.building),
    );

    // Filter employees assigned to this branch
    employees = Array.isArray(empRes) ? empRes : [];

    // Find manager
    if (branch.head_id && Array.isArray(usersRes)) {
      manager = usersRes.find((u) => u.id === branch.head_id) || null;
    }
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  loadBranchData();
});

async function refresh() {
  isRefreshing = true;
  try {
    await loadBranchData();
    toast.success(m.ui_copy_branch_details_refreshed_28c083b4());
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isRefreshing = false;
  }
}

// Child sub-units
const childSubUnits = $derived(
  allUnits.filter(
    (u) =>
      u.id !== branch.id &&
      (u.parent_id === branch.id || (u.path && u.path.includes(branch.id))),
  ),
);

// Derived Telemetry
const totalStudentsCount = $derived(
  classes.reduce((sum, c) => sum + (c.current_students || 0), 0),
);

const branchSettings = $derived(() => {
  const s = branch.settings;
  if (typeof s === "object" && s !== null) return s as Record<string, any>;
  return {};
});

// Grid Definitions for Classes Tab
const classColumns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    align: "center",
    pin: "left",
  },
  {
    id: "name",
    prop: "name",
    header: m.ui_copy_class_name_d6838016(),
    name: "Class Name",
    width: 200,
    size: 200,
    readonly: true,
    pin: "left",
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "level",
    prop: "level",
    header: m.ui_copy_grade_level_75f9548e(),
    name: "Grade Level",
    width: 140,
    size: 140,
    readonly: true,
  },
  {
    id: "division",
    prop: "division",
    header: m.ui_copy_division_57edc3a3(),
    name: "Division",
    width: 100,
    size: 100,
    readonly: true,
    align: "center",
  },
  {
    id: "students",
    prop: "students",
    header: m.ui_copy_enrolled_0d835d63(),
    name: "Enrolled",
    width: 120,
    size: 120,
    readonly: true,
    align: "center",
    type: "number",
  },
  {
    id: "capacity",
    prop: "capacity",
    header: m.settings_units_column_capacity(),
    name: "Capacity",
    width: 100,
    size: 100,
    readonly: true,
    align: "center",
    type: "number",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_status(),
    name: "Status",
    width: 100,
    size: 100,
    readonly: true,
    align: "center",
    type: "badge",
  },
];

const classGridSource = $derived(
  classes.map((c, i) => ({
    ...c,
    __idx: i + 1,
    name: c.name_ar || c.name,
    level: c.level || "—",
    division: c.division || "—",
    students: c.current_students ?? 0,
    capacity: c.capacity ?? "—",
    status: c.is_active ? "active" : "inactive",
  })),
);

// Grid Definitions for Rooms Tab
const roomColumns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    align: "center",
    pin: "left",
  },
  {
    id: "name",
    prop: "name",
    header: m.ui_copy_room_name_f2cc6a9e(),
    name: "Room Name",
    width: 220,
    size: 220,
    readonly: true,
    pin: "left",
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "room_type",
    prop: "room_type",
    header: m.ui_copy_room_type_1a8fbda3(),
    name: "Room Type",
    width: 140,
    size: 140,
    readonly: true,
  },
  {
    id: "exam_committee",
    prop: "exam_committee",
    header: m.common_rooms_exam_committee(),
    name: m.common_rooms_exam_committee(),
    width: 150,
    size: 150,
    readonly: true,
    type: "badge",
    align: "center",
  },
  {
    id: "capacity",
    prop: "capacity",
    header: m.settings_units_column_capacity(),
    name: "Capacity",
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
    type: "number",
  },
  {
    id: "building",
    prop: "building",
    header: m.common_rooms_label_building(),
    name: "Building",
    width: 140,
    size: 140,
    readonly: true,
  },
  {
    id: "floor",
    prop: "floor",
    header: m.common_rooms_label_floor(),
    name: "Floor",
    width: 90,
    size: 90,
    readonly: true,
    align: "center",
  },
];

const roomGridSource = $derived(
  rooms.map((r, i) => ({
    ...r,
    __idx: i + 1,
    name: r.name || "—",
    room_type: r.room_type || "—",
    exam_committee: r.is_exam_hall
      ? m.common_rooms_exam_committee()
      : m.common_rooms_general_room(),
    capacity: r.capacity ?? "—",
    building: r.building || branch.building || "—",
    floor:
      r.floor !== null && r.floor !== undefined
        ? String(r.floor)
        : branch.floor
          ? String(branch.floor)
          : "—",
  })),
);

// Grid Definitions for Staff Tab
const staffColumns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    align: "center",
    pin: "left",
  },
  {
    id: "fullName",
    prop: "fullName",
    header: m.common_col_name(),
    name: "Name",
    width: 220,
    size: 220,
    readonly: true,
    pin: "left",
    cellClass: "font-semibold text-foreground",
  },
  {
    id: "empNumber",
    prop: "empNumber",
    header: m.ui_copy_employee_da995e19(),
    name: "Employee #",
    width: 130,
    size: 130,
    readonly: true,
    cellClass: "font-mono text-xs",
  },
  {
    id: "jobTitle",
    prop: "jobTitle",
    header: m.employees_job_title(),
    name: "Job Title",
    width: 180,
    size: 180,
    readonly: true,
  },
  {
    id: "empType",
    prop: "empType",
    header: m.common_word_type(),
    name: "Type",
    width: 130,
    size: 130,
    readonly: true,
    align: "center",
  },
  {
    id: "phone",
    prop: "phone",
    header: m.ui_copy_phone_f2f307cb(),
    name: "Phone",
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "font-mono text-xs",
  },
];

const staffGridSource = $derived(
  employees.map((e, i) => ({
    ...e,
    __idx: i + 1,
    fullName:
      [e.first_name_ar || e.first_name, e.last_name_ar || e.last_name]
        .filter(Boolean)
        .join(" ") || "—",
    empNumber: e.employee_number || "—",
    jobTitle: e.job_title || "—",
    empType: e.employment_type || "—",
    phone: e.phone || "—",
  })),
);
</script>

<div class="h-full w-full flex flex-col gap-4 overflow-y-auto p-1 scrollbar-hide text-start font-sans">
  <!-- Top Bar with Back Button & Actions -->
  <div class="flex items-center justify-between gap-3 border-b border-border/50 pb-3">
    <div class="flex items-center gap-3">
      <Button
        variant="ghost"
        size="sm"
        onclick={onBack}
        class="h-9 px-3 gap-1.5 text-xs text-muted-foreground hover:text-foreground cursor-pointer"
      >
        <ArrowRight size={14} class={isAr() ? "" : "rotate-180"} />
        <span>{m.ui_copy_back_to_branches_e9cb6af9()}</span>
      </Button>

      <div class="h-4 w-px bg-border/60"></div>

      <div class="flex items-center gap-2">
        <h2 class="text-lg font-bold text-foreground">{branch.name_ar || branch.name}</h2>
        {#if branch.code}
          <Badge variant="outline" class="font-mono text-xs uppercase px-2 py-0.5 bg-muted/40 font-semibold">
            {branch.code}
          </Badge>
        {/if}
        {#if branch.is_active}
          <Badge variant="secondary" class="text-[10px] bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20">
            {m.common_phase_active()}
          </Badge>
        {:else}
          <Badge variant="outline" class="text-[10px] text-muted-foreground">
            {m.common_status_disabled()}
          </Badge>
        {/if}
      </div>
    </div>

    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={refresh}
        disabled={isRefreshing || isLoading}
        class="h-8 px-3 text-xs gap-1.5 cursor-pointer"
      >
        {#if isRefreshing}<LoadingIndicator  size={13} />{:else}<Refresh size={13}   />{/if}
        <span>{m.common_refresh()}</span>
      </Button>

      <Button
        variant="default"
        size="sm"
        onclick={() => onEdit(branch)}
        class="h-8 px-3 text-xs font-semibold gap-1.5 cursor-pointer"
      >
        <Pencil size={13} />
        <span>{m.ui_copy_edit_branch_a1c270d0()}</span>
      </Button>
    </div>
  </div>

  <!-- Telemetry Metrics Summary (4 Cards) -->
  <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
    <!-- Classes Count -->
    <Card class="border border-border/50 bg-card/60">
      <CardContent class="p-4">
        <span class="text-xs text-muted-foreground">{m.ui_copy_classes_2d1babad()}</span>
        <div class="text-2xl font-bold tabular-nums mt-0.5 text-foreground">{classes.length}</div>
        <span class="text-[11px] text-muted-foreground mt-0.5 block">{m.ui_copy_assigned_classes_346a061f()}</span>
      </CardContent>
    </Card>

    <!-- Rooms Count -->
    <Card class="border border-border/50 bg-card/60">
      <CardContent class="p-4">
        <span class="text-xs text-muted-foreground">{m.ui_copy_rooms_halls_05e4b3ce()}</span>
        <div class="text-2xl font-bold tabular-nums mt-0.5 text-foreground">{rooms.length}</div>
        <span class="text-[11px] text-muted-foreground mt-0.5 block">{m.ui_copy_educational_venues_f4dc5ba1()}</span>
      </CardContent>
    </Card>

    <!-- Sub-Units Count -->
    <Card class="border border-border/50 bg-card/60">
      <CardContent class="p-4">
        <span class="text-xs text-muted-foreground">{m.ui_copy_sub_units_eb5fa3d8()}</span>
        <div class="text-2xl font-bold tabular-nums mt-0.5 text-foreground">{childSubUnits.length}</div>
        <span class="text-[11px] text-muted-foreground mt-0.5 block">{m.ui_copy_stages_and_divisions_50004a37()}</span>
      </CardContent>
    </Card>

    <!-- Students & Capacity -->
    <Card class="border border-border/50 bg-card/60">
      <CardContent class="p-4">
        <span class="text-xs text-muted-foreground">{m.ui_copy_enrollment_capacity_1496075d()}</span>
        <div class="text-xl font-bold tabular-nums mt-0.5 text-foreground">
          {formatNumber(totalStudentsCount)} <span class="text-xs font-normal text-muted-foreground">/ {branch.capacity ? formatNumber(branch.capacity) : "∞"}</span>
        </div>
        <span class="text-[11px] text-muted-foreground mt-0.5 block">
          {branch.capacity ? `${Math.min(100, Math.round((totalStudentsCount / branch.capacity) * 100))}% ${m.ui_copy_occupancy_4b03a14e()}` : m.ui_copy_uncapped_capacity_169a4e4e()}
        </span>
      </CardContent>
    </Card>
  </div>

  <!-- Inner Navigation Tabs -->
  <Tabs value={activeTab} onValueChange={(val: any) => (activeTab = val)} class="w-full space-y-4">
    <TabsList class="bg-muted/30 p-1 border border-border/50 rounded-xl">
      <TabsTrigger value="overview" class="text-xs font-semibold px-4 py-2">
        {m.ui_copy_overview_31deac42()}
      </TabsTrigger>
      <TabsTrigger value="classes" class="text-xs font-semibold px-4 py-2">
        {m.ui_copy_classes_2d1babad()} ({classes.length})
      </TabsTrigger>
      <TabsTrigger value="rooms" class="text-xs font-semibold px-4 py-2">
        {m.ui_copy_rooms_halls_05e4b3ce()} ({rooms.length})
      </TabsTrigger>
      <TabsTrigger value="subunits" class="text-xs font-semibold px-4 py-2">
        {m.ui_copy_sub_units_368aef5f()} ({childSubUnits.length})
      </TabsTrigger>
      <TabsTrigger value="staff" class="text-xs font-semibold px-4 py-2">
        {m.ui_copy_staff_faculty_23997a15()} ({employees.length})
      </TabsTrigger>
    </TabsList>

    <!-- Tab 1: Overview -->
    <TabsContent value="overview" class="space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Branch General Specs -->
        <Card class="border border-border/50 bg-card/60">
          <CardHeader class="pb-3">
            <CardTitle class="text-sm font-bold">{m.ui_copy_organizational_specifications_b47f1867()}</CardTitle>
          </CardHeader>
          <CardContent class="space-y-3 text-xs">
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.common_org_unit_name_ar()}:</span>
              <span class="font-semibold">{branch.name_ar || "—"}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.common_org_unit_name_en()}:</span>
              <span class="font-semibold">{branch.name || "—"}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.ui_copy_branch_code_4ed70f95()}:</span>
              <span class="font-mono font-semibold">{branch.code || "—"}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.ui_copy_branch_manager_5e90b291()}:</span>
              <span class="font-semibold text-primary">{manager?.name_ar || manager?.full_name || manager?.username || m.common_unassigned()}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.ui_copy_data_isolation_4d35efbe()}:</span>
              <Badge variant="outline" class="text-[10px]">
                {branch.data_isolation === "isolated" ? m.ui_copy_isolated_09d34bff() : branch.data_isolation === "read_only" ? m.ui_copy_read_only_05cda70d() : m.ui_copy_shared_cdf39825()}
              </Badge>
            </div>
            <div class="flex justify-between py-1.5">
              <span class="text-muted-foreground">{m.ui_copy_created_at_01f6ea5f()}:</span>
              <span class="font-mono text-muted-foreground">{branch.created_at ? formatDate(branch.created_at, "datetime") : "—"}</span>
            </div>
          </CardContent>
        </Card>

        <!-- Location & Contact Specs -->
        <Card class="border border-border/50 bg-card/60">
          <CardHeader class="pb-3">
            <CardTitle class="text-sm font-bold">{m.ui_copy_location_contact_details_8fa7945d()}</CardTitle>
          </CardHeader>
          <CardContent class="space-y-3 text-xs">
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.ui_copy_building_d3ad3660()}:</span>
              <span class="font-semibold">{branch.building || "—"}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.common_rooms_label_floor()}:</span>
              <span class="font-semibold">{branch.floor ?? "—"}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.ui_copy_office_915f1781()}:</span>
              <span class="font-semibold">{branch.room_number || "—"}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.ui_copy_city_address_b50863f7()}:</span>
              <span class="font-semibold">{branchSettings().city || branchSettings().address || "—"}</span>
            </div>
            <div class="flex justify-between py-1.5 border-b border-border/30">
              <span class="text-muted-foreground">{m.ui_copy_phone_ac913176()}:</span>
              <span class="font-mono font-semibold">{branchSettings().phone || "—"}</span>
            </div>
            <div class="flex justify-between py-1.5">
              <span class="text-muted-foreground">{m.common_word_email()}:</span>
              <span class="font-mono">{branchSettings().email || "—"}</span>
            </div>
          </CardContent>
        </Card>
      </div>
    </TabsContent>

    <!-- Tab 2: Classes Grid -->
    <TabsContent value="classes" class="space-y-4">
      <div class="h-[480px] w-full rounded-xl border border-border/50 overflow-hidden bg-card/50">
        <OrbisGrid
          source={classGridSource}
          columns={classColumns}
          enableExport={true}
        />
      </div>
    </TabsContent>

    <!-- Tab 3: Rooms Grid -->
    <TabsContent value="rooms" class="space-y-4">
      <div class="flex items-center justify-between">
        <span class="text-xs text-muted-foreground">{m.ui_copy_registered_venues_in_this_campus_cb721c61()}</span>
        <Button
          size="sm"
          variant="outline"
          onclick={() => (isRoomSheetOpen = true)}
          class="h-8 text-xs gap-1 font-semibold cursor-pointer"
        >
          <Plus size={13} />
          <span>{m.ui_copy_add_room_95e7067e()}</span>
        </Button>
      </div>

      <div class="h-[480px] w-full rounded-xl border border-border/50 overflow-hidden bg-card/50">
        <OrbisGrid
          source={roomGridSource}
          columns={roomColumns}
          enableExport={true}
        />
      </div>
    </TabsContent>

    <!-- Tab 4: Sub-units Grid -->
    <TabsContent value="subunits" class="space-y-4">
      <div class="flex items-center justify-between">
        <span class="text-xs text-muted-foreground">{m.ui_copy_stages_departments_and_programs_under_this_650b156b()}</span>
        <Button
          size="sm"
          variant="outline"
          onclick={() => {
            editingSubUnit = null;
            isSubUnitSheetOpen = true;
          }}
          class="h-8 text-xs gap-1 font-semibold cursor-pointer"
        >
          <Plus size={13} />
          <span>{m.ui_copy_add_sub_unit_9976bb53()}</span>
        </Button>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        {#if childSubUnits.length === 0}
          <div class="col-span-2 py-12 text-center text-xs text-muted-foreground border border-dashed border-border/60 rounded-xl">
            {m.ui_copy_no_sub_units_found_1d26b1fd()}
          </div>
        {:else}
          {#each childSubUnits as sub (sub.id)}
            <div class="p-3.5 rounded-xl border border-border/50 bg-card/60 flex items-center justify-between gap-3">
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <span class="text-xs font-bold text-foreground">{sub.name_ar || sub.name}</span>
                  {#if sub.code}
                    <Badge variant="outline" class="font-mono text-[10px] px-1.5 py-0 bg-muted/40">{sub.code}</Badge>
                  {/if}
                </div>
                <div class="flex items-center gap-2 text-[11px] text-muted-foreground mt-1">
                  <span class="capitalize">{sub.unit_type}</span>
                  {#if sub.capacity}
                    <span>• {sub.capacity} {m.ui_copy_capacity_980fee3d()}</span>
                  {/if}
                  {#if sub.building}
                    <span>• {sub.building}</span>
                  {/if}
                </div>
              </div>

              <Button
                variant="ghost"
                size="sm"
                onclick={() => {
                  editingSubUnit = sub;
                  isSubUnitSheetOpen = true;
                }}
                class="h-7 px-2.5 text-xs text-muted-foreground hover:text-foreground"
              >
                <Pencil size={12} class="mr-1 inline" />
                <span>{m.academics_schedule_edit()}</span>
              </Button>
            </div>
          {/each}
        {/if}
      </div>
    </TabsContent>

    <!-- Tab 5: Staff Grid -->
    <TabsContent value="staff" class="space-y-4">
      <div class="h-[480px] w-full rounded-xl border border-border/50 overflow-hidden bg-card/50">
        <OrbisGrid
          source={staffGridSource}
          columns={staffColumns}
          enableExport={true}
        />
      </div>
    </TabsContent>
  </Tabs>
</div>

<!-- Sub-unit Sheet -->
<OrgUnitSheet
  bind:open={isSubUnitSheetOpen}
  {orgId}
  unit={editingSubUnit}
  defaultParentId={branch.id}
  onSuccess={() => {
    loadBranchData();
  }}
/>

<!-- Room Create Sheet -->
<RoomCreateSheet
  bind:open={isRoomSheetOpen}
  {orgId}
  onSuccess={() => {
    loadBranchData();
  }}
/>
