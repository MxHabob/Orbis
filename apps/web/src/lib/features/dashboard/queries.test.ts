import assert from "node:assert/strict";
import { it } from "node:test";
import { dashboardBranch } from "./queries";

it("restores a saved branch only while it remains authorized", () => {
  const units = [{ id: "a", name: "A" }, { id: "b", name: "B" }];
  assert.equal(dashboardBranch({ organization_wide: false, units }, "b"), "b");
  assert.equal(dashboardBranch({ organization_wide: false, units }, "revoked"), "a");
  assert.equal(dashboardBranch({ organization_wide: true, units }, "revoked"), undefined);
  assert.equal(dashboardBranch({ organization_wide: false, units: [] }, "revoked"), undefined);
});
