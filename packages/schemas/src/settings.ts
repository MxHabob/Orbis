import { z } from "zod";

export const generalSettingsInputSchema = z.object({
  name: z.string().trim().min(1).max(150),
  currency: z
    .string()
    .trim()
    .regex(/^[A-Za-z]{3}$/),
  timezone: z.string().trim().min(1),
  locale: z.string().trim().min(1),
  dateFormat: z.string().trim().min(1),
  formatLocale: z.string().trim().min(1),
  timeFormat: z.enum(["locale", "12-hour", "24-hour"]),
});

type AppSettingDefinition = {
  id: string;
  type: string;
  required?: boolean;
  options?: Array<{ value: string }>;
  validation?: { min?: number; max?: number };
};

export function createAppSettingsSchema(settings: AppSettingDefinition[]) {
  const shape: Record<string, z.ZodTypeAny> = {};

  for (const setting of settings) {
    let field: z.ZodTypeAny;
    if (setting.type === "switch") field = z.boolean();
    else if (setting.type === "number") field = z.coerce.number().finite();
    else if (setting.type === "url")
      field = z.string().url().or(z.literal(""));
    else if (setting.type === "select") {
      const options = (setting.options ?? []).map((option) => option.value);
      field =
        options.length > 0
          ? z.enum(options as [string, ...string[]])
          : z.string();
    } else field = z.string();

    if (setting.type !== "switch" && !setting.required)
      field = field.optional();
    if (setting.required && setting.type !== "switch") {
      field = field.refine((value) => String(value ?? "").trim().length > 0);
    }
    if (setting.type === "number") {
      field = field
        .refine(
          (value) =>
            setting.validation?.min === undefined ||
            Number(value) >= setting.validation.min,
        )
        .refine(
          (value) =>
            setting.validation?.max === undefined ||
            Number(value) <= setting.validation.max,
        );
    }
    shape[setting.id] = field;
  }

  return z.object(shape);
}

export type GeneralSettingsInput = z.infer<typeof generalSettingsInputSchema>;
