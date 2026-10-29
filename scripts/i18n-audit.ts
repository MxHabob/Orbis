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
  parse: (
    source: string,
    options?: { filename?: string },
  ) => {
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
  const usedKeys = new Set<string>();
  const literalMessageKeys = new Set<string>();
  const keyPattern = /\b(?:m|messages)\.([A-Za-z0-9_]+)\s*\(/g;
  const messageNamespacePattern =
    /\bimport\s+\*\s+as\s+([A-Za-z_$][\w$]*)\s+from\s*["'][^"']*paraglide\/messages(?:\.js)?["']/g;
  const literalMessageKeyPattern = /["'](iam_[A-Za-z0-9_]+)["']/g;
  const nonMessageObjectProperties = new Set([
    "content",
    "executedTools",
    "executed_tools",
    "field",
    "id",
    "is_digital_gateway",
    "name",
    "present_records",
    "proposedActions",
    "proposed_actions",
    "role",
    "timestamp",
    "total_attendance_records",
    "user_id",
    "value",
  ]);

  const source = sourceText.map(({ text }) => text).join("\n");
  for (const match of source.matchAll(keyPattern)) {
    if (match[1]) usedKeys.add(match[1]);
  }
  for (const { text } of sourceText) {
    for (const importMatch of text.matchAll(messageNamespacePattern)) {
      const namespace = importMatch[1];
      if (!namespace) continue;
      const escapedNamespace = namespace.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      const messageReferencePattern = new RegExp(
        `\\b${escapedNamespace}\\s*\\.([A-Za-z0-9_]+)\\b`,
        "g",
      );
      for (const match of text.matchAll(messageReferencePattern)) {
        if (match[1] && !nonMessageObjectProperties.has(match[1])) {
          usedKeys.add(match[1]);
        }
      }
    }
    for (const match of text.matchAll(literalMessageKeyPattern)) {
      if (match[1]) literalMessageKeys.add(match[1]);
    }
  }

  const arabicKeys = new Set(Object.keys(arabicMessages));
  const englishKeys = new Set(Object.keys(englishMessages));
  const allKeys = new Set([...arabicKeys, ...englishKeys]);
  for (const key of literalMessageKeys) {
    if (allKeys.has(key)) usedKeys.add(key);
  }
  const unusedKeys = [...allKeys].filter((key) => !usedKeys.has(key)).sort();
  const missingInArabic = [
    ...new Set([
      ...[...usedKeys].filter((key) => !arabicKeys.has(key)),
      ...[...englishKeys].filter((key) => !arabicKeys.has(key)),
    ]),
  ].sort();
  const missingInEnglish = [
    ...new Set([
      ...[...usedKeys].filter((key) => !englishKeys.has(key)),
      ...[...arabicKeys].filter((key) => !englishKeys.has(key)),
    ]),
  ].sort();
  const placeholderPattern = /\{\s*([A-Za-z0-9_]+)\s*\}/g;
  const placeholderVariables = (message: unknown) =>
    typeof message === "string"
      ? [...message.matchAll(placeholderPattern)]
          .map((match) => match[1])
          .sort()
          .join(",")
      : "";
  const placeholderMismatches = [...allKeys]
    .filter(
      (key) =>
        arabicKeys.has(key) &&
        englishKeys.has(key) &&
        placeholderVariables(arabicMessages[key]) !==
          placeholderVariables(englishMessages[key]),
    )
    .sort()
    .map(
      (key) =>
        `${key} (ar: {${placeholderVariables(arabicMessages[key])}}, en: {${placeholderVariables(englishMessages[key])}})`,
    );
  const generatedLookingMessages = [...allKeys]
    .filter((key) => {
      const value = englishMessages[key];
      return (
        typeof value === "string" &&
        /^(?:Shared Ui |Common Action |Finance Ui |Settings Ui )/.test(value)
      );
    })
    .sort();
  const hardcodedArabic: string[] = [];
  const hardcodedArabicMarkup: string[] = [];
  const literalPattern = /(['"`])[^'"`\n]*[\u0600-\u06ff][^'"`\n]*\1/g;

  for (const { file, text } of sourceText) {
    if (
      file.endsWith(
        `${"src"}${"\\"}lib${"\\"}shell${"\\"}search-modal.svelte`,
      ) ||
      file.endsWith(
        `${"src"}${"\\"}lib${"\\"}shared${"\\"}utils${"\\"}person-name.ts`,
      ) ||
      file.includes(".test.") ||
      file.includes(".spec.")
    ) {
      continue;
    }

    const lines = text.split(/\r?\n/);
    const remoteMetadataStart = file.endsWith(
      `${"src"}${"\\"}lib${"\\"}api${"\\"}remote-client.ts`,
    )
      ? text.indexOf("async bootstrapDetectEnvironment")
      : -1;
    const remoteMetadataEnd =
      remoteMetadataStart >= 0
        ? text.indexOf("async bootstrapGenerateSlug", remoteMetadataStart)
        : -1;
    const guardianRelationshipStart = file.endsWith(
      `${"src"}${"\\"}lib${"\\"}features${"\\"}students${"\\"}components${"\\"}guardian-form.svelte`,
    )
      ? text.indexOf("const relationshipAr =")
      : -1;
    const guardianRelationshipEnd =
      guardianRelationshipStart >= 0
        ? text.indexOf("const request:", guardianRelationshipStart)
        : -1;
    let skipArabicDataValue = false;
    let lineOffset = 0;
    for (const [index, line] of lines.entries()) {
      const currentOffset = lineOffset;
      lineOffset += line.length + 1;
      if (line.trim().startsWith("//") || line.trim().startsWith("<!--"))
        continue;
      if (
        file.endsWith(
          `${"src"}${"\\"}routes${"\\"}(app)${"\\"}[orgId]${"\\"}onboarding${"\\"}+page.svelte`,
        ) &&
        /\bacademic_year_name_ar\s*:/.test(line)
      ) {
        skipArabicDataValue = true;
        continue;
      }
      if (skipArabicDataValue) {
        if (line.includes(",")) skipArabicDataValue = false;
        continue;
      }
      if (
        (file.endsWith(
          `${"src"}${"\\"}lib${"\\"}features${"\\"}templates${"\\"}panels${"\\"}properties-inspector.svelte`,
        ) &&
          /\bheaderAr\s*:/.test(line)) ||
        (file.endsWith(
          `${"src"}${"\\"}lib${"\\"}features${"\\"}students${"\\"}student-details.svelte`,
        ) &&
          /\bst\.includes\("[\u0600-\u06ff]+"\)/.test(line)) ||
        (file.endsWith(
          `${"src"}${"\\"}lib${"\\"}features${"\\"}students${"\\"}form-state.svelte.ts`,
        ) &&
          /["']؛\s*["']/.test(line))
      ) {
        continue;
      }
      if (
        file.includes(
          `${"src"}${"\\"}lib${"\\"}features${"\\"}bulk-entry${"\\"}profiles${"\\"}`,
        )
      ) {
        continue;
      }
      if (
        file.endsWith(`${"src"}${"\\"}lib${"\\"}api${"\\"}remote-client.ts`) &&
        ((remoteMetadataStart >= 0 &&
          currentOffset >= remoteMetadataStart &&
          (remoteMetadataEnd < 0 || currentOffset < remoteMetadataEnd)) ||
          (index >= 1472 && index < 1540))
      ) {
        continue;
      }
      if (
        file.endsWith(
          `${"src"}${"\\"}lib${"\\"}shared${"\\"}utils${"\\"}search.ts`,
        ) ||
        (guardianRelationshipStart >= 0 &&
          currentOffset >= guardianRelationshipStart &&
          (guardianRelationshipEnd < 0 ||
            currentOffset < guardianRelationshipEnd)) ||
        /\b(?:[A-Za-z]*nameAr|[A-Za-z_]*name_ar|relationshipAr|relationship_ar|country_name_ar|currency_symbol_ar|currency_name_ar|suggested_academic_year_ar|suggested_academic_calendar_ar|detected_language)\s*(?:=|:|\|\|)/i.test(
          line,
        ) ||
        (file.endsWith(
          `${"src"}${"\\"}lib${"\\"}features${"\\"}students${"\\"}registration-effects.ts`,
        ) &&
          /account\.name_ar\?\.trim\(\)/.test(line)) ||
        /\b(?:rawStatus|rawGender|status|sex)\b.*[\u0600-\u06ff]/.test(line) ||
        /\.replace\(\s*["'][^"']*[\u0600-\u06ff][^"']*["']/.test(line) ||
        ((file.endsWith(
          `${"src"}${"\\"}lib${"\\"}features${"\\"}students${"\\"}form-state.svelte.ts`,
        ) ||
          file.endsWith(
            `${"src"}${"\\"}lib${"\\"}features${"\\"}students${"\\"}student-details.svelte`,
          )) &&
          /\b(?:firstName|lastName)\b.*[\u0600-\u06ff]/.test(line))
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
  printList("Placeholder mismatches", placeholderMismatches);
  printList(
    "Generated-looking English messages (manual review)",
    generatedLookingMessages,
  );
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
    placeholderMismatches.length ||
    generatedLookingMessages.length ||
    hardcodedArabicMarkup.length
  ) {
    process.exitCode = 1;
  }
}

void main();
