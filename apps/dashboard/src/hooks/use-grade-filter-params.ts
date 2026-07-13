import { parseAsInteger, parseAsString, useQueryStates } from "nuqs";
import { createLoader } from "nuqs/server";

export const gradeFilterParamsSchema = {
  q: parseAsString,
  selectedClassId: parseAsInteger,
  selectedSubjectId: parseAsInteger,
  termId: parseAsInteger,
  periodId: parseAsInteger,
  month: parseAsInteger,
};

export function useGradeFilterParams() {
  const [filter, setFilter] = useQueryStates(gradeFilterParamsSchema, {
    clearOnDefault: true,
  });

  return {
    filter,
    setFilter,
    hasFilters: Object.values(filter).some((value) => value !== null),
  };
}

export const loadGradeFilterParams = createLoader(gradeFilterParamsSchema);
