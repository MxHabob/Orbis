/**
 * Employees domain queries & mutations — Employee Affairs & Identity Provisioning.
 * Wraps OrbisApiClient with TanStack Query and queryKeys factory.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type EmployeeFilterInput,
  type PaginatedEmployeesDto,
  type RegisterEmployeeRequest,
  type UpdateEmployeeRequest,
  type ProvisionEmployeeAccountRequest,
} from "$lib/api";

const EMPLOYEE_SELECTOR_PAGE_SIZE = 100;

/** Fetch all pages of employees matching the given filter. */
export async function listAllEmployees(
  filter?: Omit<EmployeeFilterInput, "page" | "page_size">,
): Promise<PaginatedEmployeesDto> {
  const firstPage = await api.listEmployees({
    ...filter,
    page: 1,
    page_size: EMPLOYEE_SELECTOR_PAGE_SIZE,
  });

  if ((firstPage.total_pages ?? 1) <= 1) {
    return firstPage;
  }

  const pageSize = firstPage.page_size || EMPLOYEE_SELECTOR_PAGE_SIZE;
  const remainingPages = await Promise.all(
    Array.from({ length: Math.max(0, firstPage.total_pages - 1) }, (_, index) =>
      api.listEmployees({
        ...filter,
        page: index + 2,
        page_size: pageSize,
      }),
    ),
  );

  const items = [
    ...firstPage.items,
    ...remainingPages.flatMap((page) => page.items),
  ];

  return {
    ...firstPage,
    items,
    page: 1,
    page_size: items.length,
    total_pages: 1,
  };
}

/** Paginated and filtered employees directory query. */
export function createEmployeesQuery(
  orgId: () => string,
  filter?: () => EmployeeFilterInput | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.employee.list(
      orgId(),
      filter?.() as Record<string, unknown>,
    ),
    queryFn: () => api.listEmployees(filter?.()),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

/** Query all employees matching a filter (for searchable selector dropdowns). */
export function createAllEmployeesQuery(
  orgId: () => string,
  filter?: () => Omit<EmployeeFilterInput, "page" | "page_size"> | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.employee.list(orgId(), {
      ...(filter?.() as Record<string, unknown> | undefined),
      allPages: true,
    }),
    queryFn: () => listAllEmployees(filter?.()),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

/** Single employee profile dossier query (EmployeeDetail). */
export function createEmployeeQuery(employeeId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.employee.detail(employeeId()),
    queryFn: () => api.getEmployee(employeeId()),
    enabled: !!employeeId(),
    staleTime: 60 * 1000,
  }));
}

/** Employee counts by status (active, on_leave, suspended, terminated). */
export function createEmployeeStatsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.employee.stats(orgId()),
    queryFn: () => api.getEmployeeStats(),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Register new employee mutation (with optional account provisioning). */
export function createRegisterEmployeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: RegisterEmployeeRequest) => api.registerEmployee(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.auth.all });
    },
  }));
}

/** Update employee details mutation. */
export function createUpdateEmployeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateEmployeeRequest }) =>
      api.updateEmployee(id, req),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.employee.detail(variables.id),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
    },
  }));
}

/** Update employee status mutation (active, on_leave, suspended, terminated). */
export function createUpdateEmployeeStatusMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, status }: { id: string; status: string }) =>
      api.updateEmployeeStatus(id, status),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.employee.detail(variables.id),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
    },
  }));
}

/** Provision account for existing employee. */
export function createProvisionEmployeeAccountMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({
      id,
      req,
    }: {
      id: string;
      req: ProvisionEmployeeAccountRequest;
    }) => api.provisionEmployeeAccount(id, req),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.employee.detail(variables.id),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.auth.all });
    },
  }));
}

/** Assign user groups to employee. */
export function createAssignEmployeeGroupsMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, groupIds }: { id: string; groupIds: string[] }) =>
      api.assignEmployeeGroups(id, groupIds),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.employee.detail(variables.id),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

/** Soft delete employee mutation. */
export function createDeleteEmployeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteEmployee(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
    },
  }));
}

/** Restore soft-deleted employee mutation. */
export function createRestoreEmployeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.restoreEmployee(id),
    onSuccess: (_data, id) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.employee.detail(id),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
    },
  }));
}

/** Bulk register employees mutation. */
export function createBulkRegisterEmployeesMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({
      items,
      options,
    }: {
      items: any[];
      options?: { atomic?: boolean; dryRun?: boolean };
    }) =>
      (async () => {
        const job = await api.startBulkRegisterEmployeeJob(items, options);
        let latestJob = job;
        while (latestJob.status === "running") {
          await new Promise((resolve) => window.setTimeout(resolve, 250));
          const polled = await api.getBulkRegisterEmployeeJob(job.id);
          if (!polled) break;
          latestJob = polled;
        }
        if (latestJob.status === "failed") {
          throw new Error(latestJob.error || "Employee registration failed.");
        }
        if (!latestJob.result) {
          throw new Error("Employee registration did not produce a result.");
        }
        return latestJob.result;
      })(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.auth.all });
    },
  }));
}
