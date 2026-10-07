<!--
  @component ManageUserGroupsSheet
  Standalone slide-over Sheet component for managing a User's Group memberships from Settings,
  matching canonical Orbis Sheet styling.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui/sheet";
import { toast } from "@orbis/ui";
import { ArrowPath, UserGroup } from "@orbis/icons";
import { GroupSelector } from "$lib/features/groups";

interface Props {
  open?: boolean;
  orgId: string;
  userId: string;
  userName?: string;
  assignedGroupIds?: string[];
  onsaved?: () => Promise<void> | void;
}

let {
  open = $bindable(false),
  orgId,
  userId,
  userName = "",
  assignedGroupIds = [],
  onsaved,
}: Props = $props();

let selectedGroupIds = $state<string[]>([]);
let isSaving = $state(false);

$effect(() => {
  if (open) {
    selectedGroupIds = [...assignedGroupIds];
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!userId) return;

  isSaving = true;
  try {
    const currentSet = new Set(assignedGroupIds);
    const targetSet = new Set(selectedGroupIds);

    const toAdd = selectedGroupIds.filter((id) => !currentSet.has(id));
    const toRemove = assignedGroupIds.filter((id) => !targetSet.has(id));

    await Promise.all([
      ...toAdd.map((groupId) => api.addGroupMember(groupId, userId)),
      ...toRemove.map((groupId) => api.removeGroupMember(groupId, userId)),
    ]);

    toast.success(m.ui_copy_user_groups_updated_successfully_93007746());
    open = false;
    await onsaved?.();
  } catch (err: any) {
    toast.error(err?.message || m.ui_copy_failed_to_update_groups_0aac2876());
  } finally {
    isSaving = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-md md:max-w-lg overflow-hidden p-0 gap-0">
    <form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
      <!-- Sheet Header -->
      <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
        <div class="flex items-center gap-2">
          <UserGroup size={20} class="text-primary" />
          <SheetTitle class="text-base font-bold text-foreground">
            {m.ui_copy_manage_user_groups_7fb38ffc()}
          </SheetTitle>
        </div>
        <SheetDescription class="text-xs text-muted-foreground pt-1">
          {userName
            ? m.ui_update_user_groups({ userName })
            : m.ui_copy_assign_user_groups_to_grant_domain_permiss_a3400c8b()}
        </SheetDescription>
      </SheetHeader>

      <!-- Sheet Body -->
      <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide text-start">
        <GroupSelector
          {orgId}
          bind:selectedGroupIds
          maxHeight="max-h-96"
        />
      </div>

      <!-- Sheet Footer -->
      <div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
        <Button
          type="button"
          variant="ghost"
          onclick={() => (open = false)}
          disabled={isSaving}
          class="text-xs h-8 cursor-pointer"
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          variant="lime"
          class="font-semibold px-6 text-xs h-8 cursor-pointer"
          disabled={isSaving}
        >
          {#if isSaving}
            <LoadingIndicator class="me-1.5"  size={13} />
          {/if}
          {m.ui_copy_save_groups_f4b6f340()}
        </Button>
      </div>
    </form>
  </SheetContent>
</Sheet>
