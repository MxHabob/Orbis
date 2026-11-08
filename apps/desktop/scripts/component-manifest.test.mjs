import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { createComponentPayload } from "./create-component-manifest.mjs";
import { createSignedEnvelope } from "./wrap-component-manifest.mjs";

test("creates cross-platform manifest with exact SHA-256 and artifact sizes", () => {
  const directory = mkdtempSync(path.join(os.tmpdir(), "orbis-component-"));
  try {
    const files = [
      "orbis-solver-1.2.3-x86_64-pc-windows-msvc.exe",
      "orbis-solver-1.2.3-aarch64-apple-darwin",
    ];
    const contents = files.map((name) => {
      const bytes = Buffer.from(`binary:${name}`);
      writeFileSync(path.join(directory, name), bytes);
      return bytes;
    });

    const payload = createComponentPayload({
      version: "1.2.3",
      artifactDirectory: directory,
      artifactBaseUrl: "https://github.com/example/orbis/releases/download/solver-v1.2.3/",
    });

    assert.deepEqual(Object.keys(payload.platforms).sort(), [
      "aarch64-apple-darwin",
      "x86_64-pc-windows-msvc",
    ]);
    assert.equal(payload.platforms["x86_64-pc-windows-msvc"].size_bytes, contents[0].length);
    assert.equal(
      payload.platforms["x86_64-pc-windows-msvc"].sha256,
      createHash("sha256").update(contents[0]).digest("hex"),
    );
    assert.equal(
      payload.platforms["aarch64-apple-darwin"].sha256,
      createHash("sha256").update(contents[1]).digest("hex"),
    );
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("rejects invalid versions, URLs, and incomplete release artifacts", () => {
  const directory = mkdtempSync(path.join(os.tmpdir(), "orbis-component-"));
  try {
    assert.throws(
      () => createComponentPayload({
        version: "1.2",
        artifactDirectory: directory,
        artifactBaseUrl: "https://example.com/releases/",
      }),
      /SemVer/,
    );
    assert.throws(
      () => createComponentPayload({
        version: "1.2.3",
        artifactDirectory: directory,
        artifactBaseUrl: "http://example.com/releases/",
      }),
      /HTTPS/,
    );
    assert.throws(
      () => createComponentPayload({
        version: "1.2.3",
        artifactDirectory: directory,
        artifactBaseUrl: "https://example.com/releases/",
      }),
    );
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("preserves the exact signed payload bytes in the envelope", () => {
  const payload = '{\n  "schema_version": 1\n}\n';
  const envelope = createSignedEnvelope(payload, "minisign-signature\n");
  assert.equal(envelope.payload, payload);
  assert.equal(envelope.signature, "minisign-signature");
  assert.throws(() => createSignedEnvelope("", "signature"), /required/);
});
