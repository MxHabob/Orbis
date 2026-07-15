// ═══════════════════════════════════════════════════════════════
// @docforge/core — Schema Registry & Validation
// ═══════════════════════════════════════════════════════════════

import type { TemplateDocument } from "../model/document";

export interface DataSchema {
  /** The schema version */
  $schema?: string;
  /** Schema identifier */
  $id?: string;
  /** Type of the root object (usually 'object') */
  type: string;
  /** Object properties */
  properties?: Record<string, DataSchemaProperty>;
  /** Required fields */
  required?: string[];
}

export interface DataSchemaProperty {
  type: "string" | "number" | "boolean" | "object" | "array";
  description?: string;
  properties?: Record<string, DataSchemaProperty>;
  items?: DataSchemaProperty;
}

/**
 * Validates the variable bindings in a document against the provided data schema.
 * Returns an array of validation errors, or empty array if valid.
 */
export function validateBindings(doc: TemplateDocument, schema: DataSchema): string[] {
  const errors: string[] = [];

  for (const [varName, def] of Object.entries(doc.variables)) {
    // Skip computed variables as they don't map directly to the schema
    if (def.source === "$computed") {
      continue;
    }

    const pathParts = def.source.split(".");
    const isValid = checkPathExists(schema, pathParts);
    if (!isValid) {
      errors.push(`Variable '${varName}' binds to invalid source path '${def.source}'`);
    }
  }

  return errors;
}

function checkPathExists(schema: DataSchema, path: string[]): boolean {
  let current: DataSchemaProperty | DataSchema | undefined = schema;

  for (const part of path) {
    if (!current || (current.type !== "object" && current.type !== "array")) {
      return false;
    }
    
    if (current.type === "object" && current.properties) {
      current = current.properties[part];
    } else if (current.type === "array" && current.items) {
      current = current.items;
      // In arrays we might be accessing properties of the array items directly in a repeater
    } else {
      return false;
    }
  }

  return current !== undefined;
}
