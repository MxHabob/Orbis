<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { isDesktopApp } from "$lib/api";
import { invokeDesktopCommand } from "$lib/api/desktop-command-performance";
import { Camera, CheckBox, Expand, Grid, Sparkles } from "@orbis/icons";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";
import WebRtcWhepPlayer from "./components/webrtc-whep-player.svelte";

let { orgId }: { orgId: string } = $props();

// ── State ───────────────────────────────────────────────────
type GridLayout = "1x1" | "2x2" | "3x3";
let gridLayout = $state<GridLayout>("2x2");
let selectedIdx = $state<number | null>(null);
let isLoading = $state(true);
let loadError = $state(false);

interface CameraFeed {
  id: string;
  name: string;
  status: "online" | "error";
  location: string;
  whepUrl: string | null;
}

let cameras = $state<CameraFeed[]>([]);

onMount(() => {
  let disposed = false;
  const startStreams = async () => {
    try {
      if (!isDesktopApp) {
        throw new Error("Local camera streaming is available in Orbis Desktop.");
      }
      const response = await invokeDesktopCommand<{ data: CameraFeed[] }>(
        "camera_stream_start",
      );
      if (!disposed) cameras = response.data;
    } catch (error) {
      console.error("[AI Center] Local camera relay could not be started:", error);
      if (!disposed) loadError = true;
    } finally {
      if (!disposed) isLoading = false;
    }
  };
  const startTask = startStreams();

  return () => {
    disposed = true;
    void startTask.then(async () => {
      try {
        await invokeDesktopCommand("camera_stream_stop");
      } catch (error) {
        console.error("[AI Center] Local camera relay could not be stopped:", error);
      }
    });
  };
});

const gridCols: Record<GridLayout, string> = {
  "1x1": "grid-cols-1",
  "2x2": "grid-cols-2",
  "3x3": "grid-cols-3",
};
const gridSlots: Record<GridLayout, number> = { "1x1": 1, "2x2": 4, "3x3": 9 };
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
      ] as opt (opt.l)}
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
    {#if isLoading}
      <div class="flex flex-1 items-center justify-center">
        <p class="text-sm text-muted-foreground">{m.ai_ui_loading_cameras()}</p>
      </div>
    {:else if loadError}
      <div class="flex flex-1 items-center justify-center p-8 text-center">
        <p class="max-w-md text-sm text-muted-foreground">
          {m.ai_ui_camera_stream_unavailable()}
        </p>
      </div>
    {:else if cameras.length > 0}
      <!-- Camera Grid -->
      <div class="flex-1 p-4 overflow-hidden">
        <div class="h-full grid {gridCols[gridLayout]} gap-3">
          {#each Array(gridSlots[gridLayout]) as _, i (i)}
            {@const cam = cameras[i]}
            {@const streamActive = selectedIdx === null
              ? i < Math.min(gridSlots[gridLayout], 4)
              : selectedIdx === i}
            <div
              role="button"
              tabindex="0"
              class="relative bg-card rounded-xl overflow-hidden flex items-center justify-center
                     border-2 transition-all cursor-pointer shadow-inner w-full h-full text-start
                     {selectedIdx === i ? 'border-primary ring-2 ring-primary/20' : 'border-border/30 hover:border-border'}"
              onclick={() => {
                if (cam) selectedIdx = selectedIdx === i ? null : i;
              }}
              onkeydown={(event) => {
                if (event.key === "Enter" || event.key === " ") {
                  event.preventDefault();
                  if (cam) selectedIdx = selectedIdx === i ? null : i;
                }
              }}
            >
              {#if cam}
                {#if cam.status === "online" && cam.whepUrl && streamActive}
                  <WebRtcWhepPlayer
                    whepUrl={cam.whepUrl}
                    connectingLabel={m.ai_ui_camera_stream_connecting()}
                    unavailableLabel={m.ai_ui_camera_stream_unavailable()}
                  />
                {:else if cam.status === "online"}
                  <div class="absolute inset-0 flex flex-col items-center justify-center gap-2 bg-black/90 p-4 text-center">
                    <Camera size={32} class="text-white/50" />
                    <p class="text-xs font-semibold text-white/80">{cam.name}</p>
                    <p class="text-[10px] text-white/60">
                      {streamActive
                        ? m.ai_ui_camera_stream_unavailable()
                        : m.ai_ui_camera_stream_paused()}
                    </p>
                  </div>
                {/if}
                {#if cam.status === "online"}
                  <div class="absolute bottom-0 left-0 right-0 bg-background/90 p-3 border-t border-border/40">
                    <div class="flex items-center justify-between">
                      <div>
                        <p class="text-xs font-semibold text-foreground">{cam.name}</p>
                        <p class="text-[10px] text-muted-foreground">{cam.location}</p>
                      </div>
                      <div class="flex items-center gap-1.5 bg-primary/10 border border-primary/20 px-2 py-0.5 rounded-full">
                        <span class="w-1.5 h-1.5 rounded-full {streamActive ? 'bg-primary animate-pulse' : 'bg-muted-foreground'}"></span>
                        <span class="text-[9px] text-primary font-semibold tracking-wider">{streamActive ? m.ai_ui_live() : m.ai_ui_camera_stream_paused()}</span>
                      </div>
                    </div>
                  </div>
                  <button type="button" aria-label={m.template_tooltip_fullscreen()} title={m.template_tooltip_fullscreen()} onclick={(event) => {
                    event.stopPropagation();
                    const video = event.currentTarget.parentElement?.querySelector("video");
                    if (video) void video.requestFullscreen();
                  }} class="absolute top-2.5 right-2.5 p-1.5 bg-background/80 border border-border/40 rounded-md text-foreground hover:text-primary transition-colors cursor-pointer">
                    <Expand size={12} />
                  </button>
                {:else}
                  <div class="text-center p-4">
                    <Camera size={28} class="text-muted-foreground mx-auto mb-2" />
                    <p class="text-xs font-semibold text-muted-foreground">{cam.name}</p>
                    <p class="text-[10px] text-muted-foreground font-semibold mt-1">{m.ai_ui_connection_error()}</p>
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
