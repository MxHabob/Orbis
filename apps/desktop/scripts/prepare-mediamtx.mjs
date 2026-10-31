import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { createReadStream } from "node:fs";
import { chmod, copyFile, mkdir, mkdtemp, readdir, rename, rm, stat, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const version = "1.21.3";
const releaseAssets = {
  "x86_64-pc-windows-msvc": {
    name: `mediamtx_v${version}_windows_amd64.zip`,
    sha256: "b2f5246b07cb72961df03b8d5e8cad43dc657bcf3f49a87bb031bdf22721af74",
  },
  "x86_64-apple-darwin": {
    name: `mediamtx_v${version}_darwin_amd64.tar.gz`,
    sha256: "8a8416ee2ff5d17d9366498b6d88b76ab6d3afcda50b6d7321ccddb349bdbaf2",
  },
  "aarch64-apple-darwin": {
    name: `mediamtx_v${version}_darwin_arm64.tar.gz`,
    sha256: "bc1a0762fd18ffb413bcd81dcb9a072ac159fbd1f27e79ac6f14598aaac5ed3f",
  },
  "x86_64-unknown-linux-gnu": {
    name: `mediamtx_v${version}_linux_amd64.tar.gz`,
    sha256: "0a2ea7eba96023631efa1bea7804823a9145b043b6646a8a05798d4ac24ed635",
  },
  "aarch64-unknown-linux-gnu": {
    name: `mediamtx_v${version}_linux_arm64.tar.gz`,
    sha256: "27f038237cb5f77a724f6afc58eca2798f2bc4edf9989895631a80eeb41a4fe2",
  },
};

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const tauriDir = path.resolve(scriptDir, "../src-tauri");
const binariesDir = path.join(tauriDir, "binaries");
const cacheDir = path.join(binariesDir, ".cache");
const target = process.env.TAURI_ENV_TARGET_TRIPLE
  ?? process.env.TAURI_TARGET_TRIPLE
  ?? process.env.TARGET
  ?? getRustHostTriple();
const asset = releaseAssets[target];

if (!asset) {
  throw new Error(`MediaMTX ${version} is not available for target ${target}.`);
}

await mkdir(cacheDir, { recursive: true });
const archivePath = path.join(cacheDir, asset.name);
if (!(await isVerifiedArchive(archivePath, asset.sha256))) {
  await rm(archivePath, { force: true });
  await downloadVerifiedArchive(asset, archivePath);
}

const temporaryDir = await mkdtemp(path.join(os.tmpdir(), "orbis-mediamtx-"));
try {
  const extractionDir = path.join(temporaryDir, "extracted");
  await mkdir(extractionDir);
  const isWindowsArchive = asset.name.endsWith(".zip");
  execFileSync("tar", [isWindowsArchive ? "-xf" : "-xzf", archivePath, "-C", extractionDir], {
    stdio: "inherit",
  });

  const executableName = target.includes("windows") ? "mediamtx.exe" : "mediamtx";
  const executablePath = await findFile(extractionDir, executableName);
  if (!executablePath) {
    throw new Error(`The official MediaMTX archive did not contain ${executableName}.`);
  }

  const targetExecutableName = target.includes("windows")
    ? `mediamtx-${target}.exe`
    : `mediamtx-${target}`;
  const outputPath = path.join(binariesDir, targetExecutableName);
  const temporaryOutputPath = `${outputPath}.tmp`;
  await rm(temporaryOutputPath, { force: true });
  await copyFile(executablePath, temporaryOutputPath);
  if (!target.includes("windows")) {
    await chmod(temporaryOutputPath, 0o755);
  }
  await rename(temporaryOutputPath, outputPath);
  console.log(`Prepared MediaMTX ${version} for ${target}.`);
} finally {
  await rm(temporaryDir, { recursive: true, force: true });
}

async function downloadVerifiedArchive(releaseAsset, outputPath) {
  const url = `https://github.com/bluenviron/mediamtx/releases/download/v${version}/${releaseAsset.name}`;
  const response = await fetch(url, { redirect: "follow" });
  if (!response.ok) {
    throw new Error(`Could not download MediaMTX (${response.status} ${response.statusText}).`);
  }

  const contentLength = Number(response.headers.get("content-length") ?? 0);
  if (contentLength > 100 * 1024 * 1024) {
    throw new Error("The MediaMTX archive exceeds the expected download size.");
  }
  const bytes = Buffer.from(await response.arrayBuffer());
  if (bytes.length > 100 * 1024 * 1024) {
    throw new Error("The MediaMTX archive exceeds the expected download size.");
  }
  const actualHash = createHash("sha256").update(bytes).digest("hex");
  if (actualHash !== releaseAsset.sha256) {
    throw new Error("The MediaMTX archive checksum does not match the pinned official release.");
  }

  const temporaryPath = `${outputPath}.tmp`;
  await rm(temporaryPath, { force: true });
  await writeFile(temporaryPath, bytes, { flag: "wx" });
  await rename(temporaryPath, outputPath);
}

async function isVerifiedArchive(archivePath, expectedHash) {
  try {
    const file = await stat(archivePath);
    if (!file.isFile() || file.size > 100 * 1024 * 1024) return false;
    const hash = createHash("sha256");
    for await (const chunk of createReadStream(archivePath)) {
      hash.update(chunk);
    }
    return hash.digest("hex") === expectedHash;
  } catch (error) {
    if (error.code === "ENOENT") return false;
    throw error;
  }
}

async function findFile(directory, filename) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const entryPath = path.join(directory, entry.name);
    if (entry.isFile() && entry.name === filename) return entryPath;
    if (entry.isDirectory()) {
      const nested = await findFile(entryPath, filename);
      if (nested) return nested;
    }
  }
  return null;
}

function getRustHostTriple() {
  const versionOutput = execFileSync("rustc", ["-vV"], { encoding: "utf8" });
  const host = versionOutput.match(/^host:\s*(.+)$/m)?.[1];
  if (!host) throw new Error("Could not determine the Rust host target for MediaMTX.");
  return host;
}
