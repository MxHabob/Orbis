<!--
@component DataTable
The main reusable data table component for the school management system.
Supports virtualization, sticky columns, sorting, filtering, row selection,
column resizing, and export — all built on Svelte 5 Runes.

Usage:
```svelte
<DataTable
  {columns}
  {data}
  tableId="students"
  onRowClick={(row) => selectedId = row.id}
>
  {#snippet toolbar()}
    <input placeholder="Search..." bind:value={searchQuery} />
  {/snippet}
</DataTable>
```
-->
<script lang="ts" generics="T">
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
	createTableState,
	createVirtualList,
	createStickyColumns,
	createTablePersistence,
	type ColumnDef,
	type TableConfig,
	type SortDirection,
} from "@orbis/ui-svelte";
import { Skeleton } from "@orbis/ui-svelte";
import { cn } from "@orbis/ui-svelte/utils";
import { onMount, type Snippet } from "svelte";
import { ArrowUp, ArrowDown, GripVertical, Columns3, Download } from "lucide-svelte";
import { TABLE_CONFIGS, type TableId } from "$lib/config/table-configs";

// ── Props ──────────────────────────────────────────────────────────

interface Props {
	/** Column definitions */
	columns: ColumnDef<T>[];
	/** Source data */
	data: T[];
	/** Table identifier for config lookup and persistence */
	tableId: TableId;
	/** Custom table config (overrides registry) */
	config?: TableConfig;
	/** Extract unique row ID */
	getRowId?: (row: T) => string;
	/** Height of the table container */
	height?: string;

	// Feature flags
	enableSorting?: boolean;
	enableFiltering?: boolean;
	enableSelection?: boolean;
	enableResizing?: boolean;
	enableVirtualization?: boolean;
	enableExport?: boolean;
	enableColumnToggle?: boolean;
	enablePersistence?: boolean;

	// Loading / Empty
	loading?: boolean;
	skeletonRows?: number;

	// Callbacks
	onRowClick?: (row: T, index: number) => void;
	onSelectionChange?: (selectedIds: Set<string>) => void;
	onSortChange?: (column: string | null, direction: SortDirection) => void;

	// Snippets
	toolbar?: Snippet;
	bulkActions?: Snippet<[{ count: number; clear: () => void }]>;
	empty?: Snippet;
	rowActions?: Snippet<[{ row: T }]>;
}

let {
	columns,
	data,
	tableId,
	config,
	getRowId,
	height = "calc(100vh - 200px)",

	enableSorting = true,
	enableFiltering = true,
	enableSelection = false,
	enableResizing = true,
	enableVirtualization = true,
	enableExport = true,
	enableColumnToggle = true,
	enablePersistence = true,

	loading = false,
	skeletonRows = 15,

	onRowClick,
	onSelectionChange,
	onSortChange,

	toolbar,
	bulkActions,
	empty,
	rowActions,
}: Props = $props();

// ── Config ─────────────────────────────────────────────────────────

const tableConfig = $derived(config ?? TABLE_CONFIGS[tableId]);

// ── Persistence ────────────────────────────────────────────────────

const persistence = enablePersistence
	? createTablePersistence({ tableId })
	: null;

// ── Table State ────────────────────────────────────────────────────

const table = createTableState<T>({
	data,
	columns,
	getRowId,
	config: tableConfig,
	enableSorting,
	enableFiltering,
	enableColumnResizing: enableResizing,
	enableRowSelection: enableSelection,
	initialColumnVisibility: persistence?.columnVisibility,
	initialColumnSizing: persistence?.columnSizing,
	initialColumnOrder: persistence?.columnOrder,
	initialSorting: persistence?.sorting,
	onSortingChange: (s) => onSortChange?.(s.column, s.direction),
	onSelectionChange,
});

// Sync data changes from parent
$effect(() => { table.data = data; });
$effect(() => { table.columns = columns; });

// Sync state to persistence
$effect(() => {
	if (persistence) {
		persistence.columnVisibility = table.columnVisibility;
		persistence.columnSizing = table.columnSizing;
		persistence.columnOrder = table.columnOrder;
		persistence.sorting = table.sorting;
	}
});

// ── Sticky Columns ─────────────────────────────────────────────────

const sticky = createStickyColumns({
	stickyColumns: tableConfig?.stickyColumns ?? [],
	columnVisibility: table.columnVisibility,
});

// ── Virtualization ─────────────────────────────────────────────────

let containerEl: HTMLDivElement | undefined = $state();

const virtual = enableVirtualization
	? createVirtualList({
			get count() { return table.rowCount; },
			estimateSize: tableConfig?.rowHeight ?? 45,
			overscan: 12,
			getScrollElement: () => containerEl ?? null,
		})
	: null;

onMount(() => {
	if (virtual && containerEl) {
		(virtual as any).attach();
		return () => (virtual as any).detach();
	}
});

// ── Column Toggle State ────────────────────────────────────────────

let showColumnToggle = $state(false);

// ── Resize State ───────────────────────────────────────────────────

let resizingColumnId = $state<string | null>(null);
let resizeStartX = $state(0);
let resizeStartWidth = $state(0);

function startResize(event: PointerEvent, columnId: string) {
	event.preventDefault();
	event.stopPropagation();
	resizingColumnId = columnId;
	resizeStartX = event.clientX;
	resizeStartWidth = table.getColumnSize(columnId);

	const onMove = (e: PointerEvent) => {
		if (!resizingColumnId) return;
		// Use logical direction: in RTL, moving left increases width
		const dir = document.documentElement.dir === "rtl" ? -1 : 1;
		const delta = (e.clientX - resizeStartX) * dir;
		table.setColumnSize(resizingColumnId, resizeStartWidth + delta);
	};

	const onUp = () => {
		resizingColumnId = null;
		document.removeEventListener("pointermove", onMove);
		document.removeEventListener("pointerup", onUp);
	};

	document.addEventListener("pointermove", onMove);
	document.addEventListener("pointerup", onUp);
}

// ── Export ──────────────────────────────────────────────────────────

async function handleExport(format: "csv" | "json") {
	const { exportTableData } = await import("@orbis/ui-svelte");
	await exportTableData({
		data: table.processedRows,
		columns: table.visibleColumns,
		format,
		filename: tableId,
	});
}

// ── Computed helpers ───────────────────────────────────────────────

const rowHeight = $derived(tableConfig?.rowHeight ?? 45);
const rows = $derived(table.processedRows);

function getSortIcon(columnId: string): SortDirection | null {
	if (table.sorting.column !== columnId) return null;
	return table.sorting.direction;
}

function styleObj(obj: Record<string, string>): string {
	return Object.entries(obj).map(([k, v]) => `${k}:${v}`).join(";");
}
</script>

<!-- ── Template ──────────────────────────────────────────────────── -->

<div class="flex flex-col w-full">
	<!-- ── Toolbar ─────────────────────────────────────────────────── -->
	{#if toolbar || enableColumnToggle || enableExport}
		<div class="flex items-center justify-between gap-3 pb-3">
			<div class="flex-1 min-w-0">
				{#if toolbar}
					{@render toolbar()}
				{/if}
			</div>

			<div class="flex items-center gap-1.5 flex-shrink-0">
				{#if enableExport}
					<button
						type="button"
						class="inline-flex items-center justify-center gap-1.5 rounded-md border border-border bg-background px-2.5 py-1.5 text-xs font-medium text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
						onclick={() => handleExport("csv")}
					>
						<Download size={14} />
						<span>Export</span>
					</button>
				{/if}

				{#if enableColumnToggle}
					<div class="relative">
						<button
							type="button"
							class="inline-flex items-center justify-center rounded-md border border-border bg-background p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
							onclick={() => showColumnToggle = !showColumnToggle}
						>
							<Columns3 size={14} />
						</button>

						{#if showColumnToggle}
							<!-- svelte-ignore a11y_click_events_have_key_events -->
							<!-- svelte-ignore a11y_no_static_element_interactions -->
							<div
								class="absolute end-0 top-full mt-1 z-50 min-w-[180px] rounded-lg border border-border bg-popover p-2 shadow-lg"
								onclick={(e) => e.stopPropagation()}
							>
								<p class="text-xs font-semibold text-muted-foreground mb-2 px-1">Columns</p>
								{#each table.columns as col (col.id)}
									{#if col.enableHiding !== false && col.id !== "actions" && col.id !== "select"}
										<label class="flex items-center gap-2 rounded px-1 py-1 text-xs hover:bg-accent cursor-pointer select-none">
											<input
												type="checkbox"
												checked={table.columnVisibility[col.id] !== false}
												onchange={() => table.toggleColumnVisibility(col.id)}
												class="h-3.5 w-3.5 rounded border-border accent-primary"
											/>
											<span class="truncate">{typeof col.header === 'string' ? col.header : col.id}</span>
										</label>
									{/if}
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	{/if}

	<!-- ── Table Container ──────────────────────────────────────────── -->
	<div
		bind:this={containerEl}
		class="overflow-auto overscroll-none border border-border rounded-lg scrollbar-hide"
		style="height:{height}"
	>
		{#if loading}
			<!-- ── Skeleton State ──────────────────────────────────────── -->
			<Table class="w-full min-w-full">
				<TableHeader class="border-0 block sticky top-0 z-20 bg-background w-full">
					<TableRow class="flex items-center h-[45px] hover:bg-transparent !border-b-0 min-w-full">
						{#each table.visibleColumns as col (col.id)}
							<TableHead
								class={cn(
									"relative h-full px-4 border-t border-border flex items-center",
									sticky.getStickyClass(col.id),
								)}
								style="{styleObj({ width: `${table.getColumnSize(col.id)}px`, ...sticky.getStickyStyle(col.id) })}"
							>
								<Skeleton class="h-3.5 w-20" />
							</TableHead>
						{/each}
					</TableRow>
				</TableHeader>
				<TableBody class="border-l-0 border-r-0 block">
					{#each Array(skeletonRows) as _, i (i)}
						<TableRow class="flex items-center h-[{rowHeight}px] border-b border-border">
							{#each table.visibleColumns as col (col.id)}
								<TableCell
									class={cn(
										"h-full flex items-center",
										sticky.getStickyClass(col.id),
									)}
									style="{styleObj({ width: `${table.getColumnSize(col.id)}px`, ...sticky.getStickyStyle(col.id) })}"
								>
									{#if col.skeleton?.type === "avatar-text"}
										<div class="flex items-center gap-2">
											<Skeleton class="h-7 w-7 rounded-full" />
											<Skeleton class="h-3.5 {col.skeleton?.width ?? 'w-24'}" />
										</div>
									{:else if col.skeleton?.type === "badge"}
										<Skeleton class="h-5 {col.skeleton?.width ?? 'w-16'} rounded-full" />
									{:else if col.skeleton?.type === "icon"}
										<Skeleton class="h-4 w-4 rounded" />
									{:else if col.skeleton?.type === "checkbox"}
										<Skeleton class="h-4 w-4" />
									{:else}
										<Skeleton class="h-3.5 {col.skeleton?.width ?? 'w-24'}" />
									{/if}
								</TableCell>
							{/each}
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		{:else if rows.length === 0}
			<!-- ── Empty State ─────────────────────────────────────────── -->
			<div
				class="flex items-center justify-center text-center"
				style="height:{height}"
			>
				{#if empty}
					{@render empty()}
				{:else}
					<div class="flex flex-col items-center gap-2 text-muted-foreground">
						<svg class="w-10 h-10 opacity-30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
							<path stroke-linecap="round" stroke-linejoin="round" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-2.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
						</svg>
						<p class="text-xs font-medium">No data to display</p>
					</div>
				{/if}
			</div>
		{:else}
			<!-- ── Data Table ──────────────────────────────────────────── -->
			<Table class="w-full min-w-full">
				<!-- Header -->
				<TableHeader class="border-0 block sticky top-0 z-20 bg-background w-full">
					<TableRow class="flex items-center h-[45px] hover:bg-transparent !border-b-0 min-w-full">
						{#if enableSelection}
							<TableHead class="w-[50px] min-w-[50px] flex items-center justify-center h-full border-t border-border sticky inset-inline-start-0 z-10 bg-background">
								<input
									type="checkbox"
									checked={table.isAllSelected}
									onchange={() => table.toggleAllSelection()}
									class="h-3.5 w-3.5 rounded border-border accent-primary"
								/>
							</TableHead>
						{/if}

						{#each table.visibleColumns as col, colIdx (col.id)}
							{@const isLast = colIdx === table.visibleColumns.length - 1}
							{@const stickyStyle = sticky.getStickyStyle(col.id)}
							{@const stickyClass = sticky.getStickyClass(col.id)}
							{@const sortDir = getSortIcon(col.id)}
							{@const colWidth = table.getColumnSize(col.id)}

							<TableHead
								class={cn(
									"group/header relative h-full px-4 border-t border-border flex items-center select-none",
									stickyClass,
									col.sortable && "cursor-pointer hover:bg-accent/50",
									isLast && !sticky.isSticky(col.id) && "flex-1",
								)}
								style="{styleObj({
									width: `${colWidth}px`,
									'min-width': sticky.isSticky(col.id) ? `${colWidth}px` : `${col.minSize ?? 50}px`,
									...stickyStyle,
								})}"
								onclick={() => col.sortable && table.toggleSort(col.id)}
							>
								<div class="flex items-center gap-1.5 min-w-0 overflow-hidden flex-1">
									<span class="truncate text-xs font-medium">
										{typeof col.header === 'string' ? col.header : col.id}
									</span>

									{#if sortDir === "asc"}
										<ArrowDown size={13} class="text-primary flex-shrink-0" />
									{:else if sortDir === "desc"}
										<ArrowUp size={13} class="text-primary flex-shrink-0" />
									{/if}
								</div>

								<!-- Resize Handle -->
								{#if enableResizing && col.enableResizing !== false}
									<!-- svelte-ignore a11y_no_static_element_interactions -->
									<div
										class="absolute inset-inline-end-0 top-0 h-full w-1.5 cursor-col-resize select-none touch-none opacity-0 group-hover/header:opacity-100 hover:bg-primary/30 transition-opacity"
										onpointerdown={(e) => startResize(e, col.id)}
										ondblclick={() => {
											const original = columns.find(c => c.id === col.id);
											if (original?.size) table.setColumnSize(col.id, original.size);
										}}
									></div>
								{/if}
							</TableHead>
						{/each}

						{#if rowActions}
							<TableHead class="w-[80px] min-w-[80px] flex items-center justify-center h-full border-t border-border sticky inset-inline-end-0 z-10 bg-background !border-inline-start !border-border">
								<span class="text-xs text-muted-foreground">Actions</span>
							</TableHead>
						{/if}
					</TableRow>
				</TableHeader>

				<!-- Body -->
				<TableBody
					class="border-l-0 border-r-0 block"
					style={enableVirtualization && virtual
						? `height:${virtual.totalSize}px; position:relative`
						: ""}
				>
					{#if enableVirtualization && virtual}
						<!-- Virtualized Rows -->
						{#each virtual.virtualItems as item (item.index)}
							{@const row = rows[item.index]}
							{@const rowId = row ? table.getRowId(row, item.index) : ""}
							{@const selected = row ? table.isRowSelected(rowId) : false}

							{#if row}
								<TableRow
									data-index={item.index}
									class={cn(
										"group cursor-pointer select-text",
										"hover:bg-accent/40",
										selected && "bg-primary/5 hover:bg-primary/8",
										"flex items-center border-0",
										"absolute top-0 inset-inline-start-0 w-full min-w-full",
									)}
									style="height:{item.size}px; transform:translateY({item.start}px); contain:layout style paint"
									onclick={() => onRowClick?.(row, item.index)}
								>
									{#if enableSelection}
										<!-- svelte-ignore a11y_click_events_have_key_events -->
										<!-- svelte-ignore a11y_no_static_element_interactions -->
										<TableCell
											class="w-[50px] min-w-[50px] flex items-center justify-center h-full border-b border-border sticky inset-inline-start-0 z-10 bg-background group-hover:bg-accent/40"
											onclick={(e) => { e.stopPropagation(); table.toggleRowSelection(rowId); }}
										>
											<input
												type="checkbox"
												checked={selected}
												class="h-3.5 w-3.5 rounded border-border accent-primary pointer-events-none"
												tabindex="-1"
											/>
										</TableCell>
									{/if}

									{#each table.visibleColumns as col, colIdx (col.id)}
										{@const isLast = colIdx === table.visibleColumns.length - 1 && !rowActions}
										{@const ctx = table.getCellContext(row, col, item.index)}
										{@const stickyStyle = sticky.getStickyStyle(col.id)}
										{@const stickyClass = sticky.getStickyClass(col.id)}
										{@const colWidth = table.getColumnSize(col.id)}

										<TableCell
											class={cn(
												"h-full flex items-center border-b border-border",
												stickyClass,
												sticky.isSticky(col.id) && "group-hover:bg-accent/40",
												col.align === "center" && "justify-center",
												col.align === "end" && "justify-end",
												col.className,
												isLast && !sticky.isSticky(col.id) && "flex-1",
											)}
											style="{styleObj({
												width: `${colWidth}px`,
												...stickyStyle,
											})}"
										>
											<div class={cn(
												"overflow-hidden truncate",
												col.align === "center" ? "flex items-center justify-center h-full w-full" : "w-full",
											)}>
												{#if col.cell}
													{@render col.cell(ctx)}
												{:else}
													<span class="text-xs">{ctx.value ?? ""}</span>
												{/if}
											</div>
										</TableCell>
									{/each}

									{#if rowActions}
										<TableCell
											class="w-[80px] min-w-[80px] flex items-center justify-center h-full border-b border-border sticky inset-inline-end-0 z-10 bg-background group-hover:bg-accent/40 !border-inline-start !border-border"
										>
											{@render rowActions({ row })}
										</TableCell>
									{/if}
								</TableRow>
							{/if}
						{/each}
					{:else}
						<!-- Non-virtualized (simple) Rows -->
						{#each rows as row, idx (table.getRowId(row, idx))}
							{@const rowId = table.getRowId(row, idx)}
							{@const selected = table.isRowSelected(rowId)}

							<TableRow
								class={cn(
									"group cursor-pointer select-text",
									"hover:bg-accent/40",
									selected && "bg-primary/5",
									"flex items-center",
								)}
								style="height:{rowHeight}px"
								onclick={() => onRowClick?.(row, idx)}
							>
								{#if enableSelection}
									<!-- svelte-ignore a11y_click_events_have_key_events -->
									<!-- svelte-ignore a11y_no_static_element_interactions -->
									<TableCell
										class="w-[50px] min-w-[50px] flex items-center justify-center h-full border-b border-border"
										onclick={(e) => { e.stopPropagation(); table.toggleRowSelection(rowId); }}
									>
										<input
											type="checkbox"
											checked={selected}
											class="h-3.5 w-3.5 rounded border-border accent-primary pointer-events-none"
											tabindex="-1"
										/>
									</TableCell>
								{/if}

								{#each table.visibleColumns as col (col.id)}
									{@const ctx = table.getCellContext(row, col, idx)}
									<TableCell
										class={cn(
											"h-full flex items-center border-b border-border",
											sticky.getStickyClass(col.id),
											col.align === "center" && "justify-center",
											col.className,
										)}
										style="{styleObj({
											width: `${table.getColumnSize(col.id)}px`,
											...sticky.getStickyStyle(col.id),
										})}"
									>
										<div class="overflow-hidden truncate w-full">
											{#if col.cell}
												{@render col.cell(ctx)}
											{:else}
												<span class="text-xs">{ctx.value ?? ""}</span>
											{/if}
										</div>
									</TableCell>
								{/each}

								{#if rowActions}
									<TableCell class="w-[80px] min-w-[80px] flex items-center justify-center h-full border-b border-border">
										{@render rowActions({ row })}
									</TableCell>
								{/if}
							</TableRow>
						{/each}
					{/if}
				</TableBody>
			</Table>
		{/if}
	</div>

	<!-- ── Bottom Bar (Bulk Actions) ────────────────────────────────── -->
	{#if enableSelection && table.selectedCount > 0 && bulkActions}
		<div
			class="fixed bottom-6 inset-inline-start-0 inset-inline-end-0 pointer-events-none flex justify-center z-50"
		>
			<div
				class="pointer-events-auto min-w-[400px] h-12 rounded-xl border border-border backdrop-blur-lg bg-background/85 shadow-lg flex items-center justify-between px-4 animate-in slide-in-from-bottom-4 duration-200"
			>
				<span class="text-sm text-foreground">
					{table.selectedCount} selected
				</span>
				<div class="flex items-center gap-2">
					<button
						type="button"
						class="text-xs text-muted-foreground hover:text-foreground transition-colors px-2 py-1"
						onclick={() => table.clearSelection()}
					>
						Deselect all
					</button>
					{@render bulkActions({ count: table.selectedCount, clear: table.clearSelection })}
				</div>
			</div>
		</div>
	{/if}

	<!-- ── Footer Info ──────────────────────────────────────────────── -->
	{#if !loading && rows.length > 0}
		<div class="flex items-center justify-between px-1 pt-2 text-[11px] text-muted-foreground">
			<span>
				{table.rowCount}
				{table.rowCount !== table.totalCount ? ` / ${table.totalCount}` : ""}
				rows
			</span>
			{#if enableSelection && table.selectedCount > 0}
				<span>{table.selectedCount} selected</span>
			{/if}
		</div>
	{/if}
</div>

<!-- Close column toggle on outside click -->
<svelte:window onclick={() => { if (showColumnToggle) showColumnToggle = false; }} />

