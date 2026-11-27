import assert from "node:assert/strict";
import { it } from "node:test";
import { createOrbisClient } from "./create-client.js";

it("keeps institution endpoints, authorization, and abort signals per client", async () => {
  const requests: Request[] = [];
  const fetchRequest: typeof fetch = async (input) => {
    assert.ok(input instanceof Request);
    requests.push(input);
    return Response.json(true);
  };
  const first = createOrbisClient({
    baseUrl: "https://first.example/orbis",
    headers: () => ({ Authorization: "Bearer first" }),
  });
  const second = createOrbisClient({
    baseUrl: "https://second.example",
    headers: () => ({ Authorization: "Bearer second" }),
  });
  first.client.setConfig({ fetch: fetchRequest });
  second.client.setConfig({ fetch: fetchRequest });

  const controller = new AbortController();
  await Promise.all([
    first.getSystemHasOrgEndpoint({
      signal: controller.signal,
      throwOnError: true,
    }),
    second.getSystemHasOrgEndpoint({ throwOnError: true }),
  ]);

  assert.deepEqual(
    requests
      .map((request) => [
        new URL(request.url).origin,
        new URL(request.url).pathname,
        request.headers.get("Authorization"),
      ])
      .sort(),
    [
      [
        "https://first.example",
        "/orbis/api/v1/system/has-organization",
        "Bearer first",
      ],
      [
        "https://second.example",
        "/api/v1/system/has-organization",
        "Bearer second",
      ],
    ],
  );
  controller.abort();
  const firstRequest = requests.find((request) =>
    request.url.startsWith("https://first.example/"),
  );
  assert.equal(firstRequest?.signal.aborted, true);
});

it("does not send an in-flight request after its endpoint changes", async () => {
  const started = Promise.withResolvers<void>();
  const resume = Promise.withResolvers<Record<string, string>>();
  const api = createOrbisClient({
    baseUrl: "https://original.example",
    headers: () => {
      started.resolve();
      return resume.promise;
    },
  });
  let sent = false;
  api.client.setConfig({
    fetch: async () => {
      sent = true;
      return Response.json(true);
    },
  });
  const request = api.getSystemHasOrgEndpoint({ throwOnError: true });
  await started.promise;
  api.client.setConfig({ baseUrl: "https://new.example" });
  resume.resolve({ Authorization: "Bearer new" });
  await assert.rejects(request, /endpoint changed/);
  assert.equal(sent, false);
});
