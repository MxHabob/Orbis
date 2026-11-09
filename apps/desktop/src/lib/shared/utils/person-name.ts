export interface PersonNameParts {
  firstName: string;
  middleName?: string;
  lastName: string;
}

export function splitPersonName(
  fullName: string,
  fallbackName = "طالب",
): PersonNameParts {
  const parts = fullName.trim().split(/\s+/).filter(Boolean);
  const firstName = parts[0] || fallbackName;

  return {
    firstName,
    middleName: parts.length > 2 ? parts.slice(1, -1).join(" ") : undefined,
    lastName: parts.length > 1 ? parts[parts.length - 1]! : "جديد",
  };
}
