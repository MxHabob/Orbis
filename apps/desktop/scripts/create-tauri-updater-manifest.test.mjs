import assert from "node:assert/strict";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { createUpdaterManifest } from "./create-tauri-updater-manifest.mjs";
import {
  createR2UploadPlan,
  planR2ReleasePrune,
} from "./upload-tauri-updater-r2.mjs";

const release = {
  updaterEndpoint: "https://updates.mulverse.com/orbis/latest.json",
  tag: "v1.2.3",
  version: "1.2.3",
  signature: " signed-signature \n",
  notes: "# Orbis v1.2.3\n\n## Summary\n\nMaintenance release.",
};

test("creates Windows updater metadata using the signed setup installer on R2", () => {
  assert.deepEqual(
    createUpdaterManifest({
      ...release,
      target: "x86_64-pc-windows-msvc",
      artifactName: "Orbis_1.2.3_x64-setup.exe",
    }),
    {
      version: "1.2.3",
      notes: "# Orbis v1.2.3\n\n## Summary\n\nMaintenance release.",
      platforms: {
        "windows-x86_64": {
          url: "https://updates.mulverse.com/orbis/releases/v1.2.3/Orbis_1.2.3_x64-setup.exe",
          signature: "signed-signature",
        },
      },
    },
  );
});

test("creates macOS updater metadata for the Apple Silicon archive", () => {
  const manifest = createUpdaterManifest({
    ...release,
    target: "aarch64-apple-darwin",
    artifactName: "Orbis.app.tar.gz",
  });

  assert.equal(
    manifest.platforms["darwin-aarch64"].url,
    "https://updates.mulverse.com/orbis/releases/v1.2.3/Orbis.app.tar.gz",
  );
});

test("normalizes cross-platform release note line endings", () => {
  const windowsNotes = release.notes.replace(/\n/g, "\r\n");
  const manifest = createUpdaterManifest({
    ...release,
    notes: windowsNotes,
    target: "x86_64-pc-windows-msvc",
    artifactName: "Orbis_1.2.3_x64-setup.exe",
  });

  assert.equal(manifest.notes, release.notes);
});

test("rejects unsupported targets and mismatched release versions", () => {
  assert.throws(
    () =>
      createUpdaterManifest({
        ...release,
        target: "x86_64-unknown-linux-gnu",
        artifactName: "Orbis.app.tar.gz",
      }),
    /Unsupported Tauri updater target/,
  );
  assert.throws(
    () =>
      createUpdaterManifest({
        ...release,
        version: "1.2.4",
        target: "aarch64-apple-darwin",
        artifactName: "Orbis.app.tar.gz",
      }),
    /Invalid release tag or version/,
  );
});

test("rejects invalid filenames and empty signatures", () => {
  assert.throws(
    () =>
      createUpdaterManifest({
        ...release,
        target: "aarch64-apple-darwin",
        artifactName: "../Orbis.app.tar.gz",
      }),
    /Invalid updater artifact name/,
  );
  assert.throws(
    () =>
      createUpdaterManifest({
        ...release,
        target: "aarch64-apple-darwin",
        artifactName: "Orbis.app.tar.gz",
        signature: " \n ",
      }),
    /Updater signature is missing/,
  );
  assert.throws(
    () =>
      createUpdaterManifest({
        ...release,
        target: "aarch64-apple-darwin",
        artifactName: "Orbis.app.tar.gz",
        notes: " \n ",
      }),
    /Release notes are missing/,
  );
});

test("plans Orbis R2 uploads under the isolated product prefix", () => {
  const packageDir = mkdtempSync(path.join(os.tmpdir(), "orbis-r2-release-"));
  const windowsName = "Orbis_1.2.3_x64-setup.exe";
  const macName = "Orbis.app.tar.gz";
  const windows = createUpdaterManifest({
    ...release,
    target: "x86_64-pc-windows-msvc",
    artifactName: windowsName,
  });

  test("keeps the newest three Orbis releases without touching AqarDesk", () => {
    const plan = planR2ReleasePrune({
      productPath: "/orbis",
      latestTag: "v1.1.10",
      keys: [
        "orbis/latest.json",
        "orbis/releases/v1.1.8/Orbis.exe",
        "orbis/releases/v1.1.10/Orbis.exe",
        "orbis/releases/v1.1.9/Orbis.exe",
        "orbis/releases/v1.1.7/Orbis.exe",
        "aqardesk/releases/v1.0.2/AqarDesk.exe",
      ],
    });
    assert.deepEqual(plan, [{ tag: "v1.1.7", prefix: "orbis/releases/v1.1.7/" }]);
  });

  test("refuses to prune Orbis releases if the just-uploaded version is missing", () => {
    assert.throws(
      () => planR2ReleasePrune({
        productPath: "/orbis",
        latestTag: "v1.1.10",
        keys: ["orbis/releases/v1.1.9/Orbis.exe"],
      }),
      /was not found in R2; refusing to prune/,
    );
  });
  const mac = createUpdaterManifest({
    ...release,
    target: "aarch64-apple-darwin",
    artifactName: macName,
  });
  const manifest = {
    ...windows,
    platforms: { ...windows.platforms, ...mac.platforms },
  };
  writeFileSync(path.join(packageDir, windowsName), "windows bundle");
  writeFileSync(path.join(packageDir, macName), "mac bundle");
  writeFileSync(
    path.join(packageDir, "latest.json"),
    JSON.stringify(manifest),
  );

  try {
    const plan = createR2UploadPlan({
      updaterEndpoint: release.updaterEndpoint,
      accountId: "a".repeat(32),
      bucket: "mulverse-desktop-updates",
      tag: release.tag,
      packageDir,
    });
    assert.deepEqual(
      plan.map((item) => item.destination),
      [
        `s3://mulverse-desktop-updates/orbis/releases/${release.tag}/${windowsName}`,
        `s3://mulverse-desktop-updates/orbis/releases/${release.tag}/${macName}`,
        "s3://mulverse-desktop-updates/orbis/latest.json",
      ],
    );
    assert.equal(plan.at(-1).cacheControl, "no-cache, no-store, must-revalidate");
  } finally {
    rmSync(packageDir, { recursive: true, force: true });
  }
});
