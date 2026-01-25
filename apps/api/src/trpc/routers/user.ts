import { createUserSchema, updateUserSchema } from "@api/schemas/user";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { getAllUsersWithRoles, getUserWithRoles, updateUser, createManyUsers } from "@midday/db/queries/users";
import { z } from "zod";

export const userRouter = createTRPCRouter({
  get: protectedProcedure.query(async ({ ctx: { db, session } }) => {
    return getAllUsersWithRoles(db, session);
  }),

  create: protectedProcedure
    .input(z.array(createUserSchema))
    .mutation(async ({ ctx: { db }, input }) => {
      const createdUsers = await createManyUsers(db, input.map(user => ({
        name: user.name ?? user.email.split('@')[0] ?? user.email,
        email: user.email,
        role: user.role,
      })));

      return createdUsers;
    }),

  me: protectedProcedure.query(async ({ ctx: { db, session } }) => {
    return getUserWithRoles(db, session.user.id);
  }),

  update: protectedProcedure
    .input(updateUserSchema)
    .mutation(async ({ ctx: { db, session }, input }) => {
      const updatedUser = await updateUser(db, {
        id: session.user.id,
        name: input.fullName,
        image: input.avatarUrl,
      });

      return updatedUser;
    }),
});
