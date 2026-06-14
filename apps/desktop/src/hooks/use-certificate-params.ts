import { useQueryStates } from "nuqs";
import { createLoader, parseAsString, parseAsStringEnum } from "nuqs/server";

const certificateParamsSchema = {
  type: parseAsStringEnum(["edit", "create"]),
  templateId: parseAsString,
};

export function useCertificateParams() {
  const [params, setParams] = useQueryStates(certificateParamsSchema);

  return {
    ...params,
    setParams,
  };
}

export const loadCertificateParams = createLoader(certificateParamsSchema);
