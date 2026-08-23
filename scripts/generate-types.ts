import * as fs from "fs";
import * as path from "path";

const WORKSPACE_ROOT = path.resolve(__dirname, "..");
const API_CRATES_DIR = path.join(WORKSPACE_ROOT, "apps", "api-rust", "crates");
const SDK_CORE_TYPES_DIR = path.join(WORKSPACE_ROOT, "packages", "sdk-core", "src", "types");

interface Field {
  name: string;
  rustType: string;
  tsType: string;
  isOptional: boolean;
}

interface Struct {
  name: string;
  fields: Field[];
}

function toCamelCase(str: string): string {
  return str.replace(/_([a-z0-9])/g, (_, letter) => letter.toUpperCase());
}

function toPascalCase(str: string): string {
  const camel = toCamelCase(str);
  return camel.charAt(0).toUpperCase() + camel.slice(1);
}

function mapRustTypeToTs(rustType: string): string {
  let t = rustType.trim();

  // Strip lifetimes and references
  t = t.replace(/&'[a-zA-Z]+\s+/g, "");
  t = t.replace(/&mut\s+/g, "");
  t = t.replace(/&\s*/g, "");

  if (t.startsWith("Option<")) {
    const inner = t.substring(7, t.lastIndexOf(">")).trim();
    return mapRustTypeToTs(inner);
  }

  if (t.startsWith("Vec<")) {
    const inner = t.substring(4, t.lastIndexOf(">")).trim();
    return `${mapRustTypeToTs(inner)}[]`;
  }

  if (
    t === "String" ||
    t === "&str" ||
    t === "str" ||
    t === "Uuid" ||
    t.includes("Date") ||
    t.includes("Time") ||
    t.includes("Timestamp")
  ) {
    return "string";
  }

  if (t === "bool" || t === "boolean") return "boolean";

  if (
    t === "i8" ||
    t === "i16" ||
    t === "i32" ||
    t === "i64" ||
    t === "i128" ||
    t === "u8" ||
    t === "u16" ||
    t === "u32" ||
    t === "u64" ||
    t === "u128" ||
    t === "f32" ||
    t === "f64" ||
    t === "Decimal" ||
    t === "usize" ||
    t === "isize"
  ) {
    return "number";
  }

  if (
    t === "serde_json::Value" ||
    t === "Value" ||
    t === "Json" ||
    t === "JsonValue"
  ) {
    return "any";
  }

  if (t === "()" || t === "void") return "void";

  // Fallback for custom enums/identifiers
  if (/^[A-Za-z0-9_]+$/.test(t)) {
    return t;
  }

  return "any";
}

function parseRustEntities(): Record<string, Struct[]> {
  const cratesOutput: Record<string, Struct[]> = {};
  if (!fs.existsSync(API_CRATES_DIR)) {
    console.error(`API crates directory not found at: ${API_CRATES_DIR}`);
    return cratesOutput;
  }

  const crates = fs.readdirSync(API_CRATES_DIR);

  for (const crate of crates) {
    if (
      !crate.startsWith("svc-") &&
      !crate.startsWith("platform-") &&
      !crate.startsWith("shared-")
    ) {
      continue;
    }

    const entitiesDir = path.join(API_CRATES_DIR, crate, "src", "entities");
    if (!fs.existsSync(entitiesDir)) continue;

    let domain = crate.replace(/^svc-/, "").replace(/^platform-/, "");
    if (domain === "enrollment-workflow") domain = "enrollment_workflow";
    if (domain === "policy") domain = "platform_policy";

    if (!cratesOutput[domain]) cratesOutput[domain] = [];

    const files = fs.readdirSync(entitiesDir);
    for (const file of files) {
      if (!file.endsWith(".rs") || file === "mod.rs") continue;

      const content = fs.readFileSync(path.join(entitiesDir, file), "utf8");

      // Check for custom rename via schema attribute: #[schema(as = X)]
      let entityName = toPascalCase(file.replace(".rs", ""));
      const schemaMatch = content.match(/#\[schema\(as\s*=\s*([A-Za-z0-9_]+)\)\]/);
      if (schemaMatch) {
        entityName = schemaMatch[1];
      }

      // Find pub struct Model
      const structIndex = content.indexOf("pub struct Model");
      if (structIndex === -1) continue;

      const fields: Field[] = [];
      const lines = content.substring(structIndex).split("\n");

      let insideStruct = false;
      for (const line of lines) {
        if (line.includes("pub struct Model")) {
          insideStruct = true;
          continue;
        }
        if (insideStruct && line.trim() === "}") {
          break;
        }

        if (insideStruct) {
          const match = line.match(/pub\s+([a-zA-Z0-9_]+):\s*(.+?),/);
          if (match) {
            const name = match[1];
            const rawRustType = match[2].trim();
            const isOptional = rawRustType.startsWith("Option<");
            const tsType = mapRustTypeToTs(rawRustType);

            fields.push({
              name,
              rustType: rawRustType,
              tsType,
              isOptional,
            });
          }
        }
      }

      if (fields.length > 0) {
        cratesOutput[domain].push({ name: entityName, fields });
      }
    }
  }

  return cratesOutput;
}

function generateTypescript() {
  console.log("Parsing Rust Entities...");
  const parsedData = parseRustEntities();

  if (!fs.existsSync(SDK_CORE_TYPES_DIR)) {
    fs.mkdirSync(SDK_CORE_TYPES_DIR, { recursive: true });
  }

  let indexExports = "";

  // Clear out existing auto-generated type files
  const existingFiles = fs.readdirSync(SDK_CORE_TYPES_DIR);
  for (const file of existingFiles) {
    if (file.endsWith(".ts")) {
      fs.unlinkSync(path.join(SDK_CORE_TYPES_DIR, file));
    }
  }

  for (const [domain, structs] of Object.entries(parsedData)) {
    if (structs.length === 0) continue;

    let tsContent = `// AUTO-GENERATED BY scripts/generate-types.ts — DO NOT EDIT DIRECTLY\n\n`;

    for (const struct of structs) {
      tsContent += `export interface ${struct.name} {\n`;
      for (const field of struct.fields) {
        const opt = field.isOptional ? "?" : "";
        const nullType = field.isOptional ? " | null" : "";
        tsContent += `  ${field.name}${opt}: ${field.tsType}${nullType};\n`;
      }
      tsContent += `}\n\n`;
    }

    const filePath = path.join(SDK_CORE_TYPES_DIR, `${domain}.ts`);
    fs.writeFileSync(filePath, tsContent);

    indexExports += `export * from './${domain}.js';\n`;
  }

  // Create an index file to export all domain types
  const indexPath = path.join(SDK_CORE_TYPES_DIR, "index.ts");
  fs.writeFileSync(
    indexPath,
    `// AUTO-GENERATED BY scripts/generate-types.ts — DO NOT EDIT DIRECTLY\n\n${indexExports}`,
  );

  console.log("✓ Types successfully generated into packages/sdk-core/src/types/");
}

generateTypescript();
