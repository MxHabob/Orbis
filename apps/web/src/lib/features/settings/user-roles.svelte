<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, type RoleDto } from "$lib/api";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { countRolePermissions } from "$lib/core/iam";
import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { createUserQuery } from "$lib/features/settings/queries";
import { invalidateUserQueries } from "./user-detail";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

let { orgId, userId }: { orgId: string; userId: string } = $props();

const queryClient = useQueryClient();
const userQuery = createUserQuery(
  () => orgId,
  () => userId,
);

const rolesQuery = createQuery<RoleDto[]>(() => ({
  queryKey: ["iam", "roles", orgId],
  queryFn: () => api.listRoles(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const availableRoles = $derived(
  (rolesQuery.data ?? []).map((r) => ({
    id: r.id,
    nameEn: r.name,
    nameAr: r.name_ar || r.name,
    descEn: r.is_system
      ? "Institutional system role"
      : "Custom organizational role",
    descAr: r.is_system
      ? m.settings_ui_role_system_description()
      : m.settings_ui_role_custom_description(),
    isSystem: Boolean(r.is_system),
    permCount: countRolePermissions(r.permissions, r.id),
  })),
);

const initialRoles = $derived<string[]>(userQuery.data?.roles ?? []);
let checkedRoles = $state<string[]>([]);
let initializedUserId = $state<string | null>(null);

$effect(() => {
  if (!userQuery.data || initializedUserId === userId) return;
  checkedRoles = [...(userQuery.data.roles ?? [])];
  initializedUserId = userId;
});

const isDirty = $derived.by(() => {
  if (!userQuery.data) return false;
  if (checkedRoles.length !== initialRoles.length) return true;
  const initialSet = new Set(initialRoles);
  return checkedRoles.some((r) => !initialSet.has(r));
});

const isLoading = $derived(userQuery.isLoading || rolesQuery.isLoading);

function toggleRole(roleId: string, active: boolean) {
  if (active) {
    if (!checkedRoles.includes(roleId)) {
      checkedRoles = [...checkedRoles, roleId];
    }
  } else {
    checkedRoles = checkedRoles.filter((r) => r !== roleId);
  }
}

function resetRoles() {
  checkedRoles = [...initialRoles];
}

const saveRolesMutation = createMutation(() => ({
  mutationFn: (roles: string[]) => api.updateUser(userId, { roles }),
  onSuccess: async () => {
    await invalidateUserQueries(queryClient, orgId, userId);
    toast.success(
      m.settings_user_detail_saved
        ? m.settings_user_detail_saved()
        : m.ui_copy_user_roles_saved_successfully_5c90a882(),
    );
  },
  onError: (e: unknown) => {
    toast.error(extractErrorMessage(e) || m.ui_copy_save_failed_c4740d2a());
  },
}));
</script>

{#if isLoading}
  <div class="flex flex-col gap-3 text-start">
    {#each Array(4) as _, index (index)}
      <Skeleton class="h-16 w-full rounded-md" />
    {/each}
  </div>
{:else}
  <div class="flex flex-col gap-4 text-start">
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
      {#each availableRoles as role (role.id)}
        {@const isChecked = checkedRoles.includes(role.id)}
        <div
          class="p-4 flex items-center justify-between gap-3 rounded-md border transition-colors hover:bg-muted/30 {isChecked
            ? 'border-primary/40 bg-primary/5'
            : 'border-border bg-card'}"
        >
          <label
            for={`role_${role.id}`}
            class="flex items-start gap-3 min-w-0 flex-1 cursor-pointer select-none"
          >
            <div class="pt-0.5 shrink-0">
              <Checkbox
                id={`role_${role.id}`}
                checked={isChecked}
                onCheckedChange={(v) => toggleRole(role.id, Boolean(v))}
              />
            </div>
            <div class="flex flex-col gap-0.5 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-xs font-bold text-foreground">
                  {l(role.nameAr, role.nameEn)}
                </span>
                <span class="text-[10px] font-mono text-muted-foreground">
                  ({role.id})
                </span>
                {#if role.isSystem}
                  <Badge variant="secondary" class="text-[9px] px-1.5 py-0">
                    {m.ui_copy_system_e3980179()}
                  </Badge>
                {/if}
              </div>
              <p class="text-[11px] text-muted-foreground leading-relaxed">
                {l(role.descAr, role.descEn)}
              </p>
            </div>
          </label>

          <div class="shrink-0 text-end">
            <Badge
              variant="outline"
              class="text-[9px] font-mono text-muted-foreground"
            >
              {role.permCount} {m.ui_copy_perms_dd19d499()}
            </Badge>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <SettingsBottomBar
    visible={isDirty}
    {isDirty}
    isSaving={saveRolesMutation.isPending}
    onsave={() => saveRolesMutation.mutate(checkedRoles)}
    onreset={resetRoles}
  />
{/if}
