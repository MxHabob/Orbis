#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export function createSignedEnvelope(payload, signature) {
  if (!payload || !signature?.trim()) {
    throw new Error("A component payload and minisign signature are required.");
  }
  JSON.parse(payload);
  return {
    payload,
    signature: signature.trim(),
  };
}

if (
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const [payloadPath, signaturePath, outputPath] = process.argv.slice(2);
  if (!payloadPath || !signaturePath || !outputPath) {
    throw new Error("Usage: wrap-component-manifest.mjs <payload> <signature> <output>");
  }
  const envelope = createSignedEnvelope(
    readFileSync(payloadPath, "utf8"),
    readFileSync(signaturePath, "utf8"),
  );
  writeFileSync(outputPath, `${JSON.stringify(envelope, null, 2)}\n`, {
    flag: "wx",
  });
}
