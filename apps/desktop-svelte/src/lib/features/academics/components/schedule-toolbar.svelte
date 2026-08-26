<script lang="ts">
  import {
    Calendar as CalendarIcon,
    Search,
    WandSparkles,
    Printer,
    Clock,
    Plus,
    Users,
    User,
    MapPin,
    BookOpen,
    Layers,
    AlertTriangle,
    Maximize2,
    Minimize2,
  } from "@lucide/svelte";
  import { Button } from "@orbis/ui-svelte/button";
  import { Input } from "@orbis/ui-svelte/input";
  import { Badge } from "@orbis/ui-svelte/badge";
  import {
    Select,
    SelectTrigger,
    SelectContent,
    SelectItem,
  } from "@orbis/ui-svelte/select";
  import { getLocale } from "$lib/paraglide/runtime";

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

  const isArabic = $derived(getLocale() === "ar");

  const selectedPeriod = $derived(
    periods.find((p) => String(p.id) === String(activePeriodId))
  );

  function handlePeriodSelect(val: string | null) {
    if (!val) return;
    activePeriodId = val;
    onPeriodChange?.(val);
  }

  const viewModes: { id: ViewMode; labelAr: string; labelEn: string; icon: any }[] = [
    { id: "class", labelAr: "الفصول", labelEn: "Classes", icon: Users },
    { id: "teacher", labelAr: "المعلمون", labelEn: "Teachers", icon: User },
    { id: "room", labelAr: "القاعات", labelEn: "Rooms", icon: MapPin },
    { id: "subject", labelAr: "المواد", labelEn: "Subjects", icon: BookOpen },
    { id: "matrix", labelAr: "المخطط العام", labelEn: "Matrix", icon: Layers },
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
                ? (isArabic ? selectedPeriod.name_ar || selectedPeriod.name : selectedPeriod.name || selectedPeriod.name_ar)
                : (isArabic ? "اختر الفترة..." : "Select Period...")}
            </span>
          </div>
        </SelectTrigger>
        <SelectContent class="max-h-60 text-start">
          {#each periods as p (p.id)}
            <SelectItem value={String(p.id)} class="text-xs">
              {isArabic ? p.name_ar || p.name : p.name || p.name_ar}
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
            <span class="hidden md:inline">{isArabic ? mode.labelAr : mode.labelEn}</span>
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
          placeholder={isArabic ? "بحث سريع بالمعلم أو المادة..." : "Quick filter..."}
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
        title={isArabic ? "إدارة الحصص الزمنية" : "Time Slots"}
      >
        <Clock size={13} />
        <span class="hidden xl:inline">{isArabic ? "الحصص" : "Slots"}</span>
      </Button>

      
      <Button
        variant="ghost"
        size="sm"
        class="h-8 gap-1.5 px-2 text-xs text-muted-foreground hover:text-foreground"
        onclick={onExportClick}
        title={isArabic ? "طباعة وتصدير الجدول" : "Print & Export"}
      >
        <Printer size={13} />
        <span class="hidden xl:inline">{isArabic ? "تصدير" : "Export"}</span>
      </Button>
      <!-- AI Generator Studio Button -->
      <Button
        size="sm"
        class="h-8 gap-1.5 px-2.5 text-xs font-bold"
        onclick={onAutoScheduleClick}
      >
        <WandSparkles size={13} />
        <span class="hidden sm:inline">{isArabic ? "المخطط السنوي" : "Year Planner"}</span>
      </Button>
    </div>
  </div>

  <!-- Bottom Secondary Row: Contextual Resource Dropdowns & Filter Chips -->
  {#if viewMode !== "matrix"}
    <div class="flex h-10 w-full items-center justify-between px-3 gap-2 bg-muted/15 border-t border-border/40 text-xs">
      <div class="flex items-center gap-2 min-w-0 flex-1">
        <!-- Class Selector (When in Class view) -->
        {#if viewMode === "class"}
          <span class="text-muted-foreground shrink-0 font-medium">{isArabic ? "الفصل النشط:" : "Class:"}</span>
          <Select
            type="single"
            value={selectedClassId}
            onValueChange={(val) => (selectedClassId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-48 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {classes.find((c) => String(c.id) === selectedClassId)?.name_ar || classes.find((c) => String(c.id) === selectedClassId)?.name || (isArabic ? "اختر الفصل..." : "Select Class...")}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each classes as c (c.id)}
                <SelectItem value={String(c.id)} class="text-xs">
                  {c.name_ar || c.name}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

        <!-- Teacher Selector (When in Teacher view) -->
        {#if viewMode === "teacher"}
          <span class="text-muted-foreground shrink-0 font-medium">{isArabic ? "المعلم:" : "Teacher:"}</span>
          <Select
            type="single"
            value={selectedTeacherId}
            onValueChange={(val) => (selectedTeacherId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-52 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {teachers.find((t) => String(t.id) === selectedTeacherId)?.name_ar || teachers.find((t) => String(t.id) === selectedTeacherId)?.full_name || (isArabic ? "اختر المعلم..." : "Select Teacher...")}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each teachers as t (t.id)}
                <SelectItem value={String(t.id)} class="text-xs">
                  {t.name_ar || t.full_name}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

        <!-- Room Selector (When in Room view) -->
        {#if viewMode === "room"}
          <span class="text-muted-foreground shrink-0 font-medium">{isArabic ? "القاعة:" : "Room:"}</span>
          <Select
            type="single"
            value={selectedRoomId}
            onValueChange={(val) => (selectedRoomId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-48 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {rooms.find((r) => String(r.id) === selectedRoomId)?.name_ar || rooms.find((r) => String(r.id) === selectedRoomId)?.name || (isArabic ? "اختر القاعة..." : "Select Room...")}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each rooms as r (r.id)}
                <SelectItem value={String(r.id)} class="text-xs">
                  {r.name_ar || r.name} {r.building ? `(${r.building})` : ""}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

        <!-- Subject Selector (When in Subject view) -->
        {#if viewMode === "subject"}
          <span class="text-muted-foreground shrink-0 font-medium">{isArabic ? "المادة:" : "Subject:"}</span>
          <Select
            type="single"
            value={selectedSubjectId}
            onValueChange={(val) => (selectedSubjectId = val ?? "")}
          >
            <SelectTrigger class="h-7 w-48 bg-card text-xs font-semibold border-border/60">
              <span class="truncate">
                {subjects.find((s) => String(s.id) === selectedSubjectId)?.name_ar || subjects.find((s) => String(s.id) === selectedSubjectId)?.name || (isArabic ? "اختر المادة..." : "Select Subject...")}
              </span>
            </SelectTrigger>
            <SelectContent class="max-h-60 text-start">
              {#each subjects as s (s.id)}
                <SelectItem value={String(s.id)} class="text-xs">
                  <div class="flex items-center gap-1.5">
                    <span class="h-2 w-2 rounded-full" style={`background-color: ${s.color || 'oklch(0.58 0.1 285)'};`}></span>
                    <span>{s.name_ar || s.name}</span>
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
