<script lang="ts">
  import { page } from "$app/state";
  import { createQuery } from "@tanstack/svelte-query";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Button } from "@orbis/ui-svelte/button";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import { MetricProgress } from "@orbis/ui-svelte";
  import { Users, AlertCircle, RefreshCcw, MoreHorizontal } from "@lucide/svelte";
  import { localizeHref, getLocale } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";

  const classId = $derived(String(page.params.classId || ""));
  const orgId = $derived(String(page.params.orgId || ""));
  const isArabic = $derived(getLocale() === "ar");

  const studentsQuery = createQuery(() => ({
    queryKey: ["class-students-enriched-natural", classId],
    queryFn: () => commands.academic.listEnrichedStudents(classId),
    enabled: !!classId,
    staleTime: 5 * 60 * 1000,
  }));

  const students = $derived((studentsQuery.data || []) as any[]);
  const isLoading = $derived(studentsQuery.isLoading);
  const error = $derived(studentsQuery.error);

  function studentName(student: any): string {
    return student.full_name || student.name_ar || student.name || student.student_id || "-";
  }

  function refetch() {
    studentsQuery.refetch();
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
        <Users size={17} class="text-primary" />
        <h2 class="text-sm font-bold">{m.academics_class_detail_tab_students ? m.academics_class_detail_tab_students() : "Class Students"}</h2>
        <Badge variant="secondary" class="text-[9px] py-0 px-1.5 h-4.5 rounded font-sans ml-2">
          {students.length}
        </Badge>
      </div>
      <p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "سجل طلاب الفصل والمؤشرات الأكاديمية" : "Master student roster and aggregate performance metrics."}</p>
    </div>
    <div class="flex gap-2">
      <Button variant="outline" size="sm" class="h-8 gap-1.5 text-xs font-semibold" onclick={refetch}>
        <RefreshCcw size={12} class={isLoading ? "animate-spin" : ""} />
        {m.common_retry ? m.common_retry() : "Refresh"}
      </Button>
      <Button variant="default" size="sm" class="h-8 gap-1.5 text-xs font-semibold">
        {m.common_search_add ? m.common_search_add() : "Add Student"}
      </Button>
    </div>
  </div>

  {#if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2 font-sans">
      <AlertCircle size={24} />
      <h3 class="font-semibold text-xs">Failed to load students</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
    </div>
  {:else if isLoading}
    <div class="flex flex-col gap-3">
      {#each Array(5) as _}
        <Skeleton class="h-12 w-full rounded" />
      {/each}
    </div>
  {:else}
    <div class="overflow-x-auto border border-border/70 rounded-md bg-card shadow-sm">
      <table class="w-full min-w-[800px] text-xs">
        <thead class="border-b border-border/70 bg-muted/30 text-muted-foreground text-[10px] uppercase tracking-wider">
          <tr>
            <th class="px-4 py-3 text-start font-semibold">Student</th>
            <th class="px-4 py-3 text-start font-semibold">Status</th>
            <th class="px-4 py-3 text-start font-semibold w-48">Attendance (Mock)</th>
            <th class="px-4 py-3 text-start font-semibold w-48">Performance (Mock)</th>
            <th class="px-4 py-3 text-end font-semibold">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each students as student (student.id)}
            <tr class="border-b border-border/50 last:border-0 hover:bg-muted/20 transition-colors">
              <td class="px-4 py-3">
                <a href={localizeHref(`/${orgId}/students/${student.student_id}`)} class="flex flex-col group">
                  <span class="font-bold text-foreground group-hover:text-primary transition-colors">{studentName(student)}</span>
                  <span class="text-[9px] text-muted-foreground mt-0.5 font-mono">{student.student_number || student.student_id}</span>
                </a>
              </td>
              <td class="px-4 py-3">
                <Badge variant={student.status === "active" ? "secondary" : "outline"} class="text-[9px] font-bold uppercase">
                  {student.status || "active"}
                </Badge>
              </td>
              <td class="px-4 py-3">
                <!-- Placeholder for actual studentMetrics endpoint -->
                <div class="flex flex-col gap-1 w-full max-w-[140px]">
                  <div class="flex justify-between text-[9px] font-medium text-muted-foreground">
                    <span>Rate</span>
                    <span class="text-foreground">92%</span>
                  </div>
                  <MetricProgress value={92} variant="single" tone="success" class="h-1.5" showProgress={false} />
                </div>
              </td>
              <td class="px-4 py-3">
                <!-- Placeholder for actual studentMetrics endpoint -->
                <div class="flex flex-col gap-1 w-full max-w-[140px]">
                  <div class="flex justify-between text-[9px] font-medium text-muted-foreground">
                    <span>GPA</span>
                    <span class="text-foreground">3.4</span>
                  </div>
                  <MetricProgress value={85} variant="single" tone="primary" class="h-1.5" showProgress={false} />
                </div>
              </td>
              <td class="px-4 py-3 text-end">
                <Button variant="ghost" size="sm" class="h-8 w-8 p-0 text-muted-foreground hover:text-foreground">
                  <MoreHorizontal size={14} />
                </Button>
              </td>
            </tr>
          {/each}
          {#if students.length === 0}
            <tr>
              <td colspan="5" class="py-12 text-center text-xs text-muted-foreground">
                No students enrolled in this class.
              </td>
            </tr>
          {/if}
        </tbody>
      </table>
    </div>
  {/if}
</div>
