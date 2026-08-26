<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/core/ipc";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { ClipboardCheck, RefreshCcw } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

  const orgId = $derived(String(page.params.orgId || ""));
  const classId = $derived(String(page.params.classId || ""));
  const classSubjectId = $derived(String(page.params.classSubjectId || ""));
  const isArabic = $derived(getLocale() === "ar");
  let components = $state<any[]>([]);
  let isLoading = $state(true);
  let error = $state("");

  async function loadScheme() {
    isLoading = true;
    error = "";
    try {
      const periods = await commands.calendar.listAcademicPeriods();
      const activeYear = periods.find((period: any) => period.period_type === "year" && period.status === "active") || periods.find((period: any) => period.period_type === "year");
      const term = periods.find((period: any) => String(period.parent_id) === String(activeYear?.id) && (period.period_type === "term" || period.period_type === "semester"));
      components = await commands.assessment.listComponentsForClassSubject(classSubjectId, String(term?.id || activeYear?.id || ""));
    } catch (cause) {
      error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      isLoading = false;
    }
  }

  const totalWeight = $derived(components.reduce((sum, component) => sum + Number(component.weight || 0), 0));
  onMount(loadScheme);
</script>

<div class="flex flex-col gap-4 text-start"><div class="flex items-start justify-between gap-3"><div><div class="flex items-center gap-2"><ClipboardCheck size={17} class="text-primary" /><h3 class="text-sm font-bold">{isArabic ? "نظام تقييم المادة" : "Subject assessment scheme"}</h3></div><p class="mt-1 text-[10px] text-muted-foreground">{isArabic ? "مكونات التقييم والأوزان المستخدمة في دفتر الدرجات" : "Components and weights used by the gradebook"}</p></div><button class="inline-flex h-8 w-8 items-center justify-center border border-border text-muted-foreground hover:text-foreground" onclick={loadScheme} title={isArabic ? "تحديث" : "Refresh"} aria-label={isArabic ? "تحديث" : "Refresh"}><RefreshCcw size={13} class={isLoading ? "animate-spin" : ""} /></button></div>
  {#if error}<div class="border border-destructive/30 bg-destructive/5 p-3 text-xs text-destructive">{error}</div>{:else if isLoading}<div class="border border-border/60 py-16 text-center text-xs text-muted-foreground">{isArabic ? "جاري تحميل نظام التقييم..." : "Loading assessment scheme..."}</div>{:else}<div class="flex items-center justify-between border-y border-border/60 py-3 text-xs"><span class="text-muted-foreground">{isArabic ? "مجموع الأوزان" : "Total weight"}</span><Badge variant={totalWeight === 100 ? "secondary" : "destructive"} class="text-[10px]">{totalWeight}%</Badge></div><div class="overflow-x-auto border border-border/70"><table class="w-full min-w-160 text-xs"><thead class="border-b border-border/70 bg-muted/30 text-muted-foreground"><tr><th class="px-3 py-2 text-start">{isArabic ? "المكون" : "Component"}</th><th class="px-3 py-2 text-end">{isArabic ? "الوزن" : "Weight"}</th><th class="px-3 py-2 text-end">{isArabic ? "الحد الأعلى" : "Max score"}</th><th class="px-3 py-2 text-end">{isArabic ? "درجة النجاح" : "Pass score"}</th></tr></thead><tbody>{#each components as component (component.id)}<tr class="border-b border-border/50 last:border-0"><td class="px-3 py-2 font-medium">{component.name || component.component_type}</td><td class="px-3 py-2 text-end">{component.weight}%</td><td class="px-3 py-2 text-end">{component.max_score}</td><td class="px-3 py-2 text-end">{component.passing_score ?? "-"}</td></tr>{/each}</tbody></table>{#if components.length === 0}<div class="py-12 text-center text-xs text-muted-foreground">{isArabic ? "لم يتم إعداد مكونات التقييم بعد" : "No assessment components configured"}</div>{/if}</div>{/if}
</div>
