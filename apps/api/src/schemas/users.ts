import { z } from "@hono/zod-openapi";

export const roleEnum = z.enum(['admin', 'student', 'parent', 'teacher', 'staff']);
export const userSexEnum = z.enum(['unspecified', 'male', 'female']);

export const userSchema = z.object({
  id: z.string().uuid().openapi({
    description: "Unique identifier of the user",
    example: "123e4567-e89b-12d3-a456-426614174000",
  }),
  username: z.string().max(255).nullable().openapi({
    example: "johndoe",
  }),
  name: z.string().max(255).nullable().openapi({
    example: "John Doe",
  }),
  phone: z.string().max(255).nullable().openapi({
    example: "+1234567890",
  }),
  email: z.string().email().nullable().openapi({
    example: "john@example.com",
  }),
  emailVerified: z.boolean().default(false).openapi({
    example: true,
  }),
  role: roleEnum.default('student').openapi({
    example: "student",
  }),
  sex: userSexEnum.default('unspecified').openapi({
    example: "male",
  }),
  image: z.string().max(255).nullable().openapi({
    example: "https://example.com/image.jpg",
  }),
  metadata: z.record(z.string(), z.any()).nullable().openapi({
    example: {},
  }),
});

export const createUserSchema = userSchema.omit({ 
  id: true, 
  emailVerified: true 
});

export const updateUserSchema = userSchema.partial().omit({ 
  emailVerified: true 
});
