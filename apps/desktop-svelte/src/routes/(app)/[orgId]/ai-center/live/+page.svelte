<!--
@component LivePage
AI Center Live — Camera grid with live feeds.
Wired to SDK: commands.aiCenter.getIntegrationStatus('hikvision-camera')
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Video,
  VideoOff,
  Maximize2,
  Grid2x2,
  Grid3x3,
  Square,
  Wifi,
  WifiOff,
  Bell,
  Camera,
  AlertTriangle,
  Clock,
} from "lucide-svelte";
import { Button } from "@orbis/ui-svelte/button";

let { data }: { data: { orgId: string } } = $props();

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

// ── Load Camera Status ──────────────────────────────────────
onMount(async () => {
  try {
    const status = await commands.aiCenter.getIntegrationStatus("hikvision-camera");
    if (status && (status as any).cameras) {
      cameras = (status as any).cameras;
    }
  } catch (e) {
    console.warn("[AI Center] Camera status not available:", e);
  } finally {
    isLoading = false;
  }
});

// ── Layout helpers ──────────────────────────────────────────
const gridCols: Record<GridLayout, string> = { "1x1": "grid-cols-1", "2x2": "grid-cols-2", "3x3": "grid-cols-3" };
const gridSlots: Record<GridLayout, number> = { "1x1": 1, "2x2": 4, "3x3": 9 };

const eventStyles: Record<string, { color: string; icon: typeof Bell }> = {
  motion: { color: "text-amber-500", icon: Bell },
  alarm: { color: "text-red-500", icon: AlertTriangle },
  disconnect: { color: "text-red-500", icon: WifiOff },
  reconnect: { color: "text-emerald-500", icon: Wifi },
};
</script>

<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="px-6 pt-5 pb-3 shrink-0 border-b border-border/40">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-lg font-semibold text-foreground mb-1">Live</h1>
        <p class="text-[11px] text-muted-foreground">
          {cameras.length > 0
            ? `${cameras.filter(c => c.status === "online").length} of ${cameras.length} cameras online`
            : "No cameras connected"}
        </p>
      </div>
      <div class="flex border border-border/60 rounded-md overflow-hidden">
        {#each [
          { l: "1x1" as GridLayout, icon: Square },
          { l: "2x2" as GridLayout, icon: Grid2x2 },
          { l: "3x3" as GridLayout, icon: Grid3x3 },
        ] as opt}
          <button
            class="px-2.5 py-1.5 transition-colors cursor-pointer
              {gridLayout === opt.l ? 'bg-foreground text-background' : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => { gridLayout = opt.l; selectedIdx = null; }}
          >
            <opt.icon size={14} />
          </button>
        {/each}
      </div>
    </div>
  </div>

  <!-- Main -->
  <div class="flex-1 flex overflow-hidden">
    {#if cameras.length > 0}
      <!-- Camera Grid -->
      <div class="flex-1 p-3 overflow-hidden">
        <div class="h-full grid {gridCols[gridLayout]} gap-2">
          {#each Array(gridSlots[gridLayout]) as _, i}
            {@const cam = cameras[i]}
            <div
              class="relative bg-zinc-900 rounded-lg overflow-hidden flex items-center justify-center
                     border-2 transition-colors cursor-pointer
                     {selectedIdx === i ? 'border-primary' : 'border-transparent hover:border-border'}"
              onclick={() => selectedIdx = i}
              role="button"
              tabindex="0"
            >
              {#if cam}
                {#if cam.status === "online"}
                  <div class="absolute inset-0 flex items-center justify-center">
                    <Camera size={32} class="text-zinc-500" />
                  </div>
                  <div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-3">
                    <div class="flex items-center justify-between">
                      <div>
                        <p class="text-xs font-medium text-white">{cam.name}</p>
                        <p class="text-[10px] text-zinc-400">{cam.location}</p>
                      </div>
                      <div class="flex items-center gap-1">
                        <span class="w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse"></span>
                        <span class="text-[10px] text-red-400 font-medium">LIVE</span>
                      </div>
                    </div>
                  </div>
                  <button class="absolute top-2 right-2 p-1.5 bg-black/50 rounded hover:bg-black/70 cursor-pointer">
                    <Maximize2 size={12} class="text-white" />
                  </button>
                {:else}
                  <div class="text-center">
                    <VideoOff size={28} class="text-zinc-600 mx-auto mb-2" />
                    <p class="text-xs text-zinc-500">{cam.name}</p>
                    <p class="text-[10px] text-red-400 mt-1">{cam.status === "error" ? "Error" : "Offline"}</p>
                  </div>
                {/if}
              {:else}
                <div class="text-center">
                  <div class="w-10 h-10 rounded-full border-2 border-dashed border-zinc-700 flex items-center justify-center mx-auto mb-2">
                    <Video size={16} class="text-zinc-600" />
                  </div>
                  <p class="text-[10px] text-zinc-600">Empty slot</p>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>

      <!-- Events Sidebar -->
      <div class="w-64 border-l border-border/40 flex flex-col shrink-0">
        <div class="px-4 py-2.5 border-b border-border/40">
          <h3 class="text-xs font-medium text-foreground flex items-center gap-1.5"><Bell size={12} /> Events</h3>
        </div>
        <div class="flex-1 overflow-y-auto">
          {#if events.length > 0}
            {#each events as event (event.id)}
              {@const s = eventStyles[event.type]}
              <div class="px-4 py-2.5 border-b border-border/20 hover:bg-muted/30">
                <div class="flex items-start gap-2">
                  <svelte:component this={s.icon} size={11} class="{s.color} mt-0.5 shrink-0" />
                  <div>
                    <p class="text-[11px] text-foreground">{event.camera_name}</p>
                    <p class="text-[9px] text-muted-foreground">{event.timestamp}</p>
                  </div>
                </div>
              </div>
            {/each}
          {:else}
            <div class="flex flex-col items-center justify-center py-12">
              <Clock size={14} class="text-muted-foreground mb-1.5" />
              <p class="text-[10px] text-muted-foreground">No events</p>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <!-- No Cameras -->
      <div class="flex-1 flex items-center justify-center">
        <div class="text-center max-w-sm">
          <div class="w-14 h-14 rounded-full bg-muted flex items-center justify-center mx-auto mb-4">
            <Video size={24} class="text-muted-foreground" />
          </div>
          <h2 class="text-sm font-medium text-foreground mb-1.5">No cameras connected</h2>
          <p class="text-xs text-muted-foreground mb-5 leading-relaxed">
            Connect a surveillance device in Integrations to view live feeds.
          </p>
          <Button size="sm" href="/{data.orgId}/ai-center/integrations/hikvision-camera">
            <Camera size={13} class="mr-1.5" /> Connect Camera
          </Button>
        </div>
      </div>
    {/if}
  </div>
</div>
