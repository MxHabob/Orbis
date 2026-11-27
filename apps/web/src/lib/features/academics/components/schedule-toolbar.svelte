<script lang="ts">
import {
  AdjustmentsHorizontal,
  BookOpen,
  Calendar,
  ClockRotateRight,
  Keyframes,
  Pin,
  Printer,
  Search,
  Sparkles,
  User,
  Users,
  XMark,
} from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { l } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import type { OrgUnitDto } from "$lib/api";
import { getOrgUnitLocationPath } from "$lib/features/academics/room-location";

type ViewMode = "class" | "teacher" | "room" | "subject" | "matrix";

interface Props {
  activePeriodId: string;
  periods: any[];
  viewMode: ViewMode;
  selectedClassId: string;
  selectedTeacherId: string;
  selectedRoomId: string;
  selectedSubjectId: string;
  searchTerm?: string;
  compactView?: boolean;
  classes: any[];
  teachers: any[];
  rooms: any[];
  orgUnits: OrgUnitDto[];
  subjects: any[];
  onPeriodChange?: (periodId: string) => void;
  onAutoScheduleClick?: () => void;
  onTimeSlotsClick?: () => void;
  onExportClick?: () => void;
  onNewLessonClick?: () => void;
}

let {
  activePeriodId = $bindable(""),
  periods = [],
  viewMode = $bindable("class"),
  selectedClassId = $bindable(""),
  selectedTeacherId = $bindable(""),
  selectedRoomId = $bindable(""),
  selectedSubjectId = $bindable(""),
  searchTerm = $bindable(""),
  compactView = $bindable(false),
  classes = [],
  teachers = [],
  rooms = [],
  orgUnits = [],
  subjects = [],
  onPeriodChange,
  onAutoScheduleClick,
  onTimeSlotsClick,
  onExportClick,
  onNewLessonClick,
}: Props = $props();

const orgUnitsById = $derived(
  new Map(orgUnits.map((unit) => [String(unit.id), unit])),
);

const selectedPeriod = $derived(
  periods.find((p) => String(p.id) === String(activePeriodId)),
);
const selectedClass = $derived(
  classes.find((c) => String(c.id) === selectedClassId),
);
const selectedTeacher = $derived(
  teachers.find((t) => String(t.id) === selectedTeacherId),
);
const selectedRoom = $derived(
  rooms.find((r) => String(r.id) === selectedRoomId),
);
const selectedSubject = $derived(
  subjects.find((s) => String(s.id) === selectedSubjectId),
);

function handlePeriodSelect(val: string | null) {
  if (!val) return;
  activePeriodId = val;
  onPeriodChange?.(val);
}

const viewModes: { id: ViewMode; label: () => string; icon: any }[] = [
  { id: "class", label: m.academics_studio_classes, icon: Users },
  { id: "teacher", label: m.academics_studio_teachers, icon: User },
  { id: "room", label: m.academics_studio_rooms, icon: Pin },
  { id: "subject", label: m.academics_studio_subject, icon: BookOpen },
  { id: "matrix", label: m.academics_planning_studio_title, icon: Keyframes },
];
</script>

<header
  class="@container w-full shrink-0 border-b border-border/70 bg-background px-3 py-2.5 sm:px-4 select-none"
>
  <div class="flex flex-wrap items-center justify-between gap-2.5">
    <!-- Start: View Perspectives + Contextual Entity Picker -->
    <div class="flex min-w-0 flex-wrap items-center gap-2">
      <div
        role="tablist"
        aria-label={m.academics_planning_studio_title()}
        class="flex max-w-full items-center gap-0.5 overflow-x-auto rounded-md border border-border bg-muted/30 p-0.5"
      >
        {#each viewModes as mode (mode.id)}
          {@const Icon = mode.icon}
          {@const isActive = viewMode === mode.id}
          <button
            type="button"
            role="tab"
            aria-selected={isActive}
            class="flex shrink-0 cursor-pointer items-center gap-1.5 rounded-sm px-2.5 py-1 text-xs whitespace-nowrap transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isActive
              ? 'bg-card font-semibold text-foreground shadow-2xs'
              : 'font-medium text-muted-foreground hover:text-foreground'}"
            onclick={() => (viewMode = mode.id)}
          >
            <Icon
              size={13}
              class={isActive ? "text-primary" : "text-muted-foreground/80"}
            />
            <span>{mode.label()}</span>
          </button>
        {/each}
      </div>

      {#if viewMode === "class"}
        <Select
          type="single"
          value={selectedClassId}
          onValueChange={(val) => (selectedClassId = val ?? "")}
        >
          <SelectTrigger
            class="h-8 w-44 border-border bg-card text-xs font-medium @sm:w-52"
          >
            <div class="flex items-center gap-1.5 truncate">
              <Users size={13} class="shrink-0 text-muted-foreground" />
              <span class="truncate">
                {selectedClass
                  ? l(selectedClass.name_ar, selectedClass.name)
                  : m.academics_studio_select_class()}
              </span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-60 text-start">
            {#each classes as c (c.id)}
              <SelectItem value={String(c.id)} class="text-xs">
                {l(c.name_ar, c.name)}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {:else if viewMode === "teacher"}
        <Select
          type="single"
          value={selectedTeacherId}
          onValueChange={(val) => (selectedTeacherId = val ?? "")}
        >
          <SelectTrigger
            class="h-8 w-44 border-border bg-card text-xs font-medium @sm:w-52"
          >
            <div class="flex items-center gap-1.5 truncate">
              <User size={13} class="shrink-0 text-muted-foreground" />
              <span class="truncate">
                {selectedTeacher
                  ? l(selectedTeacher.name_ar, selectedTeacher.full_name)
                  : m.academics_studio_select_teacher()}
              </span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-60 text-start">
            {#each teachers as t (t.id)}
              <SelectItem value={String(t.id)} class="text-xs">
                {l(t.name_ar, t.full_name)}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {:else if viewMode === "room"}
        <Select
          type="single"
          value={selectedRoomId}
          onValueChange={(val) => (selectedRoomId = val ?? "")}
        >
          <SelectTrigger
            class="h-8 w-44 border-border bg-card text-xs font-medium @sm:w-52"
          >
            <div class="flex items-center gap-1.5 truncate">
              <Pin size={13} class="shrink-0 text-muted-foreground" />
              <span class="truncate">
                {selectedRoom
                  ? l(selectedRoom.name_ar, selectedRoom.name)
                  : m.academics_studio_select_room()}
              </span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-60 text-start">
            {#each rooms as r (r.id)}
              <SelectItem value={String(r.id)} class="text-xs">
                {l(r.name_ar, r.name)}
                {#if r.org_unit_id}
                  ({getOrgUnitLocationPath(String(r.org_unit_id), orgUnitsById) || "—"})
                {/if}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {:else if viewMode === "subject"}
        <Select
          type="single"
          value={selectedSubjectId}
          onValueChange={(val) => (selectedSubjectId = val ?? "")}
        >
          <SelectTrigger
            class="h-8 w-44 border-border bg-card text-xs font-medium @sm:w-52"
          >
            <div class="flex items-center gap-1.5 truncate">
              {#if selectedSubject}
                <span
                  class="h-2 w-2 shrink-0 rounded-full"
                  style={`background-color: ${selectedSubject.color || "var(--primary)"};`}
                ></span>
              {:else}
                <BookOpen size={13} class="shrink-0 text-muted-foreground" />
              {/if}
              <span class="truncate">
                {selectedSubject
                  ? l(selectedSubject.name_ar, selectedSubject.name)
                  : m.academics_schedule_select_subject()}
              </span>
            </div>
          </SelectTrigger>
          <SelectContent class="max-h-60 text-start">
            {#each subjects as s (s.id)}
              <SelectItem value={String(s.id)} class="text-xs">
                <div class="flex items-center gap-1.5">
                  <span
                    class="h-2 w-2 shrink-0 rounded-full"
                    style={`background-color: ${s.color || "var(--primary)"};`}
                  ></span>
                  <span>{l(s.name_ar, s.name)}</span>
                </div>
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {/if}
    </div>

    <!-- End: Period, Quick Search, Utilities & Primary Action -->
    <div class="flex min-w-0 flex-1 flex-wrap items-center justify-end gap-1.5">
      <div class="relative min-w-36 flex-1 @md:max-w-52 @md:flex-initial">
        <Search
          size={13}
          class="pointer-events-none absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground"
        />
        <Input
          bind:value={searchTerm}
          placeholder={m.academics_schedule_quick_filter()}
          class="h-8 border-border bg-card ps-8 pe-7 text-xs"
        />
        {#if searchTerm}
          <button
            type="button"
            class="absolute inset-e-1.5 top-1/2 -translate-y-1/2 rounded-sm p-0.5 text-muted-foreground hover:text-foreground"
            onclick={() => (searchTerm = "")}
            aria-label="Clear"
          >
            <XMark size={12} />
          </button>
        {/if}
      </div>

      <Select
        type="single"
        value={activePeriodId}
        onValueChange={handlePeriodSelect}
      >
        <SelectTrigger
          class="h-8 w-auto min-w-32 max-w-48 border-border bg-card text-xs font-medium"
        >
          <div class="flex items-center gap-1.5 truncate">
            <Calendar size={13} class="shrink-0 text-primary" />
            <span class="truncate">
              {selectedPeriod
                ? l(selectedPeriod.name_ar, selectedPeriod.name)
                : m.academics_studio_select_period()}
            </span>
          </div>
        </SelectTrigger>
        <SelectContent class="max-h-60 text-start">
          {#each periods as p (p.id)}
            <SelectItem value={String(p.id)} class="text-xs">
              {l(p.name_ar, p.name)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>

      <div class="flex items-center gap-1">
        {#if viewMode !== "matrix"}
          <Button
            variant={compactView ? "secondary" : "ghost"}
            size="icon"
            class="h-8 w-8 text-muted-foreground hover:text-foreground"
            aria-pressed={compactView}
            onclick={() => (compactView = !compactView)}
            title={compactView ? "Comfortable" : "Compact"}
          >
            <AdjustmentsHorizontal size={14} />
          </Button>
        {/if}

        <Button
          variant="ghost"
          size="icon"
          class="h-8 w-8 text-muted-foreground hover:text-foreground"
          onclick={onTimeSlotsClick}
          aria-label={m.academics_schedule_time_slots_title()}
          title={m.academics_schedule_time_slots_title()}
        >
          <ClockRotateRight size={14} />
        </Button>

        <Button
          variant="ghost"
          size="icon"
          class="h-8 w-8 text-muted-foreground hover:text-foreground"
          onclick={onExportClick}
          aria-label={m.academics_schedule_export_title()}
          title={m.academics_schedule_export_title()}
        >
          <Printer size={14} />
        </Button>

        <Button
          size="sm"
          class="h-8 gap-1.5 px-3 text-xs font-semibold"
          onclick={onAutoScheduleClick}
        >
          <Sparkles size={13} />
          <span class="hidden @sm:inline">
            {m.academics_schedule_year_planner()}
          </span>
        </Button>
      </div>
    </div>
  </div>
</header>
