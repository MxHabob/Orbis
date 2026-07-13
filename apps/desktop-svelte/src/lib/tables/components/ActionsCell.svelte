<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { getContext } from "svelte";
import { Button } from "@orbis/ui-svelte/button";
import * as Icons from "lucide-svelte";
import type { GridActionEvent } from "@orbis/svelte-grid";

interface ActionItem {
  id: string;
  icon?: string;
  label?: string;
  variant?:
    | "default"
    | "destructive"
    | "success"
    | "secondary"
    | "outline"
    | "ghost"
    | "link";
  visible?: (row: any) => boolean;
  disabled?: (row: any) => boolean;
}

interface Props {
  row: Record<string, any>;
  column: { id: string; meta?: Record<string, any> };
  api?: any;
}

let { row, column }: Props = $props();

const items: ActionItem[] = $derived(column.meta?.items || []);
const visibleActions = $derived(
  items.filter((item) => !item.visible || item.visible(row)),
);

const parentOnaction = getContext<(event: GridActionEvent) => void>(
  "orbis-grid-onaction",
);

function handleActionClick(actionId: string, e: MouseEvent) {
  if (parentOnaction) {
    parentOnaction({
      actionId,
      row,
      rowIndex: row.__rowIndex ?? 0,
      prop: column.id,
      originalEvent: e,
    });
  }
}

function mapVariant(variant?: string) {
  if (variant === "success") return "lime";
  if (variant === "destructive") return "destructive";
  if (variant === "secondary") return "secondary";
  return "ghost";
}
</script>

<div class="flex items-center gap-1.5 px-2 w-full h-full justify-start">
  {#each visibleActions as action}
    {@const isBtnDisabled = action.disabled?.(row) ?? false}
    {@const btnVariant = mapVariant(action.variant)}
    <Button
      variant={btnVariant}
      size="xs"
      class="h-7 w-7 p-0 cursor-pointer"
      disabled={isBtnDisabled}
      onclick={(e) => handleActionClick(action.id, e)}
      title={action.label}
    >
      {#if action.icon && (Icons as any)[action.icon]}
        {@const IconComp = (Icons as any)[action.icon]}
        <IconComp size={13} />
      {:else}
        <span class="text-[10px]">{action.label || action.id}</span>
      {/if}
    </Button>
  {/each}
</div>
