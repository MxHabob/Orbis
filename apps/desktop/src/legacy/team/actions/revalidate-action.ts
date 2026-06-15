/** Desktop stub — cache invalidation is handled via React Query. */
export async function revalidateAfterTeamChange() {
  return { ok: true as const };
}
