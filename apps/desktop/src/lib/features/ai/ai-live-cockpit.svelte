<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import {
  AppNotification,
  Bolt,
  Camera,
  CheckBox,
  ClockRotateRight,
  Cloud,
  Expand,
  Grid,
  InfoCircle,
  Sparkles,
} from "@orbis/icons";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";

let { orgId }: { orgId: string } = $props();

// ── State ───────────────────────────────────────────────────
type GridLayout = "1x1" | "2x2" | "3x3";
let gridLayout = $state<GridLayout>("2x2");
let selectedIdx = $state<number | null>(null);
let isLoading = $state(true);

interface CameraFeed {
  id: string;
  name: string;
  ip: string;
  status: "online" | "offline" | "error";
  location: string;
}

let cameras = $state<CameraFeed[]>([]);

interface CameraEvent {
  id: string;
  camera_name: string;
  type: "motion" | "alarm" | "disconnect" | "reconnect";
  timestamp: string;
}

let events = $state<CameraEvent[]>([]);

onMount(async () => {
  try {
    const status = await api
      .getIntegrationStatus("hikvision-camera")
      .catch(() => null);
    if (status && (status as any).cameras) {
      cameras = (status as any).cameras;
    } else if (
      status?.config?.settings &&
      (status.config.settings as any)?.cameras
    ) {
      cameras = (status.config.settings as any).cameras;
    }
  } catch (e) {
    console.warn("[AI Center] Camera status not available:", e);
  } finally {
    isLoading = false;
  }
});

const gridCols: Record<GridLayout, string> = {
  "1x1": "grid-cols-1",
  "2x2": "grid-cols-2",
  "3x3": "grid-cols-3",
};
const gridSlots: Record<GridLayout, number> = { "1x1": 1, "2x2": 4, "3x3": 9 };

const eventStyles: Record<
  string,
  { color: string; icon: typeof AppNotification }
> = {
  motion: { color: "text-muted-foreground", icon: AppNotification },
  alarm: { color: "text-foreground font-semibold", icon: InfoCircle },
  disconnect: { color: "text-muted-foreground", icon: Cloud },
  reconnect: { color: "text-primary", icon: Bolt },
};
</script>

<div class="h-full flex flex-col overflow-hidden font-sans">
  <!-- Top Control Bar (Midday Style) -->
  <div class="px-6 py-3 shrink-0 border-b border-border/40 bg-background flex items-center justify-between">
    <div class="flex items-center gap-2">
      <Sparkles size={15} class="text-primary animate-pulse" />
      <span class="text-xs text-muted-foreground font-semibold">
        {cameras.length > 0
          ? m.ai_ui_connected_camera_count({ online: cameras.filter(c => c.status === "online").length, total: cameras.length })
          : (m.ai_center_live_no_cameras())}
      </span>
    </div>

    <!-- Grid Selector -->
    <div class="flex items-center gap-1 border border-border/60 bg-muted/40 p-1 rounded-lg">
      {#each [
        { l: "1x1" as GridLayout, icon: CheckBox, label: m.ai_center_live_grid_1x1() },
        { l: "2x2" as GridLayout, icon: Grid, label: m.ai_center_live_grid_2x2() },
        { l: "3x3" as GridLayout, icon: Grid, label: m.ai_center_live_grid_3x3() },
      ] as opt}
        <Button
          type="button"
          size="sm"
          variant={gridLayout === opt.l ? "default" : "ghost"}
          class="h-7 text-xs font-semibold px-2.5 cursor-pointer"
          onclick={() => { gridLayout = opt.l; selectedIdx = null; }}
          title={opt.label}
        >
          <opt.icon size={13} class="ml-1" />
          <span class="hidden md:inline">{opt.l}</span>
        </Button>
      {/each}
    </div>
  </div>

  <!-- Content Main Split View -->
  <div class="flex-1 flex overflow-hidden">
    {#if cameras.length > 0}
      <!-- Camera Grid -->
      <div class="flex-1 p-4 overflow-hidden">
        <div class="h-full grid {gridCols[gridLayout]} gap-3">
          {#each Array(gridSlots[gridLayout]) as _, i}
            {@const cam = cameras[i]}
            <div
              role="button"
              tabindex="0"
              class="relative bg-card rounded-xl overflow-hidden flex items-center justify-center
                     border-2 transition-all cursor-pointer shadow-inner w-full h-full text-start
                     {selectedIdx === i ? 'border-primary ring-2 ring-primary/20' : 'border-border/30 hover:border-border'}"
              onclick={() => selectedIdx = i}
              onkeydown={(event) => {
                if (event.key === "Enter" || event.key === " ") selectedIdx = i;
              }}
            >
              {#if cam}
                {#if cam.status === "online"}
                  <div class="absolute inset-0 flex items-center justify-center opacity-40">
                    <Camera size={40} class="text-muted-foreground" />
                  </div>
                  <div class="absolute bottom-0 left-0 right-0 bg-background/90 p-3 border-t border-border/40">
                    <div class="flex items-center justify-between">
                      <div>
                        <p class="text-xs font-semibold text-foreground">{cam.name}</p>
                        <p class="text-[10px] text-muted-foreground">{cam.location}</p>
                      </div>
                      <div class="flex items-center gap-1.5 bg-primary/10 border border-primary/20 px-2 py-0.5 rounded-full">
                        <span class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></span>
                        <span class="text-[9px] text-primary font-semibold tracking-wider">{m.ai_ui_live()}</span>
                      </div>
                    </div>
                  </div>
                  <button type="button" onclick={(event) => event.stopPropagation()} class="absolute top-2.5 right-2.5 p-1.5 bg-background/80 border border-border/40 rounded-md text-foreground hover:text-primary transition-colors cursor-pointer">
                    <Expand size={12} />
                  </button>
                {:else}
                  <div class="text-center p-4">
                    <Camera size={28} class="text-muted-foreground mx-auto mb-2" />
                    <p class="text-xs font-semibold text-muted-foreground">{cam.name}</p>
                    <p class="text-[10px] text-muted-foreground font-semibold mt-1">{cam.status === "error" ? m.ai_ui_connection_error() : m.ai_ui_not_connected()}</p>
                  </div>
                {/if}
              {:else}
                <div class="text-center">
                  <div class="w-10 h-10 rounded-full border border-dashed border-border flex items-center justify-center mx-auto mb-1.5">
                    <Camera size={16} class="text-muted-foreground" />
                  </div>
                  <p class="text-[10px] text-muted-foreground">{m.ai_ui_empty_slot()}</p>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>

      <!-- Events Feed Sidebar -->
      <div class="w-72 border-r border-border/40 bg-background flex flex-col shrink-0">
        <div class="px-4 py-3 border-b border-border/40 bg-background flex items-center justify-between">
          <h3 class="text-xs font-semibold text-foreground flex items-center gap-2">
            <AppNotification size={14} class="text-primary" />
            <span>{m.ai_center_live_events()}</span>
          </h3>
          <span class="text-[10px] font-mono text-muted-foreground">{m.ai_ui_alert_count({ count: events.length })}</span>
        </div>

        <div class="flex-1 overflow-y-auto divide-y divide-border/20">
          {#if events.length > 0}
            {#each events as event (event.id)}
              {@const s = eventStyles[event.type]}
              {@const EventIcon = s.icon}
              <div class="px-4 py-3 hover:bg-muted/30 transition-colors">
                <div class="flex items-start gap-2.5">
                  <EventIcon size={14} class="{s.color} mt-0.5 shrink-0" />
                  <div>
                    <p class="text-xs font-semibold text-foreground">{event.camera_name}</p>
                    <p class="text-[10px] text-muted-foreground font-mono mt-0.5">{event.timestamp}</p>
                  </div>
                </div>
              </div>
            {/each}
          {:else}
            <div class="flex flex-col items-center justify-center py-16 text-center px-4">
              <ClockRotateRight size={18} class="text-muted-foreground mb-2" />
              <p class="text-xs font-semibold text-foreground">{m.ai_ui_no_live_events()}</p>
              <p class="text-[10px] text-muted-foreground mt-0.5">{m.ai_ui_live_events_empty_description()}</p>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <!-- Interactive AI Empty State: No Cameras Connected -->
      <div class="flex-1 flex items-center justify-center p-8">
        <AiEmptyState
          text={m.ai_ui_no_cameras_empty_state()}
          actionLabel={m.ai_ui_connect_first_camera()}
          onaction={() => goto(`/${orgId}/ai-center/integrations`)}
          hat="random"
        />
      </div>
    {/if}
  </div>
</div>
