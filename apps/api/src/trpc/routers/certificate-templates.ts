import {
  createCertificateTemplateSchema,
  upsertCertificateTemplateSchema,
} from "@api/schemas/certificate-templates";
import {
  adminProcedure,
  createTRPCRouter,
  protectedProcedure,
  teacherProcedure,
} from "@api/trpc/init";
import {
  createCertificateTemplate,
  deleteCertificateTemplate,
  getCertificateTemplateById,
  getCertificateTemplatesSummary,
  getDefaultCertificateTemplate,
  listCertificateTemplates,
  setDefaultCertificateTemplate,
  upsertCertificateTemplate,
} from "@orbis/db/queries";
import { z } from "zod";

// Best Practice 2026: All image URLs are normalized to proxy URLs
// This ensures they work in both browser AND Puppeteer headless contexts.
// Pre-signed S3 URLs are NOT used because they point to internal RustFS/S3-compatible storage
// (e.g. 172.16.x.x:9088) which may be unreachable from Puppeteer's Chromium.
function toProxyUrl(urlStr: string): string {
  if (!urlStr || typeof urlStr !== "string") return urlStr;

  // Data URIs are always passthrough
  if (urlStr.startsWith("data:")) return urlStr;

  // Already a proxy URL — leave as-is
  if (urlStr.includes("/files/proxy")) return urlStr;

  // S3/Storage HTTP URLs — extract path and build proxy URL
  if (urlStr.startsWith("http")) {
    try {
      const parsed = new URL(urlStr);
      const rawPath = parsed.pathname.startsWith("/")
        ? parsed.pathname.slice(1)
        : parsed.pathname;
      if (rawPath) {
        // Return a relative proxy URL — the dashboard will prepend NEXT_PUBLIC_API_URL
        return `/files/proxy?filePath=${encodeURIComponent(rawPath)}`;
      }
    } catch {
      // Fall through
    }
    return urlStr;
  }

  // Raw file paths (e.g., "vault/xxx/yyy.png") — build proxy URL
  if (!urlStr.startsWith("/")) {
    return `/files/proxy?filePath=${encodeURIComponent(urlStr)}`;
  }

  return urlStr;
}

function normalizeTemplateUrls(template: any) {
  if (!template) return template;

  // 1. Normalize root-level background URL
  if (template.backgroundImageUrl) {
    template.backgroundImageUrl = toProxyUrl(template.backgroundImageUrl);
  }

  // 2. Normalize backgrounds in the Figma-style Content Tree
  if (template.content?.canvasConfig?.backgroundImage) {
    template.content.canvasConfig.backgroundImage = toProxyUrl(
      template.content.canvasConfig.backgroundImage,
    );
  }

  // 3. Normalize images inside elements (nodes tree)
  if (template.content?.elements && Array.isArray(template.content.elements)) {
    template.content.elements = template.content.elements.map((el: any) => {
      if (el.type === "image" && el.content) {
        return { ...el, content: toProxyUrl(el.content) };
      }
      return el;
    });
  }

  return template;
}

export const certificateTemplateRouter = createTRPCRouter({
  list: protectedProcedure
    .input(z.object({ q: z.string().optional() }).optional())
    .query(async ({ input }) => {
      const templates = await listCertificateTemplates(input?.q);
      return templates.map(normalizeTemplateUrls);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .query(async ({ input }) => {
      const template = await getCertificateTemplateById(input.id);
      return normalizeTemplateUrls(template);
    }),

  getDefault: protectedProcedure.query(async () => {
    const template = await getDefaultCertificateTemplate();
    return normalizeTemplateUrls(template);
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
    .input(
      z.object({
        classIds: z.array(z.string()),
        templateId: z.string().optional(),
        certificateType: z.string(),
        selectedMonth: z.number(),
      }),
    )
    .mutation(async ({ input, ctx }) => {
      const { JobService } = await import("@orbis/jobs/service");
      const result = await JobService.triggerCertificateExport({
        ...input,
        userId: ctx.session.user.id,
      });

      return {
        id: result.jobId,
      };
    }),
});
