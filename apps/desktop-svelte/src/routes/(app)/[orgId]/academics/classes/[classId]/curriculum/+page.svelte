<script lang="ts">
  import { page } from "$app/state";
  import { createQuery } from "@tanstack/svelte-query";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Button } from "@orbis/ui-svelte/button";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import { BookOpen, RefreshCcw, AlertCircle, ChevronRight } from "@lucide/svelte";
  import { localizeHref, getLocale } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";
  import { academicContext } from "$lib/stores";

  const classId = $derived(String(page.params.classId || ""));
  const orgId = $derived(String(page.params.orgId || ""));
  const periodId = $derived(academicContext.effectivePeriodId);
  const isArabic = $derived(getLocale() === "ar");

  const subjectsQuery = createQuery(() => ({
    queryKey: ["class-subjects-natural", classId, periodId],
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
  const error = $derived(subjectsQuery.error);

  function subjectName(subject: any): string {
    return isArabic 
      ? subject.catalog?.name_ar || subject.catalog?.name || subject.subject_id 
      : subject.catalog?.name || subject.catalog?.name_ar || subject.subject_id;
  }

  function refetch() {
    subjectsQuery.refetch();
  }
</script>

<div class="flex flex-col gap-5 text-start h-full font-sans">
  <div class="flex items-start justify-between gap-3 border-b border-border/60 pb-3">
    <div>
      <div class="flex items-center gap-2 mb-1.5">
        <a
          href={localizeHref(`/${orgId}/academics/classes/${classId}`)}
          class="flex items-center gap-1 text-[11px] font-bold text-muted-foreground hover:text-primary transition-colors"
        >
          <span>&larr;</span>
          <span>{isArabic ? "لوحة تحكم الفصل" : "Class Dashboard"}</span>
        </a>
      </div>
      <div class="flex items-center gap-2">
        <BookOpen size={17} class="text-primary" />
        <h2 class="text-sm font-bold">{isArabic ? "مواد الفصل" : "Class subjects"}</h2>
        <Badge variant="secondary" class="text-[9px] py-0 px-1.5 h-4.5 rounded font-sans ml-2">
          {subjects.length}
        </Badge>
      </div>
      <p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "المواد والمدرسون المرتبطون بهذا الفصل" : "Assigned subjects and primary teachers for this class."}</p>
    </div>
    <div class="flex gap-2">
      <Button variant="outline" size="sm" class="h-8 gap-1.5 text-xs font-semibold" onclick={refetch}>
        <RefreshCcw size={12} class={isLoading ? "animate-spin" : ""} />
        {m.common_retry ? m.common_retry() : "Refresh"}
      </Button>
      <Button variant="default" size="sm" class="h-8 gap-1.5 text-xs font-semibold">
        {m.common_search_add ? m.common_search_add() : "Assign Subject"}
      </Button>
    </div>
  </div>

  {#if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2 font-sans">
      <AlertCircle size={24} />
      <h3 class="font-semibold text-xs">Failed to load subjects</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
    </div>
  {:else if isLoading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each Array(6) as _}
        <Skeleton class="h-24 w-full rounded-xl" />
      {/each}
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each subjects as subject (subject.id)}
        <a 
          href={localizeHref(`/${orgId}/academics/classes/${classId}/curriculum/${subject.id}`)}
          class="flex flex-col p-4 rounded-xl border border-border/60 bg-card hover:bg-muted/30 transition-all shadow-sm group"
        >
          <div class="flex items-start justify-between mb-3">
            <h3 class="font-bold text-sm text-foreground group-hover:text-primary transition-colors line-clamp-1">{subjectName(subject)}</h3>
            <Badge variant={subject.is_active === 0 ? "outline" : "secondary"} class="text-[9px] uppercase font-bold shrink-0">
              {subject.is_active === 0 ? "Inactive" : "Active"}
            </Badge>
          </div>
          
          <div class="flex flex-col gap-1 mt-auto">
            <div class="flex items-center justify-between text-[10px]">
              <span class="text-muted-foreground">Teacher:</span>
              <span class="font-medium text-foreground truncate max-w-[120px]">{subject.teacher_name || subject.primary_teacher_id || "Unassigned"}</span>
            </div>
            <div class="flex items-center justify-between text-[10px]">
              <span class="text-muted-foreground">Hours/Week:</span>
              <span class="font-medium text-foreground">{subject.weekly_hours || "-"}</span>
            </div>
          </div>
        </a>
      {/each}

      {#if subjects.length === 0}
        <div class="col-span-full py-16 text-center border border-dashed border-border/60 rounded-xl bg-card/30">
          <p class="text-xs text-muted-foreground">No subjects assigned to this class for the current period.</p>
        </div>
      {/if}
    </div>
  {/if}
</div>
