<!--
@component AcademicCalendarPage
Displays and manages academic calendar events for the selected cycle.
Uses TanStack Query for data, supports CRUD operations, and shows
phase progress as a badge stepper.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { academicContext, authStore } from "$lib/stores";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { createCalendarEventsQuery, createPendingCalendarEventsQuery } from "$lib/features/academics";
import { useQueryClient } from "@tanstack/svelte-query";
import {
  Card, CardContent, CardHeader, CardTitle,
} from "@orbis/ui-svelte/card";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Separator } from "@orbis/ui-svelte/separator";
import {
  Table, TableBody, TableHead, TableHeader, TableRow, TableCell,
} from "@orbis/ui-svelte/table";
import {
  Dialog, DialogContent, DialogHeader, DialogFooter,
  DialogTitle, DialogDescription,
} from "@orbis/ui-svelte/dialog";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Textarea } from "@orbis/ui-svelte/textarea";
import {
  Select, SelectTrigger, SelectContent, SelectItem,
} from "@orbis/ui-svelte/select";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  CalendarClock, Play, Plus, Trash2, Loader2,
  CheckCircle2, Clock, AlertCircle, XCircle, Pencil,
} from "lucide-svelte";
import type { CalendarEvent, CalendarExecutionMode, CalendarEventType } from "@orbis/desktop-client/types";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data?.orgId ?? "");
const cycleId = $derived(academicContext.effectiveCycleId ?? "");
const isAdmin = $derived(["admin", "super_admin", "principal"].includes(authStore.user?.role ?? ""));

// TanStack Query
const queryClient = useQueryClient();
const eventsQuery = createCalendarEventsQuery(orgId, cycleId);
const events = $derived(($eventsQuery.data ?? []) as CalendarEvent[]);

// Phase progress — derived from events
const PHASE_ORDER: CalendarEventType[] = [
  "enrollment_open", "classes_start", "exam_start",
  "grade_entry_open", "grade_entry_close", "results_announcement",
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

function resetForm() {
  newEvent = {
    title: "", title_ar: "", event_type: "custom",
    execution_mode: "manual", start_date: "", end_date: "", description: "",
  };
}

async function createEvent() {
  if (!newEvent.title || !newEvent.start_date || !newEvent.end_date) {
    toast.error("Title and dates are required");
    return;
  }
  isSubmitting = true;
  try {
    await commands.calendar.create(orgId, {
      cycle_id: cycleId,
      ...newEvent,
    });
    toast.success(m.calendar_event_created ? m.calendar_event_created() : "Event created");
    showCreateDialog = false;
    resetForm();
    queryClient.invalidateQueries({ queryKey: ["calendar-events", orgId, cycleId] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSubmitting = false;
  }
}

async function executeEvent(eventId: string) {
  executingId = eventId;
  try {
    const result = await commands.calendar.execute(eventId, authStore.userId);
    if (result.outcome === "executed") {
      toast.success(m.calendar_event_executed ? m.calendar_event_executed() : "Event executed");
    } else if (result.outcome === "blocked") {
      toast.error(result.message ?? "Event blocked — prerequisites not met");
    } else {
      toast.info(result.message ?? "Event processed");
    }
    queryClient.invalidateQueries({ queryKey: ["calendar-events", orgId, cycleId] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    executingId = null;
  }
}

async function deleteEvent(eventId: string) {
  try {
    await commands.calendar.delete(eventId);
    toast.success(m.calendar_event_deleted ? m.calendar_event_deleted() : "Event deleted");
    queryClient.invalidateQueries({ queryKey: ["calendar-events", orgId, cycleId] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function generateDefaults() {
  isSubmitting = true;
  try {
    await commands.calendar.generateDefaults(orgId, cycleId);
    toast.success(m.calendar_defaults_generated ? m.calendar_defaults_generated() : "Default events generated");
    queryClient.invalidateQueries({ queryKey: ["calendar-events", orgId, cycleId] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSubmitting = false;
  }
}

function statusIcon(event: CalendarEvent) {
  if (event.executed_at) return CheckCircle2;
  const today = new Date().toISOString().split("T")[0]!;
  if (event.start_date <= today && event.end_date >= today) return Clock;
  if (event.start_date > today) return AlertCircle;
  return XCircle;
}

function statusColor(event: CalendarEvent): string {
  if (event.executed_at) return "text-primary";
  const today = new Date().toISOString().split("T")[0]!;
  if (event.start_date <= today && event.end_date >= today) return "text-yellow-500";
  if (event.start_date > today) return "text-muted-foreground";
  return "text-destructive";
}

function modeBadgeVariant(mode: CalendarExecutionMode) {
  switch (mode) {
    case "automatic": return "lime" as const;
    case "suggested": return "secondary" as const;
    case "manual": return "outline" as const;
  }
}
</script>

<svelte:head>
  <title>{m.calendar_title ? m.calendar_title() : "Academic Calendar"}</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
    <!-- Header -->
    <div class="flex items-center justify-between flex-nowrap shrink-0">
      <div class="flex items-center gap-3">
        <div class="flex items-center justify-center size-10 rounded-xl bg-primary/10">
          <CalendarClock class="size-5 text-primary" />
        </div>
        <div>
          <h1 class="text-lg font-bold text-foreground">
            {m.calendar_title ? m.calendar_title() : "Academic Calendar"}
          </h1>
          <p class="text-xs text-muted-foreground">
            {academicContext.yearLabel} — {academicContext.effectiveCycleId ? academicContext.effectiveCycleId : ""}
          </p>
        </div>
      </div>
      {#if isAdmin && !academicContext.isReadOnly}
        <div class="flex items-center gap-2 flex-nowrap shrink-0">
          {#if events.length === 0}
            <Button variant="outline" class="text-xs" onclick={generateDefaults} disabled={isSubmitting}>
              {m.calendar_generate_defaults ? m.calendar_generate_defaults() : "Generate Defaults"}
            </Button>
          {/if}
          <Button variant="lime" class="text-xs" onclick={() => { showCreateDialog = true; }}>
            <Plus size={14} class="me-1.5" />
            {m.calendar_add_event ? m.calendar_add_event() : "Add Event"}
          </Button>
        </div>
      {/if}
    </div>

    <!-- Phase Progress Stepper -->
    {#if phaseProgress.some((p) => p.exists)}
      <Card>
        <CardContent class="py-4">
          <div class="flex items-center gap-1 justify-between overflow-x-auto">
            {#each phaseProgress as phase, i}
              {#if phase.exists}
                <div class="flex items-center gap-1 shrink-0">
                  <div class="flex flex-col items-center gap-1">
                    <div class="size-3 rounded-full {phase.executed ? 'bg-primary' : 'bg-muted-foreground/30'}"></div>
                    <span class="text-[10px] text-muted-foreground whitespace-nowrap capitalize">{phase.label}</span>
                  </div>
                  {#if i < phaseProgress.filter((p) => p.exists).length - 1}
                    <div class="h-px w-6 bg-border/60 mt-[-12px]"></div>
                  {/if}
                </div>
              {/if}
            {/each}
          </div>
        </CardContent>
      </Card>
    {/if}

    <!-- Events Table -->
    <Card>
      <CardHeader class="pb-3">
        <CardTitle class="text-sm">
          {m.calendar_events_title ? m.calendar_events_title() : "Events"}
          {#if events.length > 0}
            <Badge variant="secondary" class="ms-2 text-[10px]">{events.length}</Badge>
          {/if}
        </CardTitle>
      </CardHeader>
      <CardContent>
        {#if $eventsQuery.isLoading}
          <div class="flex flex-col gap-3">
            {#each Array(5) as _}
              <Skeleton class="h-10 w-full rounded-md" />
            {/each}
          </div>
        {:else if events.length === 0}
          <div class="flex flex-col items-center justify-center py-12 text-center gap-3">
            <CalendarClock class="size-10 text-muted-foreground/40" />
            <p class="text-sm text-muted-foreground">
              {m.calendar_empty ? m.calendar_empty() : "No calendar events for this cycle"}
            </p>
            {#if isAdmin && !academicContext.isReadOnly}
              <Button variant="outline" class="text-xs" onclick={generateDefaults}>
                {m.calendar_generate_defaults ? m.calendar_generate_defaults() : "Generate Defaults"}
              </Button>
            {/if}
          </div>
        {:else}
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead class="w-8"></TableHead>
                <TableHead>{m.calendar_col_type ? m.calendar_col_type() : "Type"}</TableHead>
                <TableHead>{m.calendar_col_title ? m.calendar_col_title() : "Title"}</TableHead>
                <TableHead>{m.calendar_col_dates ? m.calendar_col_dates() : "Dates"}</TableHead>
                <TableHead>{m.calendar_col_mode ? m.calendar_col_mode() : "Mode"}</TableHead>
                <TableHead>{m.calendar_col_status ? m.calendar_col_status() : "Status"}</TableHead>
                {#if isAdmin && !academicContext.isReadOnly}
                  <TableHead class="text-end">{m.calendar_col_actions ? m.calendar_col_actions() : "Actions"}</TableHead>
                {/if}
              </TableRow>
            </TableHeader>
            <TableBody>
              {#each events as event (event.id)}
                {@const StatusIcon = statusIcon(event)}
                <TableRow>
                  <TableCell>
                    <StatusIcon class="size-4 {statusColor(event)}" />
                  </TableCell>
                  <TableCell>
                    <Badge variant="outline" class="text-[10px] capitalize">
                      {event.event_type.replace(/_/g, " ")}
                    </Badge>
                  </TableCell>
                  <TableCell class="font-medium text-sm">
                    {getLocale() === "ar" && event.title_ar ? event.title_ar : event.title}
                  </TableCell>
                  <TableCell class="text-xs text-muted-foreground">
                    {event.start_date} → {event.end_date}
                  </TableCell>
                  <TableCell>
                    <Badge variant={modeBadgeVariant(event.execution_mode)} class="text-[10px] capitalize">
                      {event.execution_mode}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    {#if event.executed_at}
                      <span class="text-[10px] text-primary font-medium">
                        {m.calendar_status_done ? m.calendar_status_done() : "Done"}
                      </span>
                    {:else}
                      <span class="text-[10px] text-muted-foreground">
                        {m.calendar_status_pending ? m.calendar_status_pending() : "Pending"}
                      </span>
                    {/if}
                  </TableCell>
                  {#if isAdmin && !academicContext.isReadOnly}
                    <TableCell class="text-end">
                      <div class="flex items-center justify-end gap-1 flex-nowrap shrink-0">
                        {#if !event.executed_at}
                          <Button
                            variant="ghost" size="icon-xs"
                            onclick={() => executeEvent(event.id)}
                            disabled={executingId === event.id}
                            title={m.calendar_execute ? m.calendar_execute() : "Execute"}
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
                          title={m.delete ? m.delete() : "Delete"}
                          class="text-destructive hover:text-destructive"
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
        {/if}
      </CardContent>
    </Card>
  </div>
</div>

<!-- Create Event Dialog -->
<Dialog bind:open={showCreateDialog}>
  <DialogContent class="sm:max-w-md">
    <DialogHeader>
      <DialogTitle>{m.calendar_add_event ? m.calendar_add_event() : "Add Event"}</DialogTitle>
      <DialogDescription>
        {m.calendar_add_event_desc ? m.calendar_add_event_desc() : "Create a new academic calendar event."}
      </DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label>{m.calendar_field_title ? m.calendar_field_title() : "Title"}</Label>
          <Input bind:value={newEvent.title} placeholder="Event title" />
        </div>
        <div class="flex flex-col gap-2">
          <Label>{m.calendar_field_title_ar ? m.calendar_field_title_ar() : "Title (AR)"}</Label>
          <Input bind:value={newEvent.title_ar} placeholder="العنوان" dir="rtl" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label>{m.calendar_field_start ? m.calendar_field_start() : "Start Date"}</Label>
          <Input type="date" bind:value={newEvent.start_date} />
        </div>
        <div class="flex flex-col gap-2">
          <Label>{m.calendar_field_end ? m.calendar_field_end() : "End Date"}</Label>
          <Input type="date" bind:value={newEvent.end_date} />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label>{m.calendar_field_type ? m.calendar_field_type() : "Event Type"}</Label>
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
          <Label>{m.calendar_field_mode ? m.calendar_field_mode() : "Execution Mode"}</Label>
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
        <Label>{m.calendar_field_description ? m.calendar_field_description() : "Description"}</Label>
        <Textarea bind:value={newEvent.description} rows={2} />
      </div>
    </div>

    <DialogFooter class="flex-nowrap shrink-0">
      <Button variant="outline" onclick={() => { showCreateDialog = false; resetForm(); }} disabled={isSubmitting}>
        {m.cancel ? m.cancel() : "Cancel"}
      </Button>
      <Button onclick={createEvent} disabled={isSubmitting}>
        {#if isSubmitting}
          <Loader2 class="size-4 animate-spin me-2" />
        {/if}
        {m.calendar_create ? m.calendar_create() : "Create"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
