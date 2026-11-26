<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { api } from "$lib/api";
import DragIndicator from "../components/drag-indicator.svelte";

let {
  orgId,
  selectedClass = "all",
  selectedTerm = "all",
  period = "this-year",
  isEditing = false,
}: {
  orgId: string;
  selectedClass?: string;
  selectedTerm?: string;
  period?: string;
  isEditing?: boolean;
} = $props();

let femaleCount = $state(0);
let maleCount = $state(0);
let totalCount = $state(0);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    const params: { class_id?: string; period_id?: string } = {};
    if (selectedClass !== "all") params.class_id = selectedClass;
    if (selectedTerm !== "all") params.period_id = selectedTerm;

    const pageSize = 300;
    const firstPage = await api.listStudents({
      ...params,
      page: 1,
      page_size: pageSize,
    });
    const students = [...firstPage.items];
    for (let page = 2; page <= firstPage.total_pages; page++) {
      const result = await api.listStudents({
        ...params,
        page,
        page_size: firstPage.page_size,
      });
      students.push(...result.items);
    }

    let female = 0;
    let male = 0;
    const femaleLabel = m.common_form_female().toLowerCase();
    const maleLabel = m.common_form_male().toLowerCase();
    for (const student of students) {
      const gender = String(student.person.gender || "")
        .trim()
        .toLowerCase();
      if (gender.startsWith("f") || gender === femaleLabel) {
        female++;
      } else if (gender.startsWith("m") || gender === maleLabel) {
        male++;
      }
    }
    femaleCount = female;
    maleCount = male;
    totalCount = female + male;
  } catch (e) {
    console.error("Error loading gender distribution:", e);
    femaleCount = 0;
    maleCount = 0;
    totalCount = 0;
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

const maxVal = $derived(
  Math.max(10, Math.ceil((Math.max(femaleCount, maleCount) * 1.2) / 10) * 10),
);
const yTicks = $derived([
  0,
  Math.round(maxVal * 0.25),
  Math.round(maxVal * 0.5),
  Math.round(maxVal * 0.75),
  maxVal,
]);

const femalePct = $derived(Math.min(100, (femaleCount / maxVal) * 100));
const malePct = $derived(Math.min(100, (maleCount / maxVal) * 100));

let hoveredBarIdx = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: HTMLDivElement }) {
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;

  const isRtl = document.documentElement.dir === "rtl";
  const isFemaleHovered = isRtl
    ? mouseX > rect.width / 2
    : mouseX < rect.width / 2;
  hoveredBarIdx = isFemaleHovered ? 0 : 1;
  tooltipPos = { x: mouseX, y: mouseY };
}

function handleMouseLeave() {
  hoveredBarIdx = null;
  tooltipPos = null;
}
</script>

<div class="card-tactile h-full relative group min-h-[265px] p-5 flex flex-col justify-between font-sans border border-border/60 bg-card shadow-sm">
  {#if isLoading}
    <div class="grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Header -->
    <div class="flex flex-col w-full text-start select-none">
      <div class="flex items-center justify-between w-full h-5">
        <span class="text-xs text-muted-foreground">{m.common_gender_dist_title()}</span>
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      <div class="text-4xl font-light tracking-tight mt-1 text-foreground">
        {m.reports_gender_dist_total_students({ count: String(totalCount) })}
      </div>
      
      <div class="flex items-center gap-4 text-[10px] text-muted-foreground mt-2 font-medium">
        <div class="flex items-center gap-1.5">
          <span class="h-2 w-2 rounded-full bg-chart-1 inline-block"></span>
          <span>{m.common_form_female()}</span>
        </div>
        <div class="flex items-center gap-1.5">
          <span class="h-2 w-2 rounded-full bg-chart-2 inline-block"></span>
          <span>{m.common_form_male()}</span>
        </div>
      </div>
    </div>

    <!-- Chart -->
    <div 
      role="presentation"
      class="grow relative w-full h-[155px] select-none mt-4 flex flex-col justify-between" 
      onmouseleave={handleMouseLeave}
    >
      <!-- Grid lines -->
      <div class="absolute inset-0 flex flex-col justify-between pb-6 pt-2 pointer-events-none">
        {#each yTicks.slice().reverse() as tickVal}
          <div class="flex items-center w-full h-0">
            <span class="w-8 text-end text-[8px] font-bold text-muted-foreground/80 font-mono pr-2 select-none">
              {tickVal}
            </span>
            <div class="grow border-t border-dashed border-border/40"></div>
          </div>
        {/each}
      </div>

      <!-- Columns Container -->
      <div 
        role="presentation"
        class="absolute inset-0 flex items-end justify-around pl-8 pr-2 pb-6 pt-2 cursor-crosshair"
        onmousemove={handleMouseMove}
      >
        <!-- Female Column group -->
        <div class="flex flex-col items-center justify-end h-full w-24 relative">
          {#if hoveredBarIdx === 0}
            <div class="absolute inset-x-0 top-0 bottom-0 bg-foreground/5 rounded-md pointer-events-none"></div>
          {/if}
          <!-- Bar -->
          <div 
            class="w-10 rounded-t-md bg-chart-1 transition-all duration-300 z-10"
            style="height: {femalePct}%"
            title="Female"
          ></div>
          <!-- Label -->
          <div class="absolute bottom-[-18px] text-[8px] font-bold text-muted-foreground/80 font-mono select-none">
            {m.common_form_female()}
          </div>
        </div>

        <!-- Male Column group -->
        <div class="flex flex-col items-center justify-end h-full w-24 relative">
          {#if hoveredBarIdx === 1}
            <div class="absolute inset-x-0 top-0 bottom-0 bg-foreground/5 rounded-md pointer-events-none"></div>
          {/if}
          <!-- Bar -->
          <div 
            class="w-10 rounded-t-md bg-chart-2 transition-all duration-300 z-10"
            style="height: {malePct}%"
            title="Male"
          ></div>
          <!-- Label -->
          <div class="absolute bottom-[-18px] text-[8px] font-bold text-muted-foreground/80 font-mono select-none">
            {m.common_form_male()}
          </div>
        </div>
      </div>

      <!-- Tooltip -->
      {#if hoveredBarIdx !== null && tooltipPos}
        {@const isFemale = hoveredBarIdx === 0}
        <div
          class="absolute z-30 p-2 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none flex flex-col gap-0.5 min-w-[95px] text-start"
          style="left: {tooltipPos.x + 15}px; top: {tooltipPos.y - 45}px;"
        >
          <span class="text-foreground font-semibold">{isFemale ? m.common_form_female() : m.common_form_male()}</span>
          <span class="text-muted-foreground font-mono">count: {isFemale ? femaleCount : maleCount}</span>
        </div>
      {/if}
    </div>
  {/if}
</div>
