import { l } from "$lib/core/i18n";
import type { OrgUnitDto } from "$lib/api";

export function getOrgUnitLocationPath(
  unitId: string,
  unitsById: ReadonlyMap<string, OrgUnitDto>,
): string | null {
  const path: OrgUnitDto[] = [];
  const visited = new Set<string>();
  let current = unitsById.get(unitId);

  while (current && !visited.has(String(current.id))) {
    visited.add(String(current.id));
    path.unshift(current);
    current = current.parent_id
      ? unitsById.get(String(current.parent_id))
      : undefined;
  }

  return path
    .map((unit) => l(unit.name_ar, unit.name) || unit.name)
    .join(" / ") || null;
}
