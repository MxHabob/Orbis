import fs from "node:fs";
import path from "node:path";

const arJson = JSON.parse(fs.readFileSync("messages/ar.json", "utf-8"));
const enJson = JSON.parse(fs.readFileSync("messages/en.json", "utf-8"));

function getFiles(dir) {
  let results = [];
  const list = fs.readdirSync(dir);
  for (const file of list) {
    const full = path.join(dir, file);
    const stat = fs.statSync(full);
    if (stat && stat.isDirectory()) {
      results = results.concat(getFiles(full));
    } else if (full.endsWith(".svelte") || full.endsWith(".ts")) {
      results.push(full);
    }
  }
  return results;
}

const files = [
  ...getFiles("src/routes/(app)/[orgId]/academics"),
  ...getFiles("src/lib/features/academics"),
];

// Match m.something( or m.something ? or m.something: or m.something} or m.something,
const keyPattern = /(?<![a-zA-Z0-9_$])m\.([a-zA-Z0-9_]+)\s*(?:\(|\?|\:|\}|,)/g;

const usedKeys = new Map();

for (const file of files) {
  const content = fs.readFileSync(file, "utf-8");
  if (!content.includes('from "$lib/paraglide/messages"')) continue;
  let match;
  while ((match = keyPattern.exec(content)) !== null) {
    const key = match[1];
    if (!usedKeys.has(key)) {
      usedKeys.set(key, []);
    }
    usedKeys.get(key).push(path.basename(file));
  }
}

console.log("Total unique Paraglide message keys used:", usedKeys.size);

const missingInAr = [];
const missingInEn = [];

for (const [key, usages] of Array.from(usedKeys.entries()).sort((a, b) =>
  a[0].localeCompare(b[0]),
)) {
  if (!(key in arJson)) {
    missingInAr.push({ key, usages: Array.from(new Set(usages)) });
  }
  if (!(key in enJson)) {
    missingInEn.push({ key, usages: Array.from(new Set(usages)) });
  }
}

console.log("\n--- Missing in ar.json (" + missingInAr.length + ") ---");
for (const item of missingInAr) {
  console.log(`Key: ${item.key} (used in: ${item.usages.join(", ")})`);
}

console.log("\n--- Missing in en.json (" + missingInEn.length + ") ---");
for (const item of missingInEn) {
  console.log(`Key: ${item.key} (used in: ${item.usages.join(", ")})`);
}
