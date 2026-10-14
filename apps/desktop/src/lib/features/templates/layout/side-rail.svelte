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
} from "@orbis/ui/tooltip";
import * as m from "$lib/paraglide/messages";
import { Bold, Component, DocumentPage, Grid, H1, Photo, SettingsAlt, Wrench } from "@orbis/icons";

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
  { id: "templates", icon: DocumentPage, label: m.common_td_panel_pages() },
  { id: "elements", icon: Component, label: m.common_td_panel_elements() },
  { id: "text", icon: H1, label: m.common_td_panel_text() },
  { id: "uploads", icon: Photo, label: m.common_td_panel_media() },
  { id: "tools", icon: Wrench, label: m.common_td_panel_tools() },
  { id: "page-setup", icon: SettingsAlt, label: m.common_td_panel_settings() },
];
</script>

<TooltipProvider delayDuration={200}>
<nav class="side-rail" aria-label={m.shared_ui_editor_panels()}>
  <div class="rail-items">
    {#each topItems as item (item.id)}
      {@const isActive = store.activeRailPanel === item.id}
      {@const RailIcon = item.icon}
      <Tooltip>
        <TooltipTrigger>
          <button
            type="button"
            class="rail-btn"
            class:active={isActive}
            onclick={() => store.toggleRailPanel(item.id)}
            aria-pressed={isActive}
          >
            <RailIcon size={20} strokeWidth={isActive ? 2.2 : 1.6} />
            <span class="rail-label">{item.label}</span>
          </button>
        </TooltipTrigger>
        <TooltipContent side="inline-end" sideOffset={12}>{item.label}</TooltipContent>
      </Tooltip>
    {/each}
  </div>
</nav>
</TooltipProvider>

<style>
  .side-rail {
    width: 64px;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-card);
    border-inline-end: 1px solid var(--color-border);
    flex-shrink: 0;
    user-select: none;
    z-index: 20;
  }

  .rail-items {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;
    padding: 12px 6px;
    flex-grow: 1;
  }

  .rail-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 5px;
    width: 52px;
    height: 52px;
    border: none;
    border-radius: 11px;
    background: transparent;
    color: var(--color-muted-foreground);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
    position: relative;
  }

  .rail-btn:hover {
    background: color-mix(in srgb, var(--color-muted) 68%, transparent);
    color: var(--color-foreground);
  }

  .rail-btn.active {
    background: color-mix(in srgb, var(--color-primary) 10%, var(--color-card));
    color: var(--color-primary);
    box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--color-primary) 18%, transparent);
  }

  .rail-label {
    max-width: 100%;
    overflow: hidden;
    font-size: 9px;
    font-weight: 550;
    line-height: 1.1;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
