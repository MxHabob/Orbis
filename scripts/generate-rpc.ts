import * as fs from "fs";
import * as path from "path";

const CRATES_DIR = path.resolve(__dirname, "../apps/api-rust/crates");
const SDK_CORE_COMMANDS_DIR = path.resolve(__dirname, "../packages/sdk-core/src/commands");
const TAURI_COMMANDS_DIR = path.resolve(__dirname, "../apps/desktop-svelte/src-tauri/src/commands");

interface Arg {
  name: string;
  rustType: string;
  tsType: string;
  _wasOptionStr?: boolean;
}

interface OpFunction {
  domain: string;
  opFile: string;
  funcName: string; // e.g. "list_classes"
  tsFuncName: string; // e.g. "listClasses"
  args: Arg[];
  rustReturnType: string; // e.g. "Vec<class::Model>"
  tsReturnType: string; // e.g. "Class[]"
}

// Convert snake_case to camelCase
function toCamelCase(str: string): string {
  return str.replace(/_([a-z0-9])/g, (g) => g[1].toUpperCase());
}

// Map Rust types to TS types
function mapType(rustType: string): string {
  if (rustType.includes("serde_json::Value")) return "any";
  if (rustType === "String" || rustType === "&str" || rustType.includes("Uuid")) return "string";
  if (rustType === "bool") return "boolean";
  if (["i32", "i64", "u32", "u64", "f64", "f32", "usize"].includes(rustType)) return "number";
  
  if (rustType.startsWith("Vec<")) {
    const inner = rustType.substring(4, rustType.length - 1);
    return `${mapType(inner)}[]`;
  }
  if (rustType.startsWith("Option<")) {
    const inner = rustType.substring(7, rustType.length - 1);
    return `${mapType(inner)} | null`;
  }
  
  // Custom model mapping
  if (rustType.includes("::Model")) {
    const match = rustType.match(/([a-zA-Z0-9_]+)::Model/);
    if (match) {
      const entityName = match[1];
      const pascalName = toCamelCase(entityName);
      return "Types." + pascalName.charAt(0).toUpperCase() + pascalName.slice(1);
    }
  }

  // Fallback to any for unknown complex types
  return "any";
}

function parseOps(): OpFunction[] {
  const ops: OpFunction[] = [];
  const crates = fs.readdirSync(CRATES_DIR);

  for (const crate of crates) {
    let domain = crate.replace("svc-", ""); // e.g., "academic"
    if (domain === "autopilot" || domain === "ai-gateway") domain = "ai";
    if (domain.includes("-")) domain = domain.replace(/-/g, "_");

    const opsDir = path.join(CRATES_DIR, crate, "src/ops");
    
    if (!fs.existsSync(opsDir)) continue;

    const modRsPath = path.join(opsDir, "mod.rs");
    let exportedMods: string[] = [];
    if (fs.existsSync(modRsPath)) {
        const modRs = fs.readFileSync(modRsPath, "utf-8");
        const modRegex = /pub\s+mod\s+([a-zA-Z0-9_]+);/g;
        let match;
        while ((match = modRegex.exec(modRs)) !== null) {
            exportedMods.push(match[1]);
        }
    }

    const opFiles = fs.readdirSync(opsDir).filter(f => f.endsWith(".rs") && f !== "mod.rs");

    for (const f of opFiles) {
      const opFile = f.replace(".rs", "");
      if (exportedMods.length > 0 && !exportedMods.includes(opFile)) continue;
      const content = fs.readFileSync(path.join(opsDir, f), "utf-8");
      
      // Regex to find: pub async fn <name>(ctx: ..., arg: Type) -> Result<Type, AppError>
      const fnRegex = /pub\s+async\s+fn\s+([a-zA-Z0-9_]+)\s*\(\s*([^)]*)\s*\)\s*->\s*Result<([^,]+),\s*[^>]+>/g;
      
      let match;
      while ((match = fnRegex.exec(content)) !== null) {
        const funcName = match[1];
        const argsRaw = match[2];
        const rustReturnType = match[3].trim();
        
        // Skip functions that don't take Context (they might be internal helpers)
        if (!argsRaw.includes("ServiceContext") && !argsRaw.includes("GuardedContext")) {
          continue;
        }

        const args: Arg[] = [];
        const argParts = argsRaw.split(",").map(s => s.trim()).filter(s => s.length > 0);
        
        let skipFunc = false;
        for (const part of argParts) {
          const colonIdx = part.indexOf(":");
          if (colonIdx === -1) continue;
          let argName = part.slice(0, colonIdx).trim();
          let argTypeRaw = part.slice(colonIdx + 1).trim();
          
          if (argTypeRaw.startsWith("pub ")) {
              argTypeRaw = argTypeRaw.replace("pub ", "");
          }
          if (argName.startsWith("mut ")) {
              argName = argName.replace("mut ", "");
          }
          
          if (argTypeRaw.includes("&dyn ")) {
              skipFunc = true;
              break;
          }
          
          let wasOptionStr = false;
          if (argTypeRaw === "Option<&str>") {
              argTypeRaw = "Option<String>";
              wasOptionStr = true;
          }
          
          if (argName === "ctx" || argName === "_ctx" || argName === "state" || argName === "_state") continue;
          
          args.push({
            name: toCamelCase(argName),
            rustType: argTypeRaw,
            tsType: mapType(argTypeRaw),
            _wasOptionStr: wasOptionStr
          });
        }
        if (skipFunc) continue;

        let tsFuncName = toCamelCase(funcName);
        if (tsFuncName === "list" || tsFuncName === "get" || tsFuncName === "create" || tsFuncName === "update" || tsFuncName === "delete") {
            const camelEntity = opFile.charAt(0).toUpperCase() + opFile.slice(1).replace(/_([a-z0-9])/g, g => g[1].toUpperCase());
            if (tsFuncName === "list") {
                if (camelEntity.endsWith("ss")) tsFuncName = `list${camelEntity}es`;
                else tsFuncName = `list${camelEntity}s`;
            } else {
                tsFuncName = `${tsFuncName}${camelEntity}`;
            }
        }

        ops.push({
          domain,
          opFile,
          funcName,
          tsFuncName,
          args,
          rustReturnType,
          tsReturnType: mapType(rustReturnType)
        });
      }
    }
  }

  return ops;
}

function generateTsInterfaces(ops: OpFunction[]) {
  const byDomain = ops.reduce((acc, op) => {
    if (!acc[op.domain]) acc[op.domain] = [];
    acc[op.domain].push(op);
    return acc;
  }, {} as Record<string, OpFunction[]>);

  for (const [domain, domainOps] of Object.entries(byDomain)) {
    let content = `// AUTO-GENERATED BY scripts/generate-rpc.ts\n`;
    content += `import type * as Types from '../types/index.js';\n\n`;
    
    const domainCamel = domain.charAt(0).toUpperCase() + domain.slice(1);
    content += `export interface ${domainCamel}Commands {\n`;
    
    for (const op of domainOps) {
      const argsStr = op.args.map(a => `${a.name}: ${a.tsType}`).join(", ");
      content += `  ${op.tsFuncName}(${argsStr}): Promise<${op.tsReturnType}>;\n`;
    }
    content += `}\n`;

    fs.writeFileSync(path.join(SDK_CORE_COMMANDS_DIR, `${domain}.ts`), content);
  }

  // Rewrite packages/sdk-core/src/commands/index.ts
  let indexContent = `// AUTO-GENERATED BY scripts/generate-rpc.ts\n`;
  for (const domain of Object.keys(byDomain)) {
      const domainCamel = domain.charAt(0).toUpperCase() + domain.slice(1);
      indexContent += `import type { ${domainCamel}Commands } from './${domain}.js';\n`;
  }
  
  indexContent += `\nexport interface OrbisCommands {\n`;
  for (const domain of Object.keys(byDomain)) {
      const domainCamel = domain.charAt(0).toUpperCase() + domain.slice(1);
      indexContent += `  ${domain}: ${domainCamel}Commands;\n`;
  }
  indexContent += `}\n\n`;
  
  for (const domain of Object.keys(byDomain)) {
      const domainCamel = domain.charAt(0).toUpperCase() + domain.slice(1);
      indexContent += `export type { ${domainCamel}Commands } from './${domain}.js';\n`;
  }
  
  fs.writeFileSync(path.join(SDK_CORE_COMMANDS_DIR, "index.ts"), indexContent);
}

function generateTauriWrappers(ops: OpFunction[]) {
  const byDomain = ops.reduce((acc, op) => {
    if (!acc[op.domain]) acc[op.domain] = [];
    acc[op.domain].push(op);
    return acc;
  }, {} as Record<string, OpFunction[]>);

  let registerList = "";

  for (const [domain, domainOps] of Object.entries(byDomain)) {
    const domainDir = path.join(TAURI_COMMANDS_DIR, domain);
    if (!fs.existsSync(domainDir)) fs.mkdirSync(domainDir, { recursive: true });

    let rsContent = `// AUTO-GENERATED BY scripts/generate-rpc.ts\n`;
    const crate = domain === "enrollment_workflow" ? "svc-enrollment-workflow" : domain === "platform_policy" ? "platform-policy" : `svc-${domain}`;
    const crateName = crate.replace(/-/g, "_");
    rsContent += `use std::sync::Arc;\nuse tauri::State;\nuse crate::state::AppState;\nuse crate::commands::error::{CmdResult, CommandError};\nuse ${crateName}::ops;\n\n`;

    for (const op of domainOps) {
      const rsArgs = [];
      const passArgs = [];
      let deserializeLines = "";

      for (const a of op.args) {
        let isPrimitive = a.rustType === "&str" || a.rustType === "String" || a.rustType.startsWith("Option<String>") || a.rustType === "bool" || a.rustType === "i32" || a.rustType === "u32" || a.rustType === "i64" || a.rustType === "u64" || a.rustType === "f32" || a.rustType === "f64" || a.rustType.startsWith("Option<bool>") || a.rustType.startsWith("Option<i") || a.rustType.startsWith("Option<u") || a.rustType.startsWith("Option<f");
        
        if (isPrimitive) {
          if (a.rustType === "&str") {
            rsArgs.push(`${a.name}: String`);
            passArgs.push(`&${a.name}`);
          } else if (a.rustType === "Option<String>" && a._wasOptionStr) {
            rsArgs.push(`${a.name}: Option<String>`);
            passArgs.push(`${a.name}.as_deref()`);
          } else {
            rsArgs.push(`${a.name}: ${a.rustType}`);
            passArgs.push(`${a.name}`);
          }
        } else {
          // Complex type => receive as serde_json::Value
          if (a.rustType.startsWith("Option<")) {
             rsArgs.push(`${a.name}: Option<serde_json::Value>`);
             deserializeLines += `    let ${a.name}_parsed = match ${a.name} {\n`;
             deserializeLines += `        Some(v) => Some(serde_json::from_value(v).map_err(|e| CommandError::new(e.to_string()))?),\n`;
             deserializeLines += `        None => None,\n`;
             deserializeLines += `    };\n`;
             passArgs.push(`${a.name}_parsed`);
          } else {
             rsArgs.push(`${a.name}: serde_json::Value`);
             deserializeLines += `    let ${a.name}_parsed = serde_json::from_value(${a.name}).map_err(|e| CommandError::new(e.to_string()))?;\n`;
             passArgs.push(`${a.name}_parsed`);
          }
        }
      }

      const rsArgsStr = rsArgs.join(", ");
      const passArgsStr = passArgs.join(", ");

      const tauriFuncName = `${domain}_${op.opFile}_${op.funcName}`;
      
      rsContent += `#[tauri::command]\npub async fn ${tauriFuncName}(state: State<'_, Arc<AppState>>${rsArgsStr.length > 0 ? ', ' + rsArgsStr : ''}) -> CmdResult<serde_json::Value> {\n`;
      rsContent += `    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;\n`;
      rsContent += deserializeLines;
      rsContent += `    let result = ops::${op.opFile}::${op.funcName}(&ctx${passArgsStr.length > 0 ? ', ' + passArgsStr : ''}).await.map_err(CommandError::from)?;\n`;
      
      if (op.rustReturnType === "()") {
        rsContent += `    Ok(serde_json::json!({}))\n`;
      } else {
        rsContent += `    Ok(serde_json::to_value(result).unwrap())\n`;
      }
      rsContent += `}\n\n`;
      
      registerList += `        crate::commands::${domain}::auto::${tauriFuncName},\n`;
    }

    fs.writeFileSync(path.join(domainDir, "auto.rs"), rsContent);
    fs.writeFileSync(path.join(domainDir, "mod.rs"), `pub mod auto;\n`);
  }

  let registerRs = `// AUTO-GENERATED BY scripts/generate-rpc.ts\n`;
  registerRs += `pub fn generate_handlers<R: tauri::Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {\n`;
  registerRs += `    tauri::generate_handler![\n${registerList}    ]\n}\n`;
  fs.writeFileSync(path.join(TAURI_COMMANDS_DIR, "auto_register.rs"), registerRs);
  fs.writeFileSync(path.join(TAURI_COMMANDS_DIR, "mod.rs"), `pub mod error;\n${Object.keys(byDomain).map(d => `pub mod ${d};`).join("\n")}\npub mod auto_register;\n`);
}

function main() {
  const ops = parseOps();
  console.log(`Found ${ops.length} ops.`);
  generateTsInterfaces(ops);
  generateTauriWrappers(ops);
  console.log("Done generating RPC layer.");
}

main();
