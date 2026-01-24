import { z } from "zod";

/**
 * Schema for creating a parent
 */
export const createParentSchema = z.object({
  userId: z.string().uuid().optional(),
  surname: z.string().min(1).max(255),
  phone: z.string().min(1).max(50),
  address: z.string().optional().nullable(),
  // User fields
  name: z.string().min(1).max(255).optional().nullable(),
  email: z.string().email().optional().nullable(),
  image: z.string().url().optional().nullable(),
});

/**
 * Schema for updating a parent
 */
export const updateParentSchema = z.object({
  surname: z.string().min(1).max(255).optional(),
  phone: z.string().min(1).max(50).optional(),
  address: z.string().optional().nullable(),
  // User fields
  name: z.string().min(1).max(255).optional().nullable(),
  email: z.string().email().optional().nullable(),
  image: z.string().url().optional().nullable(),
});

/**
 * Schema for getting parents
 */
export const getParentsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  q: z.string().optional().nullable(),
  sort: z.array(z.string()).length(2).optional(),
});

/**
 * Schema for getting parent by phone
 */
export const getParentByPhoneSchema = z.object({
  phone: z.string().min(1).max(50),
});
