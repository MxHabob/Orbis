import { z } from "zod";

/**
 * Figma-Style Design Persistence Schema (2026)
 *
 * We follow the philosophy that the database table is just a metadata shell.
 * 100% of the design logic, frames, nodes, and configurations are stored
 * inside the `content` JSONB field.
 */

export const upsertCertificateTemplateSchema = z.object({
  id: z.string().uuid().optional(),
  name: z.string().min(1).optional(),
  description: z.string().optional(),
  category: z.string().optional(),
  isDefault: z.boolean().optional(),
  isArchived: z.boolean().optional(),

  // High-performance metadata for list views
  thumbnailUrl: z.string().nullable().optional(),
  version: z.number().optional(),
  tags: z.array(z.string()).optional(),

  // The Complete Document Object (Figma-like Node Tree)
  // [elements, frameSettings, canvasStyles, etc.]
  content: z.any(),
});

export const createCertificateTemplateSchema = z.object({
  name: z.string().min(1),
  category: z.string().default("design"),
  content: z.any().default({
    elements: [],
    canvasConfig: { size: "a4", orientation: "portrait" },
  }),
});
