export interface RegistrationFieldPolicy {
  active: boolean;
  required: boolean;
  preventDuplicate: boolean;
}

const defaultPolicies: Record<string, RegistrationFieldPolicy> = {
  national_id: { active: true, required: true, preventDuplicate: true },
  phone: { active: true, required: true, preventDuplicate: false },
  email: { active: true, required: false, preventDuplicate: false },
  birth_date: { active: true, required: true, preventDuplicate: false },
  sex: { active: true, required: true, preventDuplicate: false },
  nationality: { active: true, required: true, preventDuplicate: false },
  address: { active: true, required: false, preventDuplicate: false },
  place_of_birth: { active: true, required: false, preventDuplicate: false },
  blood_type: { active: true, required: false, preventDuplicate: false },
  emergency_contact: { active: true, required: true, preventDuplicate: false },
  medical_notes: { active: true, required: false, preventDuplicate: false },
  previous_school: { active: true, required: false, preventDuplicate: false },
  passport_number: { active: true, required: false, preventDuplicate: false },
  special_needs_notes: {
    active: true,
    required: false,
    preventDuplicate: false,
  },
};

const aliases: Record<string, string> = {
  gender: "sex",
};

export function normalizeRegistrationFieldKey(key: string): string {
  const normalized = key.replace(
    /[A-Z]/g,
    (letter) => `_${letter.toLowerCase()}`,
  );
  return aliases[normalized] ?? normalized;
}

export function parseRegistrationFieldPolicies(
  settings: unknown,
): Record<string, RegistrationFieldPolicy> {
  let settingsObject: Record<string, any> = {};
  try {
    settingsObject =
      typeof settings === "string"
        ? JSON.parse(settings)
        : (settings as Record<string, any>) || {};
  } catch {
    settingsObject = {};
  }

  const registration =
    settingsObject.admission ||
    settingsObject.registration ||
    settingsObject.registration_config ||
    {};
  const fields =
    registration.fields ||
    registration.form_fields_list ||
    registration.formFieldsList;
  const policies = Object.fromEntries(
    Object.entries(defaultPolicies).map(([key, policy]) => [
      key,
      { ...policy },
    ]),
  );

  if (!Array.isArray(fields)) return policies;

  for (const field of fields) {
    if (!field || typeof field.key !== "string") continue;
    const key = normalizeRegistrationFieldKey(field.key);
    policies[key] = {
      active:
        field.is_active !== undefined
          ? Boolean(field.is_active)
          : (field.isActive ?? true),
      required:
        field.is_active === false || field.isActive === false
          ? false
          : field.is_required !== undefined
            ? Boolean(field.is_required)
            : (field.isRequired ?? false),
      preventDuplicate:
        field.is_active === false || field.isActive === false
          ? false
          : field.prevent_duplicate !== undefined
            ? Boolean(field.prevent_duplicate)
            : (field.preventDuplicate ?? false),
    };
  }

  return policies;
}
