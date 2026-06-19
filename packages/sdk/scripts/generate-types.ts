import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

// Resolve __dirname under ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const GO_MODEL_PATH = path.resolve(
  __dirname,
  "../../../apps/api-go/internal/model/model.go",
);
const TS_TYPES_PATH = path.resolve(__dirname, "../src/types.ts");

function goTypeToTs(goType: string): { type: string; optional: boolean } {
  let isPointer = false;
  let cleanType = goType;
  if (goType.startsWith("*")) {
    isPointer = true;
    cleanType = goType.slice(1);
  }

  let tsType = "any";
  switch (cleanType) {
    case "string":
      tsType = "string";
      break;
    case "int":
    case "int64":
    case "float64":
      tsType = "number";
      break;
    case "int32":
      tsType = "number";
      break;
    case "bool":
      tsType = "boolean";
      break;
    case "NullString":
      tsType = "string | null";
      break;
    default:
      // If it's a known model or custom type
      tsType = cleanType;
      break;
  }

  return {
    type: tsType,
    optional: isPointer,
  };
}

function generate() {
  console.log(`Reading Go models from ${GO_MODEL_PATH}...`);
  if (!fs.existsSync(GO_MODEL_PATH)) {
    console.error(`Error: Go model file not found at ${GO_MODEL_PATH}`);
    process.exit(1);
  }

  const goContent = fs.readFileSync(GO_MODEL_PATH, "utf-8");

  // Regex to match structs
  // Matches type StructName struct { ... }
  const structRegex = /type\s+(\w+)\s+struct\s*\{([\s\S]*?)\}/g;

  let match;
  const interfaces: string[] = [];

  while ((match = structRegex.exec(goContent)) !== null) {
    const structName = match[1];
    const structBody = match[2];

    if (structName === "NullString") {
      continue; // Skip NullString wrapper
    }

    let tsInterface = `export interface ${structName} {\n`;

    // Parse body lines
    const lines = structBody.split("\n");
    for (let line of lines) {
      line = line.trim();
      if (!line || line.startsWith("//")) {
        continue;
      }

      // Format: FieldName GoType `json:"json_name"` or similar
      // Regex to match field name, type, and json tag
      const fieldRegex = /^(\w+)\s+([\w\*\.]+)\s+`[^`]*json:"([^"]+)"[^`]*`/;
      const fieldMatch = fieldRegex.exec(line);

      if (fieldMatch) {
        const fieldName = fieldMatch[1];
        const goType = fieldMatch[2];
        const jsonTag = fieldMatch[3];

        // If json tag is "-", the field is ignored
        if (jsonTag === "-") {
          continue;
        }

        // Handle tags like "json_name,omitempty"
        const cleanJsonTag = jsonTag.split(",")[0];

        const { type: tsType, optional } = goTypeToTs(goType);

        // Some specific semantic roles
        let finalType = tsType;
        let isOptional = optional;

        if (structName === "User" && cleanJsonTag === "primary_role") {
          finalType =
            "'super_admin' | 'admin' | 'principal' | 'teacher' | 'student' | 'parent' | 'staff' | 'accountant' | 'librarian'";
        } else if (structName === "User" && cleanJsonTag === "status") {
          finalType =
            "'active' | 'inactive' | 'suspended' | 'pending' | 'banned'";
        } else if (structName === "User" && cleanJsonTag === "sex") {
          finalType = "'male' | 'female' | 'unspecified'";
          isOptional = true;
        } else if (structName === "Enrollment" && cleanJsonTag === "status") {
          finalType =
            "'active' | 'graduated' | 'transferred' | 'withdrawn' | 'suspended' | 'on_leave'";
        } else if (
          structName === "BehaviorRecord" &&
          cleanJsonTag === "severity"
        ) {
          finalType = "'minor' | 'moderate' | 'major' | 'critical'";
        } else if (
          structName === "BehaviorRecord" &&
          cleanJsonTag === "ai_source"
        ) {
          finalType = "'camera_ai' | 'manual' | 'face_recognition'";
          isOptional = true;
        } else if (structName === "AcademicCycle" && cleanJsonTag === "type") {
          finalType = "'year' | 'term' | 'quarter'";
        }

        tsInterface += `  ${cleanJsonTag}${isOptional ? "?" : ""}: ${finalType};\n`;
      }
    }

    tsInterface += "}";
    interfaces.push(tsInterface);
  }

  const outputHeader = `// Orbis Shared Types — Auto-generated TypeScript bindings
// Mirroring apps/api-go/internal/model/model.go schemas
// Generated at: ${new Date().toISOString()}

`;

  const outputContent = outputHeader + interfaces.join("\n\n") + "\n";

  // Ensure target folder exists
  const targetDir = path.dirname(TS_TYPES_PATH);
  if (!fs.existsSync(targetDir)) {
    fs.mkdirSync(targetDir, { recursive: true });
  }

  fs.writeFileSync(TS_TYPES_PATH, outputContent, "utf-8");
  console.log(`Successfully generated TypeScript types at ${TS_TYPES_PATH}`);
}

generate();
