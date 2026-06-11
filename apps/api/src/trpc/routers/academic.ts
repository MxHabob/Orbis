import {
  createAcademicTermSchema,
  createAcademicYearSchema,
  updateAcademicTermSchema,
  updateAcademicYearSchema,
} from "@api/schemas/academic";
import {
  adminProcedure,
  createTRPCRouter,
  protectedProcedure,
} from "@api/trpc/init";
import {
  createAcademicTerm,
  createAcademicYear,
  deleteAcademicTerm,
  deleteAcademicYear,
  getAcademicYearById,
  getAcademicYears,
  getCurrentAcademicYear,
  getCurrentTerm,
  getTermsByYear,
  updateAcademicTerm,
  updateAcademicYear,
} from "@orbis/db/queries";
import { z } from "zod";

export const academicRouter = createTRPCRouter({
  getYears: protectedProcedure
    .input(
      z
        .object({
          cursor: z.string().nullish(),
          pageSize: z.number().optional(),
          sort: z.array(z.string()).optional(),
          activeOnly: z.boolean().optional(),
        })
        .optional(),
    )
    .query(async () => {
      return getAcademicYears();
    }),

  getCurrentYear: protectedProcedure.query(async () => {
    return getCurrentAcademicYear();
  }),

  getYearById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input }) => {
      return getAcademicYearById(input.id);
    }),

  getTerms: protectedProcedure
    .input(z.object({ yearId: z.number() }))
    .query(async ({ input }) => {
      return getTermsByYear(input.yearId);
    }),

  createYear: adminProcedure
    .input(createAcademicYearSchema)
    .mutation(async ({ input }) => {
      return createAcademicYear(input);
    }),

  updateYear: adminProcedure
    .input(updateAcademicYearSchema)
    .mutation(async ({ input }) => {
      return updateAcademicYear(input.id, input);
    }),

  deleteYear: adminProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteAcademicYear(input.id);
    }),

  createTerm: adminProcedure
    .input(createAcademicTermSchema)
    .mutation(async ({ input }) => {
      return createAcademicTerm(input);
    }),

  updateTerm: adminProcedure
    .input(updateAcademicTermSchema)
    .mutation(async ({ input }) => {
      return updateAcademicTerm(input.id, input);
    }),

  deleteTerm: adminProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteAcademicTerm(input.id);
    }),

  connectionStatus: protectedProcedure.query(async () => {
    const currentYear = await getCurrentAcademicYear();
    const currentTerm = await getCurrentTerm();

    const integrations = [
      {
        id: "Bank",
        name: "Bank",
        status: "connected",
        type: "Bank",
      },
      {
        id: "whatsapp",
        name: "whatsapp",
        status: "disconnected",
        type: "whatsapp",
      },
      {
        id: "sms",
        name: "sms",
        status: "warning",
        type: "sms",
      },
    ];

    const issues = [...integrations];

    if (!currentYear) {
      issues.push({
        id: "academic-year",
        name: "academic year",
        status: "disconnected",
        type: "academic",
      });
    }

    return {
      integrations: issues,
    };
  }),
});
