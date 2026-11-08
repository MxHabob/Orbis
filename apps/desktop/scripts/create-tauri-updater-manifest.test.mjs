import assert from "node:assert/strict";
import test from "node:test";
import { createUpdaterManifest } from "./create-tauri-updater-manifest.mjs";

const release = {
  repository: "MxHabob/Orbis",
  tag: "v1.2.3",
  version: "1.2.3",
  signature: " signed-signature \n",
  notes: "# Orbis v1.2.3\n\n## Summary\n\nMaintenance release.",
};

test("creates Windows updater metadata using the signed setup installer", () => {
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
          url: "https://github.com/MxHabob/Orbis/releases/download/v1.2.3/Orbis_1.2.3_x64-setup.exe",
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
    "https://github.com/MxHabob/Orbis/releases/download/v1.2.3/Orbis.app.tar.gz",
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
