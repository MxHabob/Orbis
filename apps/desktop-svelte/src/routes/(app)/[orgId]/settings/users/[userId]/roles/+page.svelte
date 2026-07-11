<!--
@component UserRolesPage
Roles tab for user detail — checklist of available roles with search and bulk ops.
Extracted from the original monolith TabsContent "roles".
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";
import SearchField from "$lib/shared/components/search-field.svelte";

const userId = $derived(page.params.userId);

let isLoading = $state(true);
let isSaving = $state(false);
let checkedRoles = $state<string[]>([]);
let allAvailableRoles = $state<string[]>([]);
let roleSearchQuery = $state("");

let filteredRoles = $derived(
  allAvailableRoles.filter((r) =>
    r.toLowerCase().includes(roleSearchQuery.toLowerCase()),
  ),
);

const AVAILABLE_ROLES = ["super_admin", "admin", "registrar", "teacher", "accountant", "student", "guardian"];

async function loadRoles() {
  if (!userId) return;
  isLoading = true;
  try {
    const userRoles = await commands.iam.listUserRoles(userId);
    checkedRoles = userRoles.map((r: any) => r.role);
    allAvailableRoles = AVAILABLE_ROLES;
  } catch (e) {
    console.error("Failed to load roles:", e);
  } finally {
    isLoading = false;
  }
}

onMount(loadRoles);

function selectAllRoles() {
  checkedRoles = [...allAvailableRoles];
}

function unselectAllRoles() {
  checkedRoles = [];
}

function toggleRole(roleName: string, isChecked: boolean) {
  if (isChecked) {
    if (!checkedRoles.includes(roleName)) {
      checkedRoles = [...checkedRoles, roleName];
    }
  } else {
    checkedRoles = checkedRoles.filter((r) => r !== roleName);
  }
}

async function saveRoles() {
  if (!userId) return;
  isSaving = true;
  try {
    await commands.iam.saveUserRoles(userId, checkedRoles);
    toast.success(m.settings_user_detail_saved());
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Save failed");
  } finally {
    isSaving = false;
  }
}
</script>

{#if isLoading}
  <div class="flex flex-col gap-6 text-start">
    <Skeleton class="h-9 w-full max-w-xs" />
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
      {#each Array(6) as _}
        <Skeleton class="h-12 w-full" />
      {/each}
    </div>
  </div>
{:else}
  <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
    <div
      class="flex flex-col sm:flex-row items-center gap-3 w-full justify-between"
    >
      <div class="w-full sm:max-w-xs">
        <SearchField
          placeholder={m.settings_user_detail_search_roles()}
          bind:value={roleSearchQuery}
          class="sm:max-w-xs"
        />
      </div>
      <div class="flex items-center gap-2 w-full sm:w-auto justify-end">
        <Button
          variant="outline"
          size="sm"
          class="text-xs"
          onclick={selectAllRoles}
        >
          {m.settings_user_detail_select_all()}
        </Button>
        <Button
          variant="outline"
          size="sm"
          class="text-xs"
          onclick={unselectAllRoles}
        >
          {m.settings_user_detail_unselect_all()}
        </Button>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 pt-2">
      {#each filteredRoles as roleName (roleName)}
        <div class="flex items-center justify-between p-3.5 select-none">
          <span class="text-xs font-semibold text-foreground">{roleName}</span>
          <Checkbox
            checked={checkedRoles.includes(roleName)}
            onCheckedChange={(v) => toggleRole(roleName, Boolean(v))}
          />
        </div>
      {/each}
    </div>

    <div class="flex justify-end border-t border-border/20 pt-6 mt-4">
      <Button
        variant="lime"
        size="sm"
        class="font-bold"
        onclick={saveRoles}
        disabled={isSaving}
      >
        {isSaving ? "..." : m.settings_user_detail_save()}
      </Button>
    </div>
  </div>
{/if}
