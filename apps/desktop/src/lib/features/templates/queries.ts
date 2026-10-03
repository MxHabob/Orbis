/**
 * Templates domain queries and mutations.
 * Wraps OrbisApiClient with TanStack Query for caching and lifecycle management.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type TemplateDto,
  type CreateTemplateRequest,
  type UpdateTemplateRequest,
  type TemplateVersionDto,
  type TemplateBindingDto,
} from "$lib/api";

/** Query for listing templates with optional filters */
export function createTemplatesQuery(
  orgId: () => string,
  filter?: () => { documentType?: string; entityType?: string; status?: string } | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.report.templates(orgId(), filter?.()),
    queryFn: () => api.listTemplates(filter?.()),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Query for fetching a single template by ID */
export function createTemplateQuery(templateId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.report.template(templateId()),
    queryFn: () => api.getTemplate(templateId()),
    enabled: !!templateId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Query for fetching template version history */
export function createTemplateVersionsQuery(templateId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.report.templateVersions(templateId()),
    queryFn: () => api.getTemplateVersions(templateId()),
    enabled: !!templateId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Query for fetching template bindings for a report definition */
export function createTemplateBindingsQuery(definitionId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.report.templateBindings(definitionId()),
    queryFn: () => api.listTemplateBindings(definitionId()),
    enabled: !!definitionId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Mutation for creating a new template */
export function createCreateTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateTemplateRequest) => api.createTemplate(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.report.all });
    },
  }));
}

/** Mutation for updating an existing template */
export function createUpdateTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateTemplateRequest }) =>
      api.updateTemplate(id, req),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.report.template(data.id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.report.all });
    },
  }));
}

/** Mutation for publishing a template */
export function createPublishTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.publishTemplate(id),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.report.template(data.id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.report.all });
    },
  }));
}

/** Mutation for deleting a template */
export function createDeleteTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteTemplate(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.report.all });
    },
  }));
}

/** Mutation for archiving a template */
export function createArchiveTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.archiveTemplate(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.report.all });
    },
  }));
}

