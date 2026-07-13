import * as m from "$lib/paraglide/messages";
<!--
@component ClassDetailLayout
Master-Detail layout for a single class.
Provides a DetailPageHeader (back to classes list) and SecondaryMenu tabs
(Overview | Assessments | Teachers | Grades & Students).
Uses Pattern A: MainViewport with class info sidebar.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Badge } from "@orbis/ui-svelte/badge";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
} from "@orbis/ui-svelte/dropdown-menu";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import AskOrbisButton from "$lib/shared/components/ask-orbis-button.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { toast } from "svelte-sonner";
import {
  GraduationCap,
  Users,
  Award,
  BookUser,
  ClipboardList,
  Bell,
  Share2,
  MoreHorizontal,
} from "lucide-svelte";
import type { Snippet } from "svelte";
import * as academics from "$lib/paraglide/messages/academics";
import * as common from "$lib/paraglide/messages/common";
import { getLocale } from "$lib/paraglide/runtime";

let { children }: { children: Snippet } = $props();

const orgId = $derived(page.params.orgId);
const classId = $derived(page.params.classId);

let classData = $state<any>(null);
let studentsCount = $state(0);
let isLoading = $state(true);

let selectedPriority = $state("high");

async function loadClassData() {
  if (!orgId || !classId) return;
  isLoading = true;
  try {
    const [classes, enrollments] = await Promise.all([
      commands.academic.listClasses(orgId),
      commands.academic.listEnrollments(orgId),
    ]);
    classData = classes.find((c) => String(c.id) === classId);
    studentsCount = enrollments.filter(
      (s) => String(s.class_id) === classId,
    ).length;
  } catch (e) {
    console.error("Error loading class data:", e);
  } finally {
    isLoading = false;
  }
}

onMount(loadClassData);

const basePath = $derived(`/${orgId}/academics/classes/${classId}`);

const menuItems = $derived([
  {
    path: "",
    label: academics.class_detail_tab_overview
      ? academics.class_detail_tab_overview()
      : "نظرة عامة",
    icon: GraduationCap,
  },
  {
    path: "assessments",
    label: academics.class_detail_tab_assessments
      ? academics.class_detail_tab_assessments()
      : "التقييمات",
    icon: Award,
  },
  {
    path: "teachers",
    label: academics.class_detail_tab_teachers
      ? academics.class_detail_tab_teachers()
      : "المعلمون",
    icon: BookUser,
  },
  {
    path: "grades",
    label: academics.class_detail_tab_grades
      ? academics.class_detail_tab_grades()
      : "الدرجات والطلاب",
    icon: ClipboardList,
  },
]);
</script>

{#snippet headerActions()}
  <div class="flex flex-wrap items-center gap-2 select-none">
    {#if classData}
      <AskOrbisButton
        query={academics.class_evaluation_ai_ask_orbis_query({ name: classData.name || classData.name_ar || "" })}
        label={academics.class_evaluation_ask_orbis_btn({ name: classData.name || classData.name_ar || "" })}
        variant="outline"
        size="sm"
        class="h-8 text-[11px]"
      />
    {/if}

    <!-- Bell icon-button -->
    <Button
      variant="outline"
      size="sm"
      class="h-8 text-[11px] gap-1.5 font-semibold shrink-0"
      onclick={() => toast.success(academics.class_overview_notifs_enabled ? academics.class_overview_notifs_enabled() : "Notifications enabled for this class")}
    >
      <Bell size={12} />
      {common.header_notifications ? common.header_notifications() : "Notifications"}
    </Button>

    <DropdownMenu>
      <DropdownMenuTrigger>
        {#snippet child({ props })}
          <Button
            variant="outline"
            size="sm"
            class="h-8 w-8 p-0 cursor-pointer"
            {...props}
          >
            <MoreHorizontal size={14} />
          </Button>
        {/snippet}
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" class="w-48 p-1 z-50 bg-popover border border-border shadow-md rounded-md select-none">
        <DropdownMenuItem
          class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-foreground hover:bg-accent rounded-sm text-start"
          onclick={() => toast.success(academics.class_overview_link_copied ? academics.class_overview_link_copied() : "Diagnostics link copied to clipboard")}
        >
          <Share2 size={12} class="text-muted-foreground shrink-0" />
          <span>{academics.class_layout_share_link()}</span>
        </DropdownMenuItem>

        <DropdownMenuSeparator class="bg-border/60 my-1" />

        <DropdownMenuItem
          class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-foreground hover:bg-accent rounded-sm text-start"
          onclick={() => toast.info(academics.class_overview_more_opts ? academics.class_overview_more_opts() : "More diagnostic options")}
        >
          <MoreHorizontal size={12} class="text-muted-foreground shrink-0" />
          <span>{academics.class_layout_more_options()}</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>

    <!-- Separator divider -->
    <div class="h-4 w-[1px] bg-border/60 mx-1"></div>

    <!-- Priority Indicator -->
    <div class="flex items-center gap-1.5 font-sans">
      <span class="text-[10px] text-muted-foreground">{academics.class_overview_priority ? academics.class_overview_priority() : "Priority:"}</span>
      <Badge variant={selectedPriority === "critical" ? "destructive" : "secondary"} class="text-[9px] py-0.5 px-2 font-bold select-none cursor-pointer">
        {selectedPriority}
      </Badge>
    </div>
  </div>
{/snippet}

<div
  class="h-full w-full flex flex-col overflow-hidden bg-background"
  dir={getLocale() === "ar" ? "rtl" : "ltr"}
>
  <!-- Detail Page Header -->
  <DetailPageHeader
    backHref={`/${orgId}/academics/classes`}
    backLabel={academics.layout_nav_classes
      ? academics.layout_nav_classes()
      : "الفصول"}
    actions={headerActions}
  >
    {#if isLoading}
      <Skeleton class="h-6 w-32" />
      <Skeleton class="h-4.5 w-16" />
    {:else if classData}
      <h1
        class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0 font-sans"
      >
        {classData.name_ar || classData.name || classId}
      </h1>
      <Badge
        variant="secondary"
        class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0 font-sans"
      >
        <Users size={10} class="me-1" />
        {studentsCount}
      </Badge>
      {#if classData.grade_level}
        <Badge
          variant="outline"
          class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0 font-sans"
        >
          {classData.grade_level}
        </Badge>
      {/if}
    {:else}
      <span class="text-sm text-muted-foreground font-sans">{ m.error_not_found() || "غير موجود"}</span
      >
    {/if}
  </DetailPageHeader>

  <div class="flex-1 w-full overflow-y-auto p-6 scrollbar-hide ">
    <div class="flex flex-col gap-6 w-full h-full">
      <SecondaryMenu items={menuItems} {basePath} />
      <div class="w-full flex-1">
        {@render children()}
      </div>
    </div>
  </div>
</div>
