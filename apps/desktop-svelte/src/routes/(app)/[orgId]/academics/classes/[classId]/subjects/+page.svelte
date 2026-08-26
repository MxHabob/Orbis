<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { BookOpen, RefreshCcw } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

  const classId = $derived(String(page.params.classId || ""));
  const orgId = $derived(String(page.params.orgId || ""));
  const isArabic = $derived(getLocale() === "ar");
  let subjects = $state<any[]>([]);
  let isLoading = $state(true);
  let error = $state("");

  async function loadSubjects() {
    isLoading = true;
    error = "";
    try {
      const [classes, catalog] = await Promise.all([
        commands.academic.listClasses(),
        commands.academic.listSubjects(),
      ]);
      const classData = classes.find((item: any) => String(item.id) === classId);
      const offerings = await commands.academic.listClassSubjects(
        classId,
        String(classData?.period_id || ""),
      );
      const byId = new Map(catalog.map((subject: any) => [String(subject.id), subject]));
      subjects = offerings.map((offering: any) => ({ ...offering, catalog: byId.get(String(offering.subject_id)) }));
    } catch (cause) {
      error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      isLoading = false;
    }
  }

  function subjectName(subject: any): string {
    return isArabic ? subject.catalog?.name_ar || subject.catalog?.name || subject.subject_id : subject.catalog?.name || subject.catalog?.name_ar || subject.subject_id;
  }

  onMount(loadSubjects);
</script>

<div class="flex flex-col gap-5 text-start">
  <div class="flex items-start justify-between gap-3"><div><div class="flex items-center gap-2"><BookOpen size={17} class="text-primary" /><h2 class="text-sm font-bold">{isArabic ? "مواد الفصل" : "Class subjects"}</h2></div><p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "المواد والمدرسون وأنظمة التقييم المرتبطة بالفصل" : "Subjects, teachers, and assessment setup for this class"}</p></div><button class="inline-flex h-8 w-8 items-center justify-center border border-border text-muted-foreground hover:text-foreground" onclick={loadSubjects} title={isArabic ? "تحديث" : "Refresh"} aria-label={isArabic ? "تحديث" : "Refresh"}><RefreshCcw size={13} class={isLoading ? "animate-spin" : ""} /></button></div>
  {#if error}<div class="border border-destructive/30 bg-destructive/5 p-3 text-xs text-destructive">{error}</div>
  {:else if isLoading}<div class="border border-border/60 py-16 text-center text-xs text-muted-foreground">{isArabic ? "جاري تحميل المواد..." : "Loading subjects..."}</div>
  {:else}<div class="overflow-x-auto border border-border/70"><table class="w-full min-w-160 text-xs"><thead class="border-b border-border/70 bg-muted/30 text-muted-foreground"><tr><th class="px-3 py-2 text-start">{isArabic ? "المادة" : "Subject"}</th><th class="px-3 py-2 text-start">{isArabic ? "المدرس" : "Teacher"}</th><th class="px-3 py-2 text-end">{isArabic ? "الساعات" : "Hours"}</th><th class="px-3 py-2 text-center">{isArabic ? "الحالة" : "Status"}</th><th class="px-3 py-2 text-end">{isArabic ? "فتح" : "Open"}</th></tr></thead><tbody>{#each subjects as subject (subject.id)}<tr class="border-b border-border/50 last:border-0 hover:bg-muted/20"><td class="px-3 py-2 font-medium">{subjectName(subject)}</td><td class="px-3 py-2">{subject.teacher_name || subject.primary_teacher_id || "-"}</td><td class="px-3 py-2 text-end">{subject.weekly_hours || "-"}</td><td class="px-3 py-2 text-center"><Badge variant={subject.is_active === 0 ? "outline" : "secondary"} class="text-[9px]">{subject.is_active === 0 ? (isArabic ? "غير نشطة" : "Inactive") : (isArabic ? "نشطة" : "Active")}</Badge></td><td class="px-3 py-2 text-end"><a class="text-primary hover:underline" href={`/${orgId}/academics/classes/${classId}/subjects/${subject.id}`}>{isArabic ? "عرض المادة" : "Open subject"}</a></td></tr>{/each}</tbody></table>{#if subjects.length === 0}<div class="py-12 text-center text-xs text-muted-foreground">{isArabic ? "لا توجد مواد مرتبطة بالفصل" : "No subjects assigned"}</div>{/if}</div>{/if}
</div>