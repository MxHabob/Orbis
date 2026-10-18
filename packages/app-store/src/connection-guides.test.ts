import { expect, test } from "bun:test";
import { apps } from "./index";

test("Authen is available when its live verifier is implemented", () => {
  expect(apps.find((app) => app.id === "authen")?.active).toBe(true);
});

test("Orbis Mobile is available for explicit activation in the store", () => {
  expect(apps.find((app) => app.id === "orbis_mobile")?.active).toBe(true);
});

test("Orbis AI stays unavailable until its service endpoint is configured", () => {
  expect(apps.find((app) => app.id === "orbis-ai")?.active).toBe(false);
});
