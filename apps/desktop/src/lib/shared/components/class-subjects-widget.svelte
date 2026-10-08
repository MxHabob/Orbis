<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { createQuery } from "@tanstack/svelte-query";
import { api } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { ArrowLeft, ArrowRight, BookOpen, ChevronRight, ClockRotateRight, Plus, User } from "@orbis/icons";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import WorkspaceCard from "./workspace-card.svelte";
import { page } from "$app/state";

let { classId, periodId }: { classId: string; periodId: string } = $props();

const orgId = $derived(String(page.params.orgId || ""));
const isArabic = $derived(getLocale() === "ar");

const subjectsQuery = createQuery(() => ({
  queryKey: ["class-subjects-widget", classId, periodId],
  queryFn: async () => {
    const offerings = await api.listClassSubjects(classId, periodId);
    const catalog = await api.listSubjects();
    const byId = new Map(catalog.map((s: any) => [String(s.id), s]));
    return offerings.map((o: any) => ({
      ...o,
      catalog: byId.get(String(o.subject_id)),
    }));
  },
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const subjects = $derived((subjectsQuery.data || []) as any[]);
const isLoading = $derived(subjectsQuery.isLoading);

function subjectName(subject: any): string {
  return isArabic
    ? subject.catalog?.name_ar || subject.catalog?.name || subject.subject_id
    : subject.catalog?.name || subject.catalog?.name_ar || subject.subject_id;
}
</script>

<WorkspaceCard title={m.ui_class_subjects_class_subjects()}>
  {#snippet actions()}
    <div class="flex items-center gap-2">
      <Badge variant="secondary" class="text-[10px] py-0 px-1.5 h-5 font-bold font-sans">
        {subjects.length}
      </Badge>
      <Button
        variant="ghost"
        size="xs"
        class="text-primary hover:text-primary hover:bg-primary/10 h-6 text-[11px] font-bold gap-1 px-2"
        onclick={() =>
          goto(localizeHref(`/${orgId}/academics/classes/${classId}/curriculum`))}
      >
        <Plus size={12} />
        {m.ui_class_subjects_link_subject()}
      </Button>
    </div>
  {/snippet}

  {#if isLoading}
    <div class="flex flex-col gap-2.5">
      {#each Array(4) as _, index (index)}
        <Skeleton class="h-14 w-full rounded-xl" />
      {/each}
    </div>
  {:else if subjects.length === 0}
    <div class="flex flex-col items-center justify-center h-full py-8 text-center border border-dashed border-border/60 rounded-xl bg-muted/10">
      <BookOpen size={24} class="text-muted-foreground opacity-40 mb-2" />
      <p class="text-xs font-bold text-foreground">{m.ui_class_subjects_no_subjects_linked()}</p>
      <p class="text-[10px] text-muted-foreground mt-1 max-w-50">
        {m.ui_class_subjects_link_curriculum_subjects_to_enable_gradebook_and_timetables()}
      </p>
      <Button
        variant="outline"
        size="sm"
        class="mt-3 text-xs h-7 font-semibold"
        onclick={() =>
          goto(localizeHref(`/${orgId}/academics/classes/${classId}/curriculum`))}
      >
        <Plus size={12} class="me-1" />
        {m.ui_class_subjects_link_new_subject()}
      </Button>
    </div>
  {:else}
    <div class="flex flex-col gap-2 overflow-y-auto max-h-80 scrollbar-hide pe-1">
      {#each subjects as subject (subject.id)}
        <a
          href={localizeHref(`/${orgId}/academics/classes/${classId}/curriculum/${subject.id}`)}
          class="flex items-center justify-between p-3 rounded-md bg-muted/10 hover:bg-muted/40 transition-colors group"
        >
          <div class="flex items-center gap-3 min-w-0">
            <div class="flex items-center justify-center w-8 h-8 rounded-lg bg-primary/10 text-primary font-bold text-xs shrink-0 group-hover:bg-primary group-hover:text-primary-foreground transition-colors">
              <BookOpen size={15} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-bold text-xs text-foreground truncate group-hover:text-primary transition-colors">
                {subjectName(subject)}
              </span>
              <div class="flex items-center gap-2 text-[10px] text-muted-foreground mt-0.5">
                <span class="flex items-center gap-1 truncate">
                  <User size={10} class="opacity-70" />
                  {subject.teacher_name || (m.academics_studio_unassigned())}
                </span>
                {#if subject.weekly_hours}
                  <span class="opacity-40">•</span>
                  <span class="flex items-center gap-1 shrink-0">
                    <ClockRotateRight size={10} class="opacity-70" />
                    {subject.weekly_hours} {m.ui_class_subjects_h()}
                  </span>
                {/if}
              </div>
            </div>
          </div>

          <div class="shrink-0 text-muted-foreground opacity-50 group-hover:opacity-100 group-hover:text-primary transition-all">
            {#if isArabic}
              <ArrowLeft size={14} />
            {:else}
              <ArrowRight size={14} />
            {/if}
          </div>
        </a>
      {/each}
    </div>
  {/if}
</WorkspaceCard>
