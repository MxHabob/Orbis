/**
 * @orbis/data-grid — Renderer Infrastructure
 *
 * Exports types and utilities for building cell renderers.
 * Actual renderer implementations live in the consumer app.
 */

// Types
export type {
  CellRenderer,
  RenderContext,
  RendererFactory,
  GridActionEvent,
  ActionItem,
} from "./types";

// Utilities
export {
  createInitials,
  resolveThreshold,
  iconSvg,
  registerIcon,
  type Threshold,
} from "./utils";
