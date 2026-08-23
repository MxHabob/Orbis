import fs from "fs";
import path from "path";

const srcDir = path.resolve("d:/projects/Orbis/apps/desktop-svelte/src");

function getAllFiles(dir: string, ext: string[]): string[] {
  let results: string[] = [];
  const list = fs.readdirSync(dir);
  for (const file of list) {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);
    if (stat && stat.isDirectory()) {
      results = results.concat(getAllFiles(filePath, ext));
    } else if (ext.includes(path.extname(file))) {
      results.push(filePath);
    }
  }
  return results;
}

const files = getAllFiles(srcDir, [".svelte", ".ts"]);
let totalReplacements = 0;
let modifiedFiles = 0;

for (const filePath of files) {
  let content = fs.readFileSync(filePath, "utf-8");
  let original = content;

  // 1. Find all query and mutation variable identifiers in this file
  // e.g.: const xxxQuery = create... / const xxxMutation = create... / const ... = usePeriodGate(...)
  const queryVarNames = new Set<string>();
  
  // Patterns:
  // const myQuery = ...
  // const myMutation = ...
  // const gate = usePeriodGate(...)
  // let myQuery = ...
  const varDeclRegex = /(?:const|let)\s+([a-zA-Z0-9_]+)\s*=\s*(?:create[a-zA-Z0-9_]*(?:Query|Mutation)|use[a-zA-Z0-9_]*(?:Query|Mutation|Gate|Configure|RunTick)|usePeriodGate)\s*\(/g;
  let match;
  while ((match = varDeclRegex.exec(content)) !== null) {
    queryVarNames.add(match[1]);
  }

  // Also include any variable whose name ends in Query or Mutation if it interacts with .data, .isLoading, .error, .refetch, .isPending, .mutate, .fetchNextPage, .hasNextPage
  const queryPropertyRegex = /\$([a-zA-Z0-9_]+)\.(?:data|isLoading|isPending|isError|error|refetch|mutate|mutateAsync|fetchNextPage|hasNextPage|isFetchingNextPage|pages)/g;
  while ((match = queryPropertyRegex.exec(content)) !== null) {
    queryVarNames.add(match[1]);
  }

  // 2. Replace $varName with varName for all identified query variables
  for (const varName of queryVarNames) {
    // We want to replace $varName when it is used as an identifier
    // i.e., preceded by non-word and followed by . or other boundary
    const replaceRegex = new RegExp(`\\$${varName}\\b`, "g");
    const newContent = content.replace(replaceRegex, varName);
    if (newContent !== content) {
      const count = (content.match(replaceRegex) || []).length;
      console.log(`[${path.relative(srcDir, filePath)}] Replaced $${varName} -> ${varName} (${count} times)`);
      totalReplacements += count;
      content = newContent;
    }
  }

  if (content !== original) {
    fs.writeFileSync(filePath, content, "utf-8");
    modifiedFiles++;
  }
}

console.log(`\nDone! Replaced ${totalReplacements} query store references across ${modifiedFiles} files.`);
