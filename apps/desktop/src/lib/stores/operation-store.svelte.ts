import type { BulkCommitResult } from "$lib/features/bulk-entry/types";
import type { BulkRegisterJobDto } from "$lib/api/types";
import type { BulkRegisterEmployeeJobDto } from "$lib/api/types";
import type { BulkProfileId } from "$lib/features/bulk-entry/types";
import type { BulkTransferStudentCommandResultDto } from "$lib/api/types";
import type { ChunkProgress } from "$lib/features/bulk-entry/types";

export type OperationStatus =
  | "running"
  | "completed"
  | "failed"
  | "interrupted";

export type AppOperation = {
  id: string;
  kind: "bulk-import" | "student-transfer";
  jobId?: string;
  jobProfile?: BulkProfileId;
  title: string;
  status: OperationStatus;
  processed: number;
  total: number;
  committed: number;
  failed: number;
  updatedAt: number;
};

const STORAGE_KEY = "orbis:active-operations";

function readOperations(): AppOperation[] {
  if (typeof window === "undefined") return [];
  try {
    const parsed = JSON.parse(
      window.localStorage.getItem(STORAGE_KEY) ?? "[]",
    ) as AppOperation[];
    return parsed
      .filter(
        (item) =>
          (item.kind === "bulk-import" || item.kind === "student-transfer") &&
          item.status !== "completed",
      )
      .map((item) =>
        item.status === "running"
          ? { ...item, status: "interrupted" as const }
          : item,
      );
  } catch {
    return [];
  }
}

class OperationStore {
  operations = $state<AppOperation[]>(readOperations());
  results = $state<Record<string, BulkCommitResult>>({});

  constructor() {
    if (typeof window !== "undefined") {
      window.addEventListener("beforeunload", () => {
        this.persist();
      });
    }
  }

  start(
    title: string,
    total: number,
    kind: AppOperation["kind"] = "bulk-import",
  ): AppOperation {
    const operation: AppOperation = {
      id: crypto.randomUUID(),
      kind,
      title,
      status: "running",
      processed: 0,
      total,
      committed: 0,
      failed: 0,
      updatedAt: Date.now(),
    };
    this.operations = [operation, ...this.operations].slice(0, 5);
    const retainedIds = new Set(this.operations.map((item) => item.id));
    for (const id of Object.keys(this.results)) {
      if (!retainedIds.has(id)) delete this.results[id];
    }
    this.persist();
    return operation;
  }

  finish(id: string, result: BulkCommitResult) {
    this.results[id] = result;
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            status:
              result.failedCount > 0
                ? ("failed" as const)
                : ("completed" as const),
            processed: result.totalRows,
            total: result.totalRows,
            committed: result.committedCount,
            failed: result.failedCount,
            updatedAt: Date.now(),
          }
        : operation,
    );
    this.persist();
  }

  finishTransfer(id: string, result: BulkTransferStudentCommandResultDto) {
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            status:
              result.failed > 0 ? ("failed" as const) : ("completed" as const),
            processed: result.total,
            total: result.total,
            committed: result.succeeded,
            failed: result.failed,
            updatedAt: Date.now(),
          }
        : operation,
    );
    this.persist();
  }

  updateProgress(id: string, progress: ChunkProgress) {
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            processed: progress.processedRows,
            total: progress.totalRows,
            committed: progress.committedRows,
            failed: progress.failedRows,
            updatedAt: Date.now(),
          }
        : operation,
    );
    this.persist();
  }

  setJobId(id: string, jobId: string, profile: BulkProfileId = "students") {
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? { ...operation, jobId, jobProfile: profile }
        : operation,
    );
    this.persist();
  }

  syncJob(id: string, job: BulkRegisterJobDto) {
    const result: BulkCommitResult | undefined = job.result
      ? {
          batchId: job.id,
          profileId: "students",
          totalRows: job.result.total,
          committedCount: job.result.succeeded,
          failedCount: job.result.failed,
          skippedCount: 0,
          licenseRejected: false,
          failedRowNumbers: job.result.items
            .filter((item) => !item.success)
            .map((item) => item.row_index),
          errors: job.result.items
            .filter((item) => !item.success)
            .map((item) => ({
              rowNumber: item.row_index,
              reason: item.error ?? "",
            })),
          warnings: [],
        }
      : undefined;
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            status:
              job.status === "completed"
                ? ("completed" as const)
                : job.status === "failed"
                  ? ("failed" as const)
                  : ("running" as const),
            processed: job.processed,
            committed: job.succeeded,
            failed: job.failed,
            updatedAt: Date.now(),
          }
        : operation,
    );
    if (result) this.results[id] = result;
    this.persist();
  }

  syncEmployeeJob(id: string, job: BulkRegisterEmployeeJobDto) {
    const result: BulkCommitResult | undefined = job.result
      ? {
          batchId: job.id,
          profileId: "employees",
          totalRows: job.result.total,
          committedCount: job.result.succeeded,
          failedCount: job.result.failed,
          skippedCount: 0,
          licenseRejected: false,
          failedRowNumbers: job.result.items
            .filter((item) => !item.success)
            .map((item) => item.row_index),
          errors: job.result.items
            .filter((item) => !item.success)
            .map((item) => ({
              rowNumber: item.row_index,
              reason: item.error ?? "",
            })),
          warnings: [],
        }
      : undefined;
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            status:
              job.status === "completed"
                ? ("completed" as const)
                : job.status === "failed"
                  ? ("failed" as const)
                  : ("running" as const),
            processed: job.processed,
            committed: job.succeeded,
            failed: job.failed,
            updatedAt: Date.now(),
          }
        : operation,
    );
    if (result) this.results[id] = result;
    this.persist();
  }

  fail(id: string) {
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            status: "failed" as const,
            failed: operation.failed + 1,
            updatedAt: Date.now(),
          }
        : operation,
    );
    this.persist();
  }

  dismiss(id: string) {
    this.operations = this.operations.filter(
      (operation) => operation.id !== id,
    );
    delete this.results[id];
    this.persist();
  }

  resultFor(id: string): BulkCommitResult | null {
    return this.results[id] ?? null;
  }

  private persist() {
    if (typeof window === "undefined") return;
    window.localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify(
        this.operations.filter((operation) => operation.status !== "completed"),
      ),
    );
  }
}

export const operationStore = new OperationStore();
