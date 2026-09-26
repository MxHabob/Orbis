<!--
  @component ManageEmployeeGroupsSheet
  Standalone slide-over Sheet component for managing an employee's assigned User Groups.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
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
import { extractErrorMessage } from "$lib/api";
import { createAssignEmployeeGroupsMutation } from "../queries";
import { GroupSelector } from "$lib/features/groups";

interface Props {
  open?: boolean;
  orgId: string;
  employeeId: string;
  employeeName?: string;
  currentGroupIds?: string[];
  onsaved?: () => void;
}

let {
  open = $bindable(false),
  orgId,
  employeeId,
  employeeName = "",
  currentGroupIds = [],
  onsaved,
}: Props = $props();

const assignMutation = createAssignEmployeeGroupsMutation();
let selectedGroupIds = $state<string[]>([]);

$effect(() => {
  if (open) {
    selectedGroupIds = [...currentGroupIds];
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!employeeId) return;

  try {
    await assignMutation.mutateAsync({
      id: employeeId,
      groupIds: selectedGroupIds,
    });

    toast.success(l("تم تحديث مجموعات الموظف بنجاح", "Groups updated successfully"));
    open = false;
    onsaved?.();
  } catch (err) {
    toast.error(extractErrorMessage(err));
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
            {l("إسناد مجموعات المستخدمين", "Manage User Groups")}
          </SheetTitle>
        </div>
        <SheetDescription class="text-xs text-muted-foreground pt-1">
          {employeeName
            ? l(`تعديل مجموعات وصلاحيات الموظف (${employeeName})`, `Assign user groups to ${employeeName}`)
            : l("حدد المجموعات لمنح الصلاحيات والأدوار المناسبة.", "Select user groups to grant permissions.")}
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
          disabled={assignMutation.isPending}
          class="text-xs h-8 cursor-pointer"
        >
          {m.common_action_cancel?.() || "إلغاء"}
        </Button>
        <Button
          type="submit"
          variant="lime"
          class="font-semibold px-6 text-xs h-8 cursor-pointer"
          disabled={assignMutation.isPending}
        >
          {#if assignMutation.isPending}
            <ArrowPath size={13} class="animate-spin me-1.5" />
          {/if}
          {l("حفظ التعيينات", "Save Groups")}
        </Button>
      </div>
    </form>
  </SheetContent>
</Sheet>
