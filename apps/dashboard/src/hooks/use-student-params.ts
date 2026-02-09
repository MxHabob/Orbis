import { parseAsBoolean, parseAsString, parseAsStringEnum, parseAsInteger, useQueryStates } from "nuqs";
import { useParams } from "next/navigation";

export function useStudentParams() {
  const pathParams = useParams();
  
  const [queryStates, setQueryStates] = useQueryStates({
    studentId: parseAsString,
    createStudent: parseAsBoolean,
    classId: parseAsString,
    name: parseAsString,
    q: parseAsString,
    details: parseAsBoolean,
    certificate: parseAsBoolean,
    certificateType: parseAsStringEnum(["monthly", "half-yearly", "final"]),
    templateId: parseAsString,
    month: parseAsInteger,
  });

  return {
    ...queryStates,
    classId: queryStates.classId || (pathParams?.Id as string),
    setParams: setQueryStates,
  };
}