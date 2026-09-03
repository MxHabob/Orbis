import type { Component } from "svelte";

type ToastVariant = "default" | "success" | "error" | "info" | "warning" | "progress" | "spinner" | "destructive" | "ai";
type ToastMessage = string | Component<any>;

export interface ToastAction {
  label: string;
  onClick: () => void;
}

export interface ToastData {
  id: string;
  title?: ToastMessage;
  description?: string;
  variant: ToastVariant;
  progress?: number;
  action?: ToastAction;
  footer?: string;
  duration?: number;
  open: boolean;
}

export type ToastOptions = Partial<Omit<ToastData, "id" | "open" | "variant">> & {
  id?: string;
  variant?: ToastVariant;
};

class ToastStore {
  toasts = $state<ToastData[]>([]);
  private nextId = 0;
  private timers = new Map<string, ReturnType<typeof setTimeout>>();

  private createId() {
    this.nextId += 1;
    return `toast-${this.nextId}`;
  }

  add(title?: ToastMessage, options: ToastOptions = {}): ToastHandle {
    const id = options.id ?? this.createId();
    const toast: ToastData = {
      id,
      title,
      description: options.description,
      variant: options.variant ?? "default",
      progress: options.progress,
      action: options.action,
      footer: options.footer,
      duration: options.duration ?? 4000,
      open: true,
    };

    const existing = this.toasts.findIndex((item) => item.id === id);
    if (existing >= 0) this.toasts[existing] = toast;
    else this.toasts = [toast, ...this.toasts].slice(0, 5);

    this.scheduleDismiss(toast);
    return this.handle(id);
  }

  update(id: string, data: Partial<ToastData>) {
    const index = this.toasts.findIndex((item) => item.id === id);
    if (index < 0) return;
    const current = this.toasts[index];
    if (!current) return;
    const updated: ToastData = {
      id,
      title: data.title ?? current.title,
      description: data.description ?? current.description,
      variant: data.variant ?? current.variant,
      progress: data.progress ?? current.progress,
      action: data.action ?? current.action,
      footer: data.footer ?? current.footer,
      duration: data.duration ?? current.duration,
      open: true,
    };
    this.toasts[index] = updated;
    this.scheduleDismiss(updated);
  }

  dismiss(id?: string) {
    for (const item of this.toasts) {
      if (!id || item.id === id) item.open = false;
    }
    if (id) this.removeLater(id, 300);
  }

  remove(id?: string) {
    if (id) this.toasts = this.toasts.filter((item) => item.id !== id);
    else this.toasts = [];
  }

  promise<T>(promise: Promise<T> | (() => Promise<T>), options: {
    loading?: string;
    success?: string | ((value: T) => string);
    error?: string | ((error: unknown) => string);
    finally?: () => void | Promise<void>;
  } & ToastOptions = {}) {
    const handle = this.add(options.loading ?? "Loading...", {
      ...options,
      variant: "spinner",
      duration: Number.POSITIVE_INFINITY,
    });
    const task = typeof promise === "function" ? promise() : promise;
    task.then((value) => {
      this.update(handle.id, {
        title: typeof options.success === "function" ? options.success(value) : options.success ?? "Completed",
        variant: "success",
        duration: 4000,
      });
      return value;
    }).catch((error) => {
      this.update(handle.id, {
        title: typeof options.error === "function" ? options.error(error) : options.error ?? "Something went wrong",
        variant: "error",
        duration: 5000,
      });
    }).finally(() => options.finally?.());
    return handle.id;
  }

  private scheduleDismiss(item: ToastData) {
    const duration = item.duration;
    if (duration === undefined || !Number.isFinite(duration) || duration <= 0) return;
    this.removeLater(item.id, duration);
  }

  private removeLater(id: string, delay: number) {
    const current = this.timers.get(id);
    if (current) clearTimeout(current);
    this.timers.set(id, setTimeout(() => {
      this.timers.delete(id);
      this.remove(id);
    }, delay));
  }

  private handle(id: string): ToastHandle {
    return {
      id,
      dismiss: () => this.dismiss(id),
      update: (data) => this.update(id, data),
    };
  }
}

export interface ToastHandle {
  id: string;
  dismiss: () => void;
  update: (data: Partial<ToastData>) => void;
}

export const toastStore = new ToastStore();

function createShortcut(variant: ToastVariant) {
  return (title: ToastMessage, options: ToastOptions = {}) =>
    toastStore.add(title, { ...options, variant });
}

export const toast = Object.assign(
  (title: ToastMessage, options?: ToastOptions) => toastStore.add(title, options),
  {
    success: createShortcut("success"),
    error: createShortcut("error"),
    info: createShortcut("info"),
    warning: createShortcut("warning"),
    loading: createShortcut("spinner"),
    promise: toastStore.promise.bind(toastStore),
    update: (id: string, data: Partial<ToastData>) => toastStore.update(id, data),
    dismiss: (id?: string) => toastStore.dismiss(id),
  },
);
