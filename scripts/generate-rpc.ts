import * as fs from "fs";
import * as path from "path";

const WORKSPACE_ROOT = path.resolve(__dirname, "..");
const API_CRATES_DIR = path.join(WORKSPACE_ROOT, "apps", "api-rust", "crates");
const SDK_CORE_COMMANDS_DIR = path.join(WORKSPACE_ROOT, "packages", "sdk-core", "src", "commands");
const SDK_CORE_TYPES_DIR = path.join(WORKSPACE_ROOT, "packages", "sdk-core", "src", "types");
const TAURI_COMMANDS_DIR = path.join(WORKSPACE_ROOT, "apps", "desktop-svelte", "src-tauri", "src", "commands");
const SDK_DESKTOP_DIR = path.join(WORKSPACE_ROOT, "packages", "sdk-desktop", "src");

const DOMAINS = [
  "iam", "organization", "academic", "assessment",
  "scheduling", "finance", "exam", "document",
  "calendar", "enrollment_workflow", "platform_policy"
];

interface RustField {
  name: string;
  rustType: string;
}

interface RustStruct {
  name: string;
  fields: RustField[];
}

interface RustEnum {
  name: string;
  variants: string[];
}

interface ParsedTypes {
  structs: RustStruct[];
  enums: RustEnum[];
}

function toCamelCase(str: string): string {
  return str.replace(/_([a-z0-9])/g, g => g[1].toUpperCase());
}

function mapRustTypeToTs(rustType: string): string {
  rustType = rustType.trim();
  rustType = rustType.replace("&str", "string").replace("String", "string");
  rustType = rustType.replace(/\bbool\b/g, "boolean");
  rustType = rustType.replace(/i32|u32|i64|u64|f32|f64|usize|isize/g, "number");
  rustType = rustType.replace("serde_json::Value", "any");
  rustType = rustType.replace("()", "void");
  
  // strip lifetimes and references
  rustType = rustType.replace(/&'[a-zA-Z]+\s+/g, "");
  rustType = rustType.replace(/&mut\s+/g, "");
  rustType = rustType.replace(/&\s*/g, "");
  
  if (rustType.startsWith("Option<")) {
      const inner = rustType.match(/Option<(.*)>/)?.[1] || "any";
      return `${mapRustTypeToTs(inner)} | null`;
  }
  if (rustType.startsWith("Vec<")) {
      const inner = rustType.match(/Vec<(.*)>/)?.[1] || "any";
      return `${mapRustTypeToTs(inner)}[]`;
  }
  
  if (rustType.includes("tauri::ipc::Channel")) return "any";
  if (rustType.includes("State<")) return "any";
  if (rustType.includes("::")) return "any";

  return rustType;
}

function isPrimitiveTs(ts: string) {
    if (ts.includes("any") || ts.includes("string") || ts.includes("number") || ts.includes("boolean") || ts.includes("void")) return true;
    return false;
}

function formatTsType(ts: string, knownTypes: Set<string>) {
    if (isPrimitiveTs(ts)) return ts;
    let isArray = false;
    let isNullable = false;
    let base = ts;
    if (base.endsWith("[]")) {
        isArray = true;
        base = base.slice(0, -2);
    }
    if (base.endsWith(" | null")) {
        isNullable = true;
        base = base.replace(" | null", "");
    }
    
    if (!knownTypes.has(base)) {
        base = "any";
    } else {
        base = `Types.${base}`;
    }
    
    if (isArray) return `${base}[]`;
    if (isNullable) return `${base} | null`;
    return base;
}

// ============================================================================
// KNOWN TYPES DISCOVERY
// ============================================================================

function getKnownTypes(): Set<string> {
  const knownTypes = new Set<string>();
  if (!fs.existsSync(SDK_CORE_TYPES_DIR)) return knownTypes;

  const files = fs.readdirSync(SDK_CORE_TYPES_DIR);
  for (const file of files) {
    if (!file.endsWith(".ts")) continue;
    const content = fs.readFileSync(path.join(SDK_CORE_TYPES_DIR, file), "utf8");
    const regex = /export\s+(?:interface|type)\s+([A-Za-z0-9_]+)/g;
    let match;
    while ((match = regex.exec(content)) !== null) {
      knownTypes.add(match[1]);
    }
  }
  return knownTypes;
}

// ============================================================================
// COMMAND GENERATION
// ============================================================================

interface OpFunction {
  domain: string;
  opFile: string;
  funcName: string;
  tsFuncName: string;
  args: { name: string; rustName: string; rustType: string; tsType: string; _wasOptionStr: boolean }[];
  rustReturnType: string;
  tsReturnType: string;
  isCustom: boolean;
  _passArgsToBackend?: string[];
}

function getEntityName(opFile: string): { singular: string; plural: string } {
  let singular = opFile.charAt(0).toUpperCase() + opFile.slice(1).replace(/_([a-z0-9])/g, g => g[1].toUpperCase());

  if (opFile === "journal") singular = "JournalEntry";
  else if (opFile === "audit") singular = "AuditLog";
  else if (opFile.startsWith("bulk_export")) singular = "Export";
  else if (opFile.startsWith("bulk_import")) singular = "Import";

  let plural = singular;
  if (singular.endsWith("y") && !/[aeiou]y$/i.test(singular)) {
    plural = singular.slice(0, -1) + "ies";
  } else if (singular.endsWith("s") || singular.endsWith("x") || singular.endsWith("z") || singular.endsWith("ch") || singular.endsWith("sh")) {
    plural = singular + "es";
  } else {
    plural = singular + "s";
  }

  return { singular, plural };
}

function parseOps(): OpFunction[] {
  const ops: OpFunction[] = [];
  const byDomain: Record<string, string[]> = {};

  for (const domain of DOMAINS) {
    const crate = domain === "enrollment_workflow" ? "svc-enrollment-workflow" : domain === "platform_policy" ? "platform-policy" : `svc-${domain}`;
    const crateDir = path.join(API_CRATES_DIR, crate, "src", "ops");
    if (!fs.existsSync(crateDir)) continue;

    const files = fs.readdirSync(crateDir).filter(f => {
      if (!f.endsWith(".rs") || f === "mod.rs" || f.endsWith("_core.rs")) return false;
      const modContent = fs.readFileSync(path.join(crateDir, "mod.rs"), "utf8");
      return modContent.includes(`pub mod ${f.replace(".rs", "")};`);
    });
    byDomain[domain] = files.map(f => f.replace(".rs", ""));

    for (const file of files) {
      const opFile = file.replace(".rs", "");
      const content = fs.readFileSync(path.join(crateDir, file), "utf8");
      const entity = getEntityName(opFile);

      const fnRegex = /pub\s+async\s+fn\s+([a-zA-Z0-9_]+)\s*\(\s*([\s\S]*?)\)\s*->\s*Result<([\s\S]*?),\s*AppError>/g;
      let match;
      while ((match = fnRegex.exec(content)) !== null) {
        const funcName = match[1];
        const argsStr = match[2];
        const rustReturnType = match[3].trim();
        if (rustReturnType === "Self" || rustReturnType.endsWith("::Self")) continue;

        const argsRaw = argsStr.split(",").map(a => a.trim()).filter(a => a.length > 0);
        const args = [];
        const passArgsToBackend: string[] = [];
        let skipFunc = false;

        for (const raw of argsRaw) {
          const parts = raw.split(":");
          if (parts.length < 2) continue;
          let argName = parts[0].trim();
          if (argName === "state" || argName === "_state") continue;
          if (argName === "ctx" || argName === "_ctx") {
              passArgsToBackend.push("&ctx");
              continue;
          }
          if (argName === "db") {
              passArgsToBackend.push("&ctx.db");
              continue;
          }
          if (argName.startsWith("mut ")) argName = argName.substring(4);
          let argTypeRaw = parts.slice(1).join(":").trim();

          if (argTypeRaw.includes("&dyn ")) {
              skipFunc = true;
              break;
          }
          
          let wasOptionStr = false;
          if (argTypeRaw === "Option<&str>") {
              argTypeRaw = "Option<String>";
              wasOptionStr = true;
          }
          
          args.push({
            name: toCamelCase(argName),
            rustName: argName,
            rustType: argTypeRaw,
            tsType: mapRustTypeToTs(argTypeRaw),
            _wasOptionStr: wasOptionStr
          });
          
          let isPrimitive = argTypeRaw === "&str" || argTypeRaw === "String" || argTypeRaw.startsWith("Option<String>") || argTypeRaw === "bool" || argTypeRaw === "i32" || argTypeRaw === "u32" || argTypeRaw === "i64" || argTypeRaw === "u64" || argTypeRaw === "f32" || argTypeRaw === "f64" || argTypeRaw.startsWith("Option<bool>") || argTypeRaw.startsWith("Option<i") || argTypeRaw.startsWith("Option<u") || argTypeRaw.startsWith("Option<f");
          if (isPrimitive) {
              if (argTypeRaw === "&str") {
                  passArgsToBackend.push(`&${argName}`);
              } else if (argTypeRaw === "Option<String>" && wasOptionStr) {
                  passArgsToBackend.push(`${argName}.as_deref()`);
              } else {
                  passArgsToBackend.push(argName);
              }
          } else {
              passArgsToBackend.push(`${argName}_parsed`);
          }
        }
        if (skipFunc) continue;

        let tsFuncName = toCamelCase(funcName);
        if (tsFuncName === "list" || tsFuncName === "get" || tsFuncName === "create" || tsFuncName === "update" || tsFuncName === "delete") {
            if (tsFuncName === "list") {
                tsFuncName = `list${entity.plural}`;
            } else {
                tsFuncName = `${tsFuncName}${entity.singular}`;
            }
        } else if (tsFuncName === "listPaginated") {
            tsFuncName = `list${entity.plural}Paginated`;
        } else if (tsFuncName.startsWith("listBy")) {
            const rest = tsFuncName.replace("listBy", "");
            tsFuncName = `list${entity.plural}By${rest}`;
        } else if (tsFuncName.startsWith("getBy")) {
            const rest = tsFuncName.replace("getBy", "");
            tsFuncName = `get${entity.singular}By${rest}`;
        } else if (tsFuncName === "listItems") {
            tsFuncName = `list${entity.singular}Items`;
        } else if (tsFuncName === "createItem") {
            tsFuncName = `create${entity.singular}Item`;
        } else if (tsFuncName === "updateItem") {
            tsFuncName = `update${entity.singular}Item`;
        } else if (tsFuncName === "deleteItem") {
            tsFuncName = `delete${entity.singular}Item`;
        } else if (tsFuncName === "getItem") {
            tsFuncName = `get${entity.singular}Item`;
        } else if (opFile === "bulk_export") {
            tsFuncName = `export${tsFuncName.charAt(0).toUpperCase() + tsFuncName.slice(1)}`;
        } else if (opFile === "bulk_import") {
            tsFuncName = `import${tsFuncName.charAt(0).toUpperCase() + tsFuncName.slice(1)}`;
        } else if (opFile === "dashboard" && funcName === "get_stats") {
            tsFuncName = "getDashboardStats";
        }

        ops.push({
          domain,
          opFile,
          funcName,
          tsFuncName,
          args,
          rustReturnType,
          tsReturnType: mapRustTypeToTs(rustReturnType),
          isCustom: false,
          _passArgsToBackend: passArgsToBackend
        });
      }
    }
  }

  const commandsDirs = fs.readdirSync(TAURI_COMMANDS_DIR);
  for (const dir of commandsDirs) {
      const fullDir = path.join(TAURI_COMMANDS_DIR, dir);
      if (!fs.statSync(fullDir).isDirectory()) continue;
      
      const customFile = path.join(fullDir, "custom.rs");
      if (fs.existsSync(customFile)) {
          const content = fs.readFileSync(customFile, "utf8");
          const customFnRegex = /#\[tauri::command\][\s\S]*?pub\s+async\s+fn\s+([a-zA-Z0-9_]+)\s*\(\s*([\s\S]*?)\)\s*->\s*CmdResult<([^>]+)>/g;
          let match;
          while ((match = customFnRegex.exec(content)) !== null) {
              const funcName = match[1];
              const argsStr = match[2];
              const rustReturnType = match[3].trim();
              
              const argsRaw = argsStr.split(",").map(a => a.trim()).filter(a => a.length > 0);
              const args = [];
              for (const raw of argsRaw) {
                  const parts = raw.split(":");
                  if (parts.length < 2) continue;
                  let argName = parts[0].trim();
                  if (argName === "state" || argName === "ctx" || argName === "_state" || argName === "_ctx") continue;
                  if (argName.startsWith("mut ")) argName = argName.substring(4);
                  let argTypeRaw = parts.slice(1).join(":").trim();
                  if (argTypeRaw.includes("State<'_,") || argTypeRaw.includes("AppHandle") || argTypeRaw.includes("Window")) continue;
                  
                  args.push({
                      name: toCamelCase(argName),
                      rustName: argName,
                      rustType: argTypeRaw,
                      tsType: mapRustTypeToTs(argTypeRaw),
                      _wasOptionStr: false
                  });
              }
              
              ops.push({
                  domain: dir,
                  opFile: "custom",
                  funcName,
                  tsFuncName: toCamelCase(funcName),
                  args,
                  rustReturnType,
                  tsReturnType: mapRustTypeToTs(rustReturnType),
                  isCustom: true
              });
          }
      }
  }

  // Deduplicate ops per domain
  const finalOps: OpFunction[] = [];
  const byDomainDedupe = ops.reduce((acc, op) => {
    if (!acc[op.domain]) acc[op.domain] = [];
    acc[op.domain].push(op);
    return acc;
  }, {} as Record<string, OpFunction[]>);

  for (const domainOps of Object.values(byDomainDedupe)) {
    const usedNames = new Map<string, OpFunction>();
    for (const op of domainOps) {
      if (usedNames.has(op.tsFuncName)) {
        const existing = usedNames.get(op.tsFuncName)!;
        // If same opFile or identical function signatures, skip exact duplicate
        if (existing.opFile === op.opFile && existing.funcName === op.funcName) {
          continue;
        }
        let counter = 2;
        let candidate = op.tsFuncName + counter;
        while (usedNames.has(candidate)) {
          counter++;
          candidate = op.tsFuncName + counter;
        }
        op.tsFuncName = candidate;
      }
      usedNames.set(op.tsFuncName, op);
      finalOps.push(op);
    }
  }

  return finalOps;
}

function generateTsInterfaces(ops: OpFunction[], knownTypes: Set<string>) {
    if (!fs.existsSync(SDK_CORE_COMMANDS_DIR)) fs.mkdirSync(SDK_CORE_COMMANDS_DIR, { recursive: true });
  
    const byDomain = ops.reduce((acc, op) => {
      if (!acc[op.domain]) acc[op.domain] = [];
      acc[op.domain].push(op);
      return acc;
    }, {} as Record<string, OpFunction[]>);
  
    for (const [domain, domainOps] of Object.entries(byDomain)) {
      let content = `// AUTO-GENERATED BY scripts/generate-rpc.ts — DO NOT EDIT DIRECTLY\n`;
      content += `import type * as Types from '../types/index.js';\n\n`;
      
      const domainCamel = domain.charAt(0).toUpperCase() + domain.slice(1);
      content += `export interface ${domainCamel}Commands {\n`;
      
      for (const op of domainOps) {
        const argsStr = op.args.map(a => `${a.name}: ${formatTsType(a.tsType, knownTypes)}`).join(", ");
        const retType = formatTsType(op.tsReturnType, knownTypes);
        content += `  ${op.tsFuncName}(${argsStr}): Promise<${retType}>;\n`;
      }

      content += `}\n`;

      fs.writeFileSync(path.join(SDK_CORE_COMMANDS_DIR, `${domain}.ts`), content);
    }
  
    let indexContent = `// AUTO-GENERATED BY scripts/generate-rpc.ts — DO NOT EDIT DIRECTLY\n`;
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
  const customTauriNames = new Set(ops.filter(o => o.isCustom).map(o => o.funcName));
  const autoOps = ops.filter(o => !o.isCustom && !customTauriNames.has(`${o.domain}_${o.opFile}_${o.funcName}`) && !customTauriNames.has(o.funcName));
  const byDomain = autoOps.reduce((acc, op) => {
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
      let deserializeLines = "";

      for (const a of op.args) {
        let isPrimitive = a.rustType === "&str" || a.rustType === "String" || a.rustType.startsWith("Option<String>") || a.rustType === "bool" || a.rustType === "i32" || a.rustType === "u32" || a.rustType === "i64" || a.rustType === "u64" || a.rustType === "f32" || a.rustType === "f64" || a.rustType.startsWith("Option<bool>") || a.rustType.startsWith("Option<i") || a.rustType.startsWith("Option<u") || a.rustType.startsWith("Option<f");
        
        if (isPrimitive) {
          if (a.rustType === "&str") {
            rsArgs.push(`${a.rustName}: String`);
          } else if (a.rustType === "Option<String>" && a._wasOptionStr) {
            rsArgs.push(`${a.rustName}: Option<String>`);
          } else {
            rsArgs.push(`${a.rustName}: ${a.rustType}`);
          }
        } else {
          if (a.rustType.startsWith("Option<")) {
             rsArgs.push(`${a.rustName}: Option<serde_json::Value>`);
             deserializeLines += `    let ${a.rustName}_parsed = match ${a.rustName} {\n`;
             deserializeLines += `        Some(v) => Some(serde_json::from_value(v).map_err(|e| CommandError::new(e.to_string()))?),\n`;
             deserializeLines += `        None => None,\n`;
             deserializeLines += `    };\n`;
          } else {
             rsArgs.push(`${a.rustName}: Option<serde_json::Value>`);
             if (a.rustType.includes("CursorRequest")) {
                deserializeLines += `    let ${a.rustName}_parsed = match ${a.rustName} {\n`;
                deserializeLines += `        Some(v) => serde_json::from_value(v).unwrap_or_default(),\n`;
                deserializeLines += `        None => shared_kernel::pagination::CursorRequest::default(),\n`;
                deserializeLines += `    };\n`;
             } else {
                deserializeLines += `    let ${a.rustName}_parsed = match ${a.rustName} {\n`;
                deserializeLines += `        Some(v) => serde_json::from_value(v).map_err(|e| CommandError::new(e.to_string()))?,\n`;
                deserializeLines += `        None => return Err(CommandError::new("Missing argument: ${a.rustName}")),\n`;
                deserializeLines += `    };\n`;
             }
          }
        }
      }

      const rsArgsStr = rsArgs.join(", ");
      const passArgsStr = (op as any)._passArgsToBackend.join(", ");

      const tauriFuncName = `${domain}_${op.opFile}_${op.funcName}`;
      
      rsContent += `#[tauri::command]\npub async fn ${tauriFuncName}(state: State<'_, Arc<AppState>>${rsArgsStr.length > 0 ? ', ' + rsArgsStr : ''}) -> CmdResult<serde_json::Value> {\n`;
      rsContent += `    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;\n`;
      rsContent += deserializeLines;
      if (op.rustReturnType === "()") {
        rsContent += `    ops::${op.opFile}::${op.funcName}(${passArgsStr}).await.map_err(CommandError::from)?;\n`;
        rsContent += `    Ok(serde_json::json!({}))\n`;
      } else {
        rsContent += `    let result = ops::${op.opFile}::${op.funcName}(${passArgsStr}).await.map_err(CommandError::from)?;\n`;
        rsContent += `    Ok(serde_json::to_value(result).unwrap())\n`;
      }
      rsContent += `}\n\n`;
      
      registerList += `        crate::commands::${domain}::auto::${tauriFuncName},\n`;
    }

    fs.writeFileSync(path.join(domainDir, "auto.rs"), rsContent);
    
    let modContent = `pub mod auto;\n`;
    if (fs.existsSync(path.join(domainDir, "custom.rs"))) {
        modContent += `pub mod custom;\n`;
    }
    fs.writeFileSync(path.join(domainDir, "mod.rs"), modContent);
  }

  const allDomains = new Set([...Object.keys(byDomain), ...ops.map(o => o.domain)]);
  for (const domain of allDomains) {
      const domainDir = path.join(TAURI_COMMANDS_DIR, domain);
      if (fs.existsSync(path.join(domainDir, "custom.rs")) && !fs.existsSync(path.join(domainDir, "auto.rs"))) {
          fs.writeFileSync(path.join(domainDir, "mod.rs"), `pub mod custom;\n`);
      }
  }

  for (const domain of allDomains) {
      const customOps = ops.filter(o => o.domain === domain && o.isCustom);
      for (const op of customOps) {
          registerList += `        crate::commands::${domain}::custom::${op.funcName},\n`;
      }
  }

  let registerRs = `// AUTO-GENERATED BY scripts/generate-rpc.ts\n`;
  registerRs += `pub fn generate_handlers<R: tauri::Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {\n`;
  registerRs += `    tauri::generate_handler![\n`;
  registerRs += `        crate::show_window,\n`;
  registerRs += `        crate::open_agent_window,\n`;
  registerRs += `        crate::check_for_updates,\n`;
  registerRs += `${registerList}    ]\n}\n`;
  fs.writeFileSync(path.join(TAURI_COMMANDS_DIR, "auto_register.rs"), registerRs);
  
  let mainMod = `pub mod error;\n`;
  for (const domain of allDomains) {
      mainMod += `pub mod ${domain};\n`;
  }
  mainMod += `pub mod auto_register;\n`;
  fs.writeFileSync(path.join(TAURI_COMMANDS_DIR, "mod.rs"), mainMod);
}

// ============================================================================
// DESKTOP WRAPPER GENERATION
// ============================================================================

function generateSdkDesktopWrappers(ops: OpFunction[]) {
  if (!fs.existsSync(SDK_DESKTOP_DIR)) fs.mkdirSync(SDK_DESKTOP_DIR, { recursive: true });

  const byDomain = ops.reduce((acc, op) => {
    if (!acc[op.domain]) acc[op.domain] = [];
    acc[op.domain].push(op);
    return acc;
  }, {} as Record<string, OpFunction[]>);

  let content = `/**
 * @orbis/sdk-desktop — OrbisCommands implementation via Tauri IPC.
 * AUTO-GENERATED BY scripts/generate-rpc.ts — DO NOT EDIT DIRECTLY
 */
import { invoke } from "@tauri-apps/api/core";
import type { OrbisCommands } from "@orbis/sdk-core";

async function cmd<T = any>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args ?? {});
}

export const commands: OrbisCommands = {\n`;

  for (const [domain, domainOps] of Object.entries(byDomain)) {
    content += `  ${domain}: {\n`;
    for (const op of domainOps) {
      const argsStr = op.args.map(a => `${a.name === "cmd" ? "cmdPayload" : a.name}: any`).join(", ");
      const passArgsStr = op.args.map(a => `${a.name === "cmd" ? "cmd: cmdPayload" : a.name}`).join(", ");
      const tauriFuncName = op.isCustom ? op.funcName : `${domain}_${op.opFile}_${op.funcName}`;
      
      if (op.tsFuncName.endsWith("Paginated") || op.args.some(a => a.rustType.includes("CursorRequest"))) {
          content += `    ${op.tsFuncName}: (cursorOrReq?: any, limit?: any) => {\n`;
          content += `      const req = (typeof cursorOrReq === "object" && cursorOrReq !== null && !Array.isArray(cursorOrReq))\n`;
          content += `        ? cursorOrReq\n`;
          content += `        : { cursor: typeof cursorOrReq === "string" ? cursorOrReq : null, limit: typeof limit === "number" ? limit : 50 };\n`;
          content += `      return cmd("${tauriFuncName}", { request: req });\n`;
          content += `    },\n`;
      } else if (op.args.length > 0) {
          content += `    ${op.tsFuncName}: (${argsStr}) => cmd("${tauriFuncName}", { ${passArgsStr} }),\n`;
      } else {
          content += `    ${op.tsFuncName}: () => cmd("${tauriFuncName}", {}),\n`;
      }
    }
    content += `  },\n`;
  }

  content += `};\n`;
  fs.writeFileSync(path.join(SDK_DESKTOP_DIR, "commands.ts"), content);
}

function main() {
  console.log("Discovering handwritten Types...");
  const knownTypes = getKnownTypes();
  console.log(`Found ${knownTypes.size} known types.`);

  console.log("Parsing Commands...");
  const ops = parseOps();
  console.log(`Found ${ops.length} ops.`);
  
  console.log("Generating TS Interfaces...");
  generateTsInterfaces(ops, knownTypes);
  
  console.log("Generating Tauri Wrappers...");
  generateTauriWrappers(ops);
  
  console.log("Generating SDK Desktop Wrappers...");
  generateSdkDesktopWrappers(ops);
  
  console.log("Done generating robust RPC layer.");
}

main();
