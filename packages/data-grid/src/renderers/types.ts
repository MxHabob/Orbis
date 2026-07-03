/**
 * @orbis/data-grid — Cell Renderer Type System
 *
 * Defines the contract for custom cell renderers that use RevoGrid's
 * hyperscript h() function directly — zero Svelte overhead.
 *
 * Renderers are defined by the CONSUMER app (not this package),
 * giving full control over design while keeping maximum performance.
 */

// ── Render Context ───────────────────────────────────────────────────────────

/**
 * All data available to a cell renderer function.
 * Passed automatically by OrbisGrid for each visible cell.
 */
export interface RenderContext {
  /**
   * RevoGrid's hyperscript function — creates VNode DOM elements.
   * Signature: h(tag, attributes?, children?)
   *
   * @example
   * h('div', { class: 'wrapper' }, [
   *   h('img', { src: url, alt: name }),
   *   h('span', {}, 'Hello'),
   * ])
   */
  h: (tag: any, attrs?: any, children?: any) => any;
  /** Cell value (raw from source data) */
  value: any;
  /** Full row data object */
  row: Record<string, any>;
  /** Column property key */
  prop: string;
  /** Row index in source array */
  rowIndex: number;
  /** Column index */
  colIndex: number;
  /** Full RevoGrid column data schema model (advanced use) */
  model: any;
}

// ── Cell Renderer ────────────────────────────────────────────────────────────

/**
 * A CellRenderer is a **pure function** that receives cell data and returns
 * a VNode tree via hyperscript h(). No Svelte, no framework, just DOM.
 *
 * Performance: ~0.05ms per cell — identical to RevoGrid's native rendering.
 *
 * @example
 * // Inline renderer
 * const bold: CellRenderer = ({ h, value }) =>
 *   h('span', { style: { fontWeight: '700' } }, String(value ?? ''));
 *
 * // Used in column definition
 * const col = { prop: 'name', cellRenderer: bold };
 */
export type CellRenderer = (ctx: RenderContext) => any;

/**
 * A factory function that creates a CellRenderer with pre-configured options.
 * All built-in renderers in the consumer app follow this pattern.
 *
 * @example
 * // Define in consumer app:
 * const avatarRenderer: RendererFactory<AvatarConfig> = (config) => {
 *   return ({ h, value, row }) => h('div', ...);
 * };
 *
 * // Use:
 * const col = { cellRenderer: avatarRenderer({ size: 'md' }) };
 */
export type RendererFactory<TConfig = any> = (config?: TConfig) => CellRenderer;

// ── Grid Action Event ────────────────────────────────────────────────────────

/**
 * Fired when a user clicks an element with `data-grid-action` attribute
 * inside a cell. Captured via event delegation at the grid container level.
 *
 * To make a clickable element in a renderer:
 * ```typescript
 * h('button', {
 *   'data-grid-action': 'edit',      // ← action ID
 *   'data-row-index': String(rowIndex),
 *   'data-prop': prop,
 * }, 'Edit')
 * ```
 */
export interface GridActionEvent {
  /** Action identifier from data-grid-action attribute */
  actionId: string;
  /** Full row data */
  row: Record<string, unknown>;
  /** Row index in source array */
  rowIndex: number;
  /** Column property key */
  prop: string;
  /** Original DOM MouseEvent (for positioning overlays, menus, etc.) */
  originalEvent: MouseEvent;
}

// ── Action Item ──────────────────────────────────────────────────────────────

/**
 * Definition for an action button in a cell.
 * Used by consumer-side actionsRenderer().
 * Provided here as a shared type so package and consumer agree on shape.
 */
export interface ActionItem {
  /** Unique action identifier — dispatched in GridActionEvent.actionId */
  id: string;
  /** Lucide icon name (e.g. 'Pencil', 'Trash2', 'Eye') */
  icon?: string;
  /** Tooltip / accessible label */
  label?: string;
  /** Visual variant for styling */
  variant?: "default" | "destructive" | "success";
  /** Return false to hide this action for specific rows */
  visible?: (row: Record<string, unknown>) => boolean;
  /** Return true to disable this action for specific rows */
  disabled?: (row: Record<string, unknown>) => boolean;
}
