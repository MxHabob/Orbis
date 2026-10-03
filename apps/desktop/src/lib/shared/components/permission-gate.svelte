<!--
  @component PermissionGate
  Declarative permission and role gate for Svelte 5.
  Hides or shows UI elements based on the current user's effective permissions and roles.

  Usage:
    <PermissionGate permission="student:write">
      <Button onclick={createStudent}>Add Student</Button>
    </PermissionGate>

    <PermissionGate permissions={["finance:read", "finance:write"]} requireAll={false}>
      <FinanceWidget />
    </PermissionGate>
-->
<script lang="ts">
import type { Snippet } from "svelte";
import { effectiveContext } from "$lib/stores/effective-context.svelte";

interface Props {
  permission?: string;
  permissions?: string[];
  role?: string;
  roles?: string[];
  requireAll?: boolean;
  fallback?: Snippet;
  children?: Snippet;
}

let {
  permission,
  permissions = [],
  role,
  roles = [],
  requireAll = false,
  fallback,
  children,
}: Props = $props();

const requiredPermissions = $derived.by(() => {
  const list = [...permissions];
  if (permission) list.push(permission);
  return list;
});

const requiredRoles = $derived.by(() => {
  const list = [...roles];
  if (role) list.push(role);
  return list;
});

const hasAccess = $derived.by(() => {
  // Permission checks
  if (requiredPermissions.length > 0) {
    if (requireAll) {
      const allPassed = requiredPermissions.every((p) =>
        effectiveContext.hasPermission(p),
      );
      if (!allPassed) return false;
    } else {
      const anyPassed = requiredPermissions.some((p) =>
        effectiveContext.hasPermission(p),
      );
      if (!anyPassed) return false;
    }
  }

  // Role checks
  if (requiredRoles.length > 0) {
    const userRole = effectiveContext.effectiveRole;
    const userRoles = effectiveContext.roles;
    if (requireAll) {
      const allPassed = requiredRoles.every(
        (r) => r === userRole || userRoles.includes(r),
      );
      if (!allPassed) return false;
    } else {
      const anyPassed = requiredRoles.some(
        (r) => r === userRole || userRoles.includes(r),
      );
      if (!anyPassed) return false;
    }
  }

  return true;
});
</script>

{#if hasAccess}
  {#if children}
    {@render children()}
  {/if}
{:else if fallback}
  {@render fallback()}
{/if}
