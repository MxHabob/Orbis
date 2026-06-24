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
import { Avatar, AvatarFallback } from "@orbis/ui-svelte/avatar";
import {
  Plus,
  Users2,
  Edit2,
  Trash2,
  Phone,
  Briefcase,
  CheckCircle,
  XCircle,
} from "lucide-svelte";
import { commands } from "$lib/services/ipc";
import type { GuardianLink, JsonRecord } from "@orbis/desktop-client";
import GuardianLinkSheet from "$lib/components/sheets/guardian-link-sheet.svelte";
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
    <!-- Guardian Cards -->
    <div class="grid grid-cols-1 gap-3">
      {#each guardians as link (link.id)}
        {@const user = getGuardianUser(link)}
        <div
          class="group relative flex items-center gap-4 p-4 rounded-xl border border-border/40
            bg-card/30 hover:bg-card/60 transition-all duration-150 hover:border-border/60"
        >
          <!-- Avatar -->
          <Avatar class="h-12 w-12 ring-2 ring-border/30 shrink-0">
            <AvatarFallback class="bg-primary/10 text-primary font-bold text-base">
              {String(user?.full_name ?? "?").charAt(0).toUpperCase()}
            </AvatarFallback>
          </Avatar>

          <!-- Info -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 flex-wrap">
              <span class="font-bold text-sm text-foreground">
                {String(user?.full_name ?? m.guardians_col_name())}
              </span>
              <span class="text-[10px] px-2 py-0.5 rounded-full bg-primary/10 text-primary font-semibold">
                {getRelationLabel(String(link.relation ?? ""))}
              </span>
              {#if link.is_payer}
                <span class="flex items-center gap-1 text-[10px] px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-600 font-semibold">
                  <CheckCircle size={9} />
                  دافع الرسوم
                </span>
              {/if}
            </div>

            <div class="flex items-center gap-4 mt-1.5">
              {#if user?.phone_number}
                <div class="flex items-center gap-1 text-[11px] text-muted-foreground">
                  <Phone size={10} />
                  <span class="font-mono">{String(user.phone_number)}</span>
                </div>
              {/if}
              {#if (link as JsonRecord).job}
                <div class="flex items-center gap-1 text-[11px] text-muted-foreground">
                  <Briefcase size={10} />
                  <span>{String((link as JsonRecord).job)}</span>
                </div>
              {/if}
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
            <Button
              variant="ghost"
              size="icon"
              class="h-7 w-7 hover:bg-primary/10 hover:text-primary"
              onclick={() => openEdit(link)}
              title="تعديل"
            >
              <Edit2 size={13} />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              class="h-7 w-7 hover:bg-destructive/10 hover:text-destructive"
              onclick={() => handleDelete(link)}
              title="إلغاء الربط"
            >
              <Trash2 size={13} />
            </Button>
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
