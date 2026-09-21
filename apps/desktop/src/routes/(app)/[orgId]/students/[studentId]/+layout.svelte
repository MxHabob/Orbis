<!--
@component StudentDetailsLayout
Streamlined layout for deep-dive student pages.
Uses shared DetailPageHeader, SecondaryMenu, and MainViewport.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { api } from "$lib/api";
  import { createClassesQuery } from "$lib/features/academics";
  import { createEnrollmentQuery } from "$lib/features/students";
  import * as m from "$lib/paraglide/messages";
  import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
  import MainViewport from "$lib/shell/main-viewport.svelte";
  import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
  import {
    FileText,
    GraduationCap,
    MoreHorizontal,
    PhoneCall,
    ShieldAlert,
    User,
    UserCheck,
    Users,
    Users2,
    Wallet,
  } from "@lucide/svelte";
  import { toast } from "@orbis/ui";
  import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui/avatar";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";
  import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
  } from "@orbis/ui/dropdown-menu";
  import { Skeleton } from "@orbis/ui/skeleton";
  import { useQueryClient } from "@tanstack/svelte-query";
  import type { Snippet } from "svelte";

  import AskOrbisButton from "$lib/shared/components/ask-orbis-button.svelte";

  let { children }: { children: Snippet } = $props();
  const queryClient = useQueryClient();

  const orgId = $derived(page.params.orgId || "");
  const studentId = $derived(page.params.studentId || "");

  // TanStack Queries
  const studentQuery = createEnrollmentQuery(
    () => orgId,
    () => studentId,
  );
  const classesQuery = createClassesQuery(() => orgId);

  const student = $derived(studentQuery.data);
  const currentClass = $derived(
    student
      ? classesQuery.data?.find(
          (c) => String(c.id) === String(student.class_id),
        )
      : null,
  );

  const basePath = $derived(`/${orgId}/students/${studentId}`);

  let isWithdrawing = $state(false);

  async function withdrawStudent() {
    if (!confirm(m.common_student_layout_withdraw_confirm?.())) return;
    isWithdrawing = true;
    try {
      await api.updateStudentStatus({ studentId, status: "withdrawn" });
      toast.success(m.common_form_success_update());
      queryClient.invalidateQueries({ queryKey: ["enrollments", orgId] });
      queryClient.invalidateQueries({ queryKey: ["student", studentId] });
    } catch (e: any) {
      console.error("Withdraw error:", e);
      toast.error(e?.message || String(e));
    } finally {
      isWithdrawing = false;
    }
  }

  let showTransferDialog = $state(false);
  let transferTargetClassId = $state("");
  let isTransferring = $state(false);

  async function transferClass() {
    if (!transferTargetClassId) {
      toast.error(m.common_form_error_class_required());
      return;
    }
    isTransferring = true;
    try {
      await api.transferClass({
        enrollmentId: studentId,
        targetClassId: transferTargetClassId,
      });
      toast.success(m.common_form_success_update());
      showTransferDialog = false;
      transferTargetClassId = "";
      queryClient.invalidateQueries({ queryKey: ["enrollments", orgId] });
      queryClient.invalidateQueries({ queryKey: ["student", studentId] });
    } catch (e: any) {
      console.error("Transfer error:", e);
      toast.error(e?.message || String(e));
    } finally {
      isTransferring = false;
    }
  }

  const menuItems = $derived([
    {
      path: "",
      label: m.common_breadcrumb_student_profile(),
      icon: User,
    },
    {
      path: "grades",
      label: m.students_detail_tab_grades(),
      icon: GraduationCap,
    },
    {
      path: "fees",
      label: m.students_detail_tab_fees(),
      icon: Wallet,
    },
    {
      path: "attendance",
      label: m.common_promo_attendance(),
      icon: UserCheck,
    },
    {
      path: "behavior",
      label: m.common_detail_tab_behavior(),
      icon: ShieldAlert,
    },
    {
      path: "guardians",
      label: m.common_guardians_tab(),
      icon: Users2,
    },
    {
      path: "documents",
      label: m.students_detail_tab_documents(),
      icon: FileText,
    },
  ]);
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <!-- Header using shared DetailPageHeader -->
  <DetailPageHeader
    backHref={`/${orgId}/students`}
    backLabel={m.students_detail_back_to_directory()}
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
          : m.common_status_not_enrolled()}
      </Badge>
    {:else}
      <Skeleton class="h-6 w-32" />
      <Skeleton class="h-4.5 w-16" />
    {/if}

    {#snippet actions()}
      {#if student}
        <AskOrbisButton
          query={m.academics_student_evaluation_ai_ask_orbis_query({
            name: student.full_name || student.name_ar || "",
          })}
          label={m.academics_student_evaluation_ask_orbis_btn({
            name: student.full_name || student.name_ar || "",
          })}
          variant="outline"
          size="sm"
          class="h-8 text-[11px]"
        />

        <Button
          variant="outline"
          size="sm"
          class="h-8 text-[11px] gap-1.5 font-semibold"
          onclick={() =>
            toast.info(
              m.students_detail_call_guardian_toast({
                phone:
                  student.phone_number ||
                  student.phone ||
                  m.students_detail_phone_unavailable(),
              }),
            )}
        >
          <PhoneCall size={12} />
          {m.students_detail_call_guardian()}
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
          <DropdownMenuContent
            align="end"
            class="w-48 p-1 z-50 bg-popover border border-border shadow-md rounded-md select-none"
          >
            <DropdownMenuItem
              class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-foreground hover:bg-accent rounded-sm text-start"
              onclick={() => showTransferDialog = true}
            >
              <Users size={12} class="text-muted-foreground shrink-0" />
              <span>{m.common_student_layout_transfer_class()}</span>
            </DropdownMenuItem>

            <DropdownMenuSeparator class="bg-border/60 my-1" />

            <DropdownMenuItem
              class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-destructive focus:text-destructive focus:bg-destructive/5 hover:bg-destructive/5 rounded-sm text-start"
              onclick={withdrawStudent}
              disabled={isWithdrawing}
            >
              <ShieldAlert size={12} class="text-destructive shrink-0" />
              <span>{m.common_inspector_withdraw()}</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
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
                  src={student.a}
                  alt={student.full_name ? String(student.full_name) : ""}
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
                    {m.common_word_academic_number()}
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
                    >{m.common_word_phone()}</span
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
                    >{m.common_word_email()}</span
                  >
                  <span
                    class="text-xs text-foreground font-mono truncate max-w-full"
                    title={String(student.email)}
                  >
                    {student.email}
                  </span>
                </div>
              {/if}

              {#if student.sex}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_form_sex()}</span
                  >
                  <span class="text-xs text-foreground font-medium">
                    {student.sex === "male"
                      ? m.common_form_male()
                      : m.common_form_female()}
                  </span>
                </div>
              {/if}

              {#if student.birth_date}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_dob()}</span
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
                    >{m.common_users_filter_guardian()}</span
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
                    >{m.common_bulk_col_guardian_phone()}</span
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

{#if showTransferDialog}
  <!-- Transfer Class Dialog Overlay -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center" onclick={() => showTransferDialog = false}>
    <div class="bg-popover border border-border rounded-xl p-6 w-80 shadow-lg" onclick={(e) => e.stopPropagation()}>
      <h3 class="text-sm font-bold mb-4">{m.common_student_layout_transfer_class()}</h3>
      <label class="text-[10px] text-muted-foreground mb-1 block">{m.common_form_label_class?.() ?? "Target Class"}</label>
      <select
        class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background mb-4"
        bind:value={transferTargetClassId}
      >
        <option value="">—</option>
        {#each (classesQuery.data ?? []) as cls}
          <option value={String(cls.id)}>{cls.name}</option>
        {/each}
      </select>
      <div class="flex gap-2 justify-end">
        <Button variant="outline" class="text-xs h-8" onclick={() => showTransferDialog = false}>
          {m.common_btn_cancel()}
        </Button>
        <Button class="text-xs h-8" onclick={transferClass} disabled={isTransferring || !transferTargetClassId}>
          {m.common_student_layout_transfer_class()}
        </Button>
      </div>
    </div>
  </div>
{/if}
