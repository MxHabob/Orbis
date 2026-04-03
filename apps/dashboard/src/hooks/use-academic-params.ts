import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

export function useAcademicParams() {
  const [queryStates, setQueryStates] = useQueryStates({
    editYearId: parseAsString,
    editTermId: parseAsString,
    parentYearId: parseAsString,
    deleteYearId: parseAsString,
    deleteTermId: parseAsString,
    createYear: parseAsBoolean,
    createTerm: parseAsBoolean,
    q: parseAsString,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}
