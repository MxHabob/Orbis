<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { toast } from "@orbis/ui";
  import { Button } from "@orbis/ui/button";
  import { confirmStore } from "$lib/stores/confirm.svelte";
  import { Trash } from "@orbis/icons";

  let {
    label,
    onDelete,
  }: {
    label: string;
    onDelete: () => Promise<void>;
  } = $props();

  let isDeleting = $state(false);

  async function confirmDelete() {
    if (isDeleting) return;
    isDeleting = true;
    try {
      const confirmed = await confirmStore.prompt({
        title: `${m.common_action_delete()} — ${label}`,
        description: m.common_confirm_delete_record(),
        confirmText: m.common_action_delete(),
        cancelText: m.common_action_cancel(),
        variant: "destructive",
      });
      if (!confirmed) return;
      await onDelete();
    } catch (error) {
      console.error("Failed to delete onboarding record:", error);
      toast.error(error instanceof Error ? error.message : m.common_action_failed());
    } finally {
      isDeleting = false;
    }
  }
</script>

<Button
  variant="ghost"
  size="icon-xs"
  aria-label={m.common_action_delete()}
  title={m.common_action_delete()}
  disabled={isDeleting}
  onclick={confirmDelete}
>
  <Trash class="size-3.5 text-destructive" />
</Button>
