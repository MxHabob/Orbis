#!/usr/bin/env node
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export function normalizeUpdaterPublicKey(rawValue) {
  if (!rawValue?.trim()) {
    throw new Error(
      "TAURI_UPDATER_PUBLIC_KEY is required for production updater builds.",
    );
  }

  let publicKey = rawValue.trim().replace(/\r\n/g, "\n");
  if (!publicKey.startsWith("untrusted comment:")) {
    const encodedValue = rawValue.trim().replace(/\s+/g, "");
    if (
      !/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/.test(
        encodedValue,
      )
    ) {
      throw new Error(
        "TAURI_UPDATER_PUBLIC_KEY must be a minisign public key or its base64 encoding.",
      );
    }

    const decodedValue = Buffer.from(encodedValue, "base64");
    if (decodedValue.toString("base64") !== encodedValue) {
      throw new Error(
        "TAURI_UPDATER_PUBLIC_KEY is not valid canonical base64.",
      );
    }
    publicKey = decodedValue.toString("utf8").replace(/\r\n/g, "\n").trim();
  }

  const [comment, keyLine, ...extraLines] = publicKey.split("\n");
  const keyBytes = Buffer.from(keyLine ?? "", "base64");
  if (
    !/^untrusted comment:\s+\S/.test(comment ?? "") ||
    !/^[A-Za-z0-9+/]+={0,2}$/.test(keyLine ?? "") ||
    keyBytes.length !== 42 ||
    keyBytes.toString("base64") !== keyLine ||
    extraLines.length > 0
  ) {
    throw new Error(
      "TAURI_UPDATER_PUBLIC_KEY is not a valid minisign public key.",
    );
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

export function normalizeUpdaterEndpoint(rawValue) {
  const endpoint = normalizeHttpsUrl(rawValue, "TAURI_UPDATER_ENDPOINT");
  const url = new URL(endpoint);
  if (!/^\/(?:[a-z0-9][a-z0-9-]*\/)+latest\.json$/.test(url.pathname)) {
    throw new Error(
      "TAURI_UPDATER_ENDPOINT must end with /<product-path>/latest.json, for example /orbis/latest.json.",
    );
  }
  return endpoint;
}

if (
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const updaterPublicKey = normalizeUpdaterPublicKey(
    process.env.TAURI_UPDATER_PUBLIC_KEY,
  );
  const updaterEndpoint = normalizeUpdaterEndpoint(
    process.env.TAURI_UPDATER_ENDPOINT,
  );
  const componentPublicKey = normalizeUpdaterPublicKey(
    process.env.ORBIS_COMPONENT_PUBLIC_KEY,
  );
  const solverManifestUrl = normalizeHttpsUrl(
    process.env.ORBIS_SOLVER_MANIFEST_URL,
    "ORBIS_SOLVER_MANIFEST_URL",
  );
  const desktopRoot = path.resolve(
    path.dirname(fileURLToPath(import.meta.url)),
    "..",
  );
  const configPath = path.resolve(
    process.env.TAURI_UPDATER_CONFIG_PATH ??
      path.join(
        desktopRoot,
        "src-tauri",
        "target",
        "tauri-updater-release.conf.json",
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
      `ORBIS_SOLVER_PUBLIC_KEY_B64=${componentPublicKey}\nORBIS_SOLVER_MANIFEST_URL=${solverManifestUrl}\n`,
    );
  }
  console.log(
    "Validated and prepared signed desktop and solver runtime configuration.",
  );
}
