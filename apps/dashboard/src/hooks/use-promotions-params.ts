import { parseAsInteger, useQueryStates } from "nuqs";

export function usePromotionsParams() {
  const [queryStates, setQueryStates] = useQueryStates({
    promotionClassId: parseAsInteger,
    sourceYearId: parseAsInteger,
    targetYearId: parseAsInteger,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}
