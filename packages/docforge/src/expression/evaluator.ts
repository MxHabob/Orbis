// ═══════════════════════════════════════════════════════════════
// @docforge/core — Expression Evaluator
// Evaluates parsed template expressions against a data context
// ═══════════════════════════════════════════════════════════════

import { getFormatter } from "./formatters";
import { parseExpression, type Token } from "./parser";

// ── Data Context ─────────────────────────────────────────────

/**
 * The data context provides variable values for expression evaluation.
 * It is a nested record that supports dot-path resolution.
 */
export type DataContext = Record<string, unknown>;

// ── Evaluator ────────────────────────────────────────────────

/**
 * Evaluate a template string against a data context.
 * Resolves all {{expressions}}, applies formatters, and evaluates conditionals.
 *
 * @param template The template string (e.g., "Hello {{name}}")
 * @param context The data context to resolve variables from
 * @returns The fully resolved string
 */
export function evaluateExpression(
  template: string,
  context: DataContext,
): string {
  const tokens = parseExpression(template);
  return evaluateTokens(tokens, context);
}

function evaluateTokens(tokens: Token[], context: DataContext): string {
  let result = "";

  for (const token of tokens) {
    switch (token.type) {
      case "literal":
        result += token.value;
        break;

      case "variable": {
        let value = resolvePath(context, token.path);

        // Apply formatters in order
        for (const fmt of token.formatters) {
          const formatter = getFormatter(fmt.name);
          if (formatter) {
            value = formatter(value, ...fmt.args);
          }
        }

        result += value != null ? String(value) : "";
        break;
      }

      case "conditional": {
        const condResult = evaluateCondition(token.condition, context);
        if (condResult) {
          result += evaluateExpression(token.thenBranch, context);
        } else if (token.elseBranch) {
          result += evaluateExpression(token.elseBranch, context);
        }
        break;
      }
    }
  }

  return result;
}

// ── Path Resolution ──────────────────────────────────────────

/**
 * Resolve a dot-separated path against a data context.
 * Supports nested access: "enrollment.student.name_ar"
 */
export function resolvePath(context: DataContext, path: string): unknown {
  const parts = path.split(".");
  let current: unknown = context;

  for (const part of parts) {
    if (current == null || typeof current !== "object") {
      return undefined;
    }
    current = (current as Record<string, unknown>)[part];
  }

  return current;
}

// ── Condition Evaluator ──────────────────────────────────────

/**
 * Evaluate a boolean condition expression against a data context.
 *
 * Supported operators: ==, !=, >, >=, <, <=, &&, ||, !
 * Supported values: string literals ('...'), numbers, true, false, null, variable paths
 */
export function evaluateCondition(
  condition: string,
  context: DataContext,
): boolean {
  const trimmed = condition.trim();

  // Handle AND (&&)
  if (trimmed.includes("&&")) {
    const parts = splitTopLevel(trimmed, "&&");
    return parts.every((part) => evaluateCondition(part, context));
  }

  // Handle OR (||)
  if (trimmed.includes("||")) {
    const parts = splitTopLevel(trimmed, "||");
    return parts.some((part) => evaluateCondition(part, context));
  }

  // Handle NOT (!)
  if (trimmed.startsWith("!")) {
    return !evaluateCondition(trimmed.substring(1), context);
  }

  // Handle parentheses
  if (trimmed.startsWith("(") && trimmed.endsWith(")")) {
    return evaluateCondition(trimmed.substring(1, trimmed.length - 1), context);
  }

  // Handle comparison operators
  const compOps = [">=", "<=", "!=", "==", ">", "<"] as const;
  for (const op of compOps) {
    const idx = trimmed.indexOf(op);
    if (idx >= 0) {
      const left = resolveValue(trimmed.substring(0, idx).trim(), context);
      const right = resolveValue(
        trimmed.substring(idx + op.length).trim(),
        context,
      );
      return compareValues(left, right, op);
    }
  }

  // If no operator, treat as truthy check
  const value = resolveValue(trimmed, context);
  return isTruthy(value);
}

/**
 * Split a condition string by a top-level operator,
 * respecting parentheses and string literals.
 */
function splitTopLevel(expr: string, operator: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let inString = false;
  let stringChar = "";
  let current = "";

  for (let i = 0; i < expr.length; i++) {
    const ch = expr[i]!;

    if (inString) {
      current += ch;
      if (ch === stringChar) inString = false;
      continue;
    }

    if (ch === "'" || ch === '"') {
      inString = true;
      stringChar = ch;
      current += ch;
      continue;
    }

    if (ch === "(") {
      depth++;
      current += ch;
      continue;
    }
    if (ch === ")") {
      depth--;
      current += ch;
      continue;
    }

    if (depth === 0 && expr.substring(i).startsWith(operator)) {
      parts.push(current.trim());
      current = "";
      i += operator.length - 1;
      continue;
    }

    current += ch;
  }

  if (current.trim()) parts.push(current.trim());
  return parts;
}

/**
 * Resolve a value token — could be a string literal, number, boolean, null, or variable path.
 */
function resolveValue(token: string, context: DataContext): unknown {
  // String literal
  if (
    (token.startsWith("'") && token.endsWith("'")) ||
    (token.startsWith('"') && token.endsWith('"'))
  ) {
    return token.substring(1, token.length - 1);
  }
  // Number
  if (/^-?\d+(\.\d+)?$/.test(token)) {
    return Number.parseFloat(token);
  }
  // Boolean / null
  if (token === "true") return true;
  if (token === "false") return false;
  if (token === "null" || token === "undefined") return null;
  // Variable path
  return resolvePath(context, token);
}

function compareValues(left: unknown, right: unknown, op: string): boolean {
  switch (op) {
    case "==":
      return left === right;
    case "!=":
      return left !== right;
    case ">":
      return Number(left) > Number(right);
    case ">=":
      return Number(left) >= Number(right);
    case "<":
      return Number(left) < Number(right);
    case "<=":
      return Number(left) <= Number(right);
    default:
      return false;
  }
}

function isTruthy(value: unknown): boolean {
  if (value == null) return false;
  if (typeof value === "boolean") return value;
  if (typeof value === "number") return value !== 0;
  if (typeof value === "string") return value.length > 0;
  if (Array.isArray(value)) return value.length > 0;
  return true;
}
