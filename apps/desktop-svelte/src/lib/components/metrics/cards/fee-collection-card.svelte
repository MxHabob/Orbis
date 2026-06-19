<script lang="ts">
	import { Skeleton } from "@orbis/ui-svelte";
	import { onMount } from "svelte";
	import { commands } from "$lib/services/ipc";
	import { formatAmount } from "@orbis/utils/format";
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

	let data = $state<any[]>([]);
	let totalCollected = $state(0);
	let totalPending = $state(0);
	let isLoading = $state(true);

	async function loadData() {
		isLoading = true;
		try {
			// Query payments
			const payments = await commands.finance.getPayments(orgId);
			// Query student fees to see outstanding fees
			const studentFees = await commands.finance.getStudentFees(orgId);

			if (payments && payments.length > 0) {
				// Parse payments and outstanding student fees by month
				const monthsMap: Record<string, { collected: number; pending: number }> = {};
				
				// Collected payments
				for (const p of payments) {
					const dateStr = (p.payment_date || p.created_at || "") as string;
					if (!dateStr) continue;
					const monthKey = dateStr.substring(0, 7); // yyyy-mm
					if (!monthsMap[monthKey]) {
						monthsMap[monthKey] = { collected: 0, pending: 0 };
					}
					monthsMap[monthKey].collected += Number(p.amount || 0);
				}

				// Outstanding / Pending fees
				if (studentFees) {
					for (const f of studentFees) {
						if (f.status === "unpaid" || f.status === "partially_paid") {
							const dateStr = (f.due_date || f.created_at || "") as string;
							if (!dateStr) continue;
							const monthKey = dateStr.substring(0, 7);
							if (!monthsMap[monthKey]) {
								monthsMap[monthKey] = { collected: 0, pending: 0 };
							}
							const unpaid = Number(f.amount || 0) - Number(f.paid_amount || 0);
							monthsMap[monthKey].pending += Math.max(0, unpaid);
						}
					}
				}

				const sortedMonths = Object.keys(monthsMap).sort().slice(-6); // last 6 months
				totalCollected = 0;
				totalPending = 0;
				data = sortedMonths.map((m) => {
					const vals = monthsMap[m]!;
					totalCollected += vals.collected;
					totalPending += vals.pending;
					// Get month short name
					const monthIndex = Number(m.split("-")[1]) - 1;
					const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
					return {
						month: monthNames[monthIndex] || m,
						collected: vals.collected,
						pending: vals.pending
					};
				});
			} else {
				// Mock data fallback
				data = [
					{ month: "Oct", collected: 24000, pending: 8000 },
					{ month: "Nov", collected: 32000, pending: 12000 },
					{ month: "Dec", collected: 18000, pending: 4000 },
					{ month: "Jan", collected: 45000, pending: 15000 },
					{ month: "Feb", collected: 38000, pending: 10000 },
					{ month: "Mar", collected: 29000, pending: 6000 }
				];
				totalCollected = 186000;
				totalPending = 55000;
			}
		} catch (e) {
			console.error("Error loading finance reports:", e);
		} finally {
			isLoading = false;
		}
	}

	$effect(() => {
		if (orgId || selectedClass || selectedTerm || period) {
			loadData();
		}
	});

	// SVG layout dimensions
	const width = 500;
	const height = 200;
	const padding = { left: 50, right: 15, top: 25, bottom: 25 };

	const maxVal = $derived(Math.max(1000, ...data.map((d) => Math.max(d.collected, d.pending))));
	const chartWidth = $derived(width - padding.left - padding.right);
	const chartHeight = $derived(height - padding.top - padding.bottom);
	const groupWidth = $derived(chartWidth / (data.length || 1));
	const barWidth = $derived(Math.max(4, (groupWidth - 16) / 2));

	const bars = $derived.by(() => {
		return data.map((item, idx) => {
			const xCol = padding.left + idx * groupWidth + (groupWidth - 2 * barWidth - 4) / 2;
			const xPend = xCol + barWidth + 4;
			
			const pctCol = item.collected / maxVal;
			const hCol = pctCol * chartHeight;
			const yCol = height - padding.bottom - hCol;

			const pctPend = item.pending / maxVal;
			const hPend = pctPend * chartHeight;
			const yPend = height - padding.bottom - hPend;

			return {
				month: item.month,
				col: { x: xCol, y: yCol, width: barWidth, height: hCol, val: item.collected },
				pend: { x: xPend, y: yPend, width: barWidth, height: hPend, val: item.pending }
			};
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
			const colLeft = bar.col.x - 6;
			const pendRight = bar.pend.x + bar.pend.width + 6;
			if (mouseSVGX >= colLeft && mouseSVGX <= pendRight) {
				closestIdx = idx;
			}
		});

		if (closestIdx === null) {
			let minDiff = Infinity;
			bars.forEach((bar, idx) => {
				const center = (bar.col.x + bar.pend.x + bar.col.width) / 2;
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

	// Formatter helper
	function formatVal(val: number) {
		if (val >= 1000) {
			return `${Math.round(val / 100) / 10}k`;
		}
		return String(val);
	}
</script>

<div class="card-tactile flex flex-col h-full relative group min-h-[460px]">
	{#if isLoading}
		<div class="mb-4 min-h-[100px] flex flex-col gap-2">
			<Skeleton class="h-[14px] w-[140px]" />
			<div class="flex items-baseline gap-3 mt-1">
				<Skeleton class="h-[28px] w-[120px]" />
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
					Tuition Fee Collection
				</h3>
				{#if isEditing}
					<div>
						<DragIndicator />
					</div>
				{/if}
			</div>
			<div class="flex items-baseline gap-3 mt-1">
				<p class="text-3xl font-semibold tracking-tight">
					{formatAmount({ amount: totalCollected, currency: "SAR", locale: "en-US", maximumFractionDigits: 0 })}
				</p>
				<span class="text-xs text-muted-foreground">
					Pending: <span class="text-foreground font-semibold">{formatAmount({ amount: totalPending, currency: "SAR", locale: "en-US", maximumFractionDigits: 0 })}</span>
				</span>
			</div>
		</div>

		<div class="h-80 w-full flex items-center justify-center bg-muted/10 border border-border/40 rounded-xl p-4 relative overflow-hidden">
			{#if data.length > 0}
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
						<!-- Left axis currency labels -->
						{@const labelVal = Math.round((1 - tick) * maxVal)}
						<text
							x={padding.left - 8}
							y={y + 3}
							class="text-[8px] fill-muted-foreground"
							text-anchor="end"
						>
							{formatVal(labelVal)}
						</text>
					{/each}

					<!-- Active Column Slice Highlight background -->
					{#if hoveredIndex !== null}
						{@const bar = bars[hoveredIndex]}
						<rect
							x={bar.col.x - 6}
							y={padding.top}
							width={bar.pend.x + bar.pend.width - bar.col.x + 12}
							height={chartHeight}
							class="fill-primary/5"
							rx={2}
						/>
					{/if}

					<!-- Bars -->
					{#each bars as bar, idx}
						<!-- Collected Bar (brand lime color) -->
						<rect
							x={bar.col.x}
							y={bar.col.y}
							width={bar.col.width}
							height={Math.max(1, bar.col.height)}
							rx={1.5}
							class="transition-colors duration-200 {hoveredIndex === idx ? 'fill-[#84cc16]' : 'fill-[#84cc16]/80'}"
						/>

						<!-- Pending Bar (muted dark-gray color) -->
						<rect
							x={bar.pend.x}
							y={bar.pend.y}
							width={bar.pend.width}
							height={Math.max(1, bar.pend.height)}
							rx={1.5}
							class="transition-colors duration-200 {hoveredIndex === idx ? 'fill-[#9ca3af] dark:fill-[#4b5563]' : 'fill-[#9ca3af]/80 dark:fill-[#4b5563]/80'}"
						/>
					{/each}

					<!-- X-Axis Labels -->
					{#each bars as bar}
						<text
							x={bar.col.x + bar.col.width + 2}
							y={height - 8}
							class="text-[9px] fill-muted-foreground"
							text-anchor="middle"
						>
							{bar.month}
						</text>
					{/each}
				</svg>

				<!-- Floating Interactive Tooltip -->
				{#if hoveredIndex !== null && tooltipPos}
					{@const activeBar = bars[hoveredIndex]}
					<div
						class="absolute z-30 p-3 text-[10px] font-sans border bg-white dark:bg-[#0c0c0c] border-gray-200 dark:border-[#1d1d1d] text-black dark:text-white rounded shadow-md pointer-events-none transition-all duration-75 flex flex-col gap-1 min-w-[130px]"
						style="left: {tooltipPos.x + 12}px; top: {tooltipPos.y - 65}px;"
					>
						<span class="text-gray-500 dark:text-[#666666] font-medium">{activeBar.month}</span>
						<div class="flex flex-col gap-0.5">
							<div class="flex items-center justify-between gap-4">
								<span class="text-[#84cc16]">Collected:</span>
								<span class="font-semibold">{formatAmount({ amount: activeBar.col.val, currency: "SAR", locale: "en-US", maximumFractionDigits: 0 })}</span>
							</div>
							<div class="flex items-center justify-between gap-4">
								<span class="text-[#9ca3af] dark:text-[#9ca3af]">Pending:</span>
								<span class="font-semibold">{formatAmount({ amount: activeBar.pend.val, currency: "SAR", locale: "en-US", maximumFractionDigits: 0 })}</span>
							</div>
						</div>
					</div>
				{/if}
			{:else}
				<div class="text-xs text-muted-foreground">No financial records found.</div>
			{/if}
		</div>
	{/if}
</div>
