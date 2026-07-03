<!--
@component StudentDetailsLayout
Streamlined layout for deep-dive student pages.
Uses shared DetailPageHeader, SecondaryMenu, and MainViewport.
-->
<script lang="ts">
import { page } from "$app/state";
import { Button } from "@orbis/ui-svelte/button";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte/avatar";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import MainViewport from "$lib/components/layout/MainViewport.svelte";
import DetailPageHeader from "$lib/components/layout/DetailPageHeader.svelte";
import SecondaryMenu from "$lib/components/layout/SecondaryMenu.svelte";
import {
  GraduationCap,
  Wallet,
  UserCheck,
  ShieldAlert,
  PhoneCall,
  Edit2,
  Users,
  Users2,
  FileText,
} from "lucide-svelte";
import { studentsStore } from "$lib/stores";
import { onMount } from "svelte";
import type { Snippet } from "svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";

let { children }: { children: Snippet } = $props();

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);

// Find active student in the store
const student = $derived(studentsStore.list.find((s) => s.id === studentId));
const currentClass = $derived(
  student ? studentsStore.classes.find((c) => c.id === student.class_id) : null,
);

onMount(async () => {
  if (studentsStore.list.length === 0 && orgId) {
    await studentsStore.load(orgId);
  }
});

const basePath = $derived(`/${orgId}/students/${studentId}`);

const menuItems = $derived([
  {
    path: "grades",
    label: m.detail_tab_grades(),
    icon: GraduationCap,
  },
  {
    path: "fees",
    label: m.detail_tab_fees(),
    icon: Wallet,
  },
  {
    path: "attendance",
    label: m.detail_tab_attendance(),
    icon: UserCheck,
  },
  {
    path: "behavior",
    label: m.detail_tab_behavior(),
    icon: ShieldAlert,
  },
  {
    path: "guardians",
    label: m.guardians_tab(),
    icon: Users2,
  },
  {
    path: "documents",
    label: m.detail_tab_documents(),
    icon: FileText,
  },
]);
</script>

<div
  class="h-full w-full flex flex-col overflow-hidden bg-background"
  dir={getLocale() === "ar" ? "rtl" : "ltr"}
>
  <!-- Header using shared DetailPageHeader -->
  <DetailPageHeader
    backHref={`/${orgId}/students`}
    backLabel={m.detail_back_to_directory()}
  >
    {#if student}
      <h1
        class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0"
      >
        {student.full_name || student.name_ar || ""}
      </h1>
      <Badge
        variant="secondary"
        class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
      >
        {currentClass
          ? String(currentClass.name || currentClass.name_ar || "")
          : m.detail_not_enrolled()}
      </Badge>
    {:else}
      <Skeleton class="h-6 w-32" />
      <Skeleton class="h-4.5 w-16" />
    {/if}

    {#snippet actions()}
      {#if student}
        <Button
          href={localizeHref(`/${orgId}/students/${studentId}/edit`)}
          variant="outline"
          size="sm"
          class="h-8 text-[11px] gap-1.5 font-semibold"
        >
          <Edit2 size={12} />
          {m.detail_edit_data()}
        </Button>

        <Button
          variant="outline"
          size="sm"
          class="h-8 text-[11px] gap-1.5 font-semibold"
          onclick={() =>
            toast.info(
              m.detail_call_guardian_toast({
                phone:
                  student.phone_number ||
                  student.phone ||
                  m.detail_phone_unavailable(),
              }),
            )}
        >
          <PhoneCall size={12} />
          {m.detail_call_guardian()}
        </Button>

        <Button
          variant="outline"
          size="sm"
          class="h-8 text-[11px] gap-1.5 font-semibold"
          onclick={() => toast.info(m.student_layout_transfer_coming_soon())}
        >
          <Users size={12} />
          {m.student_layout_transfer_class()}
        </Button>

        <Button
          variant="outline"
          size="sm"
          class="h-8 text-[11px] gap-1.5 font-semibold text-destructive border-destructive/20 hover:bg-destructive/5 hover:text-destructive"
          onclick={() => toast.info(m.student_layout_withdraw_coming_soon())}
        >
          <ShieldAlert size={12} />
          {m.student_layout_withdraw_student()}
        </Button>
      {/if}
    {/snippet}
  </DetailPageHeader>

  <!-- Resizable split view below the header -->
  <div class="flex-1 w-full overflow-hidden">
    <MainViewport
      name="student-detail-viewport"
      leftWidth={{ default: 240, min: 200 }}
    >
      <!-- Left Sidebar (Avatar and basic student info) -->
      {#snippet left()}
        <div
          class="h-full p-4 flex flex-col gap-5 overflow-y-auto scrollbar-hide text-start bg-transparent select-none"
        >
          {#if student}
            <!-- Squared Avatar Display -->
            <div class="w-full flex justify-center">
              <Avatar
                class="w-[85%] max-w-[180px] min-w-[120px] aspect-square h-auto rounded-xl"
              >
                <AvatarImage
                  src={student.avatar
                    ? String(student.avatar)
                    : undefined}
                  alt={student.full_name
                    ? String(student.full_name)
                    : ""}
                />
                <AvatarFallback
                  class="text-5xl font-bold rounded-xl flex items-center justify-center"
                >
                  {String(student.full_name || "S").slice(0, 1)}
                </AvatarFallback>
              </Avatar>
            </div>

            <!-- Student Metadata List -->
            <div
              class="flex flex-col gap-4 border-t border-border/40 pt-4 w-full text-start"
            >
              {#if student.student_number || student.id}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                  >
                    {m.form_student_number
                      ? m.form_student_number()
                      : "الرقم الأكاديمي"}
                  </span>
                  <span class="text-xs font-mono font-bold text-foreground">
                    {student.student_number || student.id}
                  </span>
                </div>
              {/if}

              {#if student.phone}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >الهاتف</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {student.phone}
                  </span>
                </div>
              {/if}

              {#if student.email}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >البريد الإلكتروني</span
                  >
                  <span
                    class="text-xs text-foreground font-mono truncate max-w-full"
                    title={student.email}
                  >
                    {student.email}
                  </span>
                </div>
              {/if}

              {#if student.sex}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >الجنس</span
                  >
                  <span class="text-xs text-foreground font-medium">
                    {student.sex === "male" ? "ذكر" : "أنثى"}
                  </span>
                </div>
              {/if}

              {#if student.birth_date}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >تاريخ الميلاد</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {student.birth_date}
                  </span>
                </div>
              {/if}

              {#if student.guardian_name}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >ولي الأمر</span
                  >
                  <span class="text-xs text-foreground font-medium">
                    {student.guardian_name}
                  </span>
                </div>
              {/if}

              {#if student.guardian_phone}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >هاتف ولي الأمر</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {student.guardian_phone}
                  </span>
                </div>
              {/if}
            </div>
          {:else}
            <!-- Skeleton for sidebar -->
            <div class="w-full flex justify-start">
              <Skeleton class="h-28 w-28 rounded-xl" />
            </div>
            <div class="space-y-4 pt-4 border-t border-border/40">
              <div class="space-y-1">
                <Skeleton class="h-3 w-16" />
                <Skeleton class="h-4 w-24" />
              </div>
              <div class="space-y-1">
                <Skeleton class="h-3 w-16" />
                <Skeleton class="h-4 w-28" />
              </div>
            </div>
          {/if}
        </div>
      {/snippet}

      <!-- Right/Middle Panel (SecondaryMenu & sub-pages content) -->
      {#snippet middle()}
        <div
          class="h-full w-full overflow-y-auto p-6 scrollbar-hide text-start"
        >
          <div class="flex flex-col gap-6 w-full h-full">
            <!-- Tabs using shared SecondaryMenu -->
            <SecondaryMenu items={menuItems} {basePath} />

            <!-- Children Sub-Route Viewport -->
            <div class="w-full flex-1">
              {@render children()}
            </div>
          </div>
        </div>
      {/snippet}
    </MainViewport>
  </div>
</div>
