import { createTRPCRouter, protectedProcedure } from "../init";
import { z } from "zod";
import { students, classes, users, subjects, documents } from "@midday/db/schema";
import { ilike, or, and, eq } from "drizzle-orm";

export const searchRouter = createTRPCRouter({
  global: protectedProcedure
    .input(z.object({ searchTerm: z.string() }))
    .query(async ({ ctx: { db, session }, input }) => {
      const { searchTerm } = input;
      if (!searchTerm || searchTerm.length < 2) return [];

      const query = `%${searchTerm}%`;

      const [studentResults, classResults, subjectResults, staffResults, documentResults] = await Promise.all([
        db
          .select({
            id: students.userId,
            title: users.name,
            studentNumber: students.studentNumber,
          })
          .from(students)
          .innerJoin(users, eq(students.userId, users.id))
          .where(
            or(
              ilike(users.name, query),
              ilike(students.studentNumber, query)
            )
          )
          .limit(5),
        db
          .select({
            id: classes.id,
            title: classes.name,
          })
          .from(classes)
          .where(ilike(classes.name, query))
          .limit(5),
        db
          .select({
            id: subjects.id,
            title: subjects.name,
          })
          .from(subjects)
          .where(ilike(subjects.name, query))
          .limit(5),
        db
          .select({
            id: users.id,
            title: users.name,
            email: users.email,
            role: users.role,
          })
          .from(users)
          .where(
            and(
              or(eq(users.role, 'teacher'), eq(users.role, 'admin'), eq(users.role, 'staff')),
              or(ilike(users.name, query), ilike(users.email, query))
            )
          )
          .limit(5),
        db
          .select({
            id: documents.id,
            name: documents.name,
            title: documents.title,
            mimetype: documents.mimetype,
            pathTokens: documents.pathTokens,
          })
          .from(documents)
          .where(
            and(
              eq(documents.userId, session.user.id),
              or(ilike(documents.name, query), ilike(documents.title, query))
            )
          )
          .limit(5),
      ]);

      const results = [
        ...studentResults.map((s) => ({
          id: s.id,
          type: "student",
          title: s.title ?? "Unknown",
          data: { name: s.title ?? "Unknown", student_number: s.studentNumber ?? undefined },
        })),
        ...classResults.map((c) => ({
          id: c.id.toString(),
          type: "class",
          title: c.title ?? "Untitled Class",
          data: { name: c.title ?? "Untitled Class" },
        })),
        ...subjectResults.map((s) => ({
          id: s.id.toString(),
          type: "subject",
          title: s.title ?? "Untitled Subject",
          data: { name: s.title ?? "Untitled Subject" },
        })),
        ...staffResults.map((t) => ({
          id: t.id,
          type: "staff",
          title: t.title ?? "Unknown Staff",
          data: { name: t.title ?? "Unknown Staff", email: t.email ?? undefined, status: t.role ?? undefined },
        })),
        ...documentResults.map((d) => ({
          id: d.id,
          type: "vault",
          title: d.title || d.name || "Untitled Document",
          data: { 
            name: d.name || "Untitled Document", 
            title: d.title ?? undefined, 
            metadata: { mimetype: d.mimetype ?? undefined },
            path_tokens: (d.pathTokens as string[]) ?? []
          },
        })),
      ];

      return results;
    }),
});
