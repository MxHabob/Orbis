import { parseAsBoolean, useQueryStates } from "nuqs";

export function useReportExportParams() {
  const [params, setParams] = useQueryStates({
    exportReport: parseAsBoolean,
  });

  return {
    ...params,
    setParams,
  };
}
