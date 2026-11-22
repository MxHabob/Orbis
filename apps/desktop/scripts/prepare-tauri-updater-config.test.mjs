import assert from "node:assert/strict";
import test from "node:test";
import {
  normalizeUpdaterEndpoint,
  normalizeHttpsUrl,
  normalizeUpdaterPublicKey,
} from "./prepare-tauri-updater-config.mjs";

const publicKey = `untrusted comment: minisign public key\n${Buffer.alloc(42).toString("base64")}`;

test("validates and base64-encodes a minisign public key", () => {
  assert.equal(
    Buffer.from(normalizeUpdaterPublicKey(publicKey), "base64").toString(
      "utf8",
    ),
    publicKey,
  );
});

test("decodes a base64-encoded minisign public key", () => {
  const encoded = Buffer.from(`${publicKey}\n`).toString("base64");
  assert.equal(
    Buffer.from(normalizeUpdaterPublicKey(encoded), "base64").toString("utf8"),
    publicKey,
  );
});

test("rejects malformed or missing minisign public keys", () => {
  assert.throws(() => normalizeUpdaterPublicKey(""), /required/);
  assert.throws(
    () => normalizeUpdaterPublicKey("untrusted comment: invalid\nabcd"),
    /valid minisign public key/,
  );
  assert.throws(
    () => normalizeUpdaterPublicKey(Buffer.alloc(42).toString("base64")),
    /valid minisign public key/,
  );
  assert.throws(
    () =>
      normalizeUpdaterPublicKey(
        `untrusted comment: minisign public key\n${Buffer.alloc(32).toString("base64")}`,
      ),
    /valid minisign public key/,
  );
  assert.throws(
    () => normalizeUpdaterPublicKey("not-a-valid-key"),
    /minisign public key or its base64 encoding/,
  );
  assert.throws(
    () => normalizeUpdaterPublicKey(`${publicKey}\nextra`),
    /valid minisign public key/,
  );
});

test("accepts HTTPS endpoints and canonicalizes URLs", () => {
  assert.equal(
    normalizeHttpsUrl(
      "https://updates.example.com/orbis/latest.json",
      "endpoint",
    ),
    "https://updates.example.com/orbis/latest.json",
  );
});

test("requires a product-specific updater manifest path", () => {
  assert.equal(
    normalizeUpdaterEndpoint("https://updates.mulverse.com/orbis/latest.json"),
    "https://updates.mulverse.com/orbis/latest.json",
  );
  assert.throws(
    () => normalizeUpdaterEndpoint("https://updates.mulverse.com/latest.json"),
    /product-path/,
  );
  assert.throws(
    () =>
      normalizeUpdaterEndpoint(
        "https://updates.mulverse.com/orbis/manifest.json",
      ),
    /product-path/,
  );
});

test("rejects non-HTTPS and ambiguous endpoints", () => {
  for (const value of [
    "http://updates.example.com/latest.json",
    "******updates.example.com/latest.json",
    "https://updates.example.com/latest.json?token=secret",
    "https://updates.example.com/latest.json#fragment",
    "not-a-url",
  ]) {
    assert.throws(
      () => normalizeHttpsUrl(value, "endpoint"),
      /valid HTTPS URL|must use HTTPS/,
    );
  }
  assert.throws(
    () =>
      normalizeUpdaterEndpoint("http://updates.mulverse.com/orbis/latest.json"),
    /must use HTTPS/,
  );
});
