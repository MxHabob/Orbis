import type {
  BulkCommitResult,
  BulkImportProfile,
  BulkReferenceData,
  BulkRowState,
  ChunkProgress,
} from "../types";

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

  const validRows = rows.filter((r) => r.status === "valid" || r.status === "warning");
  const totalRows = validRows.length;
  const totalChunks = Math.ceil(totalRows / chunkSize);
  const batchId = `BATCH-${Date.now()}-${Math.floor(Math.random() * 10000)}`;

  let processedRows = 0;
  let committedCount = 0;
  let failedCount = 0;
  const failedRowNumbers: number[] = [];
  const errors: Array<{ rowNumber: number; reason: string }> = [];

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

    try {
      const plainChunk = chunkRows.map((r) => r.data as T);
      const res = await profile.commitChunk(plainChunk, orgId, refData);

      if (res.success) {
        for (const row of chunkRows) {
          row.status = "committed";
          row.data.status = "🟢 تم الحفظ بنجاح";
          committedCount++;
        }
      } else {
        // Quarantine failed rows
        for (let idx = 0; idx < chunkRows.length; idx++) {
          const row = chunkRows[idx]!;
          const errReason = res.errors?.[idx] || res.errors?.[0] || "فشل تنفيذ المعاملة";
          row.status = "failed";
          row.commitError = errReason;
          row.data.status = `🔴 فشل: ${errReason}`;
          failedCount++;
          failedRowNumbers.push(row.rowNumber);
          errors.push({ rowNumber: row.rowNumber, reason: errReason });
        }
      }
    } catch (err: any) {
      for (const row of chunkRows) {
        const errReason = err.message || String(err);
        row.status = "failed";
        row.commitError = errReason;
        row.data.status = `🔴 خطأ فادح: ${errReason}`;
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
    failedRowNumbers,
    errors,
  };
}
