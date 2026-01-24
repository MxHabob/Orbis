import { updateUserSchema } from "@api/schemas/user";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { getUserWithRoles, updateUser } from "@midday/db/queries/users";

export const userRouter = createTRPCRouter({
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
