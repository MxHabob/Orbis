// ═══════════════════════════════════════════════════════════════
// @docforge/core — Model Barrel Export
// ═══════════════════════════════════════════════════════════════

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
} from "./document";

export { DOCUMENT_SCHEMA_VERSION } from "./document";

export type {
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
} from "./nodes";

export { isContainerNode, isContentNode, hasPosition } from "./nodes";
