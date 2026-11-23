export type OperationStatus =
  | "running"
  | "completed"
  | "failed"
  | "interrupted";

export type AppOperation = {
  id: string;
  kind: "bulk-import";
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
      .filter((item) => item.kind === "bulk-import")
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

  constructor() {
    if (typeof window !== "undefined") {
      window.addEventListener("beforeunload", () => this.persist());
    }
  }

  start(title: string, total: number): AppOperation {
    const operation: AppOperation = {
      id: crypto.randomUUID(),
      kind: "bulk-import",
      title,
      status: "running",
      processed: 0,
      total,
      committed: 0,
      failed: 0,
      updatedAt: Date.now(),
    };
    this.operations = [operation, ...this.operations].slice(0, 5);
    this.persist();
    return operation;
  }

  update(
    id: string,
    progress: {
      processedRows: number;
      committedRows: number;
      failedRows: number;
      totalRows: number;
      isFinished: boolean;
      hasErrors: boolean;
    },
  ) {
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            status: progress.isFinished
              ? progress.hasErrors
                ? ("failed" as const)
                : ("completed" as const)
              : ("running" as const),
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

  finish(
    id: string,
    result: { committedCount: number; failedCount: number },
  ) {
    this.operations = this.operations.map((operation) =>
      operation.id === id
        ? {
            ...operation,
            status:
              result.failedCount > 0
                ? ("failed" as const)
                : ("completed" as const),
            processed: operation.total,
            committed: result.committedCount,
            failed: result.failedCount,
            updatedAt: Date.now(),
          }
        : operation,
    );
    this.persist();
    if (typeof window !== "undefined") {
      window.setTimeout(() => this.dismiss(id), 8000);
    }
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
    if (typeof window !== "undefined") {
      window.setTimeout(() => this.dismiss(id), 12000);
    }
  }

  dismiss(id: string) {
    this.operations = this.operations.filter(
      (operation) => operation.id !== id,
    );
    this.persist();
  }

  private persist() {
    if (typeof window === "undefined") return;
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(this.operations));
  }
}

export const operationStore = new OperationStore();
