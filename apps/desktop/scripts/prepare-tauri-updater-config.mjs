#!/usr/bin/env node
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export function normalizeUpdaterPublicKey(rawValue) {
  if (!rawValue?.trim()) {
    throw new Error("TAURI_UPDATER_PUBLIC_KEY is required for release builds.");
  }

  const publicKey = rawValue.trim().replace(/\r\n/g, "\n");
  const [comment, keyLine, ...extraLines] = publicKey.split("\n");
  const keyBytes = Buffer.from(keyLine ?? "", "base64");
  if (
    !/^untrusted comment:\s+\S/.test(comment ?? "") ||
    !/^[A-Za-z0-9+/]+={0,2}$/.test(keyLine ?? "") ||
    keyBytes.length !== 42 ||
    keyBytes.toString("base64") !== keyLine ||
    extraLines.length > 0
  ) {
    throw new Error("TAURI_UPDATER_PUBLIC_KEY is not a valid minisign public key.");
  }

  return Buffer.from(publicKey, "utf8").toString("base64");
}

export function normalizeHttpsUrl(rawValue, label) {
  let url;
  try {
    url = new URL(rawValue?.trim() ?? "");
  } catch {
    throw new Error(`${label} must be a valid HTTPS URL.`);
  }
  if (
    url.protocol !== "https:" ||
    url.username ||
    url.password ||
    url.hash ||
    url.search
  ) {
    throw new Error(
      `${label} must use HTTPS and cannot include credentials, a query, or a fragment.`,
    );
  }
  return url.toString();
}

if (
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const updaterPublicKey = process.env.TAURI_UPDATER_PUBLIC_KEY?.trim();
  normalizeUpdaterPublicKey(updaterPublicKey);
  const updaterEndpoint = normalizeHttpsUrl(
    process.env.TAURI_UPDATER_ENDPOINT,
    "TAURI_UPDATER_ENDPOINT",
  );
  const componentPublicKey = process.env.ORBIS_COMPONENT_PUBLIC_KEY;
  const encodedComponentPublicKey = normalizeUpdaterPublicKey(componentPublicKey);
  const solverManifestUrl = normalizeHttpsUrl(
    process.env.ORBIS_SOLVER_MANIFEST_URL,
    "ORBIS_SOLVER_MANIFEST_URL",
  );
  const configPath = path.resolve(
    process.env.TAURI_UPDATER_CONFIG_PATH ??
      path.join(
        path.dirname(fileURLToPath(import.meta.url)),
        "..",
        "src-tauri",
        "tauri.updater.conf.json",
      ),
  );

  mkdirSync(path.dirname(configPath), { recursive: true });
  writeFileSync(
    configPath,
    `${JSON.stringify(
      {
        bundle: { createUpdaterArtifacts: true },
        plugins: {
          updater: {
            pubkey: updaterPublicKey,
            endpoints: [updaterEndpoint],
          },
        },
      },
      null,
      2,
    )}\n`,
    { mode: 0o600 },
  );

  if (process.env.GITHUB_ENV) {
    appendFileSync(
      process.env.GITHUB_ENV,
      `ORBIS_SOLVER_PUBLIC_KEY_B64=${encodedComponentPublicKey}\nORBIS_SOLVER_MANIFEST_URL=${solverManifestUrl}\n`,
    );
  }
  console.log("Validated and prepared signed desktop and solver runtime configuration.");
}
