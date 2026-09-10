// ═══════════════════════════════════════════════════════════════
// @docforge/core — Model Barrel Export (v2.0)
// ═══════════════════════════════════════════════════════════════

export type {
  AssetReference,
  AssetType,
  DocumentMeta,
  Orientation,
  PageSetup,
  PageSize,
  Position,
  PositionMode,
  Spacing,
  StyleDefinition,
  TemplateDocument,
  ThemeDefinition,
  VariableDefinition,
  VariableType,
} from "./document";

export {
  DOCUMENT_SCHEMA_VERSION,
  getPages,
  getPrimaryPage,
} from "./document";

export type {
  BarcodeFormat,
  BarcodeNode,
  BaseNode,
  ComponentRefNode,
  ConditionalNode,
  FooterNode,
  FrameNode,
  GridNode,
  HeaderNode,
  ImageNode,
  NodeType,
  PageBreakNode,
  PageNode,
  QRCodeNode,
  RepeaterNode,
  SeparatorNode,
  ShapeNode,
  SpacerNode,
  TableColumn,
  TableNode,
  TableRow,
  TemplateNode,
  TextNode,
  WatermarkNode,
} from "./nodes";

export { hasPosition, isContainerNode, isContentNode } from "./nodes";
