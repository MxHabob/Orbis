<!--
@component StudentDetailsLayout
Streamlined layout for deep-dive student pages.
Includes a header (name, class, action buttons, navigation tabs) and the main content viewport below.
-->
<script lang="ts">
import { page } from "$app/state";
import { Button } from "@orbis/ui-svelte/button";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte/avatar";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  ArrowRight,
  ArrowLeft,
  GraduationCap,
  Wallet,
  UserCheck,
  ShieldAlert,
  PhoneCall,
  Edit2,
  Users,
} from "lucide-svelte";
import { studentsStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { onMount } from "svelte";
import type { Snippet } from "svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, deLocalizeUrl, localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatPercent } from "$lib/i18n-utils";

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

function isActive(tabHref: string) {
  const unlocalizedPath = deLocalizeUrl(page.url).pathname;
  return unlocalizedPath === tabHref;
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <!-- Main Header Block (Name, class, actions, tabs) -->
  <div class="border-b border-border/50 bg-card/15 shrink-0 flex flex-col select-none">
    <!-- Top Row: Student info & actions -->
    <div class="h-16 flex items-center justify-between px-6 border-b border-border/40">
      <div class="flex items-center gap-4">
        <Button
          href={localizeHref(`/${orgId}/students`)}
          variant="ghost"
          size="icon"
          class="h-8 w-8 hover:bg-accent/80 cursor-pointer shrink-0"
          title={m.detail_back_to_directory()}
        >
          {#if getLocale() === "ar"}
            <ArrowRight class="h-4 w-4" />
          {:else}
            <ArrowLeft class="h-4 w-4" />
          {/if}
        </Button>
        <div class="h-4 w-px bg-border/80 shrink-0"></div>
        
        {#if student}
          <div class="flex items-center gap-3">
            <Avatar class="h-8 w-8 ring-2 ring-primary/5">
              <AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
              <AvatarFallback class="bg-primary/5 text-primary text-xs font-bold">
                {String(student.full_name || "S").slice(0, 1)}
              </AvatarFallback>
            </Avatar>
            <div class="text-start leading-tight">
              <h1 class="text-sm font-bold text-foreground flex items-center gap-2">
                {student.full_name || student.name_ar || ''}
                <span class="badge-brand-lime text-[9px] py-0.5 px-2 rounded-full font-sans font-medium">
                  {currentClass ? String(currentClass.name || currentClass.name_ar || '') : m.detail_not_enrolled()}
                </span>
              </h1>
              <span class="text-[10px] text-muted-foreground font-mono">
                ID: {student.student_number || student.id}
              </span>
            </div>
          </div>
        {:else}
          <div class="flex items-center gap-3">
            <Skeleton class="h-8 w-8 rounded-full" />
            <div class="space-y-1">
              <Skeleton class="h-4 w-28" />
              <Skeleton class="h-3 w-16" />
            </div>
          </div>
        {/if}
      </div>

      {#if student}
        <!-- Action Buttons -->
        <div class="flex items-center gap-2">
          <Button
            href={localizeHref(`/${orgId}/students/${studentId}/edit`)}
            variant="outline"
            size="sm"
            class="h-8 text-[11px] gap-1.5"
          >
            <Edit2 size={12} />
            {m.detail_edit_data()}
          </Button>

          <Button
            variant="outline"
            size="sm"
            class="h-8 text-[11px] gap-1.5"
            onclick={() => toast.info(m.detail_call_guardian_toast({ phone: student.phone_number || m.detail_phone_unavailable() }))}
          >
            <PhoneCall size={12} />
            {m.detail_call_guardian()}
          </Button>

          <Button
            variant="outline"
            size="sm"
            class="h-8 text-[11px] gap-1.5"
            onclick={() => toast.info(m.student_layout_transfer_coming_soon())}
          >
            <Users size={12} />
            {m.student_layout_transfer_class()}
          </Button>

          <Button
            variant="outline"
            size="sm"
            class="h-8 text-[11px] gap-1.5 text-destructive hover:bg-destructive/5 hover:text-destructive"
            onclick={() => toast.info(m.student_layout_withdraw_coming_soon())}
          >
            <ShieldAlert size={12} />
            {m.student_layout_withdraw_student()}
          </Button>
        </div>
      {/if}
    </div>

    <!-- Bottom Row: Navigation Tabs -->
    <div class="h-11 flex items-center px-6 gap-1 bg-card/5">
      <!-- Svelte 5 derived tabs navigation -->
      {#each [
        { id: "grades", label: m.detail_tab_grades(), icon: GraduationCap, href: `/${orgId}/students/${studentId}/grades` },
        { id: "fees", label: m.detail_tab_fees(), icon: Wallet, href: `/${orgId}/students/${studentId}/fees` },
        { id: "attendance", label: m.detail_tab_attendance(), icon: UserCheck, href: `/${orgId}/students/${studentId}/attendance` },
        { id: "behavior", label: m.detail_tab_behavior(), icon: ShieldAlert, href: `/${orgId}/students/${studentId}/behavior` }
      ] as tab (tab.id)}
        <Button
          href={localizeHref(tab.href)}
          variant={isActive(tab.href) ? 'default' : 'ghost'}
          class="text-xs font-semibold px-4 py-1.5 h-auto transition-all duration-150 cursor-pointer"
        >
          <tab.icon size={13} class="me-1.5" />
          {tab.label}
        </Button>
      {/each}
    </div>
  </div>

  <!-- Content Viewport -->
  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    {@render children()}
  </div>
</div>
