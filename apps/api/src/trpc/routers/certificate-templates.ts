import { createTRPCRouter, protectedProcedure, adminProcedure, teacherProcedure } from "@api/trpc/init";
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
import { getDownloadUrl } from "@midday/storage";

// Helper to sign background image URLs if they are proxy URLs or internal paths
// Best Practice 2026: Provide pre-authorized URLs for protected assets to avoid CORS/Auth issues in <img> tags
async function signUrl(urlStr: string) {
  if (!urlStr || typeof urlStr !== 'string') return urlStr;
  
  // 1. If it's already a public URL (e.g., S3, Cloudinary), leave it
  if (urlStr.startsWith('http') && !urlStr.includes('/files/proxy')) {
    return urlStr;
  }

  try {
    let filePath: string | null = null;
    
    // 2. If it's a proxy URL, extract filePath
    if (urlStr.includes('/files/proxy')) {
       // Support relative or absolute proxy URLs
       const url = new URL(urlStr.startsWith('http') ? urlStr : `http://localhost${urlStr}`);
       filePath = url.searchParams.get("filePath");
    } else if (urlStr.startsWith('vault/')) {
       // 3. If it's a raw path
       filePath = urlStr;
    }

    if (filePath?.startsWith("vault/")) {
      const key = filePath.replace("vault/", "");
      return await getDownloadUrl({
        bucket: "vault",
        key,
        expiresIn: 3600, // Valid for 1 hour
      });
    }
  } catch (e) {
    console.error("Failed to sign URL", e);
  }

  return urlStr;
}

async function signTemplateBackground(template: any) {
  if (!template) return template;

  // Sign main background image
  if (template.backgroundImageUrl) {
    template.backgroundImageUrl = await signUrl(template.backgroundImageUrl);
  }

  // Sign images inside elements (logos, signatures, etc.)
  if (template.content?.elements && Array.isArray(template.content.elements)) {
    template.content.elements = await Promise.all(
      template.content.elements.map(async (el: any) => {
        if (el.type === "image" && el.content) {
          return {
            ...el,
            content: await signUrl(el.content),
          };
        }
        return el;
      })
    );
  }

  return template;
}

export const certificateTemplateRouter = createTRPCRouter({
  list: protectedProcedure
    .input(z.object({ q: z.string().optional() }).optional())
    .query(async ({ input }) => {
      const templates = await listCertificateTemplates(input?.q);
      // Even for lists, we want to sign backgrounds if we decide to show them in the gallery
      return Promise.all(templates.map(signTemplateBackground));
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .query(async ({ input }) => {
      const template = await getCertificateTemplateById(input.id);
      return signTemplateBackground(template);
    }),

  getDefault: protectedProcedure.query(async () => {
    const template = await getDefaultCertificateTemplate();
    return signTemplateBackground(template);
  }),

  upsert: adminProcedure
    .input(upsertCertificateTemplateSchema)
    .mutation(({ input }) => upsertCertificateTemplate(input)),

  create: adminProcedure
    .input(createCertificateTemplateSchema)
    .mutation(({ input }) => createCertificateTemplate(input)),

  setDefault: adminProcedure
    .input(z.object({ id: z.string().uuid() }))
    .mutation(({ input }) => setDefaultCertificateTemplate(input.id)),

  delete: adminProcedure
    .input(z.object({ id: z.string().uuid() }))
    .mutation(({ input }) => deleteCertificateTemplate(input.id)),

  summary: protectedProcedure.query(() => getCertificateTemplatesSummary()),

  startExport: teacherProcedure
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
