<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { ArrowRight, ChartBar, Sparkles } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  CardAction,
} from "@orbis/ui/card";

let {
  orgId,
  isLoading = false,
  metricsData = [],
}: {
  orgId: string;
  isLoading?: boolean;
  metricsData?: { label: string; requests: number; latencyMs: number }[];
} = $props();

const displayData = $derived(
  metricsData.length > 0
    ? metricsData
    : [
        { label: "08:00", requests: 0, latencyMs: 0 },
        { label: "10:00", requests: 0, latencyMs: 0 },
        { label: "12:00", requests: 0, latencyMs: 0 },
        { label: "14:00", requests: 0, latencyMs: 0 },
        { label: "16:00", requests: 0, latencyMs: 0 },
        { label: "18:00", requests: 0, latencyMs: 0 },
      ],
);

const maxVal = $derived(
  Math.max(10, ...displayData.map((d) => d.requests)) * 1.2,
);

const bars = $derived(
  displayData.map((item) => ({
    label: item.label,
    reqPct: Math.min(100, (item.requests / maxVal) * 100),
    latPct: Math.min(100, (item.latencyMs / 500) * 100),
    requests: item.requests,
    latencyMs: item.latencyMs,
  })),
);

let hoveredIdx = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: HTMLDivElement }) {
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;

  const isRtl = document.documentElement.dir === "rtl";
  const colWidth = rect.width / displayData.length;
  const rawIdx = Math.floor(mouseX / colWidth);
  const idx = Math.min(
    displayData.length - 1,
    Math.max(0, isRtl ? displayData.length - 1 - rawIdx : rawIdx),
  );

  hoveredIdx = idx;
  tooltipPos = { x: mouseX, y: mouseY };
}

function handleMouseLeave() {
  hoveredIdx = null;
  tooltipPos = null;
}
</script>

<Card class="font-sans">
  {#if isLoading}
    <CardContent class="p-6 h-64 flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </CardContent>
  {:else}
    <!-- Top Row Header + Legend -->
    <CardHeader>
      <div class="flex items-center gap-2">
        <Sparkles size={14} class="text-primary" />
        <div class="flex flex-col text-start">
          <CardTitle>
            {m.ai_center_overview_trend_title ? m.ai_center_overview_trend_title() : "نشاط وأداء الذكاء الاصطناعي"}
          </CardTitle>
          <CardDescription>
            {m.ai_center_overview_trend_subtitle ? m.ai_center_overview_trend_subtitle() : "معدل سرعة المعالجة والطلبات عبر الوكلاء والمحركات"}
          </CardDescription>
        </div>
      </div>

      <!-- Legends -->
      <CardAction>
        <div class="flex items-center gap-4 text-[10px] text-muted-foreground font-semibold">
          <div class="flex items-center gap-1.5">
            <span class="h-2.5 w-2.5 rounded-sm bg-primary inline-block shadow-sm"></span>
            <span>الطلبات (Requests)</span>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="h-2.5 w-2.5 rounded-sm bg-muted-foreground/40 inline-block shadow-sm"></span>
            <span>زمن الاستجابة (Latency)</span>
          </div>
        </div>
      </CardAction>
    </CardHeader>

    <!-- Chart Visual Area -->
    <CardContent class="p-6 relative">
      <div
        class="relative w-full h-44 select-none flex flex-col justify-between"
        onmouseleave={handleMouseLeave}
        role="region"
        aria-label="مخطط أداء الذكاء الاصطناعي"
      >
        <!-- Grid lines -->
        <div class="absolute inset-0 flex flex-col justify-between pb-8 pt-4 pointer-events-none">
          {#each Array(4) as _, i}
            <div class="flex items-center w-full h-0">
              <span class="w-12 text-end text-[9px] font-bold text-muted-foreground/60 font-mono pe-2">
                {Math.round(Math.max(10, maxVal) - i * (maxVal / 3))}
              </span>
              <div class="grow border-t border-dashed border-border/40"></div>
            </div>
          {/each}
        </div>

        <!-- Bars Container -->
        <div
          class="absolute inset-0 flex items-end justify-between ps-12 pe-2 pb-8 pt-4 cursor-crosshair"
          onmousemove={handleMouseMove}
          role="group"
        >
          {#each bars as bar, idx}
            {@const isActive = hoveredIdx === idx}
            <div class="grow flex flex-col items-center justify-end h-full relative group/col">
              {#if isActive}
                <div class="absolute inset-x-1 top-0 bottom-0 bg-primary/10 rounded-md pointer-events-none border border-primary/20"></div>
              {/if}

              <div class="flex items-end gap-1.5 h-full pb-1 z-10">
                <div
                  class="w-3 sm:w-4 rounded-t-sm bg-primary transition-all duration-150"
                  style="height: {bar.reqPct}%"
                ></div>
                <div
                  class="w-3 sm:w-4 rounded-t-sm bg-muted-foreground/30 transition-all duration-150"
                  style="height: {bar.latPct}%"
                ></div>
              </div>

              <div class="absolute -bottom-5.5 text-[10px] font-bold text-muted-foreground font-mono">
                {bar.label}
              </div>
            </div>
          {/each}
        </div>

        <!-- Tooltip -->
        {#if hoveredIdx !== null && tooltipPos}
          {@const activeBar = bars[hoveredIdx]}
          <div
            class="absolute z-30 p-2.5 text-[10px] border bg-popover backdrop-blur border-border text-popover-foreground rounded-lg shadow-xl pointer-events-none flex flex-col gap-1 min-w-[140px]"
            style="left: {tooltipPos.x + 10}px; top: {tooltipPos.y - 60}px;"
          >
            <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activeBar.label}</span>
            <div class="flex items-center justify-between gap-3">
              <span class="text-muted-foreground">عدد الطلبات:</span>
              <span class="font-bold font-mono text-primary">{activeBar.requests} req</span>
            </div>
            <div class="flex items-center justify-between gap-3">
              <span class="text-muted-foreground">زمن الاستجابة:</span>
              <span class="font-bold font-mono text-foreground">{activeBar.latencyMs} ms</span>
            </div>
          </div>
        {/if}

        <!-- Zero Overlay -->
        {#if displayData.every((d) => d.requests === 0)}
          <div class="absolute inset-0 flex items-center justify-center pointer-events-none z-20">
            <div class="px-3 py-1.5 rounded-lg bg-background/80 backdrop-blur-xs border border-border/60 text-[10px] text-muted-foreground font-medium flex items-center gap-1.5 shadow-xs">
              <ChartBar size={12} class="text-primary" />
              <span>جاهز للاستخدام — يبدأ المخطط بالرسم فور تنفيذ الوكلاء للعمليات والاستفسارات</span>
            </div>
          </div>
        {/if}
      </div>
    </CardContent>

    <!-- Footer link -->
    <CardFooter class="justify-between text-[10px] font-medium">
      <div class="flex items-center gap-2 text-muted-foreground">
        <ChartBar size={12} class="text-primary" />
        <span>مراقبة الأداء والاستجابة الفعلية للنظام.</span>
      </div>
      <a href={localizeHref(`/${orgId}/ai-center/ai-logs`)} class="text-primary hover:underline flex items-center gap-1 group font-semibold">
        <span>عرض سجل الأنشطة الكامل</span>
        <ArrowRight size={11} class="transition-transform group-hover:translate-x-0.5 rtl:rotate-180" />
      </a>
    </CardFooter>
  {/if}
</Card>
