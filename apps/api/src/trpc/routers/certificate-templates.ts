import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  listCertificateTemplates,
  getCertificateTemplateById,
  getDefaultCertificateTemplate,
  upsertCertificateTemplate,
  createCertificateTemplate,
  setDefaultCertificateTemplate,
  deleteCertificateTemplate,
} from "@midday/db/queries";
import {
  upsertCertificateTemplateSchema,
  createCertificateTemplateSchema,
} from "@api/schemas/certificate-templates";
import { z } from "zod";

export const certificateTemplateRouter = createTRPCRouter({
  list: protectedProcedure.query(() => listCertificateTemplates()),

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
});
