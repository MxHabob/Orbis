import { OrbisError } from "$lib/api";
import { createForm } from "@tanstack/svelte-form";
import { z } from "zod";

export type FormErrors = Record<string, string[]>;

export type AsyncFormValidator<Values> = (
  values: Values,
  signal: AbortSignal,
) => Promise<FormErrors | void>;

export interface FormControllerOptions<Values, Payload = Values> {
  initialValues: Values;
  schema: z.ZodType<Payload>;
  normalize?: (values: Values) => unknown;
  validateAsync?: AsyncFormValidator<Values>;
}

function pathKey(path: PropertyKey[]): string {
  return path.map(String).join(".");
}

function zodErrors(error: z.ZodError): FormErrors {
  const errors: FormErrors = {};
  for (const issue of error.issues) {
    const key = pathKey(issue.path);
    if (!errors[key]) errors[key] = [];
    errors[key].push(issue.message);
  }
  return errors;
}

function mergeErrors(...sources: Array<FormErrors | undefined>): FormErrors {
  const result: FormErrors = {};
  for (const source of sources) {
    if (!source) continue;
    for (const [key, messages] of Object.entries(source)) {
      result[key] = [...(result[key] ?? []), ...messages];
    }
  }
  return result;
}

export function mapCommandError(error: unknown): {
  formError?: string;
  fieldErrors: FormErrors;
} {
  const normalized = OrbisError.fromUnknown(error);
  const fieldErrors: FormErrors = {};
  const details = normalized.details;

  if (details && typeof details === "object") {
    const record = details as Record<string, unknown>;
    if (typeof record.field === "string") {
      fieldErrors[record.field] = [normalized.message];
      return { fieldErrors };
    }

    if (Array.isArray(record.issues)) {
      for (const issue of record.issues) {
        if (!issue || typeof issue !== "object") continue;
        const value = issue as Record<string, unknown>;
        const path = Array.isArray(value.path) ? value.path.map(String).join(".") : "";
        if (path && typeof value.message === "string") {
          fieldErrors[path] = [...(fieldErrors[path] ?? []), value.message];
        }
      }
      if (Object.keys(fieldErrors).length > 0) return { fieldErrors };
    }
  }

  return { formError: normalized.message, fieldErrors };
}

export class FormController<Values, Payload = Values> {
  values = $state<Values>({} as Values);
  errors = $state<FormErrors>({});
  touched = $state<Record<string, boolean>>({});
  isSubmitting = $state(false);
  formError = $state<string | undefined>();

  private readonly options: FormControllerOptions<Values, Payload>;
  private asyncAbort?: AbortController;
  private readonly tanstackForm;

  constructor(options: FormControllerOptions<Values, Payload>) {
    this.options = options;
    this.values = structuredClone(options.initialValues);
    this.tanstackForm = createForm(() => ({
      defaultValues: structuredClone(options.initialValues),
      validators: {
        onChange: ({ value }: { value: Values }) => {
          const candidate = options.normalize ? options.normalize(value) : value;
          const result = options.schema.safeParse(candidate);
          return result.success ? undefined : result.error.issues[0]?.message;
        },
      },
    }));
  }

  private syncToTanStack(): void {
    for (const [field, value] of Object.entries(this.values as Record<string, unknown>)) {
      this.tanstackForm.setFieldValue(field as never, value as never);
    }
  }

  setField<Key extends keyof Values>(field: Key, value: Values[Key]): void {
    this.values = { ...this.values, [field]: value };
    this.tanstackForm.setFieldValue(String(field) as never, value as never);
    this.formError = undefined;
  }

  markTouched(field: string): void {
    this.touched = { ...this.touched, [field]: true };
    this.validate([field]);
  }

  validate(fields?: string[]): boolean {
    this.syncToTanStack();
    const candidate = this.options.normalize
      ? this.options.normalize(this.values)
      : this.values;
    const result = this.options.schema.safeParse(candidate);
    const nextErrors = result.success ? {} : zodErrors(result.error);
    this.errors = fields
      ? Object.fromEntries(
          Object.entries(nextErrors).filter(([key]) =>
            fields.some((field) => key === field || key.startsWith(`${field}.`)),
          ),
        )
      : nextErrors;
    return Object.keys(this.errors).length === 0;
  }

  async validateAsync(): Promise<boolean> {
    if (!this.options.validateAsync) return true;
    this.asyncAbort?.abort();
    const controller = new AbortController();
    this.asyncAbort = controller;
    const asyncErrors = await this.options.validateAsync(this.values, controller.signal);
    if (controller.signal.aborted) return false;
    this.errors = mergeErrors(this.errors, asyncErrors || undefined);
    return Object.keys(this.errors).length === 0;
  }

  async submit(submit: (payload: Payload) => Promise<unknown>): Promise<boolean> {
    this.formError = undefined;
    this.touched = { ...this.touched, __form: true };
    if (!this.validate()) return false;
    if (!(await this.validateAsync())) return false;

    const candidate = this.options.normalize
      ? this.options.normalize(this.values)
      : this.values;
    const parsed = this.options.schema.safeParse(candidate);
    if (!parsed.success) {
      this.errors = zodErrors(parsed.error);
      return false;
    }

    this.isSubmitting = true;
    try {
      await submit(parsed.data);
      return true;
    } catch (error) {
      const mapped = mapCommandError(error);
      this.errors = mapped.fieldErrors;
      this.formError = mapped.formError;
      return false;
    } finally {
      this.isSubmitting = false;
    }
  }

  reset(values = this.options.initialValues): void {
    this.asyncAbort?.abort();
    this.values = structuredClone(values);
    this.errors = {};
    this.touched = {};
    this.formError = undefined;
    this.isSubmitting = false;
    this.tanstackForm.reset(structuredClone(values));
  }
}
