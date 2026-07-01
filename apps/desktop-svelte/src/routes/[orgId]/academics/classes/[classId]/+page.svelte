<!--
@component ClassOverviewPage
Overview page for a single class. Shows summary stats (students, subjects,
teachers) and quick-access cards for sub-sections.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Button } from "@orbis/ui-svelte/button";
import {
  Users,
  BookOpen,
  BookUser,
  Award,
  ClipboardList,
  ArrowRight,
} from "lucide-svelte";
import { fade } from "svelte/transition";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

const orgId = $derived(page.params.orgId);
const classId = $derived(page.params.classId);

let studentsCount = $state(0);
let subjectsCount = $state(0);
let teachersCount = $state(0);
let assessmentsCount = $state(0);
let isLoading = $state(true);

async function loadStats() {
  isLoading = true;
  try {
    const [enrollments, classSubjects, assessments] = await Promise.all([
      commands.enrollments.getAll(orgId),
      commands.schedule.getClassSubjects(),
      commands.assessments.getByClass({ classId }).catch(() => []),
    ]);

    studentsCount = enrollments.filter(
      (s) => String(s.class_id) === classId,
    ).length;

    const classAssignments = classSubjects.filter(
      (cs) => String(cs.class_id) === classId,
    );
    subjectsCount = new Set(classAssignments.map((cs) => String(cs.subject_id)))
      .size;
    teachersCount = new Set(
      classAssignments
        .filter((cs) => cs.teacher_id)
        .map((cs) => String(cs.teacher_id)),
    ).size;
    assessmentsCount = assessments.length;
  } catch (e) {
    console.error("Error loading class stats:", e);
  } finally {
    isLoading = false;
  }
}

onMount(loadStats);

const statCards = $derived([
  {
    label: m.classes_enrolled_students
      ? m.classes_enrolled_students()
      : "الطلاب المسجلون",
    value: studentsCount,
    icon: Users,
    color: "bg-primary/10 text-primary",
    href: `/${orgId}/academics/classes/${classId}/grades`,
  },
  {
    label: m.academics_layout_nav_subjects
      ? m.academics_layout_nav_subjects()
      : "المواد",
    value: subjectsCount,
    icon: BookOpen,
    color: "bg-brand-lime/10 text-brand-lime",
    href: `/${orgId}/academics/classes/${classId}/teachers`,
  },
  {
    label: m.academics_class_detail_tab_teachers
      ? m.academics_class_detail_tab_teachers()
      : "المعلمون",
    value: teachersCount,
    icon: BookUser,
    color: "bg-brand-pink/10 text-brand-pink",
    href: `/${orgId}/academics/classes/${classId}/teachers`,
  },
  {
    label: m.academics_class_detail_tab_assessments
      ? m.academics_class_detail_tab_assessments()
      : "التقييمات",
    value: assessmentsCount,
    icon: Award,
    color: "bg-amber-500/10 text-amber-500",
    href: `/${orgId}/academics/classes/${classId}/assessments`,
  },
]);
</script>

<div class="flex flex-col gap-6">
  <!-- Stats Grid -->
  {#if isLoading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {#each Array(4) as _}
        <Skeleton class="h-24 w-full rounded-xl" />
      {/each}
    </div>
  {:else}
    <div
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4"
      in:fade={{ duration: 200 }}
    >
      {#each statCards as card (card.label)}
        <a
          href={localizeHref(card.href)}
          class="card-tactile bg-card flex items-center justify-between p-4 group cursor-pointer transition-all duration-150 hover:border-primary/20"
        >
          <div class="flex flex-col gap-1 text-start">
            <span
              class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider"
            >
              {card.label}
            </span>
            <span class="text-2xl font-bold text-foreground font-mono">
              {card.value}
            </span>
          </div>
          <div
            class="h-10 w-10 rounded-lg {card.color} flex items-center justify-center"
          >
            <card.icon size={20} />
          </div>
        </a>
      {/each}
    </div>
  {/if}

  <!-- Quick Actions -->
  <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-4">
    <h2 class="text-sm font-bold text-foreground">
      {m.academics_class_detail_tab_overview
        ? m.academics_class_detail_tab_overview()
        : "الإجراءات السريعة"}
    </h2>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
      <Button
        href={localizeHref(
          `/${orgId}/academics/classes/${classId}/assessments`,
        )}
        variant="outline"
        class="justify-between h-12 text-xs"
      >
        <span class="flex items-center gap-2">
          <Award size={14} />
          {m.academics_class_detail_tab_assessments
            ? m.academics_class_detail_tab_assessments()
            : "التقييمات"}
        </span>
        <ArrowRight
          size={12}
          class="text-muted-foreground {getLocale() === 'ar'
            ? 'rotate-180'
            : ''}"
        />
      </Button>
      <Button
        href={localizeHref(
          `/${orgId}/academics/classes/${classId}/teachers`,
        )}
        variant="outline"
        class="justify-between h-12 text-xs"
      >
        <span class="flex items-center gap-2">
          <BookUser size={14} />
          {m.academics_class_detail_tab_teachers
            ? m.academics_class_detail_tab_teachers()
            : "ربط المعلمين"}
        </span>
        <ArrowRight
          size={12}
          class="text-muted-foreground {getLocale() === 'ar'
            ? 'rotate-180'
            : ''}"
        />
      </Button>
      <Button
        href={localizeHref(
          `/${orgId}/academics/classes/${classId}/grades`,
        )}
        variant="outline"
        class="justify-between h-12 text-xs"
      >
        <span class="flex items-center gap-2">
          <ClipboardList size={14} />
          {m.academics_class_detail_tab_grades
            ? m.academics_class_detail_tab_grades()
            : "الدرجات والطلاب"}
        </span>
        <ArrowRight
          size={12}
          class="text-muted-foreground {getLocale() === 'ar'
            ? 'rotate-180'
            : ''}"
        />
      </Button>
    </div>
  </div>
</div>
