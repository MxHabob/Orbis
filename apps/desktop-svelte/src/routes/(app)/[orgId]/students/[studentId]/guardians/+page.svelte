<!--
  @page Student Guardians
  Lists all guardians linked to a student. CRUD via side sheet.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import { Plus, Users2, Edit2, Trash2 } from "lucide-svelte";
import { commands } from "$lib/core/ipc";
import type { GuardianLink, JsonRecord } from "@orbis/sdk-core";
import GuardianLinkSheet from "$lib/features/students/components/guardian-link-sheet.svelte";
import * as m from "$lib/paraglide/messages";

const orgId = $derived(page.params.orgId ?? "");
const studentId = $derived(page.params.studentId ?? "");

let guardians = $state<GuardianLink[]>([]);
let guardianUsers = $state<Record<string, JsonRecord>>({});
let isLoading = $state(false);
let sheetOpen = $state(false);
let editTarget = $state<GuardianLink | null>(null);

const relations: Record<string, string> = {
  father: "أب",
  mother: "أم",
  sibling: "أخ/أخت",
  grandparent: "جد/جدة",
  uncle: "عم/خال",
  other: "أخرى",
};

async function loadData() {
  if (!studentId) return;
  isLoading = true;
  try {
    const links = await commands.guardians.getByStudent(studentId);
    guardians = links;

    // Fetch guardian user details in parallel
    const userMap: Record<string, JsonRecord> = {};
    await Promise.all(
      links.map(async (link) => {
        try {
          const u = await commands.users.getUser(String(link.guardian_id));
          if (u) userMap[String(link.guardian_id)] = u as unknown as JsonRecord;
        } catch {}
      }),
    );
    guardianUsers = userMap;
  } catch (e) {
    console.error("Failed to load guardians:", e);
  } finally {
    isLoading = false;
  }
}

onMount(loadData);

function getGuardianUser(link: GuardianLink): JsonRecord | null {
  return guardianUsers[String(link.guardian_id)] ?? null;
}

function getRelationLabel(r: string | undefined | null): string {
  return relations[String(r ?? "")] ?? String(r ?? "—");
}

function openCreate() {
  editTarget = null;
  sheetOpen = true;
}

function openEdit(link: GuardianLink) {
  editTarget = link;
  sheetOpen = true;
}

async function handleDelete(link: GuardianLink) {
  if (!confirm(m.guardians_delete_confirm())) return;
  try {
    await commands.guardians.deleteLink(String(link.id));
    guardians = guardians.filter((g) => g.id !== link.id);
    toast.success(m.guardians_delete_success());
  } catch (e: unknown) {
    toast.error(`فشل الحذف: ${e instanceof Error ? e.message : String(e)}`);
  }
}
</script>

<svelte:head>
  <title>{m.guardians_title()}</title>
</svelte:head>

<div class="flex flex-col gap-4">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-sm font-bold text-foreground">{m.guardians_title()}</h2>
      <p class="text-[11px] text-muted-foreground">{m.guardians_desc()}</p>
    </div>
    <Button onclick={openCreate} size="sm" class="h-8 gap-1.5 text-xs">
      <Plus size={13} />
      {m.guardians_add()}
    </Button>
  </div>

  <!-- Content -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      {#each Array(3) as _, i (i)}
        <Skeleton class="h-20 w-full rounded-xl" />
      {/each}
    </div>
  {:else if guardians.length === 0}
    <!-- Empty State -->
    <div class="flex flex-col items-center justify-center gap-4 py-16 text-center">
      <div class="p-5 rounded-2xl bg-muted/30 border border-border/30">
        <Users2 class="h-10 w-10 text-muted-foreground/40" />
      </div>
      <div class="space-y-1">
        <p class="text-sm font-semibold text-foreground">{m.guardians_empty()}</p>
        <p class="text-xs text-muted-foreground max-w-xs">{m.guardians_empty_desc()}</p>
      </div>
      <Button onclick={openCreate} size="sm" class="mt-2 gap-1.5 text-xs">
        <Plus size={13} />
        {m.guardians_add()}
      </Button>
    </div>
  {:else}
    <!-- Guardian Cards Redesigned -->
    <div class="grid grid-cols-1 gap-6">
      {#each guardians as link, index (link.id)}
        {@const user = getGuardianUser(link)}
        <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
          <!-- Guardian Block Header -->
          <div class="bg-muted/50 px-6 py-3 border-b border-border flex justify-between items-center select-none">
            <span class="text-[11px] font-bold text-foreground uppercase tracking-wider">
              {m.form_guardian_number()} {index + 1}
            </span>
            <div class="flex items-center gap-1.5 flex-nowrap shrink-0">
              <Button
                variant="ghost"
                size="sm"
                class="h-7 px-2.5 text-xs text-primary hover:bg-primary/10 gap-1"
                onclick={() => openEdit(link)}
              >
                <Edit2 size={12} />
                تعديل
              </Button>
              <Button
                variant="ghost"
                size="sm"
                class="h-7 px-2.5 text-xs text-destructive hover:bg-destructive/10 gap-1"
                onclick={() => handleDelete(link)}
              >
                <Trash2 size={12} />
                إلغاء الربط
              </Button>
            </div>
          </div>

          <div class="flex flex-col">
            <!-- Guardian Full Name -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.form_guardian_name_label()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">الاسم الكامل لولي الأمر</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 text-start font-medium text-sm text-foreground">
                {user?.full_name ?? user?.name_ar ?? "—"}
              </div>
            </div>

            <!-- Phone Number -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.form_guardian_phone_label()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">رقم هاتف المحمول لولي الأمر</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 text-start font-mono text-sm text-foreground" dir="ltr">
                {user?.phone ?? user?.phone_number ?? "—"}
              </div>
            </div>

            <!-- Email -->
            {#if user?.email && !String(user.email).endsWith("@orbis.local")}
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <span class="text-xs font-semibold text-foreground">البريد الإلكتروني</span>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">البريد الإلكتروني لولي الأمر</span>
                </div>
                <div class="w-full md:w-[480px] shrink-0 text-start font-mono text-sm text-foreground" dir="ltr">
                  {String(user.email)}
                </div>
              </div>
            {/if}

            <!-- Relationship -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.form_guardian_relationship_label()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">صلة القرابة بين ولي الأمر والطالب</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 text-start font-medium text-sm text-foreground">
                {getRelationLabel(String(link.relationship || link.relation || ""))}
              </div>
            </div>

            <!-- National ID -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.form_guardian_national_id_label()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">رقم الهوية الوطنية أو البطاقة الشخصية لولي الأمر</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 text-start font-mono text-sm text-foreground">
                {user?.national_id ?? "—"}
              </div>
            </div>

            <!-- Occupation -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">المهنة / الوظيفة</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">مهنة ولي الأمر الحالية وجهة العمل</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 text-start font-medium text-sm text-foreground">
                {link.occupation ?? (link as any).job ?? "—"}
              </div>
            </div>

            <!-- Permissions -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">الصلاحيات والمسؤوليات المالية</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">تحديد أدوار ولي الأمر ومسؤولياته المالية</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex flex-col gap-3">
                <div class="flex flex-wrap gap-2 justify-start">
                  {#if link.is_primary === 1 || link.is_primary === true}
                    <Badge variant="brand" class="text-[10px]">{m.form_guardian_role_primary()}</Badge>
                  {/if}
                  {#if link.is_emergency_contact === 1 || link.is_emergency_contact === true}
                    <Badge variant="secondary" class="text-[10px]">{m.form_guardian_role_emergency()}</Badge>
                  {/if}
                  {#if link.is_payer === 1 || link.is_payer === true}
                    <Badge variant="lime" class="text-[10px]">{m.form_guardian_role_payer()}</Badge>
                  {/if}
                </div>
                {#if link.is_payer === 1 || link.is_payer === true}
                  <div class="text-xs text-muted-foreground text-start">
                    {m.form_guardian_payment_share()} <span class="font-bold text-foreground">{link.payment_share_percent ?? 0}%</span>
                  </div>
                {/if}
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Sheet -->
<GuardianLinkSheet
  bind:open={sheetOpen}
  {orgId}
  {studentId}
  editData={editTarget}
  onSuccess={loadData}
/>
