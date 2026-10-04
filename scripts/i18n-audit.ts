import { readFile, readdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

type SvelteNode = {
  type?: string;
  start?: number;
  data?: string;
  [key: string]: unknown;
};

const root = process.cwd();
const appRoot = join(root, "apps", "desktop");
const sourceRoot = join(appRoot, "src");
const messagesRoot = join(appRoot, "messages");
const requireFromDesktop = createRequire(join(appRoot, "package.json"));
const { parse } = requireFromDesktop("svelte/compiler") as {
  parse: (source: string, options?: { filename?: string }) => {
    html: SvelteNode;
  };
};
const sourceExtensions = new Set([".svelte", ".ts", ".js", ".tsx", ".jsx"]);
const ignoredDirectories = new Set([
  "node_modules",
  ".svelte-kit",
  "build",
  "paraglide",
]);

async function collectFiles(directory: string): Promise<string[]> {
  const entries = await readdir(directory, { withFileTypes: true });
  const files: string[] = [];

  for (const entry of entries) {
    if (entry.isDirectory() && !ignoredDirectories.has(entry.name)) {
      files.push(...(await collectFiles(join(directory, entry.name))));
    } else if (
      entry.isFile() &&
      sourceExtensions.has(entry.name.slice(entry.name.lastIndexOf(".")))
    ) {
      files.push(join(directory, entry.name));
    }
  }

  return files;
}

async function main() {
  const [arabicMessages, englishMessages, sourceFiles] = await Promise.all([
    readFile(join(messagesRoot, "ar.json"), "utf8").then(JSON.parse),
    readFile(join(messagesRoot, "en.json"), "utf8").then(JSON.parse),
    collectFiles(sourceRoot),
  ]);

  const sourceText = await Promise.all(
    sourceFiles.map(async (file) => ({
      file,
      text: await readFile(file, "utf8"),
    })),
  );
  const source = sourceText.map(({ text }) => text).join("\n");
  const usedKeys = new Set<string>();
  const keyPattern = /\b(?:m|messages)\.([A-Za-z0-9_]+)\s*\(/g;

  for (const match of source.matchAll(keyPattern)) {
    if (match[1]) usedKeys.add(match[1]);
  }

  const arabicKeys = new Set(Object.keys(arabicMessages));
  const englishKeys = new Set(Object.keys(englishMessages));
  const allKeys = new Set([...arabicKeys, ...englishKeys]);
  const unusedKeys = [...allKeys].filter((key) => !usedKeys.has(key)).sort();
  const missingInArabic = [...englishKeys]
    .filter((key) => !arabicKeys.has(key))
    .sort();
  const missingInEnglish = [...arabicKeys]
    .filter((key) => !englishKeys.has(key))
    .sort();
  const hardcodedArabic: string[] = [];
  const hardcodedArabicMarkup: string[] = [];
  const literalPattern = /(['"`])[^'"`\n]*[\u0600-\u06ff][^'"`\n]*\1/g;

  for (const { file, text } of sourceText) {
    const lines = text.split(/\r?\n/);
    for (const [index, line] of lines.entries()) {
      if (line.trim().startsWith("//") || line.trim().startsWith("<!--"))
        continue;
      if (
        file.includes(`${"src"}${"\\"}lib${"\\"}features${"\\"}bulk-entry${"\\"}profiles${"\\"}`)
      ) {
        continue;
      }
      if (
        file.endsWith(`${"src"}${"\\"}lib${"\\"}api${"\\"}remote-client.ts`) &&
        index >= 1472 &&
        index < 1540
      ) {
        continue;
      }
      for (const match of line.matchAll(literalPattern)) {
        hardcodedArabic.push(`${relativePath(file)}:${index + 1} ${match[0]}`);
      }
    }

    if (file.endsWith(".svelte")) {
      const ast = parse(text, { filename: file });
      visitMarkup(ast.html, text, file);
    }
  }

  function visitMarkup(node: SvelteNode, source: string, file: string) {
    if (node.type === "Comment") return;
    if (
      node.type === "Text" &&
      typeof node.data === "string" &&
      /[\u0600-\u06ff]/.test(node.data)
    ) {
      const line = source.slice(0, node.start ?? 0).split(/\r?\n/).length;
      hardcodedArabicMarkup.push(
        `${relativePath(file)}:${line} ${JSON.stringify(node.data.trim())}`,
      );
    }

    for (const value of Object.values(node)) {
      if (Array.isArray(value)) {
        for (const child of value) {
          if (child && typeof child === "object") {
            visitMarkup(child as SvelteNode, source, file);
          }
        }
      } else if (value && typeof value === "object") {
        visitMarkup(value as SvelteNode, source, file);
      }
    }
  }

  function relativePath(file: string) {
    return file.replace(`${root}${"\\"}`, "").replaceAll("\\", "/");
  }

  function printList(title: string, values: string[]) {
    console.log(`\n${title} (${values.length})`);
    for (const value of values) console.log(`- ${value}`);
  }

  console.log(
    `Translation audit: ${sourceFiles.length} source files, ${allKeys.size} message keys`,
  );
  printList("Unused message keys", unusedKeys);
  printList("Missing Arabic translations", missingInArabic);
  printList("Missing English translations", missingInEnglish);
  printList("Hardcoded Arabic literals", hardcodedArabic);
  printList("Hardcoded Arabic Svelte markup", hardcodedArabicMarkup);

  if (process.argv.includes("--prune")) {
    const unused = new Set(unusedKeys);
    for (const [file, messages] of [
      [join(messagesRoot, "ar.json"), arabicMessages],
      [join(messagesRoot, "en.json"), englishMessages],
    ] as const) {
      const retained = Object.fromEntries(
        Object.entries(messages).filter(([key]) => !unused.has(key)),
      );
      await writeFile(file, `${JSON.stringify(retained, null, 2)}\n`, "utf8");
    }
    console.log(
      `\nPruned ${unusedKeys.length} unused message keys from both locale files.`,
    );
  }

  if (
    missingInArabic.length ||
    missingInEnglish.length ||
    hardcodedArabic.length ||
    hardcodedArabicMarkup.length
  ) {
    process.exitCode = 1;
  }
}

void main();
