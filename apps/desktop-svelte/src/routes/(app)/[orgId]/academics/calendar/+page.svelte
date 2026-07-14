<!--
@component AcademicCalendarPage
Displays and manages academic calendar events for the selected period.
Uses TanStack Query for data, supports CRUD operations, and shows
phase progress as a badge stepper.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { commands, extractErrorMessage } from "$lib/core/ipc";
import { academicContext, authStore } from "$lib/stores";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { createCalendarEventsQuery } from "$lib/features/academics";
import { useQueryClient } from "@tanstack/svelte-query";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@orbis/ui-svelte/card";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
  TableCell,
} from "@orbis/ui-svelte/table";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui-svelte/dialog";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Textarea } from "@orbis/ui-svelte/textarea";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  CalendarClock,
  Play,
  Plus,
  Trash2,
  Loader2,
  CheckCircle2,
  Clock,
  AlertCircle,
  XCircle,
  Pencil,
} from "lucide-svelte";
import type {
  CalendarEvent
} from "@orbis/sdk-core";

type CalendarExecutionMode = "manual" | "auto" | string;
type CalendarEventType = "enrollment_open" | "enrollment_close" | "exam_period" | "holiday" | "custom" | string;
import {
  gregorianToHijri,
  hijriToGregorian,
  HIJRI_MONTHS_EN,
  HIJRI_MONTHS_AR,
} from "$lib/core/i18n/hijri";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data?.orgId ?? "");
const periodId = $derived(academicContext.effectivePeriodId ?? "");

// TanStack Query
const queryClient = useQueryClient();
const eventsQuery = createCalendarEventsQuery(() => orgId, () => periodId);
const events = $derived(($eventsQuery.data ?? []) as CalendarEvent[]);

// Phase progress — derived from events
const PHASE_ORDER: CalendarEventType[] = [
  "enrollment_open",
  "classes_start",
  "exam_start",
  "grade_entry_open",
  "grade_entry_close",
  "results_announcement",
];

const phaseProgress = $derived.by(() => {
  return PHASE_ORDER.map((phaseType) => {
    const event = events.find((e) => e.event_type === phaseType);
    return {
      type: phaseType,
      label: phaseType.replace(/_/g, " "),
      executed: !!event?.executed_at,
      exists: !!event,
    };
  });
});

// CRUD state
let showCreateDialog = $state(false);
let isSubmitting = $state(false);
let executingId = $state<string | null>(null);
let newEvent = $state({
  title: "",
  title_ar: "",
  event_type: "custom" as CalendarEventType,
  execution_mode: "manual" as CalendarExecutionMode,
  start_date: "",
  end_date: "",
  description: "",
});

let useHijri = $state(false);

let hijriStartYear = $state(1447);
let hijriStartMonth = $state(1);
let hijriStartDay = $state(1);

let hijriEndYear = $state(1447);
let hijriEndMonth = $state(1);
let hijriEndDay = $state(1);

// Sync from Gregorian to Hijri when Gregorian changes
function syncHijriFromGregorian() {
  if (useHijri) return;

  if (newEvent.start_date) {
    const parts = newEvent.start_date.split("-");
    if (parts.length === 3) {
      const y = parseInt(parts[0], 10);
      const m = parseInt(parts[1], 10);
      const d = parseInt(parts[2], 10);
      const h = gregorianToHijri(new Date(y, m - 1, d));
      if (h) {
        hijriStartYear = h.hy;
        hijriStartMonth = h.hm;
        hijriStartDay = h.hd;
      }
    }
  }

  if (newEvent.end_date) {
    const parts = newEvent.end_date.split("-");
    if (parts.length === 3) {
      const y = parseInt(parts[0], 10);
      const m = parseInt(parts[1], 10);
      const d = parseInt(parts[2], 10);
      const h = gregorianToHijri(new Date(y, m - 1, d));
      if (h) {
        hijriEndYear = h.hy;
        hijriEndMonth = h.hm;
        hijriEndDay = h.hd;
      }
    }
  }
}

// Update Gregorian from Hijri when Hijri inputs are edited (and useHijri is true)
function handleHijriStartChange() {
  if (!useHijri) return;
  const g = hijriToGregorian(hijriStartYear, hijriStartMonth, hijriStartDay);
  const yyyy = g.getFullYear();
  const mm = String(g.getMonth() + 1).padStart(2, "0");
  const dd = String(g.getDate()).padStart(2, "0");
  newEvent.start_date = `${yyyy}-${mm}-${dd}`;
}

function handleHijriEndChange() {
  if (!useHijri) return;
  const g = hijriToGregorian(hijriEndYear, hijriEndMonth, hijriEndDay);
  const yyyy = g.getFullYear();
  const mm = String(g.getMonth() + 1).padStart(2, "0");
  const dd = String(g.getDate()).padStart(2, "0");
  newEvent.end_date = `${yyyy}-${mm}-${dd}`;
}

function formatHijriDateString(y: number, m: number, d: number) {
  const months = getLocale() === "ar" ? HIJRI_MONTHS_AR : HIJRI_MONTHS_EN;
  const monthName = months[m - 1] || "";
  const suffix = getLocale() === "ar" ? "هـ" : "AH";
  return `${d} ${monthName} ${y} ${suffix}`;
}

const computedHijriStart = $derived.by(() => {
  if (!newEvent.start_date) return "";
  const parts = newEvent.start_date.split("-");
  if (parts.length === 3) {
    const y = parseInt(parts[0], 10);
    const m = parseInt(parts[1], 10);
    const d = parseInt(parts[2], 10);
    const h = gregorianToHijri(new Date(y, m - 1, d));
    if (h) {
      return formatHijriDateString(h.hy, h.hm, h.hd);
    }
  }
  return "";
});

const computedHijriEnd = $derived.by(() => {
  if (!newEvent.end_date) return "";
  const parts = newEvent.end_date.split("-");
  if (parts.length === 3) {
    const y = parseInt(parts[0], 10);
    const m = parseInt(parts[1], 10);
    const d = parseInt(parts[2], 10);
    const h = gregorianToHijri(new Date(y, m - 1, d));
    if (h) {
      return formatHijriDateString(h.hy, h.hm, h.hd);
    }
  }
  return "";
});

function getHijriRangeString(startStr: string, endStr: string) {
  if (!startStr || !endStr) return "";
  const parseDate = (s: string) => {
    const parts = s.split("-");
    if (parts.length === 3) {
      return new Date(
        parseInt(parts[0], 10),
        parseInt(parts[1], 10) - 1,
        parseInt(parts[2], 10),
      );
    }
    return null;
  };

  const start = parseDate(startStr);
  const end = parseDate(endStr);
  if (!start || !end) return "";

  const hs = gregorianToHijri(start);
  const he = gregorianToHijri(end);
  if (!hs || !he) return "";

  return `${formatHijriDateString(hs.hy, hs.hm, hs.hd)} → ${formatHijriDateString(he.hy, he.hm, he.hd)}`;
}

function resetForm() {
  newEvent = {
    title: "",
    title_ar: "",
    event_type: "custom",
    execution_mode: "manual",
    start_date: "",
    end_date: "",
    description: "",
  };
  useHijri = false;
}

async function createEvent() {
  if (!newEvent.title || !newEvent.start_date || !newEvent.end_date) {
    toast.error("Title and dates are required");
    return;
  }
  isSubmitting = true;
  try {
    await commands.platform.createCalendarEvent({
      period_id: periodId,
      ...newEvent,
    });
    toast.success(
      m.common_calendar_event_created ? m.common_calendar_event_created() : "Event created",
    );
    showCreateDialog = false;
    resetForm();
    queryClient.invalidateQueries({
      queryKey: ["calendar-events", orgId, periodId],
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSubmitting = false;
  }
}

async function executeEvent(eventId: string) {
  executingId = eventId;
  try {
    const result = await commands.platform.executeCalendarEvent(
      eventId,
      authStore.userId,
    );
    if (result.outcome === "executed") {
      toast.success(
        m.common_calendar_event_executed
          ? m.common_calendar_event_executed()
          : "Event executed",
      );
    } else if (result.outcome === "blocked") {
      toast.error(result.message ?? "Event blocked — prerequisites not met");
    } else {
      toast.info(result.message ?? "Event processed");
    }
    queryClient.invalidateQueries({
      queryKey: ["calendar-events", orgId, periodId],
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    executingId = null;
  }
}

async function deleteEvent(eventId: string) {
  try {
    await commands.platform.deleteCalendarEvent(eventId);
    toast.success(
      m.common_calendar_event_deleted ? m.common_calendar_event_deleted() : "Event deleted",
    );
    queryClient.invalidateQueries({
      queryKey: ["calendar-events", orgId, periodId],
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function generateDefaults() {
  isSubmitting = true;
  try {
    await commands.platform.generateDefaultCalendar(periodId);
    toast.success(
      m.common_calendar_defaults_generated
        ? m.common_calendar_defaults_generated()
        : "Default events generated",
    );
    queryClient.invalidateQueries({
      queryKey: ["calendar-events", orgId, periodId],
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSubmitting = false;
  }
}

function statusIcon(event: CalendarEvent) {
  if (event.executed_at) return CheckCircle2;
  const today = new Date().toISOString().split("T")[0]!;
  if ((event.start_date || '') <= today && (event.end_date || '') >= today) return Clock;
  if ((event.start_date || '') > today) return AlertCircle;
  return XCircle;
}

function statusColor(event: CalendarEvent): string {
  if (event.executed_at) return "text-primary";
  const today = new Date().toISOString().split("T")[0]!;
  if ((event.start_date || '') <= today && (event.end_date || '') >= today)
    return "text-yellow-500";
  if ((event.start_date || '') > today) return "text-muted-foreground";
  return "text-destructive";
}

function modeBadgeVariant(mode: CalendarExecutionMode) {
  switch (mode) {
    case "automatic":
      return "success" as const;
    case "suggested":
      return "secondary" as const;
    case "manual":
      return "outline" as const;
  }
}
</script>

<svelte:head>
  <title>{m.common_calendar_title ? m.common_calendar_title() : "Academic Calendar"}</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="w-full p-6 flex flex-col gap-6">

    <!-- Phase Progress Card -->
    {#if phaseProgress.some((p) => p.exists)}
      <Card class="w-full">
        <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
          <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
            {m.common_calendar_phase_progress()}
          </CardTitle>
        </CardHeader>
        <CardContent class="p-6">
          <div class="flex items-center gap-2 justify-between overflow-x-auto select-none">
            {#each phaseProgress as phase, i}
              {#if phase.exists}
                <div class="flex items-center gap-2 shrink-0">
                  <div class="flex flex-col items-center gap-1.5">
                    <div class="size-5 rounded-full flex items-center justify-center {phase.executed ? 'bg-primary text-primary-foreground' : 'border border-muted-foreground/30 bg-muted/20'}">
                      {#if phase.executed}
                        <CheckCircle2 class="size-4 text-primary-foreground" />
                      {/if}
                    </div>
                    <span class="text-[10px] font-semibold text-muted-foreground whitespace-nowrap capitalize">{phase.label}</span>
                  </div>
                  {#if i < phaseProgress.filter((p) => p.exists).length - 1}
                    <div class="h-px w-8 bg-border/60 mt-[-16px]"></div>
                  {/if}
                </div>
              {/if}
            {/each}
          </div>
        </CardContent>
      </Card>
    {/if}

    <!-- Events Table Card -->
    <Card class="w-full">
      <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border flex flex-row items-center justify-between">
        <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none flex items-center gap-2">
          <span>{m.common_calendar_events_title ? m.common_calendar_events_title() : "Events"}</span>
          {#if events.length > 0}
            <Badge variant="secondary" class="text-[10px] py-0.5 px-1.5 font-sans">{events.length}</Badge>
          {/if}
        </CardTitle>
        {#if !academicContext.isReadOnly}
          <div class="flex items-center gap-2 flex-nowrap shrink-0">
            {#if events.length === 0}
              <Button variant="outline" size="sm" onclick={generateDefaults} disabled={isSubmitting} class="h-8 border-border text-xs flex-nowrap shrink-0">
                {m.common_calendar_generate_defaults ? m.common_calendar_generate_defaults() : "Generate Defaults"}
              </Button>
            {/if}
            <Button variant="lime" size="sm" onclick={() => { showCreateDialog = true; syncHijriFromGregorian(); }} class="h-8 text-xs font-bold flex-nowrap shrink-0">
              <Plus size={14} class="me-1.5" />
              {m.common_calendar_add_event ? m.common_calendar_add_event() : "Add Event"}
            </Button>
          </div>
        {/if}
      </CardHeader>
      <CardContent class="p-0">
        {#if $eventsQuery.isLoading}
          <div class="p-6 flex flex-col gap-3">
            {#each Array(5) as _}
              <Skeleton class="h-10 w-full rounded-md" />
            {/each}
          </div>
        {:else if events.length === 0}
          <div class="flex flex-col items-center justify-center py-16 text-center gap-3">
            <CalendarClock class="size-10 text-muted-foreground/40" />
            <p class="text-sm text-muted-foreground">
              {m.common_calendar_empty ? m.common_calendar_empty() : "No calendar events for this period"}
            </p>
            {#if !academicContext.isReadOnly}
              <Button variant="outline" size="sm" class="text-xs border-border" onclick={generateDefaults}>
                {m.common_calendar_generate_defaults ? m.common_calendar_generate_defaults() : "Generate Defaults"}
              </Button>
            {/if}
          </div>
        {:else}
          <div class="overflow-x-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead class="w-12 text-start"></TableHead>
                  <TableHead class="text-start">{m.common_word_type ? m.common_word_type() : "Type"}</TableHead>
                  <TableHead class="text-start">{m.common_word_address ? m.common_word_address() : "Title"}</TableHead>
                  <TableHead class="text-start">{m.common_calendar_col_dates ? m.common_calendar_col_dates() : "Dates"}</TableHead>
                  <TableHead class="text-start">{m.common_calendar_col_mode ? m.common_calendar_col_mode() : "Mode"}</TableHead>
                  <TableHead class="text-start">{m.common_word_status ? m.common_word_status() : "Status"}</TableHead>
                  {#if !academicContext.isReadOnly}
                    <TableHead class="text-end px-6">{m.common_je_col_actions ? m.common_je_col_actions() : "Actions"}</TableHead>
                  {/if}
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each events as event (event.id)}
                  {@const StatusIcon = statusIcon(event)}
                  <TableRow>
                    <TableCell class="ps-6">
                      <StatusIcon class="size-4 {statusColor(event)}" />
                    </TableCell>
                    <TableCell class="text-start">
                      <Badge variant="outline" class="text-[10px] capitalize font-sans">
                        {event.event_type.replace(/_/g, " ")}
                      </Badge>
                    </TableCell>
                    <TableCell class="font-medium text-sm text-start font-serif">
                      {getLocale() === "ar" && event.title_ar ? event.title_ar : event.title}
                    </TableCell>
                    <TableCell class="text-xs text-muted-foreground text-start flex flex-col gap-0.5 font-sans">
                      <span class="font-mono">{event.start_date} → {event.end_date}</span>
                      {#if getHijriRangeString(event.start_date || '', event.end_date || '')}
                        <span class="text-[10px] text-primary/80 font-medium">{getHijriRangeString(event.start_date || '', event.end_date || '')}</span>
                      {/if}
                    </TableCell>
                    <TableCell class="text-start">
                      <Badge variant={modeBadgeVariant(event.execution_mode)} class="text-[10px] capitalize font-sans">
                        {event.execution_mode}
                      </Badge>
                    </TableCell>
                    <TableCell class="text-start">
                      {#if event.executed_at}
                        <span class="text-[10px] text-primary font-medium font-sans">
                          {m.common_calendar_status_done ? m.common_calendar_status_done() : "Done"}
                        </span>
                      {:else}
                        <span class="text-[10px] text-muted-foreground font-sans">
                          {m.common_calendar_status_pending ? m.common_calendar_status_pending() : "Pending"}
                        </span>
                      {/if}
                    </TableCell>
                    {#if !academicContext.isReadOnly}
                      <TableCell class="text-end pe-6">
                        <div class="flex items-center justify-end gap-1 flex-nowrap shrink-0">
                          {#if !event.executed_at}
                            <Button
                              variant="ghost" size="icon-xs"
                              onclick={() => executeEvent(event.id)}
                              disabled={executingId === event.id}
                              title={m.common_calendar_execute ? m.common_calendar_execute() : "Execute"}
                            >
                              {#if executingId === event.id}
                                <Loader2 class="size-3.5 animate-spin" />
                              {:else}
                                <Play class="size-3.5" />
                              {/if}
                            </Button>
                          {/if}
                          <Button
                            variant="ghost" size="icon-xs"
                            onclick={() => deleteEvent(event.id)}
                            title={m.common_action_delete()}
                            class="text-destructive hover:text-destructive hover:bg-destructive/5"
                          >
                            <Trash2 class="size-3.5" />
                          </Button>
                        </div>
                      </TableCell>
                    {/if}
                  </TableRow>
                {/each}
              </TableBody>
            </Table>
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>

<!-- Create Event Dialog -->
<Dialog bind:open={showCreateDialog}>
  <DialogContent class="sm:max-w-md">
    <DialogHeader>
      <DialogTitle>{m.common_calendar_add_event ? m.common_calendar_add_event() : "Add Event"}</DialogTitle>
      <DialogDescription>
        {m.common_calendar_add_event_desc ? m.common_calendar_add_event_desc() : "Create a new academic calendar event."}
      </DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label>{m.common_word_address ? m.common_word_address() : "Title"}</Label>
          <Input bind:value={newEvent.title} placeholder="Event title" />
        </div>
        <div class="flex flex-col gap-2">
          <Label>{m.common_calendar_field_title_ar ? m.common_calendar_field_title_ar() : "Title (AR)"}</Label>
          <Input bind:value={newEvent.title_ar} placeholder="العنوان" dir="rtl" />
        </div>
      </div>

      <!-- Hijri Calendar Toggle -->
      <div class="flex items-center gap-2 bg-muted/40 p-3 rounded-lg border border-border/50 text-start">
        <Checkbox id="use_hijri" checked={useHijri} onCheckedChange={(val) => { useHijri = !!val; }} />
        <div class="grid gap-0.5">
          <Label for="use_hijri" class="text-xs font-semibold cursor-pointer">
            {m.common_calendar_use_hijri ? m.common_calendar_use_hijri() : "Use Hijri Date"}
          </Label>
          <span class="text-[10px] text-muted-foreground leading-relaxed">
            {m.common_calendar_use_hijri_desc ? m.common_calendar_use_hijri_desc() : "Convert Hijri months and days to calendar dates"}
          </span>
        </div>
      </div>

      {#if !useHijri}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-start">
          <div class="flex flex-col gap-2">
            <Label>{m.common_year_start_date ? m.common_year_start_date() : "Start Date"}</Label>
            <Input type="date" bind:value={newEvent.start_date} oninput={syncHijriFromGregorian} onchange={syncHijriFromGregorian} />
            {#if computedHijriStart}
              <p class="text-[10px] text-primary font-medium">
                {m.common_calendar_hijri_equivalent ? m.common_calendar_hijri_equivalent() : "Equivalent Hijri"}: {computedHijriStart}
              </p>
            {/if}
          </div>
          <div class="flex flex-col gap-2">
            <Label>{m.common_year_end_date ? m.common_year_end_date() : "End Date"}</Label>
            <Input type="date" bind:value={newEvent.end_date} oninput={syncHijriFromGregorian} onchange={syncHijriFromGregorian} />
            {#if computedHijriEnd}
              <p class="text-[10px] text-primary font-medium">
                {m.common_calendar_hijri_equivalent ? m.common_calendar_hijri_equivalent() : "Equivalent Hijri"}: {computedHijriEnd}
              </p>
            {/if}
          </div>
        </div>
      {:else}
        <!-- Hijri Inputs -->
        <div class="space-y-4 text-start">
          <!-- Hijri Start -->
          <div class="flex flex-col gap-2">
            <Label class="text-xs font-bold text-foreground">
              {m.common_year_start_date ? m.common_year_start_date() : "Start Date"} ({m.common_calendar_use_hijri ? m.common_calendar_use_hijri() : "Hijri"})
            </Label>
            <div class="grid grid-cols-3 gap-2">
              <div class="flex flex-col gap-1">
                <span class="text-[10px] text-muted-foreground">{m.common_calendar_hijri_day ? m.common_calendar_hijri_day() : "Day"}</span>
                <Select type="single" value={String(hijriStartDay)} onValueChange={(v) => { if (v) { hijriStartDay = parseInt(v, 10); handleHijriStartChange(); } }}>
                  <SelectTrigger class="h-9 text-xs">{hijriStartDay}</SelectTrigger>
                  <SelectContent>
                    {#each Array(30) as _, i}
                      <SelectItem value={String(i + 1)}>{i + 1}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-[10px] text-muted-foreground">{m.common_calendar_hijri_month ? m.common_calendar_hijri_month() : "Month"}</span>
                <Select type="single" value={String(hijriStartMonth)} onValueChange={(v) => { if (v) { hijriStartMonth = parseInt(v, 10); handleHijriStartChange(); } }}>
                  <SelectTrigger class="h-9 text-xs">
                    {(getLocale() === "ar" ? HIJRI_MONTHS_AR : HIJRI_MONTHS_EN)[hijriStartMonth - 1]}
                  </SelectTrigger>
                  <SelectContent>
                    {#each (getLocale() === "ar" ? HIJRI_MONTHS_AR : HIJRI_MONTHS_EN) as monthName, i}
                      <SelectItem value={String(i + 1)}>{monthName}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-[10px] text-muted-foreground">{m.common_calendar_hijri_year ? m.common_calendar_hijri_year() : "Year"}</span>
                <Input type="number" bind:value={hijriStartYear} oninput={handleHijriStartChange} class="h-9 font-mono" />
              </div>
            </div>
          </div>

          <!-- Hijri End -->
          <div class="flex flex-col gap-2">
            <Label class="text-xs font-bold text-foreground">
              {m.common_year_end_date ? m.common_year_end_date() : "End Date"} ({m.common_calendar_use_hijri ? m.common_calendar_use_hijri() : "Hijri"})
            </Label>
            <div class="grid grid-cols-3 gap-2">
              <div class="flex flex-col gap-1">
                <span class="text-[10px] text-muted-foreground">{m.common_calendar_hijri_day ? m.common_calendar_hijri_day() : "Day"}</span>
                <Select type="single" value={String(hijriEndDay)} onValueChange={(v) => { if (v) { hijriEndDay = parseInt(v, 10); handleHijriEndChange(); } }}>
                  <SelectTrigger class="h-9 text-xs">{hijriEndDay}</SelectTrigger>
                  <SelectContent>
                    {#each Array(30) as _, i}
                      <SelectItem value={String(i + 1)}>{i + 1}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-[10px] text-muted-foreground">{m.common_calendar_hijri_month ? m.common_calendar_hijri_month() : "Month"}</span>
                <Select type="single" value={String(hijriEndMonth)} onValueChange={(v) => { if (v) { hijriEndMonth = parseInt(v, 10); handleHijriEndChange(); } }}>
                  <SelectTrigger class="h-9 text-xs">
                    {(getLocale() === "ar" ? HIJRI_MONTHS_AR : HIJRI_MONTHS_EN)[hijriEndMonth - 1]}
                  </SelectTrigger>
                  <SelectContent>
                    {#each (getLocale() === "ar" ? HIJRI_MONTHS_AR : HIJRI_MONTHS_EN) as monthName, i}
                      <SelectItem value={String(i + 1)}>{monthName}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-[10px] text-muted-foreground">{m.common_calendar_hijri_year ? m.common_calendar_hijri_year() : "Year"}</span>
                <Input type="number" bind:value={hijriEndYear} oninput={handleHijriEndChange} class="h-9 font-mono" />
              </div>
            </div>
          </div>

          <!-- Gregorian Equivalent Preview -->
          <div class="bg-primary/5 border border-primary/20 rounded-lg p-2.5 flex items-center justify-between text-xs font-sans">
            <span class="text-muted-foreground">{m.common_calendar_gregorian_preview()}:</span>
            <span class="font-bold text-primary font-mono">{newEvent.start_date || "—"} → {newEvent.end_date || "—"}</span>
          </div>
        </div>
      {/if}

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label>{m.common_calendar_field_type ? m.common_calendar_field_type() : "Event Type"}</Label>
          <Select type="single" value={newEvent.event_type} onValueChange={(v) => { if (v) newEvent.event_type = v as CalendarEventType; }}>
            <SelectTrigger class="text-xs">{newEvent.event_type.replace(/_/g, " ")}</SelectTrigger>
            <SelectContent>
              {#each ["enrollment_open", "enrollment_close", "classes_start", "classes_end", "exam_start", "exam_end", "grade_entry_open", "grade_entry_close", "results_announcement", "holiday", "break", "custom"] as type}
                <SelectItem value={type}>{type.replace(/_/g, " ")}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
        <div class="flex flex-col gap-2">
          <Label>{m.common_calendar_field_mode ? m.common_calendar_field_mode() : "Execution Mode"}</Label>
          <Select type="single" value={newEvent.execution_mode} onValueChange={(v) => { if (v) newEvent.execution_mode = v as CalendarExecutionMode; }}>
            <SelectTrigger class="text-xs capitalize">{newEvent.execution_mode}</SelectTrigger>
            <SelectContent>
              <SelectItem value="manual">Manual</SelectItem>
              <SelectItem value="suggested">Suggested</SelectItem>
              <SelectItem value="automatic">Automatic</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="flex flex-col gap-2">
        <Label>{m.common_calendar_field_description ? m.common_calendar_field_description() : "Description"}</Label>
        <Textarea bind:value={newEvent.description} rows={2} />
      </div>
    </div>

    <DialogFooter class="flex-nowrap shrink-0">
      <Button variant="outline" onclick={() => { showCreateDialog = false; resetForm(); }} disabled={isSubmitting}>
        {m.common_action_cancel()}
      </Button>
      <Button onclick={createEvent} disabled={isSubmitting}>
        {#if isSubmitting}
          <Loader2 class="size-4 animate-spin me-2" />
        {/if}
        {m.common_form_btn_create ? m.common_form_btn_create() : "Create"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
