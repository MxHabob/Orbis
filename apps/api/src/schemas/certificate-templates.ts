import { z } from "zod";

export const upsertCertificateTemplateSchema = z.object({
  id: z.string().uuid().optional(),
  name: z.string().min(1).optional(),
  isDefault: z.boolean().optional(),
  
  // Layout
  orientation: z.enum(["portrait", "landscape"]).optional(),
  size: z.enum(["a4", "letter"]).optional(),
  primaryColor: z.string().optional(),
  
  // Content JSON
  headerContent: z.any().nullable().optional(),
  footerContent: z.any().nullable().optional(),
  tableConfig: z.any().nullable().optional(),
});

export const createCertificateTemplateSchema = z.object({
  name: z.string().min(1),
  isDefault: z.boolean().default(false),
  
  // Layout
  orientation: z.enum(["portrait", "landscape"]).default("portrait"),
  size: z.enum(["a4", "letter"]).default("a4"),
  primaryColor: z.string().default("#000000"),
  
  // Content JSON
  headerContent: z.any().nullable().optional(),
  footerContent: z.any().nullable().optional(),
  tableConfig: z.any().nullable().optional(),
});
