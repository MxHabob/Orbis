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

// ── Canvas Utilities ──────────────────────────────────────────
export * from "./canvas";
export type {
  DataFieldSchema,
  DataFilter,
  DataResolver,
  FilterCondition,
  OrgVars,
  SystemVars,
} from "./data";
// ── Data Binding ─────────────────────────────────────────────
export { createFullContext } from "./data";
// ── Editor State ─────────────────────────────────────────────
export { EditorState } from "./editor";

export type { DataContext, FormatterFn, Token } from "./expression";
// ── Expression Engine ────────────────────────────────────────
export {
  evaluateCondition,
  evaluateExpression,
  extractVariableRefs,
  getAvailableFormatters,
  getFormatter,
  parseExpression,
  registerFormatter,
  resolvePath,
} from "./expression";
// ── Model Types ──────────────────────────────────────────────
export type {
  AssetReference,
  AssetType,
  BarcodeFormat,
  BarcodeNode,
  BaseNode,
  ComponentRefNode,
  ConditionalNode,
  DocumentMeta,
  FooterNode,
  FrameNode,
  GridNode,
  HeaderNode,
  ImageNode,
  NodeType,
  Orientation,
  PageBreakNode,
  PageNode,
  PageSetup,
  PageSize,
  Position,
  PositionMode,
  QRCodeNode,
  RepeaterNode,
  SeparatorNode,
  ShapeNode,
  SpacerNode,
  Spacing,
  StyleDefinition,
  TableColumn,
  TableNode,
  TableRow,
  TemplateDocument,
  TemplateNode,
  TextNode,
  ThemeDefinition,
  VariableDefinition,
  VariableType,
  WatermarkNode,
} from "./model";
export {
  DOCUMENT_SCHEMA_VERSION,
  getPages,
  getPrimaryPage,
  hasPosition,
  isContainerNode,
  isContentNode,
} from "./model";
export type {
  FillOrder,
  MultiUpConfig,
  MultiUpLayout,
  SheetSize,
  SlotPosition,
} from "./multi-up";
// ── Multi-Up Layout ──────────────────────────────────────────
export {
  computeMultiUpLayout,
  computeTotalSheets,
  createDefaultMultiUpConfig,
  layoutLabel,
  layoutLabelAr,
  suggestLayouts,
} from "./multi-up";
// ── Operations ───────────────────────────────────────────────
export {
  addNode,
  cloneNode,
  createDocument,
  createNode,
  findNode,
  findNodePath,
  moveNode,
  removeNode,
  updateNodeProps,
  walkNodes,
} from "./operations";
export type { DataSchema, DataSchemaProperty } from "./schema";
// ── Schema Validation ────────────────────────────────────────
export { validateBindings } from "./schema";
export type { DocumentDiff } from "./serialization";
// ── Serialization ────────────────────────────────────────────
export {
  deserializeDocument,
  diffDocuments,
  serializeDocument,
} from "./serialization";

export type { PageDimensions } from "./utils";
// ── Utilities ────────────────────────────────────────────────
export {
  containsArabic,
  darken,
  detectDirection,
  formatArabicNumber,
  generateNodeId,
  generateUUID,
  getPageDimensions,
  getPrintableArea,
  hexToRgb,
  lighten,
  mmToPt,
  mmToPx,
  PAGE_SIZES,
  ptToMm,
  ptToPx,
  pxToMm,
  pxToPt,
  rgbToHex,
  toArabicNumerals,
  toArabicOrdinal,
  toWesternNumerals,
  withOpacity,
} from "./utils";
export type { ValidationIssue, ValidationResult } from "./validation";
// ── Validation ───────────────────────────────────────────────
export { validateDocument } from "./validation";
