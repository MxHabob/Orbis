<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Users, RefreshCcw } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

  const classId = $derived(String(page.params.classId || ""));
  const orgId = $derived(String(page.params.orgId || ""));
  const isArabic = $derived(getLocale() === "ar");
  let students = $state<any[]>([]);
  let isLoading = $state(true);
  let error = $state("");

  async function loadStudents() {
    isLoading = true;
    error = "";
    try {
      const enrollments = await commands.academic.listEnrollments();
      students = enrollments.filter((student: any) => String(student.class_id) === classId);
    } catch (cause) {
      error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      isLoading = false;
    }
  }

  function studentName(student: any): string {
    return student.full_name || student.name_ar || student.name || student.student_id || "-";
  }

  onMount(loadStudents);
</script>

<div class="flex flex-col gap-5 text-start">
  <div class="flex items-start justify-between gap-3">
    <div>
      <div class="flex items-center gap-2"><Users size={17} class="text-primary" /><h2 class="text-sm font-bold">{isArabic ? "طلاب الفصل" : "Class students"}</h2></div>
      <p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "إدارة التسجيلات المرتبطة بهذا الفصل" : "Manage enrollments belonging to this class"}</p>
    </div>
    <button class="inline-flex h-8 w-8 items-center justify-center border border-border text-muted-foreground hover:text-foreground" onclick={loadStudents} title={isArabic ? "تحديث" : "Refresh"} aria-label={isArabic ? "تحديث" : "Refresh"}><RefreshCcw size={13} class={isLoading ? "animate-spin" : ""} /></button>
  </div>

  {#if error}<div class="border border-destructive/30 bg-destructive/5 p-3 text-xs text-destructive">{error}</div>
  {:else if isLoading}<div class="border border-border/60 py-16 text-center text-xs text-muted-foreground">{isArabic ? "جاري تحميل الطلاب..." : "Loading students..."}</div>
  {:else}
    <div class="overflow-x-auto border border-border/70">
      <table class="w-full min-w-160 text-xs"><thead class="border-b border-border/70 bg-muted/30 text-muted-foreground"><tr><th class="px-3 py-2 text-start">#</th><th class="px-3 py-2 text-start">{isArabic ? "الطالب" : "Student"}</th><th class="px-3 py-2 text-start">{isArabic ? "الرقم" : "Number"}</th><th class="px-3 py-2 text-center">{isArabic ? "الحالة" : "Status"}</th></tr></thead>
        <tbody>{#each students as student, index (student.id)}<tr class="border-b border-border/50 last:border-0 hover:bg-muted/20"><td class="px-3 py-2 text-muted-foreground">{index + 1}</td><td class="px-3 py-2 font-medium">{studentName(student)}</td><td class="px-3 py-2">{student.student_number || "-"}</td><td class="px-3 py-2 text-center"><Badge variant={student.status === "active" ? "secondary" : "outline"} class="text-[9px]">{student.status || "-"}</Badge></td></tr>{/each}</tbody>
      </table>
      {#if students.length === 0}<div class="py-12 text-center text-xs text-muted-foreground">{isArabic ? "لا يوجد طلاب مسجلون" : "No students enrolled"}</div>{/if}
    </div>
  {/if}
</div>