import * as fs from "fs";
import * as path from "path";

const WORKSPACE_ROOT = path.resolve(__dirname, "..");
const API_CRATES_DIR = path.join(WORKSPACE_ROOT, "apps", "api-rust", "crates");
const SDK_CORE_TYPES_DIR = path.join(WORKSPACE_ROOT, "packages", "sdk-core", "src", "types");

interface Field {
    name: string;
    type: string;
    isOptional: boolean;
}

interface Struct {
    name: string;
    fields: Record<string, Field>;
}

function parseRustEntities(): Record<string, Struct> {
    const rustStructs: Record<string, Struct> = {};
    const crates = fs.readdirSync(API_CRATES_DIR);
    
    for (const crate of crates) {
        if (!crate.startsWith("svc-") && !crate.startsWith("platform-") && !crate.startsWith("shared-")) continue;
        const entitiesDir = path.join(API_CRATES_DIR, crate, "src", "entities");
        if (!fs.existsSync(entitiesDir)) continue;

        const files = fs.readdirSync(entitiesDir);
        for (const file of files) {
            if (!file.endsWith(".rs") || file === "mod.rs") continue;
            
            const content = fs.readFileSync(path.join(entitiesDir, file), "utf8");
            const entityName = file.replace(".rs", "").split("_").map(p => p.charAt(0).toUpperCase() + p.slice(1)).join("");
            
            const fields: Record<string, Field> = {};
            const regex = /pub\s+([a-zA-Z0-9_]+):\s*(Option<)?([^,]+?)(>)?,/g;
            let match;
            while ((match = regex.exec(content)) !== null) {
                const isOption = !!match[2];
                let rawType = match[3].trim();
                fields[match[1]] = {
                    name: match[1],
                    type: rawType,
                    isOptional: isOption
                };
            }
            if (Object.keys(fields).length > 0) {
                rustStructs[entityName] = { name: entityName, fields };
            }
        }
    }
    return rustStructs;
}

function parseTypescriptInterfaces(): Record<string, Struct> {
    const tsInterfaces: Record<string, Struct> = {};
    const files = fs.readdirSync(SDK_CORE_TYPES_DIR);
    
    for (const file of files) {
        if (!file.endsWith(".ts") || file === "index.ts") continue;
        
        const content = fs.readFileSync(path.join(SDK_CORE_TYPES_DIR, file), "utf8");
        const interfaceRegex = /export\s+interface\s+([a-zA-Z0-9_]+)\s*\{([^}]+)\}/g;
        let match;
        
        while ((match = interfaceRegex.exec(content)) !== null) {
            const interfaceName = match[1];
            const body = match[2];
            
            const fields: Record<string, Field> = {};
            const fieldRegex = /([a-zA-Z0-9_]+)(\?)?:\s*([^;]+);/g;
            let fieldMatch;
            
            while ((fieldMatch = fieldRegex.exec(body)) !== null) {
                const name = fieldMatch[1];
                const isOptional = !!fieldMatch[2] || fieldMatch[3].includes("null");
                fields[name] = {
                    name,
                    type: fieldMatch[3].trim(),
                    isOptional
                };
            }
            tsInterfaces[interfaceName] = { name: interfaceName, fields };
        }
    }
    return tsInterfaces;
}

function reconcile() {
    const rust = parseRustEntities();
    const ts = parseTypescriptInterfaces();
    
    const report: any = {
        mismatches: [],
        missing_in_ts: [],
        missing_in_rust: []
    };
    
    for (const [name, rustStruct] of Object.entries(rust)) {
        if (!ts[name]) {
            report.missing_in_ts.push(name);
            continue;
        }
        
        const tsStruct = ts[name];
        const mismatches = [];
        
        for (const [fieldName, rustField] of Object.entries(rustStruct.fields)) {
            const tsField = tsStruct.fields[fieldName];
            if (!tsField) {
                mismatches.push({ field: fieldName, issue: "Missing in TS" });
            } else if (rustField.isOptional !== tsField.isOptional) {
                mismatches.push({ field: fieldName, issue: "Optionality mismatch", rustOptional: rustField.isOptional, tsOptional: tsField.isOptional });
            }
        }
        
        for (const [fieldName, tsField] of Object.entries(tsStruct.fields)) {
            if (!rustStruct.fields[fieldName]) {
                mismatches.push({ field: fieldName, issue: "Missing in Rust (Extra in TS)" });
            }
        }
        
        if (mismatches.length > 0) {
            report.mismatches.push({ entity: name, mismatches });
        }
    }
    
    for (const name of Object.keys(ts)) {
        if (!rust[name]) {
            report.missing_in_rust.push(name);
        }
    }
    
    fs.writeFileSync(path.join(WORKSPACE_ROOT, "reconciliation_report.json"), JSON.stringify(report, null, 2));
    console.log("Report generated at reconciliation_report.json");
}

reconcile();
