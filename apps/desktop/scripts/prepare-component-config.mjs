#!/usr/bin/env node
import { appendFileSync } from "node:fs";
import {
  normalizeHttpsUrl,
  normalizeUpdaterPublicKey,
} from "./prepare-tauri-updater-config.mjs";

const publicKey = normalizeUpdaterPublicKey(
  process.env.ORBIS_COMPONENT_PUBLIC_KEY,
);
const manifestUrl = normalizeHttpsUrl(
  process.env.ORBIS_SOLVER_MANIFEST_URL,
  "ORBIS_SOLVER_MANIFEST_URL",
);

if (process.env.GITHUB_ENV) {
  appendFileSync(
    process.env.GITHUB_ENV,
    `ORBIS_SOLVER_PUBLIC_KEY_B64=${publicKey}\nORBIS_SOLVER_MANIFEST_URL=${manifestUrl}\n`,
  );
}

console.log("Validated solver component runtime configuration.");
