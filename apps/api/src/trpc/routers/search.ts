import {
  globalSearchSchema,
  searchAttachmentsSchema,
} from "@api/schemas/search";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { getClasses } from "@midday/db/queries/class";
import { getStudents } from "@midday/db/queries/student";
import { getSubjects } from "@midday/db/queries/subject";
import { getTeachers } from "@midday/db/queries/teacher";

export const searchRouter = createTRPCRouter({
  global: protectedProcedure
    .input(globalSearchSchema)
    .query(async ({ input, ctx: { db } }) => {
      const { searchTerm } = input;

      if (!searchTerm || searchTerm.length < 2) {
        return [];
      }

      // Parallel search across key entities
      const [students, teachers, classes, subjects] = await Promise.all([
        getStudents(db, { q: searchTerm, pageSize: 5 }),
        getTeachers(db, { q: searchTerm, pageSize: 5 }),
        getClasses(db, { q: searchTerm, pageSize: 5 }),
        getSubjects(db, { q: searchTerm, pageSize: 5 }),
      ]);

      const results = [
        ...students.data.map((s) => ({
          type: "student",
          id: s.userId,
          relevance: 1,
          created_at: new Date().toISOString(),
          data: {
            title: s.name || s.surname,
            subtitle: `Student - ${s.studentNumber}`,
            url: `/students/${s.userId}`,
            ...s,
          },
        })),
        ...teachers.data.map((t) => ({
          type: "teacher",
          id: t.userId,
          relevance: 1,
          created_at: new Date().toISOString(),
          data: {
            title: t.name,
            subtitle: `Teacher - ${t.employeeNumber}`,
            url: `/teachers/${t.userId}`,
            ...t,
          },
        })),
        ...classes.data.map((c) => ({
          type: "class",
          id: c.id.toString(),
          relevance: 1,
          created_at: new Date().toISOString(),
          data: {
            title: c.name,
            subtitle: `Class - ${c.code || ""}`,
            url: `/classes/${c.id}`,
            ...c,
          },
        })),
        ...subjects.data.map((s) => ({
          type: "subject",
          id: s.id.toString(),
          relevance: 1,
          created_at: new Date().toISOString(),
          data: {
            title: s.name,
            subtitle: `Subject - ${s.code || ""}`,
            url: `/subjects/${s.id}`,
            ...s,
          },
        })),
      ];

      return results;
    }),

  attachments: protectedProcedure
    .input(searchAttachmentsSchema)
    .query(async () => {
      // TODO: Implement attachment search for student documents/homework
      return [];
    }),
});
