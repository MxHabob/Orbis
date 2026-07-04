<script lang="ts">
import { onMount } from "svelte";
import type { ChartId, ChartLayoutItem, ColSpan } from "./utils/types";
import { DEFAULT_CHART_LAYOUT } from "./utils/types";
import MetricsSettings from "./components/metrics-settings.svelte";
import MetricsGrid from "./components/metrics-grid.svelte";
import DraggableChartCard from "./components/draggable-chart-card.svelte";

// Cards
import AttendanceTrendsCard from "./cards/attendance-trends-card.svelte";
import GradeDistributionCard from "./cards/grade-distribution-card.svelte";
import FeeCollectionCard from "./cards/fee-collection-card.svelte";
import BehaviorSummaryCard from "./cards/behavior-summary-card.svelte";

let { orgId }: { orgId: string } = $props();

let isEditing = $state(false);
let period = $state("this-year");
let selectedTerm = $state("all");
let selectedClass = $state("all");

let layout = $state<ChartLayoutItem[]>(DEFAULT_CHART_LAYOUT);

onMount(() => {
  const saved = localStorage.getItem("orbis_reports_chart_layout");
  if (saved) {
    try {
      layout = JSON.parse(saved);
    } catch (e) {
      console.error("Error parsing saved layout:", e);
    }
  }
});

function handleResizeChart(id: ChartId, newColSpan: ColSpan) {
  layout = layout.map((item) =>
    item.id === id ? { ...item, colSpan: newColSpan } : item,
  );
  localStorage.setItem("orbis_reports_chart_layout", JSON.stringify(layout));
}
</script>

<div>
	<MetricsSettings
		{orgId}
		bind:isEditing
		bind:period
		bind:selectedTerm
		bind:selectedClass
	/>

	<MetricsGrid bind:layout={layout} {isEditing}>
		{#snippet children(chartId, colSpan)}
			<DraggableChartCard
				id={chartId}
				{isEditing}
				onResize={(newSpan) => handleResizeChart(chartId, newSpan)}
			>
				{#if chartId === "attendance-trends"}
					<AttendanceTrendsCard
						{orgId}
						{isEditing}
						{period}
						{selectedClass}
						{selectedTerm}
					/>
				{:else if chartId === "grade-distribution"}
					<GradeDistributionCard
						{orgId}
						{isEditing}
						{period}
						{selectedClass}
						{selectedTerm}
					/>
				{:else if chartId === "fee-collection"}
					<FeeCollectionCard
						{orgId}
						{isEditing}
						{period}
						{selectedClass}
						{selectedTerm}
					/>
				{:else if chartId === "behavior-analysis"}
					<BehaviorSummaryCard
						{orgId}
						{isEditing}
						{period}
						{selectedClass}
						{selectedTerm}
					/>
				{/if}
			</DraggableChartCard>
		{/snippet}
	</MetricsGrid>
</div>
