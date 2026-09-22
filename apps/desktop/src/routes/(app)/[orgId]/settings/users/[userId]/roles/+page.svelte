<!--
@component UserRolesPage
Roles tab for user detail — clean checklist of available system and custom roles using standard Orbis form styling.
-->
<script lang="ts">
import { FloppyDisk } from "@orbis/icons";

import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { fade } from "svelte/transition";
import { l } from "$lib/core/i18n";
import {
  SYSTEM_ROLE_DEFINITIONS,
  countRolePermissions,
} from "$lib/core/iam";
import type { RoleDto } from "$lib/api/types";

const userId = $derived(page.params.userId);

let isLoading = $state(true);
let isSaving = $state(false);
let checkedRoles = $state<string[]>([]);
let availableRoles = $state<Array<{
  id: string;
  nameEn: string;
  nameAr: string;
  descEn: string;
  descAr: string;
  isSystem: boolean;
  permCount: number;
}>>([]);

async function loadRoles() {
  if (!userId) return;
  isLoading = true;
  try {
    const [fetchedRoles, userDetails] = await Promise.all([
      api.listRoles().catch(() => []),
      api.getUser(userId).catch(() => null) as Promise<any>,
    ]);

    const roles: typeof availableRoles = [];

    // 1. Add system roles
    for (const sys of SYSTEM_ROLE_DEFINITIONS) {
      const apiRole = fetchedRoles.find((r) => r.id === sys.id);
      roles.push({
        id: sys.id,
        nameEn: sys.nameEn,
        nameAr: sys.nameAr,
        descEn: sys.descEn,
        descAr: sys.descAr,
        isSystem: true,
        permCount: countRolePermissions(apiRole?.permissions || sys.defaultPermissions),
      });
    }

    // 2. Add custom roles
    for (const r of fetchedRoles) {
      if (!roles.some((existing) => existing.id === r.id)) {
        roles.push({
          id: r.id,
          nameEn: r.name,
          nameAr: r.name_ar || r.name,
          descEn: (r as any).description || "Custom role",
          descAr: (r as any).description || "دور مخصص في المؤسسة",
          isSystem: false,
          permCount: countRolePermissions(r.permissions),
        });
      }
    }

    availableRoles = roles;

    // Load active roles for this user
    checkedRoles = Array.isArray(userDetails?.roles) ? [...userDetails.roles] : [];
  } catch (e) {
    console.error("Failed to load user roles:", e);
  } finally {
    isLoading = false;
  }
}

onMount(loadRoles);

function toggleRole(roleId: string, active: boolean) {
  if (active) {
    if (!checkedRoles.includes(roleId)) {
      checkedRoles = [...checkedRoles, roleId];
    }
  } else {
    checkedRoles = checkedRoles.filter((r) => r !== roleId);
  }
}

async function saveRoles() {
  if (!userId) return;
  isSaving = true;
  try {
    await api.updateUser(userId, {
      roles: checkedRoles,
    } as any);

    toast.success(
      m.settings_user_detail_saved
        ? m.settings_user_detail_saved()
        : l("تم حفظ وتحديث أدوار المستخدم بنجاح", "User roles saved successfully")
    );
  } catch (e: any) {
    toast.error(e?.message || l("فشل حفظ الأدوار", "Save failed"));
  } finally {
    isSaving = false;
  }
}
</script>

{#if isLoading}
  <div class="flex flex-col gap-3 text-start">
    {#each Array(4) as _}
      <Skeleton class="h-16 w-full rounded-lg" />
    {/each}
  </div>
{:else}
  <div in:fade={{ duration: 100 }} class="flex flex-col gap-4 text-start">
    <!-- List of Roles -->
    <div class="flex flex-col divide-y divide-border/50 rounded-xl border border-border/70 bg-card overflow-hidden">
      {#each availableRoles as role (role.id)}
        {@const isChecked = checkedRoles.includes(role.id)}

        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          onclick={() => toggleRole(role.id, !isChecked)}
          class="p-3.5 flex items-center justify-between gap-3 cursor-pointer select-none transition-colors hover:bg-muted/30 {isChecked ? 'bg-primary/5' : ''}"
        >
          <div class="flex items-start gap-3 min-w-0">
            <div class="pt-0.5 shrink-0" onclick={(e) => e.stopPropagation()}>
              <Checkbox
                id={`role_${role.id}`}
                checked={isChecked}
                onCheckedChange={(v) => toggleRole(role.id, Boolean(v))}
              />
            </div>
            <div class="flex flex-col gap-0.5 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-xs font-bold text-foreground">
                  {l(role.nameAr, role.nameEn)}
                </span>
                <span class="text-[10px] font-mono text-muted-foreground">
                  ({role.id})
                </span>
                {#if role.isSystem}
                  <Badge variant="secondary" class="text-[9px] px-1.5 py-0">
                    {l("نظام قياسي", "System")}
                  </Badge>
                {/if}
              </div>
              <p class="text-[11px] text-muted-foreground leading-relaxed">
                {l(role.descAr, role.descEn)}
              </p>
            </div>
          </div>

          <div class="shrink-0 text-end">
            <Badge variant="outline" class="text-[9px] font-mono text-muted-foreground">
              {role.permCount} {l("صلاحية", "perms")}
            </Badge>
          </div>
        </div>
      {/each}
    </div>

    <!-- FloppyDisk Button -->
    <div class="flex justify-end pt-2">
      <Button
        variant="lime"
        size="sm"
        class="font-bold px-6 h-8 text-xs cursor-pointer"
        onclick={saveRoles}
        disabled={isSaving}
      >
        {isSaving
          ? "..."
          : (m.settings_action_save_changes ? m.settings_action_save_changes() : l("حفظ التغييرات", "Save Changes"))}
      </Button>
    </div>
  </div>
{/if}
