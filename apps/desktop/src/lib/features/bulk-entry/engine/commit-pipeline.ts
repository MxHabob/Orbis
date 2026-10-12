import * as m from "$lib/paraglide/messages";
import { extractErrorMessage } from "$lib/api";
import type {
  BulkCommitResult,
  BulkImportProfile,
  BulkReferenceData,
  BulkRowState,
  ChunkProgress,
} from "../types";

function isOrganizationLicenseError(reason: string): boolean {
  const normalized = reason.toLocaleLowerCase();
  return (
    normalized.includes("license is not valid for the active organization") ||
    normalized.includes("product key is not valid for the active organization") ||
    normalized.startsWith("license required:")
  );
}

export interface CommitPipelineOptions<T = Record<string, unknown>> {
  rows: BulkRowState[];
  profile: BulkImportProfile<T, any>;
  refData: BulkReferenceData;
  orgId: string;
  chunkSize?: number;
  onProgress?: (progress: ChunkProgress) => void;
}

export async function executeCommitPipeline<T = Record<string, unknown>>(
  options: CommitPipelineOptions<T>,
): Promise<BulkCommitResult> {
  const { rows, profile, refData, orgId, chunkSize = 50, onProgress } = options;
  if (!Number.isSafeInteger(chunkSize) || chunkSize <= 0) {
    throw new RangeError("chunkSize must be a positive safe integer");
  }

  const validRows = rows.filter((r) => r.status === "valid" || r.status === "warning");
  const totalRows = validRows.length;
  const totalChunks = Math.ceil(totalRows / chunkSize);
  const batchId = `BATCH-${Date.now()}-${Math.floor(Math.random() * 10000)}`;

  let processedRows = 0;
  let committedCount = 0;
  let failedCount = 0;
  const failedRowNumbers: number[] = [];
  const errors: Array<{ rowNumber: number; reason: string }> = [];
  const warnings: Array<{ rowNumber: number; reason: string }> = [];
  let blockingError: { reason: string; isLicenseError: boolean } | null = null;
  let licenseRejected = false;

  for (let c = 0; c < totalChunks; c++) {
    const start = c * chunkSize;
    const end = Math.min(start + chunkSize, totalRows);
    const chunkRows = validRows.slice(start, end);

    const progress: ChunkProgress = {
      totalChunks,
      currentChunk: c + 1,
      chunkSize,
      totalRows,
      processedRows,
      committedRows: committedCount,
      failedRows: failedCount,
      isFinished: false,
      hasErrors: failedCount > 0,
    };
    onProgress?.(progress);

    if (blockingError) {
      for (const row of chunkRows) {
        row.status = "failed";
        row.commitError = blockingError.reason;
        row.data.status = blockingError.isLicenseError
          ? m.bulk_entry_status_failed({ reason: blockingError.reason })
          : m.bulk_entry_status_fatal_error({ reason: blockingError.reason });
        failedCount++;
        failedRowNumbers.push(row.rowNumber);
        errors.push({ rowNumber: row.rowNumber, reason: blockingError.reason });
      }
      processedRows += chunkRows.length;
      continue;
    }

    try {
      const plainChunk = chunkRows.map((r) => r.data as T);
      const res = await profile.commitChunk(plainChunk, orgId, refData);
      warnings.push(...(res.warnings ?? []));

      if (res.success) {
        const licenseRowError = res.rowErrors?.find((error) =>
          isOrganizationLicenseError(error.reason),
        );
        if (licenseRowError) {
          blockingError = {
            reason: m.bulk_entry_result_license_invalid(),
            isLicenseError: true,
          };
          licenseRejected = true;
        }

        for (const row of chunkRows) {
          const rowError = licenseRowError
            ? undefined
            : res.rowErrors?.find((error) => error.rowNumber === row.rowNumber);
          if (licenseRowError || rowError) {
            row.status = "failed";
            const reason = licenseRowError
              ? m.bulk_entry_result_license_invalid()
              : rowError && isOrganizationLicenseError(rowError.reason)
                ? m.bulk_entry_result_license_invalid()
                : rowError?.reason ?? m.bulk_entry_commit_transaction_failed();
            row.commitError = reason;
            row.data.status = m.bulk_entry_status_failed({ reason });
            failedCount++;
            failedRowNumbers.push(row.rowNumber);
            errors.push({ rowNumber: row.rowNumber, reason });
          } else {
            row.status = "committed";
            row.data.status = m.bulk_entry_status_committed();
            committedCount++;
          }
        }
      } else {
        const licenseError = res.errors?.find(isOrganizationLicenseError);
        blockingError = licenseError
          ? {
              reason: m.bulk_entry_result_license_invalid(),
              isLicenseError: true,
            }
          : null;
        licenseRejected ||= Boolean(licenseError);
        // Quarantine failed rows
        for (let idx = 0; idx < chunkRows.length; idx++) {
          const row = chunkRows[idx]!;
          const rawError = licenseError
            ? m.bulk_entry_result_license_invalid()
            : res.errors?.[idx] ||
              res.errors?.[0] ||
              m.bulk_entry_commit_transaction_failed();
          const errReason = rawError;
          row.status = "failed";
          row.commitError = errReason;
          row.data.status = m.bulk_entry_status_failed({ reason: errReason });
          failedCount++;
          failedRowNumbers.push(row.rowNumber);
          errors.push({ rowNumber: row.rowNumber, reason: errReason });
        }
      }
    } catch (err) {
      const rawError = extractErrorMessage(err);
      const isLicenseError = isOrganizationLicenseError(rawError);
      if (isLicenseError) {
        licenseRejected = true;
      }
      blockingError = {
        reason: isLicenseError
          ? m.bulk_entry_result_license_invalid()
          : rawError,
        isLicenseError,
      };
      for (const row of chunkRows) {
        const errReason = blockingError.reason;
        row.status = "failed";
        row.commitError = errReason;
        row.data.status = isLicenseError
          ? m.bulk_entry_status_failed({ reason: errReason })
          : m.bulk_entry_status_fatal_error({ reason: errReason });
        failedCount++;
        failedRowNumbers.push(row.rowNumber);
        errors.push({ rowNumber: row.rowNumber, reason: errReason });
      }
    }

    processedRows += chunkRows.length;
  }

  const finalProgress: ChunkProgress = {
    totalChunks,
    currentChunk: totalChunks,
    chunkSize,
    totalRows,
    processedRows,
    committedRows: committedCount,
    failedRows: failedCount,
    isFinished: true,
    hasErrors: failedCount > 0,
  };
  onProgress?.(finalProgress);

  return {
    batchId,
    profileId: profile.id,
    totalRows,
    committedCount,
    failedCount,
    skippedCount: rows.length - totalRows,
    licenseRejected,
    failedRowNumbers,
    errors,
    warnings,
  };
}
