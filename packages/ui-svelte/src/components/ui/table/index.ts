// ── @orbis/ui-svelte/table ──────────────────────────────────────────
// Table infrastructure: primitive components + headless hooks + utilities.

// ── Primitive Components (shadcn-svelte) ────────────────────────────

import Root from "./table.svelte";
import Body from "./table-body.svelte";
import Caption from "./table-caption.svelte";
import Cell from "./table-cell.svelte";
import Footer from "./table-footer.svelte";
import Head from "./table-head.svelte";
import Header from "./table-header.svelte";
import Row from "./table-row.svelte";

export {
  Root,
  Body,
  Caption,
  Cell,
  Footer,
  Head,
  Header,
  Row,
  //
  Root as Table,
  Body as TableBody,
  Caption as TableCaption,
  Cell as TableCell,
  Footer as TableFooter,
  Head as TableHead,
  Header as TableHeader,
  Row as TableRow,
};

// ── Headless Hooks (Svelte 5 Runes) ────────────────────────────────

export {
  createTableState,
  type TableState,
} from "./create-table-state.svelte.js";
export { createVirtualList } from "./create-virtual-list.svelte.js";
export {
  createStickyColumns,
  type StickyColumnsState,
} from "./create-sticky-columns.svelte.js";
export {
  createTablePersistence,
  type TablePersistenceState,
} from "./create-table-persistence.svelte.js";

// ── Utilities ──────────────────────────────────────────────────────

export {
  exportTableData,
  getExportableColumnCount,
} from "./create-table-export.js";

// ── Types ──────────────────────────────────────────────────────────

export type {
  ColumnDef,
  CellContext,
  HeaderContext,
  SortDirection,
  SortingState,
  FilterState,
  SkeletonType,
  SkeletonConfig,
  StickyColumnConfig,
  TableConfig,
  TableSettings,
  TableOptions,
  VirtualItem,
  VirtualListOptions,
  VirtualListState,
  ExportFormat,
  ExportOptions,
} from "./table-types.js";
