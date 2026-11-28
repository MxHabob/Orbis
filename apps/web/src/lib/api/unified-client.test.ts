import assert from "node:assert/strict";
import { describe, it } from "node:test";
import type { StudentDto } from "./types";
import {
  mapStudentDtoToDetail,
  UnifiedClient as RemoteClient,
} from "./unified-client";

describe("mapStudentDtoToDetail", () => {
  it("preserves the flattened student avatar on the person used by UI consumers", () => {
    const student: StudentDto = {
      id: "student-id",
      org_id: "org-id",
      person_id: "person-id",
      student_number: "STD-001",
      first_name: "Mona",
      last_name: "Ali",
      full_name: "Mona Ali",
      gender: "female",
      standing: "good_standing",
      status: "active",
      admitted_at: "2026-01-01T00:00:00Z",
      created_at: "2026-01-01T00:00:00Z",
      updated_at: "2026-01-01T00:00:00Z",
      avatar_url: "students/avatar.png",
    };

    const detail = mapStudentDtoToDetail(student);

    assert.equal(detail.student.avatar_url, "students/avatar.png");
    assert.equal(detail.person.avatar_url, "students/avatar.png");
  });
});

it("uses the backend response envelope for startup and session authorization", async () => {
  const originalFetch = globalThis.fetch;
  globalThis.fetch = (async (input) => {
    assert.ok(input instanceof Request);
    const path = new URL(input.url).pathname;
    switch (path) {
      case "/api/v1/system/has-organization":
        return Response.json({ data: { has_organization: false } });
      case "/api/v1/auth/login":
        return Response.json({ data: { token: "issued-token" } });
      case "/api/v1/auth/session":
        assert.equal(input.headers.get("Authorization"), "Bearer issued-token");
        return Response.json({ data: { user: { id: "signed-in-user" } } });
      default:
        throw new Error(`Unexpected request to ${path}`);
    }
  }) as typeof fetch;

  try {
    const client = new RemoteClient("https://orbis.example");
    assert.equal(await client.systemHasOrganization(), false);
    const login = await client.login({
      email: "example@example.test",
      password: "test",
    });
    assert.equal(login.token, "issued-token");
    assert.equal((await client.getSession()).user.id, "signed-in-user");
  } finally {
    globalThis.fetch = originalFetch;
  }
});
