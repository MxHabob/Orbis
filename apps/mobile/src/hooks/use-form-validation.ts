import { useCallback, useState } from "react";

export type FieldValidator<T extends Record<string, string>, K extends keyof T> = (
  value: T[K],
  values: T,
) => string | null;

export type FormValidators<T extends Record<string, string>> = {
  [K in keyof T]?: FieldValidator<T, K>;
};

export interface UseFormValidationOptions<T extends Record<string, string>> {
  initialValues: T;
  validators: FormValidators<T>;
}

export function useFormValidation<T extends Record<string, string>>({
  initialValues,
  validators,
}: UseFormValidationOptions<T>) {
  const [values, setValues] = useState<T>(initialValues);
  const [errors, setErrors] = useState<Partial<Record<keyof T, string | null>>>({});
  const [touched, setTouched] = useState<Partial<Record<keyof T, boolean>>>({});
  const [formError, setFormError] = useState<string | null>(null);

  const runValidator = useCallback(
    <K extends keyof T>(field: K, value: T[K], nextValues: T): string | null => {
      const validator = validators[field];
      return validator ? validator(value, nextValues) : null;
    },
    [validators],
  );

  const setValue = useCallback(
    <K extends keyof T>(field: K, value: T[K]) => {
      setFormError(null);
      setValues((prev) => {
        const next = { ...prev, [field]: value };
        if (touched[field]) {
          const fieldError = runValidator(field, value, next);
          setErrors((prevErrors) => ({ ...prevErrors, [field]: fieldError }));
        } else if (errors[field]) {
          setErrors((prevErrors) => ({ ...prevErrors, [field]: null }));
        }
        return next;
      });
    },
    [errors, runValidator, touched],
  );

  const markTouched = useCallback(
    <K extends keyof T>(field: K) => {
      setTouched((prev) => ({ ...prev, [field]: true }));
      const fieldError = runValidator(field, values[field], values);
      setErrors((prev) => ({ ...prev, [field]: fieldError }));
    },
    [runValidator, values],
  );

  const validateAll = useCallback((): boolean => {
    const nextErrors: Partial<Record<keyof T, string | null>> = {};
    const nextTouched: Partial<Record<keyof T, boolean>> = {};
    let isValid = true;

    (Object.keys(values) as (keyof T)[]).forEach((field) => {
      nextTouched[field] = true;
      const err = runValidator(field, values[field], values);
      nextErrors[field] = err;
      if (err) {
        isValid = false;
      }
    });

    setTouched(nextTouched);
    setErrors(nextErrors);
    return isValid;
  }, [runValidator, values]);

  const reset = useCallback(
    (nextValues?: T) => {
      setValues(nextValues ?? initialValues);
      setErrors({});
      setTouched({});
      setFormError(null);
    },
    [initialValues],
  );

  return {
    values,
    errors,
    touched,
    formError,
    setFormError,
    setValue,
    markTouched,
    validateAll,
    reset,
  };
}
