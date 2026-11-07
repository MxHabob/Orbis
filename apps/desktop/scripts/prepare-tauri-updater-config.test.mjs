import assert from "node:assert/strict";
import test from "node:test";
import { normalizeUpdaterPublicKey } from "./prepare-tauri-updater-config.mjs";

const publicKey = `untrusted comment: minisign public key\n${Buffer.alloc(42).toString("base64")}`;

test("accepts a raw minisign public key", () => {
  assert.equal(normalizeUpdaterPublicKey(publicKey), publicKey);
});

test("decodes a base64-encoded minisign public key", () => {
  const encoded = Buffer.from(`${publicKey}\n`).toString("base64");
  assert.equal(normalizeUpdaterPublicKey(encoded), publicKey);
});

test("rejects a public key without its minisign comment", () => {
  assert.throws(() => normalizeUpdaterPublicKey(Buffer.alloc(42).toString("base64")), /minisign comment or key/);
});

test("rejects a minisign key with an invalid payload length", () => {
  const invalidKey = `untrusted comment: minisign public key\n${Buffer.alloc(32).toString("base64")}`;
  assert.throws(() => normalizeUpdaterPublicKey(invalidKey), /minisign comment or key/);
});

test("rejects invalid base64", () => {
  assert.throws(() => normalizeUpdaterPublicKey("not-a-valid-key"), /minisign public key or its base64 encoding/);
});

test("rejects extra public-key lines", () => {
  assert.throws(() => normalizeUpdaterPublicKey(`${publicKey}\nextra`), /minisign comment or key/);
});
