import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import type { BulkProfileId, BulkReferenceData } from "./types";
import { getBulkProfile } from "./profiles";

export const bulkEntryQueryKeys = {
  all: ["bulk-entry"] as const,
  referenceData: (profileId: BulkProfileId, currency: string) =>
    [...bulkEntryQueryKeys.all, "reference-data", profileId, currency] as const,
};

export function createBulkReferenceDataQuery(
  profileId: () => BulkProfileId,
  currency: () => string,
) {
  return createQuery<BulkReferenceData>(() => {
    const pId = profileId();
    const curr = currency();
    const profile = getBulkProfile(pId);

    return {
      queryKey: bulkEntryQueryKeys.referenceData(pId, curr),
      queryFn: () => profile.buildReferenceData(curr),
      staleTime: 1000 * 60 * 5, // 5 minutes
    };
  });
}
