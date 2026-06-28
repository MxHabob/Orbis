// ═══════════════════════════════════════════════════════════════
// @docforge/core — Document Validation Engine
// Validates template documents for correctness and completeness
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument } from "../model/document";
import type { TemplateNode } from "../model/nodes";
import { isContainerNode } from "../model/nodes";
import { walkNodes } from "../operations/node-ops";
import { extractVariableRefs } from "../expression/parser";

// ── Validation Result Types ──────────────────────────────────

export interface ValidationResult {
  isValid: boolean;
  errors: ValidationIssue[];
  warnings: ValidationIssue[];
}

export interface ValidationIssue {
  /** Node ID where the issue was found (if applicable) */
  nodeId?: string;
  /** Property/field name */
  field?: string;
  /** Machine-readable error code */
  code: string;
  /** English message */
  message: string;
  /** Arabic message */
  messageAr: string;
  /** Severity */
  severity: "error" | "warning" | "info";
}

// ── Validator ────────────────────────────────────────────────

/**
 * Validate a template document for correctness.
 * Checks structure, expressions, layout constraints, and print readiness.
 */
export function validateDocument(doc: TemplateDocument): ValidationResult {
  const errors: ValidationIssue[] = [];
  const warnings: ValidationIssue[] = [];

  // 1. Schema validation
  validateSchema(doc, errors);

  // 2. Expression validation
  validateExpressions(doc, errors, warnings);

  // 3. Layout validation
  validateLayout(doc, warnings);

  // 4. Print readiness
  validatePrintReadiness(doc, warnings);

  return {
    isValid: errors.length === 0,
    errors,
    warnings,
  };
}

// ── Schema Validation ────────────────────────────────────────

function validateSchema(doc: TemplateDocument, errors: ValidationIssue[]): void {
  if (!doc.version) {
    errors.push({
      code: "MISSING_VERSION",
      message: "Document is missing version field",
      messageAr: "المستند يفتقر إلى حقل الإصدار",
      severity: "error",
    });
  }

  if (!doc.meta?.documentType) {
    errors.push({
      code: "MISSING_DOCUMENT_TYPE",
      message: "Document type is not specified",
      messageAr: "نوع المستند غير محدد",
      severity: "error",
    });
  }

  if (!doc.meta?.entityType) {
    errors.push({
      code: "MISSING_ENTITY_TYPE",
      message: "Entity type is not specified",
      messageAr: "نوع الكيان غير محدد",
      severity: "error",
    });
  }

  if (!doc.meta?.pageSetup) {
    errors.push({
      code: "MISSING_PAGE_SETUP",
      message: "Page setup is not configured",
      messageAr: "إعداد الصفحة غير مهيأ",
      severity: "error",
    });
  }

  if (doc.root?.type !== "Page") {
    errors.push({
      code: "INVALID_ROOT",
      message: "Root node must be of type 'Page'",
      messageAr: "العقدة الجذرية يجب أن تكون من نوع 'Page'",
      severity: "error",
    });
  }

  // Check for duplicate node IDs
  const nodeIds = new Set<string>();
  walkNodes(doc, (node) => {
    if (nodeIds.has(node.id)) {
      errors.push({
        nodeId: node.id,
        code: "DUPLICATE_ID",
        message: `Duplicate node ID: ${node.id}`,
        messageAr: `معرف عقدة مكرر: ${node.id}`,
        severity: "error",
      });
    }
    nodeIds.add(node.id);
  });
}

// ── Expression Validation ────────────────────────────────────

function validateExpressions(
  doc: TemplateDocument,
  errors: ValidationIssue[],
  warnings: ValidationIssue[],
): void {
  const definedVars = new Set(Object.keys(doc.variables));

  walkNodes(doc, (node) => {
    // Validate text content expressions
    if (node.type === "Text" && "props" in node) {
      const content = (node.props as { content?: string }).content;
      if (content) {
        const refs = extractVariableRefs(content);
        for (const ref of refs) {
          const rootVar = ref.split(".")[0]!;
          if (!definedVars.has(rootVar) && !definedVars.has(ref)) {
            warnings.push({
              nodeId: node.id,
              field: "content",
              code: "UNDEFINED_VARIABLE",
              message: `Variable '${ref}' is used but not defined`,
              messageAr: `المتغير '${ref}' مستخدم ولكن غير معرّف`,
              severity: "warning",
            });
          }
        }
      }
    }

    // Validate conditional expressions
    if (node.type === "Conditional" && "props" in node) {
      const condition = (node.props as { condition?: string }).condition;
      if (!condition || condition.trim() === "") {
        errors.push({
          nodeId: node.id,
          field: "condition",
          code: "EMPTY_CONDITION",
          message: "Conditional node has an empty condition",
          messageAr: "عقدة الشرط لا تحتوي على تعبير شرطي",
          severity: "error",
        });
      }
    }

    // Validate repeater data source
    if (node.type === "Repeater" && "props" in node) {
      const dataSource = (node.props as { dataSource?: string }).dataSource;
      if (!dataSource || dataSource.trim() === "") {
        errors.push({
          nodeId: node.id,
          field: "dataSource",
          code: "EMPTY_DATA_SOURCE",
          message: "Repeater node has no data source",
          messageAr: "عقدة التكرار لا تحتوي على مصدر بيانات",
          severity: "error",
        });
      }
    }
  });
}

// ── Layout Validation ────────────────────────────────────────

function validateLayout(doc: TemplateDocument, warnings: ValidationIssue[]): void {
  if (!doc.meta?.pageSetup) return;

  const { pageSetup } = doc.meta;
  const pageWidth = pageSetup.width ?? (pageSetup.size === "A4" ? 210 : 210);
  const pageHeight = pageSetup.height ?? (pageSetup.size === "A4" ? 297 : 297);
  const margins = pageSetup.margins;

  const printableWidth = pageWidth - margins.left - margins.right;
  const printableHeight = pageHeight - margins.top - margins.bottom;

  walkNodes(doc, (node) => {
    if ("props" in node) {
      const position = (node.props as { position?: { x: number; y: number; width: number | string; height?: number | string } }).position;
      if (position && typeof position.width === "number") {
        if (position.x + position.width > pageWidth) {
          warnings.push({
            nodeId: node.id,
            field: "position",
            code: "OUTSIDE_PAGE",
            message: `Node extends beyond page width (${position.x + position.width}mm > ${pageWidth}mm)`,
            messageAr: `العقدة تتجاوز عرض الصفحة`,
            severity: "warning",
          });
        }
      }
    }
  });
}

// ── Print Readiness ──────────────────────────────────────────

function validatePrintReadiness(doc: TemplateDocument, warnings: ValidationIssue[]): void {
  walkNodes(doc, (node) => {
    if (node.type === "Text" && "props" in node) {
      const fontSize = (node.props as { fontSize?: number }).fontSize;
      if (fontSize !== undefined && fontSize < 6) {
        warnings.push({
          nodeId: node.id,
          field: "fontSize",
          code: "SMALL_FONT",
          message: `Font size ${fontSize}pt may be too small for print`,
          messageAr: `حجم الخط ${fontSize}pt قد يكون صغيراً جداً للطباعة`,
          severity: "warning",
        });
      }
    }
  });
}
