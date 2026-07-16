import * as fs from "fs";

function mapRustTypeToTs(rustType: string): string {
  rustType = rustType.trim();
  if (rustType.startsWith("Option<")) {
    const inner = rustType.slice(7, -1);
    return `${mapRustTypeToTs(inner)} | null`;
  }
  if (rustType.startsWith("Vec<")) {
    const inner = rustType.slice(4, -1);
    return `${mapRustTypeToTs(inner)}[]`;
  }
  if (["String", "&str", "Uuid", "chrono::NaiveDate", "NaiveDate", "chrono::DateTime<Utc>", "DateTime<Utc>", "Uuid"].includes(rustType)) return "string";
  if (["i32", "i64", "u32", "u64", "f32", "f64", "usize"].includes(rustType)) return "number";
  if (rustType === "bool") return "boolean";
  if (rustType.includes("serde_json::Value") || rustType === "Value") return "any";
  
  // Custom types
  return rustType;
}

function parseRustFile(code: string) {
  const tsInterfaces = [];
  
  // Remove comments to avoid false positives
  code = code.replace(/\/\/[^\n]*\n/g, "\n").replace(/\/\*[\s\S]*?\*\//g, "");

  const structRegex = /pub\s+struct\s+([A-Za-z0-9_]+)(?:<[^>]+>)?\s*\{([^}]*)\}/g;
  let match;
  while ((match = structRegex.exec(code)) !== null) {
    const name = match[1];
    const fieldsRaw = match[2];
    
    let tsInterface = `export interface ${name} {\n`;
    
    const fieldRegex = /pub\s+([A-Za-z0-9_]+)\s*:\s*([^,]+),/g;
    let fieldMatch;
    let foundFields = false;
    while ((fieldMatch = fieldRegex.exec(fieldsRaw)) !== null) {
      foundFields = true;
      const fieldName = fieldMatch[1];
      const fieldType = mapRustTypeToTs(fieldMatch[2]);
      tsInterface += `  ${fieldName}: ${fieldType};\n`;
    }
    tsInterface += `}\n`;
    
    if (foundFields || fieldsRaw.trim() === "") {
        tsInterfaces.push(tsInterface);
    }
  }

  const enumRegex = /pub\s+enum\s+([A-Za-z0-9_]+)\s*\{([^}]*)\}/g;
  while ((match = enumRegex.exec(code)) !== null) {
    const name = match[1];
    const variantsRaw = match[2];
    
    let tsEnum = `export type ${name} = `;
    const variantRegex = /([A-Za-z0-9_]+)/g;
    let variantMatch;
    const variants = [];
    while ((variantMatch = variantRegex.exec(variantsRaw)) !== null) {
      variants.push(`'${variantMatch[1]}'`);
    }
    if (variants.length > 0) {
      tsEnum += variants.join(" | ") + ";\n";
      tsInterfaces.push(tsEnum);
    }
  }

  return tsInterfaces.join("\n");
}

const file = fs.readFileSync("apps/api-rust/crates/docforge-orbis/src/template_service.rs", "utf8");
console.log(parseRustFile(file));
