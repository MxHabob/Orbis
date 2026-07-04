<!--
@component ActionOverlay
A reusable dropdown menu portal positioned dynamically on demand.
Used to render cell actions overflow menu without mounting Svelte components inside every cell.
-->
<script lang="ts">
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@orbis/ui-svelte/dropdown-menu";
import type { ActionItem, GridActionEvent } from "@orbis/data-grid";

interface Props {
  /** Complete list of possible menu actions */
  actions: ActionItem[];
  /** Dispatched when an action is selected from the menu */
  onselect?: (event: GridActionEvent) => void;
}

let { actions, onselect }: Props = $props();

let open = $state(false);
let x = $state(0);
let y = $state(0);

let activeRow = $state<Record<string, any>>({});
let activeRowIndex = $state(-1);
let activeProp = $state("");
let originalEvent = $state<MouseEvent | null>(null);

// Filter actions based on conditional visibility for the active row
const visibleActions = $derived(
  actions.filter((action) => !action.visible || action.visible(activeRow)),
);

/**
 * Public trigger method to show the dropdown menu near the click target.
 */
export function show(event: GridActionEvent) {
  activeRow = event.row;
  activeRowIndex = event.rowIndex;
  activeProp = event.prop;
  originalEvent = event.originalEvent;

  const clickTarget = event.originalEvent.target as HTMLElement;
  if (clickTarget) {
    const rect = clickTarget.getBoundingClientRect();
    // Position dropdown directly below the clicked ellipsis trigger button
    x = rect.left + window.scrollX;
    y = rect.bottom + window.scrollY;
    open = true;
  }
}

function handleSelect(action: ActionItem) {
  open = false;
  if (onselect && originalEvent) {
    onselect({
      actionId: action.id,
      row: activeRow,
      rowIndex: activeRowIndex,
      prop: activeProp,
      originalEvent,
    });
  }
}
</script>

{#if open}
  <div
    class="fixed z-50 pointer-events-none"
    style="left: {x}px; top: {y}px;"
  >
    <DropdownMenu bind:open>
      <DropdownMenuTrigger class="sr-only">Open Menu</DropdownMenuTrigger>
      <DropdownMenuContent class="pointer-events-auto min-w-[120px] bg-popover text-popover-foreground border border-border rounded-md shadow-md p-1">
        {#each visibleActions as action}
          <DropdownMenuItem
            disabled={action.disabled?.(activeRow)}
            class="flex items-center w-full px-2 py-1.5 text-xs rounded-sm hover:bg-accent hover:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none cursor-pointer {action.variant === 'destructive' ? 'text-destructive focus:bg-destructive/10' : ''}"
            onclick={() => handleSelect(action)}
          >
            {action.label ?? action.id}
          </DropdownMenuItem>
        {/each}
      </DropdownMenuContent>
    </DropdownMenu>
  </div>
{/if}
