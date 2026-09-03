import { toast } from "@orbis/ui-svelte";

type OperationStatus = "running" | "success" | "error";

export interface OperationToastOptions {
  title: string;
  message?: string;
  actionLabel?: string;
  onAction?: () => void;
}

export interface OperationToastController {
  id: string | number;
  update: (progress: number, message?: string) => void;
  success: (message?: string) => void;
  error: (message?: string) => void;
  dismiss: () => void;
}

export function createOperationToast(
  options: OperationToastOptions,
): OperationToastController {
  const id = `operation-${Date.now()}-${Math.random().toString(36).slice(2)}`;
  let status: OperationStatus = "running";
  let progress = 0;
  let message = options.message || "";
  let hasRendered = false;

  function render() {
    const data = {
      id,
      duration: status === "running" ? Infinity : 5000,
      description: message,
      variant: status === "running" ? "progress" : status,
      progress,
      action: options.actionLabel && options.onAction
        ? { label: options.actionLabel, onClick: options.onAction }
        : undefined,
    } as const;

    if (!hasRendered) {
      toast(options.title, data);
      hasRendered = true;
    } else {
      toast.update(id, { title: options.title, ...data });
    }
  }

  render();

  return {
    id,
    update(nextProgress, nextMessage) {
      if (status !== "running") return;
      progress = Math.min(100, Math.max(0, nextProgress));
      if (nextMessage !== undefined) message = nextMessage;
      render();
    },
    success(nextMessage) {
      status = "success";
      progress = 100;
      if (nextMessage !== undefined) message = nextMessage;
      render();
    },
    error(nextMessage) {
      status = "error";
      if (nextMessage !== undefined) message = nextMessage;
      render();
    },
    dismiss() {
      toast.dismiss(id);
    },
  };
}
