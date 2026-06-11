import {
  adminProcedure,
  createTRPCRouter,
  protectedProcedure,
} from "@api/trpc/init";
import {
  getChildrenByParentId,
  getParentsByStudentId,
  getUserById,
  linkParentToStudent,
  linkParentToStudents,
  unlinkParentFromStudent,
  updateParentStudentLink,
} from "@orbis/db/queries";
import { TRPCError } from "@trpc/server";
import { z } from "zod";

const relationshipEnum = z.enum(["parent", "mother", "father", "guardian"]);

export const parentStudentsRouter = createTRPCRouter({
  // ==========================================
  // Link a parent to a single student
  // ==========================================
  link: adminProcedure
    .input(
      z.object({
        parentId: z.string().uuid(),
        studentId: z.string().uuid(),
        relationship: relationshipEnum.default("parent"),
        isPrimary: z.boolean().default(false),
      }),
    )
    .mutation(async ({ ctx: { db }, input }) => {
      // Verify the parent user actually has the "parent" role
      const parentUser = await getUserById(db, input.parentId);
      if (!parentUser || parentUser.role !== "parent") {
        throw new TRPCError({
          code: "BAD_REQUEST",
          message: "The specified user is not a parent",
        });
      }

      return linkParentToStudent(db, {
        parentId: input.parentId,
        studentId: input.studentId,
        relationship: input.relationship,
        isPrimary: input.isPrimary,
      });
    }),

  // ==========================================
  // Bulk link: parent → multiple students
  // ==========================================
  linkMany: adminProcedure
    .input(
      z.object({
        parentId: z.string().uuid(),
        students: z
          .array(
            z.object({
              studentId: z.string().uuid(),
              relationship: relationshipEnum.default("parent"),
              isPrimary: z.boolean().default(false),
            }),
          )
          .min(1)
          .max(20),
      }),
    )
    .mutation(async ({ ctx: { db }, input }) => {
      const parentUser = await getUserById(db, input.parentId);
      if (!parentUser || parentUser.role !== "parent") {
        throw new TRPCError({
          code: "BAD_REQUEST",
          message: "The specified user is not a parent",
        });
      }

      return linkParentToStudents(db, input.parentId, input.students);
    }),

  // ==========================================
  // Unlink parent from student
  // ==========================================
  unlink: adminProcedure
    .input(
      z.object({
        parentId: z.string().uuid(),
        studentId: z.string().uuid(),
      }),
    )
    .mutation(async ({ ctx: { db }, input }) => {
      await unlinkParentFromStudent(db, input.parentId, input.studentId);
      return { success: true };
    }),

  // ==========================================
  // Update relationship details (e.g. change to primary contact)
  // ==========================================
  updateLink: adminProcedure
    .input(
      z.object({
        parentId: z.string().uuid(),
        studentId: z.string().uuid(),
        relationship: relationshipEnum.optional(),
        isPrimary: z.boolean().optional(),
      }),
    )
    .mutation(async ({ ctx: { db }, input }) => {
      const { parentId, studentId, ...data } = input;
      return updateParentStudentLink(db, parentId, studentId, data);
    }),

  // ==========================================
  // Get children for a parent
  // ==========================================
  getChildren: protectedProcedure
    .input(z.object({ parentId: z.string().uuid() }))
    .query(async ({ ctx: { db, session }, input }) => {
      // Parents can view their own children, admins can view any
      const currentUser = await getUserById(db, session.user.id);
      if (currentUser?.role !== "admin" && session.user.id !== input.parentId) {
        throw new TRPCError({ code: "FORBIDDEN" });
      }

      return getChildrenByParentId(db, input.parentId);
    }),

  // ==========================================
  // Get parents for a student
  // ==========================================
  getParents: protectedProcedure
    .input(z.object({ studentId: z.string().uuid() }))
    .query(async ({ ctx: { db }, input }) => {
      return getParentsByStudentId(db, input.studentId);
    }),
});
