import { z } from "zod";

export const registrationSettingsSchema = z
  .object({
    institution_type: z.string().trim().min(1),
    student_number_pattern: z.string().trim().min(1).max(80),
    stages: z.array(z.string().trim().min(1)).min(1),
    active_fields: z.array(z.string().trim().min(1)),
    required_fields: z.array(z.string().trim().min(1)),
    prevent_duplicates: z.array(z.enum(["national_id", "email", "phone"])),
    required_documents: z.array(z.string().trim().min(1)),
    student_number_prefix: z.string().trim().min(1).max(20),
    student_number_digits: z.number().int().min(1).max(12),
    requires_approval: z.boolean(),
    max_class_capacity: z.number().int().positive().nullable(),
    allow_no_guardian: z.boolean(),
    allow_duplicate_national_id: z.boolean(),
    auto_assign_fees: z.boolean(),
    default_fee_template_ids: z.array(z.string().trim().min(1)),
  })
  .superRefine((value, context) => {
    for (const field of value.required_fields) {
      if (!value.active_fields.includes(field)) {
        context.addIssue({
          code: "custom",
          path: ["required_fields"],
          message: `Required field must be active: ${field}`,
        });
      }
    }
  });

export type RegistrationSettingsInput = z.infer<
  typeof registrationSettingsSchema
>;
