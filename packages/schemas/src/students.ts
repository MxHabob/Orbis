import { z } from "zod";

const emptyToNull = (val: unknown) =>
  typeof val === "string" && val.trim() === "" ? null : val;

export const studentRegistrationSchema = z.object({
  name: z.string().trim().min(1, "الاسم مطلوب").max(150),
  name_en: z.preprocess(
    emptyToNull,
    z.string().trim().max(150).nullable().optional(),
  ),
  email: z.preprocess(
    emptyToNull,
    z.string().trim().email("البريد الإلكتروني غير صالح").nullable().optional(),
  ),
  phone: z.preprocess(
    emptyToNull,
    z
      .string()
      .trim()
      .regex(/^\+?[0-9]{7,15}$/, "رقم الهاتف غير صالح")
      .nullable()
      .optional(),
  ),
  gender: z.preprocess(
    emptyToNull,
    z.string().trim().min(1).nullable().optional(),
  ),
  birth_date: z.preprocess(
    emptyToNull,
    z.string().trim().min(1).nullable().optional(),
  ),
  nationality: z.preprocess(
    emptyToNull,
    z.string().trim().nullable().optional(),
  ),
  national_id: z.preprocess(
    emptyToNull,
    z
      .string()
      .trim()
      .regex(/^[0-9A-Za-z]{6,20}$/, "رقم الهوية غير صالح")
      .nullable()
      .optional(),
  ),
  passport_number: z.preprocess(
    emptyToNull,
    z.string().trim().nullable().optional(),
  ),
  address: z.preprocess(emptyToNull, z.string().trim().nullable().optional()),
  class_id: z.string().trim().min(1, "يرجى اختيار الفصل الدراسي"),
  guardian_name: z.preprocess(
    emptyToNull,
    z.string().trim().nullable().optional(),
  ),
  guardian_phone: z.preprocess(
    emptyToNull,
    z
      .string()
      .trim()
      .regex(/^\+?[0-9]{7,15}$/, "رقم هاتف ولي الأمر غير صالح")
      .nullable()
      .optional(),
  ),
  guardian_relationship: z.preprocess(
    emptyToNull,
    z.string().trim().min(1).nullable().optional(),
  ),
});

export type StudentRegistrationInput = z.infer<
  typeof studentRegistrationSchema
>;

export const guardianFormInputSchema = z.object({
  guardianId: z.string().trim().min(1, "يرجى اختيار ولي الأمر من القائمة"),
  fullName: z.string().trim().optional(),
  phone: z.string().trim().optional(),
  email: z.preprocess(
    emptyToNull,
    z.string().trim().nullable().optional(),
  ),
  nationalId: z.preprocess(
    emptyToNull,
    z.string().trim().nullable().optional(),
  ),
  relationship: z.string().min(1, "يرجى تحديد صلة القرابة"),
  relationshipAr: z.preprocess(emptyToNull, z.string().nullable().optional()),
  occupation: z.preprocess(emptyToNull, z.string().nullable().optional()),
  isPrimary: z.boolean().default(false),
  isEmergencyContact: z.boolean().default(true),
  isAuthorizedPickup: z.boolean().default(false),
  isPayer: z.boolean().default(false),
  paymentSharePercent: z.number().finite().min(0).max(100).default(0),
});

export type GuardianFormInput = z.infer<typeof guardianFormInputSchema>;

export const updateStudentProfileSchema = z.object({
  student_id: z.string().min(1),
  first_name: z.string().trim().min(1, "الاسم الأول مطلوب"),
  middle_name: z.preprocess(emptyToNull, z.string().nullable().optional()),
  last_name: z.string().trim().min(1, "اللقب مطلوب"),
  first_name_ar: z.preprocess(emptyToNull, z.string().nullable().optional()),
  middle_name_ar: z.preprocess(emptyToNull, z.string().nullable().optional()),
  last_name_ar: z.preprocess(emptyToNull, z.string().nullable().optional()),
  display_name: z.preprocess(emptyToNull, z.string().nullable().optional()),
  national_id: z.preprocess(emptyToNull, z.string().nullable().optional()),
  passport_number: z.preprocess(emptyToNull, z.string().nullable().optional()),
  gender: z.preprocess(emptyToNull, z.string().nullable().optional()),
  birth_date: z.preprocess(emptyToNull, z.string().nullable().optional()),
  place_of_birth: z.preprocess(emptyToNull, z.string().nullable().optional()),
  nationality: z.preprocess(emptyToNull, z.string().nullable().optional()),
  blood_type: z.preprocess(emptyToNull, z.string().nullable().optional()),
  medical_notes: z.preprocess(emptyToNull, z.string().nullable().optional()),
  special_needs_notes: z.preprocess(emptyToNull, z.string().nullable().optional()),
  phone: z.preprocess(emptyToNull, z.string().nullable().optional()),
  email: z.preprocess(emptyToNull, z.string().nullable().optional()),
  address: z.preprocess(emptyToNull, z.string().nullable().optional()),
  avatar_url: z.preprocess(emptyToNull, z.string().nullable().optional()),
});

export type UpdateStudentProfileSchemaInput = z.infer<typeof updateStudentProfileSchema>;
