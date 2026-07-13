import { parseAsInteger, useQueryStates } from "nuqs";

export function usePromotionsParams() {
  const [queryStates, setQueryStates] = useQueryStates({
    promotionClassId: parseAsInteger,
    sourcePeriodId: parseAsInteger,
    targetPeriodId: parseAsInteger,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}
