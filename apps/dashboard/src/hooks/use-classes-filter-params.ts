import { useQueryStates } from "nuqs";
import {
  createLoader,
  parseAsArrayOf,
  parseAsString,
} from "nuqs/server";

const classesFilterParamsSchema = {
  q: parseAsString,
  parentId: parseAsString,
  academicYearId: parseAsString,
  ids: parseAsArrayOf(parseAsString),
};

export function useClassesFilterParams() {
  const [filter, setFilter] = useQueryStates(classesFilterParamsSchema);

  return {
    filter,
    setFilter,
    hasFilters: Object.values(filter).some((value) => value !== null),
  };
}

export const loadClassesFilterParams = createLoader(classesFilterParamsSchema);
