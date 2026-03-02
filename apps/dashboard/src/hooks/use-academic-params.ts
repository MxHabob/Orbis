import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

export function useAcademicParams() {
  const [queryStates, setQueryStates] = useQueryStates({
    editYearId: parseAsString,
    editTermId: parseAsString,
    parentYearId: parseAsString,
    createYear: parseAsBoolean,
    createTerm: parseAsBoolean,
    q: parseAsString,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}
