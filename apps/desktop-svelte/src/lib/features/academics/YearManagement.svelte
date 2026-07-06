<!--
@component YearManagement
Settings page for managing academic years: create, archive, view stats.
Follows the settings page layout pattern.
-->
<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import { createAcademicYearsQuery, createAcademicYearMutation, createArchiveYearMutation } from "$lib/features/academics/year-queries";
import {
  Card, CardContent, CardDescription, CardHeader, CardTitle, CardFooter,
} from "@orbis/ui-svelte/card";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Textarea } from "@orbis/ui-svelte/textarea";
import { Badge } from "@orbis/ui-svelte/badge";
import { Separator } from "@orbis/ui-svelte/separator";
import {
  Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  AlertDialog, AlertDialogContent, AlertDialogHeader, AlertDialogFooter,
  AlertDialogTitle, AlertDialogDescription, AlertDialogAction, AlertDialogCancel,
} from "@orbis/ui-svelte/alert-dialog";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  CalendarDays, Plus, Archive, Users, GraduationCap, Building2, Database,
} from "lucide-svelte";
import type { CreateAcademicYearRequest, AcademicYearInfo } from "@orbis/desktop-client/types";

let { orgId }: { orgId: string } = $props();

// Data
const yearsQuery = createAcademicYearsQuery(orgId);
const createMutation = createAcademicYearMutation(orgId);
const archiveMutation = createArchiveYearMutation();

// Create year dialog state
let showCreate = $state(false);
let createForm = $state<CreateAcademicYearRequest>({
  name: "",
  name_ar: null,
  start_date: "",
  end_date: "",
  notes: null,
});

// Archive dialog state
let archiveTarget = $state<AcademicYearInfo | null>(null);

async function handleCreate() {
  try {
    await $createMutation.mutateAsync(createForm);
    toast.success(m.year_create());
    showCreate = false;
    createForm = { name: "", name_ar: null, start_date: "", end_date: "", notes: null };
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleArchive() {
  if (!archiveTarget) return;
  try {
    await $archiveMutation.mutateAsync({
      yearId: archiveTarget.year.id,
      userId: "", // Will be set from auth context
    });
    toast.success(m.year_archive());
    archiveTarget = null;
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

function statusVariant(status: string) {
  switch (status) {
    case "active": return "success" as const;
    case "archived": return "secondary" as const;
    case "promoting": return "brand" as const;
    default: return "outline" as const;
  }
}

function statusLabel(status: string) {
  switch (status) {
    case "active": return m.year_status_active();
    case "archived": return m.year_status_archived();
    case "promoting": return m.year_status_promoting();
    default: return status;
  }
}
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">

    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-foreground">{m.year_title()}</h1>
        <p class="text-sm text-muted-foreground mt-1">{m.year_create_desc()}</p>
      </div>
      <Button variant="lime" size="sm" onclick={() => showCreate = true}>
        <Plus class="size-4 me-2" />
        {m.year_create()}
      </Button>
    </div>

    <Separator />

    <!-- Year Cards -->
    {#if $yearsQuery.isLoading}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {#each Array(4) as _}
          <Card>
            <CardHeader>
              <Skeleton class="h-5 w-32" />
              <Skeleton class="h-4 w-20 mt-1" />
            </CardHeader>
            <CardContent>
              <div class="grid grid-cols-3 gap-4">
                <Skeleton class="h-12 w-full" />
                <Skeleton class="h-12 w-full" />
                <Skeleton class="h-12 w-full" />
              </div>
            </CardContent>
          </Card>
        {/each}
      </div>
    {:else if ($yearsQuery.data ?? []).length === 0}
      <Card class="border-dashed">
        <CardContent class="flex flex-col items-center justify-center py-12 text-center">
          <Database class="size-12 text-muted-foreground/40 mb-4" />
          <h3 class="text-lg font-semibold text-foreground">{m.year_no_years()}</h3>
          <p class="text-sm text-muted-foreground mt-1">{m.year_no_years_desc()}</p>
          <Button variant="lime" class="mt-4" onclick={() => showCreate = true}>
            <Plus class="size-4 me-2" />
            {m.year_create()}
          </Button>
        </CardContent>
      </Card>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {#each $yearsQuery.data ?? [] as yearInfo}
          <Card class="relative {yearInfo.is_active_in_app ? 'ring-1 ring-primary/30' : ''}">
            <CardHeader class="pb-3">
              <div class="flex items-center justify-between">
                <CardTitle class="text-base">{yearInfo.year.name}</CardTitle>
                <Badge variant={statusVariant(yearInfo.year.status)}>
                  {statusLabel(yearInfo.year.status)}
                </Badge>
              </div>
              {#if yearInfo.year.name_ar}
                <CardDescription>{yearInfo.year.name_ar}</CardDescription>
              {/if}
            </CardHeader>

            <CardContent class="pb-3">
              <!-- Date range -->
              <div class="flex items-center gap-2 text-xs text-muted-foreground mb-3">
                <CalendarDays class="size-3.5" />
                <span>{yearInfo.year.start_date} → {yearInfo.year.end_date}</span>
              </div>

              <!-- Stats grid -->
              <div class="grid grid-cols-3 gap-3">
                <div class="flex flex-col items-center p-2 rounded-lg bg-muted/50">
                  <Users class="size-4 text-muted-foreground mb-1" />
                  <span class="text-lg font-semibold text-foreground">
                    {yearInfo.year.student_count ?? 0}
                  </span>
                  <span class="text-[10px] text-muted-foreground">{m.year_students()}</span>
                </div>
                <div class="flex flex-col items-center p-2 rounded-lg bg-muted/50">
                  <GraduationCap class="size-4 text-muted-foreground mb-1" />
                  <span class="text-lg font-semibold text-foreground">
                    {yearInfo.year.teacher_count ?? 0}
                  </span>
                  <span class="text-[10px] text-muted-foreground">{m.year_teachers()}</span>
                </div>
                <div class="flex flex-col items-center p-2 rounded-lg bg-muted/50">
                  <Building2 class="size-4 text-muted-foreground mb-1" />
                  <span class="text-lg font-semibold text-foreground">
                    {yearInfo.year.class_count ?? 0}
                  </span>
                  <span class="text-[10px] text-muted-foreground">{m.year_classes()}</span>
                </div>
              </div>
            </CardContent>

            <CardFooter class="pt-0">
              <div class="flex items-center gap-2 flex-nowrap shrink-0 w-full">
                {#if yearInfo.year.status === "active" && !yearInfo.is_active_in_app}
                  <Button variant="outline" size="xs" class="flex-1"
                    onclick={() => commands.academicYears.setActive(orgId, yearInfo.year.id)}>
                    {m.year_set_active()}
                  </Button>
                {/if}
                {#if yearInfo.year.status === "active" && !yearInfo.is_active_in_app}
                  <Button variant="ghost" size="xs"
                    onclick={() => archiveTarget = yearInfo}>
                    <Archive class="size-3.5 me-1" />
                    {m.year_archive()}
                  </Button>
                {/if}
              </div>
            </CardFooter>
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Create Year Dialog -->
<Dialog bind:open={showCreate}>
  <DialogContent class="sm:max-w-lg">
    <DialogHeader>
      <DialogTitle>{m.year_create()}</DialogTitle>
      <DialogDescription>{m.year_create_desc()}</DialogDescription>
    </DialogHeader>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 py-4">
      <div class="space-y-2">
        <Label>{m.year_name()}</Label>
        <Input bind:value={createForm.name} placeholder={m.year_name_placeholder()} />
      </div>
      <div class="space-y-2">
        <Label>{m.year_name_ar()}</Label>
        <Input bind:value={createForm.name_ar} dir="rtl" />
      </div>
      <div class="space-y-2">
        <Label>{m.year_start_date()}</Label>
        <Input type="date" bind:value={createForm.start_date} />
      </div>
      <div class="space-y-2">
        <Label>{m.year_end_date()}</Label>
        <Input type="date" bind:value={createForm.end_date} />
      </div>
      <div class="space-y-2 md:col-span-2">
        <Label>{m.year_notes()}</Label>
        <Textarea bind:value={createForm.notes} rows={2} />
      </div>
    </div>

    <DialogFooter>
      <Button variant="ghost" onclick={() => showCreate = false}>{m.form_cancel()}</Button>
      <Button variant="lime" onclick={handleCreate}
        disabled={!createForm.name || !createForm.start_date || !createForm.end_date}>
        {m.year_create()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<!-- Archive Confirmation -->
<AlertDialog open={!!archiveTarget} onOpenChange={(v) => { if (!v) archiveTarget = null; }}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.year_archive()}</AlertDialogTitle>
      <AlertDialogDescription>{m.year_archive_confirm()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel onclick={() => archiveTarget = null}>{m.form_cancel()}</AlertDialogCancel>
      <AlertDialogAction onclick={handleArchive}>{m.year_archive()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
