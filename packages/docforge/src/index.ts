// ═══════════════════════════════════════════════════════════════
// @docforge/core — Public API
//
// Standalone document template engine.
// Zero external dependencies. Zero Orbis dependencies.
//
// Usage:
//   import { createDocument, createNode, evaluateExpression } from "@docforge/core";
//   import type { TemplateDocument, TextNode } from "@docforge/core/model";
// ═══════════════════════════════════════════════════════════════

// ── Model Types ──────────────────────────────────────────────
export type {
  TemplateDocument,
  DocumentMeta,
  PageSetup,
  PageSize,
  Orientation,
  Spacing,
  VariableDefinition,
  VariableType,
  AssetReference,
  AssetType,
  Position,
  PositionMode,
  TemplateNode,
  NodeType,
  BaseNode,
  PageNode,
  FrameNode,
  GridNode,
  TextNode,
  ImageNode,
  ShapeNode,
  TableNode,
  TableColumn,
  TableRow,
  BarcodeNode,
  BarcodeFormat,
  QRCodeNode,
  SeparatorNode,
  SpacerNode,
  RepeaterNode,
  ConditionalNode,
  ComponentRefNode,
  PageBreakNode,
  HeaderNode,
  FooterNode,
  WatermarkNode,
} from "./model";

export {
  DOCUMENT_SCHEMA_VERSION,
  isContainerNode,
  isContentNode,
  hasPosition,
} from "./model";

// ── Operations ───────────────────────────────────────────────
export {
  createDocument,
  createNode,
  cloneNode,
  findNode,
  findNodePath,
  walkNodes,
  updateNodeProps,
  addNode,
  removeNode,
  moveNode,
} from "./operations";

// ── Expression Engine ────────────────────────────────────────
export {
  evaluateExpression,
  evaluateCondition,
  resolvePath,
  parseExpression,
  extractVariableRefs,
  registerFormatter,
  getFormatter,
  getAvailableFormatters,
} from "./expression";

export type { DataContext, Token, FormatterFn } from "./expression";

// ── Serialization ────────────────────────────────────────────
export {
  serializeDocument,
  deserializeDocument,
  diffDocuments,
} from "./serialization";
export type { DocumentDiff } from "./serialization";

// ── Data Binding ─────────────────────────────────────────────
export { createFullContext } from "./data";
export type {
  DataSchema,
  DataFieldSchema,
  DataResolver,
  DataFilter,
  FilterCondition,
  SystemVars,
  OrgVars,
} from "./data";

// ── Editor State ─────────────────────────────────────────────
export { EditorState } from "./editor";

// ── Validation ───────────────────────────────────────────────
export { validateDocument } from "./validation";
export type { ValidationResult, ValidationIssue } from "./validation";

// ── Utilities ────────────────────────────────────────────────
export {
  mmToPx,
  pxToMm,
  ptToMm,
  mmToPt,
  ptToPx,
  pxToPt,
  PAGE_SIZES,
  getPageDimensions,
  getPrintableArea,
  toArabicNumerals,
  toWesternNumerals,
  formatArabicNumber,
  toArabicOrdinal,
  containsArabic,
  detectDirection,
  generateNodeId,
  generateUUID,
  hexToRgb,
  rgbToHex,
  withOpacity,
  lighten,
  darken,
} from "./utils";

export type { PageDimensions } from "./utils";

// ── Multi-Up Layout ──────────────────────────────────────────
export {
  computeMultiUpLayout,
  computeTotalSheets,
  suggestLayouts,
  createDefaultMultiUpConfig,
  layoutLabel,
  layoutLabelAr,
} from "./multi-up";

export type {
  MultiUpConfig,
  MultiUpLayout,
  SlotPosition,
  SheetSize,
  FillOrder,
} from "./multi-up";
