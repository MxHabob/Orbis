<!--
  @component StudentGuardians
  Guardians linked to a student, with contact details and management sheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import { Pencil, Plus, Trash, Users } from "@orbis/icons";
import { api, extractErrorMessage } from "$lib/api";
import type { GuardianLink, JsonRecord } from "@orbis/sdk";
import GuardianLinkSheet from "$lib/features/students/components/guardian-link-sheet.svelte";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();
const queryClient = useQueryClient();

const relations: Record<string, string> = {
  father: m.common_guardians_relationship_father(),
  mother: m.common_guardians_relationship_mother(),
  sibling: m.common_guardians_relation_sibling(),
  grandparent: m.common_guardians_relation_grandparent(),
  uncle: m.common_guardians_relation_uncle(),
  other: m.common_guardians_relationship_other(),
};

const guardiansQuery = createQuery(() => ({
  queryKey: ["student-guardians", studentId],
  queryFn: async () => {
    const links = (await api.listGuardians(studentId)) as unknown as GuardianLink[];
    const userMap: Record<string, JsonRecord> = {};
    await Promise.all(
      links.map(async (link: any) => {
        try {
          const u = await api.getUser(String(link.guardian_id));
          if (u) userMap[String(link.guardian_id)] = u as unknown as JsonRecord;
        } catch {}
      }),
    );
    return { links, userMap };
  },
  enabled: !!studentId,
  staleTime: 3 * 60 * 1000,
}));

const guardians = $derived(guardiansQuery.data?.links ?? []);
const guardianUsers = $derived(guardiansQuery.data?.userMap ?? {});
const isLoading = $derived(guardiansQuery.isLoading);
let sheetOpen = $state(false);
let editTarget = $state<GuardianLink | null>(null);

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
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete() + "؟",
    description: m.common_guardians_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  try {
    toast.success(m.common_guardians_delete_success());
    await queryClient.invalidateQueries({ queryKey: ["student-guardians", studentId] });
  } catch (e: unknown) {
    toast.error(
      `${m.common_guardians_delete_failed()}: ${extractErrorMessage(e)}`,
    );
  }
}
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-sm font-bold text-foreground">{m.common_guardians_tab()}</h2>
      <p class="text-[11px] text-muted-foreground">{m.common_guardians_desc()}</p>
    </div>
    <Button onclick={openCreate} size="sm" class="h-8 gap-1.5 text-xs">
      <Plus size={13} />
      {m.common_guardians_add()}
    </Button>
  </div>

  {#if isLoading}
    <div class="flex flex-col gap-3">
      {#each Array(3) as _, i (i)}
        <Skeleton class="h-20 w-full rounded-xl" />
      {/each}
    </div>
  {:else if guardians.length === 0}
    <div class="flex flex-col items-center justify-center gap-4 py-16 text-center">
      <div class="p-5 rounded-2xl bg-muted/30 border border-border/30">
        <Users class="h-10 w-10 text-muted-foreground/40" />
      </div>
      <div class="space-y-1">
        <p class="text-sm font-semibold text-foreground">{m.common_guardians_empty()}</p>
        <p class="text-xs text-muted-foreground max-w-xs">{m.common_guardians_empty_desc()}</p>
      </div>
      <Button onclick={openCreate} size="sm" class="mt-2 gap-1.5 text-xs">
        <Plus size={13} />
        {m.common_guardians_add()}
      </Button>
    </div>
  {:else}
    <div class="grid grid-cols-1 gap-6">
      {#each guardians as link, index (link.id)}
        {@const user = getGuardianUser(link)}
        <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
          <div class="bg-muted/50 px-6 py-3 border-b border-border flex justify-between items-center select-none">
            <span class="text-[11px] font-bold text-foreground uppercase tracking-wider">
              {m.common_form_guardian_number()} {index + 1}
            </span>
            <div class="flex items-center gap-1.5 flex-nowrap shrink-0">
              <Button
                variant="ghost"
                size="sm"
                class="h-7 px-2.5 text-xs text-primary hover:bg-primary/10 gap-1"
                onclick={() => openEdit(link)}
              >
                <Pencil size={12} />
                {m.common_action_edit_simple()}
              </Button>
              <Button
                variant="ghost"
                size="sm"
                class="h-7 px-2.5 text-xs text-destructive hover:bg-destructive/10 gap-1"
                onclick={() => handleDelete(link)}
              >
                <Trash size={12} />
                {m.common_action_unlink()}
              </Button>
            </div>
          </div>

          <div class="flex flex-col">
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.common_word_fullname()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_fullname_desc()}</span>
              </div>
              <div class="w-full md:w-120 shrink-0 text-start font-medium text-sm text-foreground">
                {user?.full_name ?? user?.name_ar ?? "—"}
              </div>
            </div>

            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.common_word_phone()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_phone_desc()}</span>
              </div>
              <div class="w-full md:w-120 shrink-0 text-start font-mono text-sm text-foreground">
                {user?.phone ?? user?.phone_number ?? "—"}
              </div>
            </div>

            {#if user?.email && !String(user.email).endsWith("@orbis.local")}
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <span class="text-xs font-semibold text-foreground">{m.common_word_email()}</span>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_guardians_email_desc()}</span>
                </div>
                <div class="w-full md:w-120 shrink-0 text-start font-mono text-sm text-foreground">
                  {String(user.email)}
                </div>
              </div>
            {/if}

            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.common_bulk_col_guardian_rel()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_rel_desc()}</span>
              </div>
              <div class="w-full md:w-120 shrink-0 text-start font-medium text-sm text-foreground">
                {getRelationLabel(String(link.relationship || ""))}
              </div>
            </div>

            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.common_form_guardian_national_id_label()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_nid_desc()}</span>
              </div>
              <div class="w-full md:w-120 shrink-0 text-start font-mono text-sm text-foreground">
                {user?.national_id ?? "—"}
              </div>
            </div>

            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">{m.common_guardians_roles_title()}</span>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_guardians_roles_desc()}</span>
              </div>
              <div class="w-full md:w-120 shrink-0 flex flex-col gap-3">
                <div class="flex flex-wrap gap-2 justify-start">
                  {#if link.is_primary === 1 || (link as any).is_primary === true}
                    <Badge variant="brand" class="text-[10px]">{m.common_form_guardian_role_primary()}</Badge>
                  {/if}
                </div>
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<GuardianLinkSheet
  bind:open={sheetOpen}
  {orgId}
  {studentId}
  editData={editTarget}
  onSuccess={() => queryClient.invalidateQueries({ queryKey: ["student-guardians", studentId] })}
/>
