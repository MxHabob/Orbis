import assert from "node:assert/strict";
import { describe, it } from "node:test";
import type { StudentDto } from "./types";
import { mapStudentDtoToDetail } from "./remote-client";

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
