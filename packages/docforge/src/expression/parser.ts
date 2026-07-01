// ═══════════════════════════════════════════════════════════════
// @docforge/core — Expression Parser
// Tokenizes template strings containing {{expressions}}
// ═══════════════════════════════════════════════════════════════

// ── Token Types ──────────────────────────────────────────────

export type Token = LiteralToken | VariableToken | ConditionalToken;

export interface LiteralToken {
  type: "literal";
  value: string;
}

export interface VariableToken {
  type: "variable";
  path: string;
  formatters: FormatterCall[];
}

export interface FormatterCall {
  name: string;
  args: string[];
}

export interface ConditionalToken {
  type: "conditional";
  condition: string;
  thenBranch: string;
  elseBranch?: string;
}

// ── Parser ───────────────────────────────────────────────────

/**
 * Parse a template string into a sequence of tokens.
 *
 * Supported syntax:
 * - `{{variable.path}}` — variable interpolation
 * - `{{variable | formatter:'arg'}}` — variable with formatter
 * - `{{#if condition}}then{{#else}}else{{/if}}` — conditional
 */
export function parseExpression(template: string): Token[] {
  const tokens: Token[] = [];
  let remaining = template;

  while (remaining.length > 0) {
    // Check for conditional block
    const condMatch = remaining.match(/^\{\{#if\s+(.+?)\}\}/);
    if (condMatch) {
      const condition = condMatch[1]!;
      const afterOpen = remaining.substring(condMatch[0].length);

      // Find matching {{/if}}
      const { thenBranch, elseBranch, rest } = parseConditionalBody(afterOpen);

      tokens.push({
        type: "conditional",
        condition,
        thenBranch,
        elseBranch: elseBranch ?? undefined,
      });

      remaining = rest;
      continue;
    }

    // Check for variable expression
    const varMatch = remaining.match(/^\{\{(.+?)\}\}/);
    if (varMatch) {
      const expr = varMatch[1]!.trim();
      tokens.push(parseVariableExpression(expr));
      remaining = remaining.substring(varMatch[0].length);
      continue;
    }

    // Find the next {{ or end of string
    const nextOpen = remaining.indexOf("{{");
    if (nextOpen === -1) {
      tokens.push({ type: "literal", value: remaining });
      break;
    }

    if (nextOpen > 0) {
      tokens.push({ type: "literal", value: remaining.substring(0, nextOpen) });
    }
    remaining = remaining.substring(nextOpen);
  }

  return tokens;
}

/**
 * Parse a variable expression (the content between {{ and }}).
 * Handles pipe-based formatters: `variable | format:'arg1':'arg2'`
 */
function parseVariableExpression(expr: string): VariableToken {
  const parts = expr.split("|").map((p) => p.trim());
  const path = parts[0]!;
  const formatters: FormatterCall[] = [];

  for (let i = 1; i < parts.length; i++) {
    const fmtStr = parts[i]!;
    const colonIdx = fmtStr.indexOf(":");
    if (colonIdx === -1) {
      formatters.push({ name: fmtStr, args: [] });
    } else {
      const name = fmtStr.substring(0, colonIdx).trim();
      const argsStr = fmtStr.substring(colonIdx + 1);
      const args = argsStr
        .split(":")
        .map((a) => a.trim().replace(/^['"]|['"]$/g, ""));
      formatters.push({ name, args });
    }
  }

  return { type: "variable", path, formatters };
}

/**
 * Parse the body of a conditional block.
 * Handles nested conditionals and {{#elseif}}/{{#else}}.
 */
function parseConditionalBody(input: string): {
  thenBranch: string;
  elseBranch: string | null;
  rest: string;
} {
  let depth = 1;
  let i = 0;
  let thenBranch = "";
  let elseBranch: string | null = null;
  let inElse = false;

  while (i < input.length && depth > 0) {
    // Check for nested {{#if
    if (input.substring(i).startsWith("{{#if ")) {
      depth++;
      const token = input.substring(i).match(/^\{\{#if\s+.+?\}\}/)!;
      const chunk = token[0];
      if (inElse) {
        elseBranch = (elseBranch ?? "") + chunk;
      } else {
        thenBranch += chunk;
      }
      i += chunk.length;
      continue;
    }

    // Check for {{/if}}
    if (input.substring(i).startsWith("{{/if}}")) {
      depth--;
      if (depth === 0) {
        i += "{{/if}}".length;
        break;
      }
      const chunk = "{{/if}}";
      if (inElse) {
        elseBranch = (elseBranch ?? "") + chunk;
      } else {
        thenBranch += chunk;
      }
      i += chunk.length;
      continue;
    }

    // Check for {{#else}} at current depth
    if (depth === 1 && input.substring(i).startsWith("{{#else}}")) {
      inElse = true;
      elseBranch = "";
      i += "{{#else}}".length;
      continue;
    }

    // Check for {{#elseif ...}} at current depth
    const elseifMatch = input.substring(i).match(/^\{\{#elseif\s+(.+?)\}\}/);
    if (depth === 1 && elseifMatch) {
      // Treat elseif as a nested if in the else branch
      inElse = true;
      elseBranch = `{{#if ${elseifMatch[1]}}}`;
      i += elseifMatch[0].length;
      continue;
    }

    // Regular character
    const ch = input[i]!;
    if (inElse) {
      elseBranch = (elseBranch ?? "") + ch;
    } else {
      thenBranch += ch;
    }
    i++;
  }

  return { thenBranch, elseBranch, rest: input.substring(i) };
}

// ── Variable Extraction ──────────────────────────────────────

/**
 * Extract all variable paths referenced in a template string.
 * Does NOT include system variables ($system.*, $org.*).
 */
export function extractVariableRefs(template: string): string[] {
  const refs = new Set<string>();
  const tokens = parseExpression(template);

  for (const token of tokens) {
    if (token.type === "variable") {
      if (!token.path.startsWith("$")) {
        refs.add(token.path);
      }
    } else if (token.type === "conditional") {
      // Extract from condition expression
      const condVars = extractConditionVars(token.condition);
      for (const v of condVars) refs.add(v);

      // Extract from branches
      for (const v of extractVariableRefs(token.thenBranch)) refs.add(v);
      if (token.elseBranch) {
        for (const v of extractVariableRefs(token.elseBranch)) refs.add(v);
      }
    }
  }

  return Array.from(refs);
}

/** Extract variable references from a condition expression */
function extractConditionVars(condition: string): string[] {
  // Match word.path patterns that aren't keywords or numbers
  const keywords = new Set([
    "true",
    "false",
    "null",
    "undefined",
    "and",
    "or",
    "not",
  ]);
  const matches = condition.match(/[a-zA-Z_]\w*(?:\.\w+)*/g) ?? [];
  return matches.filter(
    (m) => !keywords.has(m) && !m.startsWith("$") && !/^\d/.test(m),
  );
}
