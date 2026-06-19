<script lang="ts">
	import { Skeleton } from "@orbis/ui-svelte";
	import { onMount } from "svelte";
	import { commands } from "$lib/services/ipc";
	import DragIndicator from "../components/drag-indicator.svelte";

	let {
		orgId,
		selectedClass = "all",
		selectedTerm = "all",
		period = "this-year",
		isEditing = false
	}: {
		orgId: string;
		selectedClass: string;
		selectedTerm: string;
		period: string;
		isEditing?: boolean;
	} = $props();

	let stats = $state<any>(null);
	let isLoading = $state(true);

	async function loadData() {
		isLoading = true;
		try {
			const params: any = {};
			if (selectedClass !== "all") params.class_id = selectedClass;
			if (selectedTerm !== "all") params.cycle_id = selectedTerm;

			const res = await commands.grades.getAnalytics(orgId, params);
			if (res && res.grade_distribution) {
				stats = res;
			} else {
				// Fallback mock data if none exists in database
				stats = {
					class_average: 78.4,
					pass_rate: 94.2,
					grade_distribution: {
						a_plus: 8,
						a: 14,
						b_plus: 22,
						b: 18,
						c_plus: 15,
						c: 9,
						d_plus: 6,
						d: 4,
						f: 2
					}
				};
			}
		} catch (e) {
			console.error("Error loading grade analytics:", e);
		} finally {
			isLoading = false;
		}
	}

	$effect(() => {
		if (orgId || selectedClass || selectedTerm || period) {
			loadData();
		}
	});

	// Map distribution to an array of items
	const distributionArray = $derived.by(() => {
		if (!stats?.grade_distribution) return [];
		const dist = stats.grade_distribution;
		return [
			{ name: "A+", value: dist.a_plus ?? 0 },
			{ name: "A", value: dist.a ?? 0 },
			{ name: "B+", value: dist.b_plus ?? 0 },
			{ name: "B", value: dist.b ?? 0 },
			{ name: "C+", value: dist.c_plus ?? 0 },
			{ name: "C", value: dist.c ?? 0 },
			{ name: "D+", value: dist.d_plus ?? 0 },
			{ name: "D", value: dist.d ?? 0 },
			{ name: "F", value: dist.f ?? 0 }
		];
	});

	// SVG calculation
	const width = 500;
	const height = 200;
	const padding = { left: 30, right: 10, top: 25, bottom: 25 };

	const maxValue = $derived(Math.max(1, ...distributionArray.map((d) => d.value)));
	const chartWidth = $derived(width - padding.left - padding.right);
	const chartHeight = $derived(height - padding.top - padding.bottom);
	const barWidth = $derived(Math.max(5, (chartWidth / distributionArray.length) - 12));

	const bars = $derived.by(() => {
		return distributionArray.map((item, idx) => {
			const xStep = chartWidth / distributionArray.length;
			const x = padding.left + idx * xStep + (xStep - barWidth) / 2;
			const pct = item.value / maxValue;
			const h = pct * chartHeight;
			const y = height - padding.bottom - h;
			return { x, y, width: barWidth, height: h, item };
		});
	});

	// Interactive Hover State
	let hoveredIndex = $state<number | null>(null);
	let tooltipPos = $state<{ x: number; y: number } | null>(null);

	function handleMouseMove(e: MouseEvent & { currentTarget: SVGSVGElement }) {
		if (bars.length === 0) return;
		const rect = e.currentTarget.getBoundingClientRect();
		const containerRect = e.currentTarget.parentElement!.getBoundingClientRect();
		
		const localX = e.clientX - containerRect.left;
		const localY = e.clientY - containerRect.top;
		tooltipPos = { x: localX, y: localY };

		const mouseSVGX = ((e.clientX - rect.left) / rect.width) * width;
		
		let closestIdx: number | null = null;
		bars.forEach((bar, idx) => {
			const barLeft = bar.x - 6;
			const barRight = bar.x + bar.width + 6;
			if (mouseSVGX >= barLeft && mouseSVGX <= barRight) {
				closestIdx = idx;
			}
		});

		if (closestIdx === null) {
			let minDiff = Infinity;
			bars.forEach((bar, idx) => {
				const center = bar.x + bar.width / 2;
				const diff = Math.abs(center - mouseSVGX);
				if (diff < minDiff) {
					minDiff = diff;
					closestIdx = idx;
				}
			});
		}

		hoveredIndex = closestIdx;
	}

	function handleMouseLeave() {
		hoveredIndex = null;
		tooltipPos = null;
	}
</script>

<div class="card-tactile flex flex-col h-full relative group min-h-[460px]">
	{#if isLoading}
		<div class="mb-4 min-h-[100px] flex flex-col gap-2">
			<Skeleton class="h-[14px] w-[140px]" />
			<div class="flex items-baseline gap-3 mt-1">
				<Skeleton class="h-[28px] w-[90px]" />
				<Skeleton class="h-[14px] w-[110px]" />
			</div>
		</div>
		<div class="h-80 w-full bg-muted/5 border border-border/40 rounded-xl p-4">
			<Skeleton class="h-full w-full rounded-lg" />
		</div>
	{:else}
		<div class="mb-4 min-h-[100px]">
			<div class="flex items-start justify-between h-7">
				<h3 class="text-sm font-normal text-muted-foreground">
					Grade Distribution
				</h3>
				{#if isEditing}
					<div>
						<DragIndicator />
					</div>
				{/if}
			</div>
			<div class="flex items-baseline gap-3 mt-1">
				<p class="text-3xl font-semibold tracking-tight">
					{stats?.class_average ?? 0}%
				</p>
				<span class="text-xs text-muted-foreground">
					Pass Rate: <span class="text-foreground font-semibold">{stats?.pass_rate ?? 0}%</span>
				</span>
			</div>
		</div>

		<div class="h-80 w-full flex items-center justify-center bg-muted/10 border border-border/40 rounded-xl p-4 relative overflow-hidden">
			{#if distributionArray.length > 0}
				<svg
					viewBox="0 0 {width} {height}"
					class="w-full h-full select-none overflow-visible cursor-crosshair"
					onmousemove={handleMouseMove}
					onmouseleave={handleMouseLeave}
				>
					<!-- Grid Lines -->
					{#each [0, 0.25, 0.5, 0.75, 1] as tick}
						{@const y = padding.top + tick * chartHeight}
						<line
							x1={padding.left}
							y1={y}
							x2={width - padding.right}
							y2={y}
							class="stroke-border/30"
							stroke-width="1"
							stroke-dasharray="3,3"
						/>
					{/each}

					<!-- Active Column Slice Highlight background -->
					{#if hoveredIndex !== null}
						{@const bar = bars[hoveredIndex]}
						<rect
							x={bar.x - 6}
							y={padding.top}
							width={bar.width + 12}
							height={chartHeight}
							class="fill-primary/5"
							rx={2}
						/>
					{/if}

					<!-- Bars -->
					{#each bars as bar, idx}
						<!-- Actual Bar -->
						<rect
							x={bar.x}
							y={bar.y}
							width={bar.width}
							height={Math.max(2, bar.height)}
							rx={3}
							class="transition-colors duration-200 {hoveredIndex === idx ? 'fill-primary' : 'fill-primary/80'}"
						/>

						<!-- Value Label -->
						{#if bar.item.value > 0}
							<text
								x={bar.x + bar.width / 2}
								y={bar.y - 6}
								class="text-[9px] font-semibold fill-foreground"
								text-anchor="middle"
							>
								{bar.item.value}
							</text>
						{/if}
					{/each}

					<!-- X-Axis Labels -->
					{#each bars as bar}
						<text
							x={bar.x + bar.width / 2}
							y={height - 8}
							class="text-[9px] fill-muted-foreground"
							text-anchor="middle"
						>
							{bar.item.name}
						</text>
					{/each}
				</svg>

				<!-- Floating Interactive Tooltip -->
				{#if hoveredIndex !== null && tooltipPos}
					{@const activeBar = bars[hoveredIndex]}
					<div
						class="absolute z-30 p-2 text-[10px] font-sans border bg-white dark:bg-[#0c0c0c] border-gray-200 dark:border-[#1d1d1d] text-black dark:text-white rounded shadow-md pointer-events-none transition-all duration-75 flex flex-col gap-0.5"
						style="left: {tooltipPos.x + 12}px; top: {tooltipPos.y - 45}px;"
					>
						<span class="text-gray-500 dark:text-[#666666]">Grade {activeBar.item.name}</span>
						<span class="font-semibold">{activeBar.item.value} Students</span>
					</div>
				{/if}
			{:else}
				<div class="text-xs text-muted-foreground">No grades data found.</div>
			{/if}
		</div>
	{/if}
</div>
