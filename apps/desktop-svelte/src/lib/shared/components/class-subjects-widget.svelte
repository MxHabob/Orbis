<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import { commands } from "$lib/core/ipc";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Button } from "@orbis/ui-svelte/button";
  import { BookOpen, Plus, User, Clock, ChevronRight, ArrowLeft, ArrowRight } from "@lucide/svelte";
  import { getLocale, localizeHref } from "$lib/paraglide/runtime";
  import WorkspaceCard from "./workspace-card.svelte";
  import { page } from "$app/state";

  let { classId, periodId }: { classId: string; periodId: string } = $props();

  const orgId = $derived(String(page.params.orgId || ""));
  const isArabic = $derived(getLocale() === "ar");

  const subjectsQuery = createQuery(() => ({
    queryKey: ["class-subjects-widget", classId, periodId],
    queryFn: async () => {
      const offerings = await commands.academic.listClassSubjects(classId, periodId);
      const catalog = await commands.academic.listSubjects();
      const byId = new Map(catalog.map((s: any) => [String(s.id), s]));
      return offerings.map((o: any) => ({ ...o, catalog: byId.get(String(o.subject_id)) }));
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

<WorkspaceCard title={isArabic ? "المواد الدراسية" : "Class Subjects"} class="min-h-[300px]">
  {#snippet actions()}
    <div class="flex items-center gap-2">
      <Badge variant="secondary" class="text-[10px] py-0 px-1.5 h-5 font-bold font-sans">
        {subjects.length}
      </Badge>
      <Button
        variant="ghost"
        size="xs"
        class="text-primary hover:text-primary hover:bg-primary/10 h-6 text-[11px] font-bold gap-1 px-2"
        onclick={() => {
          window.location.href = localizeHref(`/${orgId}/academics/classes/${classId}/curriculum`);
        }}
      >
        <Plus size={12} />
        {isArabic ? "ربط مادة" : "Link Subject"}
      </Button>
    </div>
  {/snippet}

  {#if isLoading}
    <div class="flex flex-col gap-2.5">
      {#each Array(4) as _}
        <Skeleton class="h-14 w-full rounded-xl" />
      {/each}
    </div>
  {:else if subjects.length === 0}
    <div class="flex flex-col items-center justify-center h-full py-8 text-center border border-dashed border-border/60 rounded-xl bg-muted/10">
      <BookOpen size={24} class="text-muted-foreground opacity-40 mb-2" />
      <p class="text-xs font-bold text-foreground">{isArabic ? "لم يتم ربط أي مواد" : "No Subjects Linked"}</p>
      <p class="text-[10px] text-muted-foreground mt-1 max-w-[200px]">
        {isArabic ? "قم بربط مواد المنهاج الدراسي بهذا الفصل لتمكين دفتر الدرجات والجدول." : "Link curriculum subjects to enable gradebook and timetables."}
      </p>
      <Button
        variant="outline"
        size="sm"
        class="mt-3 text-xs h-7 font-semibold"
        onclick={() => {
          window.location.href = localizeHref(`/${orgId}/academics/classes/${classId}/curriculum`);
        }}
      >
        <Plus size={12} class="me-1" />
        {isArabic ? "ربط مادة جديدة" : "Link New Subject"}
      </Button>
    </div>
  {:else}
    <div class="flex flex-col gap-2 overflow-y-auto max-h-[320px] scrollbar-hide pe-1">
      {#each subjects as subject (subject.id)}
        <a
          href={localizeHref(`/${orgId}/academics/classes/${classId}/curriculum/${subject.id}`)}
          class="flex items-center justify-between p-3 rounded-xl border border-border/50 bg-card hover:bg-muted/40 hover:border-primary/40 transition-all shadow-xs group"
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
                  {subject.teacher_name || (isArabic ? "غير معين" : "Unassigned")}
                </span>
                {#if subject.weekly_hours}
                  <span class="opacity-40">•</span>
                  <span class="flex items-center gap-1 shrink-0">
                    <Clock size={10} class="opacity-70" />
                    {subject.weekly_hours} {isArabic ? "س" : "h"}
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
