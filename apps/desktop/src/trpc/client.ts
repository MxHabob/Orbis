import { api } from "@/lib/tauri-api";
import { queryKeys } from "@/lib/query-keys";

// Helpers to extract active user/org details
function getOrgId() {
  if (typeof window === "undefined") return "";
  const savedUser = localStorage.getItem("session_user");
  if (savedUser) {
    try {
      const u = JSON.parse(savedUser);
      return u.orgId || u.org_id || "";
    } catch (_) {}
  }
  return "";
}

function getUserId() {
  if (typeof window === "undefined") return "";
  const savedUser = localStorage.getItem("session_user");
  if (savedUser) {
    try {
      const u = JSON.parse(savedUser);
      return u.id || u.userId || "";
    } catch (_) {}
  }
  return "";
}

// Custom client-side adapters for complex tRPC routes mapping to simple Tauri IPC commands
const customFetchers: Record<string, (params: any) => Promise<any>> = {
  "classes.getById": async (params: any) => {
    const classes = await api.classes.getAll("");
    const id = params?.id || params;
    return classes.find((c: any) => String(c.id) === String(id)) || null;
  },
  "subjects.getById": async (params: any) => {
    const subjects = await api.subjects.getAll("");
    const id = params?.id || params;
    return subjects.find((s: any) => String(s.id) === String(id)) || null;
  },
  "students.getById": async (params: any) => {
    const id = params?.id || params;
    return api.enrollments.getById(String(id));
  },
  "students.getStudentByUserId": async (params: any) => {
    const userId = params?.userId || params;
    const enrollments = await api.enrollments.getAll("", { student_id: String(userId) });
    return enrollments?.[0] || null;
  },
  "students.getByClassId": async (params: any) => {
    const classId = params?.id || params?.classId || params;
    return api.enrollments.getAll("", { class_id: String(classId) });
  },
  "students.getByUserIds": async (params: any) => {
    const userIds = params?.userIds || [];
    const enrollments = await Promise.all(
      userIds.map(async (id: string) => {
        const list = await api.enrollments.getAll("", { student_id: id });
        return list?.[0];
      })
    );
    return enrollments.filter(Boolean);
  },
  "academic.getCurrentYear": async () => {
    return api.academics.getCurrentYear("");
  },
  "academic.getYears": async () => {
    return api.academics.getYears("");
  },
  "academic.getTerms": async (params: any) => {
    const yearId = params?.yearId || params;
    return api.academics.getTerms("", String(yearId));
  },
  "reports.getSchoolMetrics": async () => {
    return api.dashboard.getStats("");
  },
  "grades.getTopStudents": async (params: any) => {
    const analytics = await api.grades.getAnalytics("", { class_id: String(params?.classId) });
    return (analytics as any)?.topStudents || [];
  },
  "certificateTemplates.getById": async (params: any) => {
    const id = params?.id || params;
    return api.certificates.getById(String(id));
  },
  "certificateTemplates.list": async () => {
    return api.certificates.getAll();
  },
  "certificateTemplates.upsert": async (params: any) => {
    const id = params?.id;
    if (id) {
      return api.certificates.update(String(id), params);
    } else {
      return api.certificates.create(params);
    }
  },
  "certificateTemplates.delete": async (params: any) => {
    const id = params?.id || params;
    return api.certificates.delete(String(id));
  }
};

// Recursive proxy to dynamically build tRPC call interfaces
const createTRPCProxy = (path: string[] = []): any => {
  return new Proxy(() => {}, {
    get(_target, prop: string) {
      if (prop === "queryOptions") {
        return (params: any) => {
          const domain = path[0];
          const method = path[1];
          const fullPath = `${domain}.${method}`;
          
          let queryKey: any = [...path, params];
          if (queryKeys[domain] && (queryKeys[domain] as any)[method]) {
            queryKey = (queryKeys[domain] as any)[method](params);
          }

          const queryFn = async () => {
            // Check custom fetchers first
            if (customFetchers[fullPath]) {
              return customFetchers[fullPath](params);
            }

            const orgId = getOrgId();
            const apiDomain = api[domain as keyof typeof api];
            if (apiDomain && (apiDomain as any)[method]) {
              const apiMethod = (apiDomain as any)[method];
              
              const noOrgIdDomains = ["auth", "assessments", "documents", "certificates", "widgets", "notifications"];
              const noOrgIdMethods = ["update", "delete", "getById", "getCycle", "updateCycle", "lockCycle", "withdraw", "transfer", "getReportCard", "getReport", "submitAI", "updateRecord", "deleteRecord"];
              
              if (noOrgIdDomains.includes(domain) || noOrgIdMethods.includes(method)) {
                return apiMethod(params);
              } else {
                return apiMethod(orgId, params);
              }
            }
            
            // Fallback command invoke
            const commandName = `get_${method.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`)}`;
            const { invoke } = await import("@tauri-apps/api/core");
            return invoke(commandName, { orgId, params });
          };

          return { queryKey, queryFn };
        };
      }

      if (prop === "mutationOptions") {
        return (options?: any) => {
          const domain = path[0];
          const method = path[1];
          const fullPath = `${domain}.${method}`;

          const mutationFn = async (variables: any) => {
            if (customFetchers[fullPath]) {
              return customFetchers[fullPath](variables);
            }

            const orgId = getOrgId();
            const userId = getUserId();
            const apiDomain = api[domain as keyof typeof api];
            if (apiDomain && (apiDomain as any)[method]) {
              const apiMethod = (apiDomain as any)[method];
              
              const noOrgIdDomains = ["auth", "assessments", "documents", "certificates", "widgets", "notifications"];
              const noOrgIdMethods = ["update", "delete", "getById", "updateUser", "getCycle", "updateCycle", "lockCycle", "withdraw", "transfer", "getReportCard", "getReport", "submitAI", "updateRecord", "deleteRecord"];

              if (noOrgIdDomains.includes(domain) || noOrgIdMethods.includes(method)) {
                return apiMethod(variables);
              } else {
                const gradedByMethods = ["submit", "submitBulk"];
                const recordedByMethods = ["record", "recordBulk"];
                const createdByMethods = ["createUser", "recordPayment"];

                if (gradedByMethods.includes(method) || recordedByMethods.includes(method) || createdByMethods.includes(method)) {
                  return apiMethod(orgId, userId, variables);
                }
                return apiMethod(orgId, variables);
              }
            }

            const commandName = `${method.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`)}`;
            const { invoke } = await import("@tauri-apps/api/core");
            return invoke(commandName, { orgId, request: variables });
          };

          return { mutationFn, ...options };
        };
      }

      if (prop === "queryKey" || prop === "infiniteQueryKey") {
        return (params: any) => {
          return [...path, params];
        };
      }

      if (prop === "queryFilter") {
        return () => {
          return { queryKey: path };
        };
      }

      return createTRPCProxy([...path, prop]);
    },
    
    apply(_target, _thisArg, argArray) {
      const domain = path[0];
      const method = path[1];
      const params = argArray[0];

      return (async () => {
        const fullPath = `${domain}.${method}`;
        if (customFetchers[fullPath]) {
          return customFetchers[fullPath](params);
        }

        const orgId = getOrgId();
        const apiDomain = api[domain as keyof typeof api];
        if (apiDomain && (apiDomain as any)[method]) {
          return (apiDomain as any)[method](orgId, params);
        }
        const { invoke } = await import("@tauri-apps/api/core");
        return invoke(method, params);
      })();
    }
  });
};

export const trpc = createTRPCProxy();
export const useTRPC = () => trpc;

export function getApiUrl(): string {
  return "";
}
