<script lang="ts">
import {
  Calendar as CalendarIcon,
  Search,
  WandSparkles,
  Printer,
  Clock,
  Users,
  User,
  MapPin,
  BookOpen,
  Layers,
} from "@lucide/svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { l } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";

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
  subjects = [],
  onPeriodChange,
  onAutoScheduleClick,
  onTimeSlotsClick,
  onExportClick,
  onNewLessonClick,
}: Props = $props();

const selectedPeriod = $derived(
  periods.find((p) => String(p.id) === String(activePeriodId)),
);
function handlePeriodSelect(val: string | null) {
  if (!val) return;
  activePeriodId = val;
  onPeriodChange?.(val);
}

const viewModes: { id: ViewMode; label: () => string; icon: any }[] = [
  { id: "class", label: m.academics_studio_classes, icon: Users },
  { id: "teacher", label: m.academics_studio_teachers, icon: User },
  { id: "room", label: m.academics_studio_rooms, icon: MapPin },
  { id: "subject", label: m.academics_studio_subject, icon: BookOpen },
  { id: "matrix", label: m.academics_planning_studio_title, icon: Layers },
];
</script>

<div class="w-full shrink-0 border-b border-border/60 bg-card select-none">
  <!-- Top Primary Row: Perspectives + Period + Search + Actions -->
  <div class="flex h-12 w-full items-center justify-between px-3 gap-2">
    <!-- Left: Period Selector & View Mode Segmented Tabs -->
    <div class="flex items-center gap-2 min-w-0">
      <!-- Period Select Dropdown -->
      <Select
        type="single"
        value={activePeriodId}
        onValueChange={handlePeriodSelect}
      >
        <SelectTrigger class="h-8 w-40 bg-muted/30 text-xs font-bold border-border/60">
          <div class="flex items-center gap-1.5 truncate">
            <CalendarIcon size={13} class="text-primary shrink-0" />
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

      <div class="h-4 w-px bg-border/60 mx-1 hidden sm:block"></div>

      <!-- Segmented View Tabs -->
      <div class="flex items-center bg-muted/40 p-0.5 rounded-lg border border-border/40">
        {#each viewModes as mode}
          {@const Icon = mode.icon}
          {@const isActive = viewMode === mode.id}
          <button
            type="button"
            class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer {isActive
              ? 'bg-card text-foreground shadow-2xs font-bold'
              : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (viewMode = mode.id)}
          >
            <Icon size={12} class={isActive ? "text-primary" : "text-muted-foreground"} />
            <span class="hidden md:inline">{mode.label()}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- Center: Global Schedule Search -->
    <div class="flex-1 max-w-xs min-w-0 hidden lg:block">
      <div class="relative w-full">
        <Search size={13} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <Input
          bind:value={searchTerm}
          placeholder={m.academics_schedule_quick_filter()}
          class="h-8 ps-8 pe-3 text-xs bg-muted/20 border-border/40"
        />
      </div>
    </div>

    <!-- Right: Solver, Time Slots, Export, New Lesson -->
    <div class="flex items-center gap-1.5 shrink-0">
      
      <!-- Time Slots Configuration -->
      <Button
        variant="ghost"
        size="sm"
        class="h-8 gap-1.5 px-2 text-xs text-muted-foreground hover:text-foreground"
        onclick={onTimeSlotsClick}
        title={m.academics_schedule_time_slots_title()}
      >
        <Clock size={13} />
        <span class="hidden xl:inline">{m.academics_schedule_time_slots_label()}</span>
      </Button>

      
      <Button
        variant="ghost"
        size="sm"
        class="h-8 gap-1.5 px-2 text-xs text-muted-foreground hover:text-foreground"
        onclick={onExportClick}
        title={m.academics_schedule_export_title()}
      >
        <Printer size={13} />
        <span class="hidden xl:inline">{m.academics_schedule_export_label()}</span>
      </Button>
      <!-- AI Generator Studio Button -->
      <Button
        size="sm"
        class="h-8 gap-1.5 px-2.5 text-xs font-bold"
        onclick={onAutoScheduleClick}
      >
        <WandSparkles size={13} />
        <span class="hidden sm:inline">{m.academics_schedule_year_planner()}</span>
      </Button>
    </div>
  </div>

  <!-- Bottom Secondary Row: Contextual Resource Dropdowns & Filter Chips -->
  {#if viewMode !== "matrix"}
    <div class="flex h-10 w-full items-center justify-between px-3 gap-2 bg-muted/15 border-t border-border/40 text-xs">
      <div class="flex items-center gap-2 min-w-0 flex-1">
        <!-- Class Selector (When in Class view) -->
        {#if viewMode === "class"}
          <span class="text-muted-foreground shrink-0 font-medium">{m.academics_schedule_active_class()}</span>
          <Select
            type="single"
            value={selectedClassId}
            onValueChange={(val) => (selectedClassId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-48 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {classes.find((c) => String(c.id) === selectedClassId) ? l(classes.find((c) => String(c.id) === selectedClassId).name_ar, classes.find((c) => String(c.id) === selectedClassId).name) : m.academics_studio_select_class()}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each classes as c (c.id)}
                <SelectItem value={String(c.id)} class="text-xs">
                  {l(c.name_ar, c.name)}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

        <!-- Teacher Selector (When in Teacher view) -->
        {#if viewMode === "teacher"}
          <span class="text-muted-foreground shrink-0 font-medium">{m.academics_schedule_active_teacher()}</span>
          <Select
            type="single"
            value={selectedTeacherId}
            onValueChange={(val) => (selectedTeacherId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-52 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {teachers.find((t) => String(t.id) === selectedTeacherId) ? l(teachers.find((t) => String(t.id) === selectedTeacherId).name_ar, teachers.find((t) => String(t.id) === selectedTeacherId).full_name) : m.academics_studio_select_teacher()}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each teachers as t (t.id)}
                <SelectItem value={String(t.id)} class="text-xs">
                  {l(t.name_ar, t.full_name)}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

        <!-- Room Selector (When in Room view) -->
        {#if viewMode === "room"}
          <span class="text-muted-foreground shrink-0 font-medium">{m.academics_schedule_active_room()}</span>
          <Select
            type="single"
            value={selectedRoomId}
            onValueChange={(val) => (selectedRoomId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-48 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {rooms.find((r) => String(r.id) === selectedRoomId) ? l(rooms.find((r) => String(r.id) === selectedRoomId).name_ar, rooms.find((r) => String(r.id) === selectedRoomId).name) : m.academics_studio_select_room()}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each rooms as r (r.id)}
                <SelectItem value={String(r.id)} class="text-xs">
                  {l(r.name_ar, r.name)} {r.building ? `(${r.building})` : ""}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

        <!-- Subject Selector (When in Subject view) -->
        {#if viewMode === "subject"}
          <span class="text-muted-foreground shrink-0 font-medium">{m.academics_schedule_active_subject()}</span>
          <Select
            type="single"
            value={selectedSubjectId}
            onValueChange={(val) => (selectedSubjectId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-48 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {subjects.find((s) => String(s.id) === selectedSubjectId) ? l(subjects.find((s) => String(s.id) === selectedSubjectId).name_ar, subjects.find((s) => String(s.id) === selectedSubjectId).name) : m.academics_schedule_select_subject()}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each subjects as s (s.id)}
                <SelectItem value={String(s.id)} class="text-xs">
                  <div class="flex items-center gap-1.5">
                    <span class="h-2 w-2 rounded-full" style={`background-color: ${s.color || 'oklch(0.58 0.1 285)'};`}></span>
                    <span>{l(s.name_ar, s.name)}</span>
                  </div>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}
      </div>
    </div>
  {/if}
</div>
