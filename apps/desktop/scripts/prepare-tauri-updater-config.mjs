import { mkdirSync, writeFileSync } from "node:fs";
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
      "TAURI_UPDATER_PUBLIC_KEY is missing a valid minisign comment or key.",
    );
  }

  return publicKey;
}

if (
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const publicKey = normalizeUpdaterPublicKey(
    process.env.TAURI_UPDATER_PUBLIC_KEY,
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
    `${JSON.stringify({ plugins: { updater: { pubkey: publicKey } } }, null, 2)}\n`,
    { mode: 0o600 },
  );
  console.log(
    "Validated and prepared the Tauri updater public-key configuration.",
  );
}
