/**
 * Typed IPC query/mutation option builders — replaces tRPC proxy shim.
 * Migrate components to hooks/api/* over time.
 */
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { getAccessToken, getOrgId, getSessionUser, getUserId } from "@/lib/session";
import type { JsonRecord } from "@orbis/desktop-client/types";

const customFetchers: Record<string, (params: unknown) => Promise<unknown>> = {
  "user.me": async () => {
    const token = getAccessToken();
    if (!token) return getSessionUser();
    try {
      return await commands.auth.validateToken(token);
    } catch {
      return getSessionUser();
    }
  },
  "user.update": async (params) => {
    const p = params as JsonRecord;
    const userId = String(p.id ?? getUserId());
    return commands.users.updateUser(userId, p);
  },
  "classes.getAll": async (params) =>
    commands.classes.getAll(getOrgId(), params as JsonRecord | undefined),
  "classes.getById": async (params) => {
    const classes = await commands.classes.getAll(getOrgId());
    const id = (params as { id?: string })?.id ?? params;
    return classes.find((c) => String(c.id) === String(id)) ?? null;
  },
  "subjects.getById": async (params) => {
    const subjects = await commands.subjects.getAll(getOrgId());
    const id = (params as { id?: string })?.id ?? params;
    return subjects.find((s) => String(s.id) === String(id)) ?? null;
  },
  "students.getById": async (params) => {
    const id = (params as { id?: string })?.id ?? params;
    return commands.enrollments.getById(String(id));
  },
  "students.getStudentByUserId": async (params) => {
    const userId = (params as { userId?: string })?.userId ?? params;
    const enrollments = await commands.enrollments.getAll(getOrgId(), {
      student_id: String(userId),
    });
    return enrollments?.[0] ?? null;
  },
  "students.getByClassId": async (params) => {
    const classId =
      (params as { id?: string; classId?: string })?.id ??
      (params as { classId?: string })?.classId ??
      params;
    return commands.enrollments.getAll(getOrgId(), {
      class_id: String(classId),
    });
  },
  "students.getByUserIds": async (params) => {
    const userIds = (params as { userIds?: string[] })?.userIds ?? [];
    const enrollments = await Promise.all(
      userIds.map(async (id) => {
        const list = await commands.enrollments.getAll(getOrgId(), {
          student_id: id,
        });
        return list?.[0];
      }),
    );
    return enrollments.filter(Boolean);
  },
  "academic.getCurrentYear": async () =>
    commands.academics.getCurrentYear(getOrgId()),
  "academic.getYears": async () => commands.academics.getYears(getOrgId()),
  "academic.getTerms": async (params) => {
    const yearId = (params as { yearId?: string | number })?.yearId ?? params;
    return commands.academics.getTerms(getOrgId(), String(yearId));
  },
  "academic.getYearById": async (params) => {
    const id = (params as { id?: string | number })?.id ?? params;
    return commands.academics.getCycle(String(id));
  },
  "academic.createYear": async (params) =>
    commands.academics.createCycle(getOrgId(), params as JsonRecord),
  "academic.updateYear": async (params) => {
    const p = params as JsonRecord;
    return commands.academics.updateCycle(String(p.id), p);
  },
  "academic.deleteYear": async (params) => {
    const id = (params as { id?: string | number })?.id ?? params;
    return commands.academics.lockCycle(String(id));
  },
  "academic.createTerm": async (params) =>
    commands.academics.createCycle(getOrgId(), {
      ...(params as JsonRecord),
      cycle_type: "term",
    }),
  "academic.updateTerm": async (params) => {
    const p = params as JsonRecord;
    return commands.academics.updateCycle(String(p.id), p);
  },
  "academic.deleteTerm": async (params) => {
    const id = (params as { id?: string | number })?.id ?? params;
    return commands.academics.lockCycle(String(id));
  },
  "documents.get": async (params) =>
    commands.documents.getAll(params as JsonRecord | undefined),
  "grades.getClassStatistics": async (params) => {
    const p = params as JsonRecord;
    return commands.grades.getAnalytics(getOrgId(), p);
  },
  "reports.getSchoolMetrics": async () =>
    commands.dashboard.getStats(getOrgId()),
  "grades.getTopStudents": async (params) => {
    const analytics = await commands.grades.getAnalytics(getOrgId(), {
      class_id: String((params as { classId?: string })?.classId),
    });
    return (analytics as JsonRecord)?.topStudents ?? [];
  },
  "documentTemplates.getById": async (params) => {
    const id = (params as { id?: string })?.id ?? params;
    return commands.documentTemplates.getById(String(id));
  },
  "documentTemplates.list": async () => commands.documentTemplates.getAll(),
  "documentTemplates.upsert": async (params) => {
    const p = params as JsonRecord;
    const id = p?.id;
    if (id) return commands.documentTemplates.update(String(id), p);
    return commands.documentTemplates.create(p);
  },
  "documentTemplates.delete": async (params) => {
    const id = (params as { id?: string })?.id ?? params;
    return commands.documentTemplates.delete(String(id));
  },
  "notifications.list": async (params) => {
    const userId = getUserId();
    return {
      data: await commands.notifications.getAll(userId, params as JsonRecord),
      meta: { cursor: null, hasNextPage: false, hasPreviousPage: false },
    };
  },
  "notifications.updateStatus": async (params) => {
    const p = params as { activityId?: string | number };
    if (p.activityId) {
      await commands.notifications.markRead(String(p.activityId));
    }
    return { success: true };
  },
  "notifications.updateAllStatus": async () => ({ success: true }),
};

async function runQuery(domain: string, method: string, params?: unknown) {
  const fullPath = `${domain}.${method}`;
  if (customFetchers[fullPath]) {
    return customFetchers[fullPath](params);
  }

  const orgId = getOrgId();
  const apiDomain = commands[domain as keyof typeof commands] as
    | Record<string, (...args: unknown[]) => Promise<unknown>>
    | undefined;

  if (apiDomain?.[method]) {
    const noOrgIdDomains = [
      "auth",
      "assessments",
      "documents",
      "documentTemplates",
      "widgets",
      "notifications",
      "window",
    ];
    const noOrgIdMethods = [
      "update",
      "delete",
      "getById",
      "getCycle",
      "updateCycle",
      "lockCycle",
      "withdraw",
      "transfer",
      "getReportCard",
      "getReport",
      "submitAI",
      "updateRecord",
      "deleteRecord",
    ];

    if (noOrgIdDomains.includes(domain) || noOrgIdMethods.includes(method)) {
      return apiDomain[method](params);
    }
    return apiDomain[method](orgId, params);
  }

  const { invoke } = await import("@tauri-apps/api/core");
  const commandName = `get_${method.replace(/[A-Z]/g, (l) => `_${l.toLowerCase()}`)}`;
  return invoke(commandName, { orgId, params });
}

async function runMutation(domain: string, method: string, variables: unknown) {
  const fullPath = `${domain}.${method}`;
  if (customFetchers[fullPath]) {
    return customFetchers[fullPath](variables);
  }

  const orgId = getOrgId();
  const userId = getUserId();
  const apiDomain = commands[domain as keyof typeof commands] as
    | Record<string, (...args: unknown[]) => Promise<unknown>>
    | undefined;

  if (apiDomain?.[method]) {
    const noOrgIdDomains = [
      "auth",
      "assessments",
      "documents",
      "documentTemplates",
      "widgets",
      "notifications",
    ];
    const noOrgIdMethods = [
      "update",
      "delete",
      "getById",
      "updateUser",
      "getCycle",
      "updateCycle",
      "lockCycle",
      "withdraw",
      "transfer",
      "getReportCard",
      "getReport",
      "submitAI",
      "updateRecord",
      "deleteRecord",
    ];

    if (noOrgIdDomains.includes(domain) || noOrgIdMethods.includes(method)) {
      return apiDomain[method](variables);
    }

    const gradedBy = ["submit", "submitBulk"];
    const recordedBy = ["record", "recordBulk"];
    const createdBy = ["createUser", "recordPayment"];

    if (
      gradedBy.includes(method) ||
      recordedBy.includes(method) ||
      createdBy.includes(method)
    ) {
      return apiDomain[method](orgId, userId, variables);
    }
    return apiDomain[method](orgId, variables);
  }

  const { invoke } = await import("@tauri-apps/api/core");
  const commandName = method.replace(/[A-Z]/g, (l) => `_${l.toLowerCase()}`);
  return invoke(commandName, { orgId, request: variables });
}

function resolveQueryKey(domain: string, method: string, params: unknown) {
  const domainKeys = (queryKeys as Record<string, Record<string, unknown>>)[
    domain
  ];
  if (domainKeys && typeof domainKeys[method] === "function") {
    return (domainKeys[method] as (p?: unknown) => readonly unknown[])(params);
  }
  return [domain, method, params] as const;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const createIPCProxy = (path: string[] = []): any => {
  return new Proxy(() => {}, {
    get(_target, prop: string) {
      if (prop === "queryOptions") {
        return (params: unknown) => {
          const domain = path[0];
          const method = path[1];
          return {
            queryKey: resolveQueryKey(domain, method, params),
            queryFn: () => runQuery(domain, method, params),
          };
        };
      }

      if (prop === "infiniteQueryOptions") {
        return (params: unknown) => {
          const domain = path[0];
          const method = path[1];
          const baseKey = resolveQueryKey(domain, method, params);
          return {
            queryKey: [...baseKey, "infinite"] as const,
            queryFn: () => runQuery(domain, method, params),
            initialPageParam: null as string | null,
            getNextPageParam: (lastPage: unknown) => {
              const page = lastPage as { meta?: { cursor?: string } };
              return page?.meta?.cursor ?? undefined;
            },
          };
        };
      }

      if (prop === "mutationOptions") {
        return (options?: Record<string, unknown>) => {
          const domain = path[0];
          const method = path[1];
          return {
            mutationFn: (variables: unknown) =>
              runMutation(domain, method, variables),
            ...options,
          };
        };
      }

      if (prop === "queryKey" || prop === "infiniteQueryKey") {
        return (params?: unknown) => [...path, params];
      }

      if (prop === "queryFilter") {
        return () => ({ queryKey: path });
      }

      return createIPCProxy([...path, prop]);
    },

    apply(_target, _thisArg, argArray) {
      const domain = path[0];
      const method = path[1];
      return runQuery(domain, method, argArray[0]);
    },
  });
};

export const ipc = createIPCProxy();

/** @deprecated Use hooks/api — bridge during migration */
export function useIPC() {
  return ipc;
}

/** @deprecated Alias for migration from useTRPC */
export const useTRPC = useIPC;

export function getApiUrl(): string {
  return "";
}

export type RouterOutputs = Record<string, Record<string, unknown>>;
