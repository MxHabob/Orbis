import { useQueryStates, parseAsString } from "nuqs";
import { createLoader } from "nuqs/server";

const certificateFilterParamsSchema = {
  q: parseAsString,
};

export function useCertificateFilterParams() {
  const [filter, setFilter] = useQueryStates(certificateFilterParamsSchema);

  return {
    filter,
    setFilter,
    hasFilters: Object.values(filter).some((value) => value !== null),
  };
}

export const loadCertificateFilterParams = createLoader(certificateFilterParamsSchema);
