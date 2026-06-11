import { useParams } from "next/navigation";
import {
  parseAsBoolean,
  parseAsInteger,
  parseAsString,
  parseAsStringEnum,
  useQueryStates,
} from "nuqs";

export function useStudentParams() {
  const pathParams = useParams();

  const [queryStates, setQueryStates] = useQueryStates({
    studentId: parseAsString,
    deleteStudentId: parseAsString,
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
