import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  listCertificateTemplates,
  getCertificateTemplateById,
  getDefaultCertificateTemplate,
  upsertCertificateTemplate,
  createCertificateTemplate,
  setDefaultCertificateTemplate,
  deleteCertificateTemplate,
  getCertificateTemplatesSummary,
} from "@midday/db/queries";
import {
  upsertCertificateTemplateSchema,
  createCertificateTemplateSchema,
} from "@api/schemas/certificate-templates";
import { z } from "zod";

export const certificateTemplateRouter = createTRPCRouter({
  list: protectedProcedure
    .input(z.object({ q: z.string().optional() }).optional())
    .query(({ input }) => listCertificateTemplates(input?.q)),

  getById: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .query(({ input }) => getCertificateTemplateById(input.id)),

  getDefault: protectedProcedure.query(() => getDefaultCertificateTemplate()),

  upsert: protectedProcedure
    .input(upsertCertificateTemplateSchema)
    .mutation(({ input }) => upsertCertificateTemplate(input)),

  create: protectedProcedure
    .input(createCertificateTemplateSchema)
    .mutation(({ input }) => createCertificateTemplate(input)),

  setDefault: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .mutation(({ input }) => setDefaultCertificateTemplate(input.id)),

  delete: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .mutation(({ input }) => deleteCertificateTemplate(input.id)),

  summary: protectedProcedure.query(() => getCertificateTemplatesSummary()),

  startExport: protectedProcedure
    .input(z.object({
      classIds: z.array(z.string()),
      templateId: z.string().optional(),
      certificateType: z.string(),
      selectedMonth: z.number(),
    }))
    .mutation(async ({ input, ctx }) => {
      const { JobService } = await import("@midday/jobs/service");
      const result = await JobService.triggerCertificateExport({
        ...input,
        userId: ctx.session.user.id,
      });

      return {
        id: result.jobId,
      };
    }),
});
