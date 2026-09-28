/**
 * Employees domain queries & mutations — Employee Affairs & Identity Provisioning.
 * Wraps OrbisApiClient with TanStack Query and queryKeys factory.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type EmployeeFilterInput,
  type RegisterEmployeeRequest,
  type UpdateEmployeeRequest,
  type ProvisionEmployeeAccountInput,
} from "$lib/api";

/** Paginated and filtered employees directory query. */
export function createEmployeesQuery(
  orgId: () => string,
  filter?: () => EmployeeFilterInput | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.employee.list(orgId(), filter?.() as Record<string, unknown>),
    queryFn: () => api.listEmployees(filter?.()),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

/** Single employee profile dossier query (EmployeeDetail). */
export function createEmployeeQuery(
  employeeId: () => string,
) {
  return createQuery(() => ({
    queryKey: queryKeys.employee.detail(employeeId()),
    queryFn: () => api.getEmployee(employeeId()),
    enabled: !!employeeId(),
    staleTime: 60 * 1000,
  }));
}

/** Employee counts by status (active, on_leave, suspended, terminated). */
export function createEmployeeStatsQuery(
  orgId: () => string,
) {
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
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.detail(variables.id) });
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
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.detail(variables.id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.all });
    },
  }));
}

/** Provision account for existing employee. */
export function createProvisionEmployeeAccountMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: ProvisionEmployeeAccountInput }) =>
      api.provisionEmployeeAccount(id, req),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.detail(variables.id) });
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
      queryClient.invalidateQueries({ queryKey: queryKeys.employee.detail(variables.id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.group.all });
    },
  }));
}

