<script lang="ts">
/**
 * SideRail — Canva-style vertical icon rail.
 * A thin (56px) sidebar with icon-only buttons.
 * Each icon toggles a FlyoutPanel open/closed.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import * as m from "$lib/paraglide/messages";
import {
  LayoutTemplate,
  Shapes,
  Type,
  ImageIcon,
  Wrench,
  Settings2,
} from "lucide-svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

type RailItem = {
  id: string;
  icon: any;
  label: string;
  shortcut?: string;
};

const topItems: RailItem[] = [
  { id: m.template_auto_a4858850() },
  { id: m.template_auto_01f881e3() },
  { id: m.template_auto_c228ceda() },
  { id: m.template_auto_a29f8c2b() },
  { id: m.template_auto_21c1999e() },
  { id: m.template_auto_334f4b45() },
];
</script>

<TooltipProvider delayDuration={200}>
<nav class="side-rail" aria-label="Editor panels">
  <div class="rail-items">
    {#each topItems as item (item.id)}
      {@const isActive = store.activeRailPanel === item.id}
      <Tooltip>
        <TooltipTrigger>
          <button
            type="button"
            class="rail-btn"
            class:active={isActive}
            onclick={() => store.toggleRailPanel(item.id)}
            aria-pressed={isActive}
          >
            <svelte:component this={item.icon} size={20} strokeWidth={isActive ? 2.2 : 1.6} />
            <span class="rail-label">{item.label}</span>
          </button>
        </TooltipTrigger>
        <TooltipContent side="left" sideOffset={12}>{item.label}</TooltipContent>
      </Tooltip>
    {/each}
  </div>
</nav>
</TooltipProvider>

<style>
  .side-rail {
    width: 56px;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-card);
    border-inline-end: 1px solid color-mix(in srgb, var(--color-border) 40%, transparent);
    flex-shrink: 0;
    user-select: none;
    z-index: 20;
  }

  .rail-items {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: 8px 4px;
    flex-grow: 1;
  }

  .rail-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;
    width: 48px;
    height: 48px;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: var(--color-muted-foreground);
    cursor: pointer;
    transition: all 0.15s ease;
    position: relative;
  }

  .rail-btn:hover {
    background: color-mix(in srgb, var(--color-accent) 60%, transparent);
    color: var(--color-foreground);
  }

  .rail-btn.active {
    background: var(--color-primary);
    color: var(--color-primary-foreground);
  }

  .rail-btn.active:hover {
    background: color-mix(in srgb, var(--color-primary) 85%, black);
  }

  .rail-label {
    font-size: 8px;
    font-weight: 500;
    line-height: 1;
    white-space: nowrap;
  }
</style>
