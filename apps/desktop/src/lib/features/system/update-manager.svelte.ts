import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { api } from "$lib/api/client";
import { isDesktopApp } from "$lib/api/runtime";
import type { UpdateCheckResult } from "$lib/api/types";

type UpdateProgress = {
  downloaded_bytes: number;
  total_bytes: number | null;
};

class UpdateManager {
  updateResult = $state<UpdateCheckResult | null>(null);
  progress = $state<UpdateProgress | null>(null);
  isChecking = $state(false);
  isDownloading = $state(false);
  isApplying = $state(false);
  isReadyToRestart = $state(false);
  error = $state<string | null>(null);

  private hasChecked = false;
  private listenerPromise: Promise<UnlistenFn> | null = null;
  private networkCleanup: (() => void) | null = null;
  isOnline = $state(
    typeof navigator === "undefined" ? true : navigator.onLine,
  );

  async initialize() {
    if (!isDesktopApp) return;

    try {
      this.bindNetworkEvents();
      await this.ensureProgressListener();
      if (!this.isOnline) {
        this.error = null;
        return;
      }
      if (
        !this.hasChecked &&
        !this.isChecking &&
        !this.isDownloading &&
        !this.isApplying
      ) {
        await this.checkForUpdates();
      }
    } catch (error) {
      this.error = this.isOnline ? this.getErrorMessage(error) : null;
    }
  }

  async checkForUpdates(): Promise<UpdateCheckResult> {
    if (this.isChecking || this.isDownloading || this.isApplying) {
      throw new Error("An update operation is already in progress.");
    }
    this.error = null;
    if (!this.isOnline || (typeof navigator !== "undefined" && !navigator.onLine)) {
      throw new Error("No internet connection");
    }
    this.isChecking = true;

    try {
      const result = await api.systemCheckForUpdates();
      this.updateResult = result;
      this.hasChecked = true;
      return result;
    } catch (error) {
      this.error = this.isOnline ? this.getErrorMessage(error) : null;
      throw error;
    } finally {
      this.isChecking = false;
    }
  }

  async downloadUpdate() {
    if (!this.isOnline || (typeof navigator !== "undefined" && !navigator.onLine)) {
      this.error = null;
      throw new Error("No internet connection");
    }
    if (!this.updateResult?.has_update) {
      await this.checkForUpdates();
    }
    if (!this.updateResult?.has_update) return;

    this.error = null;
    this.progress = null;
    this.isDownloading = true;
    this.isReadyToRestart = false;

    try {
      await this.ensureProgressListener();
      await api.systemDownloadUpdate();
      this.isReadyToRestart = true;
    } catch (error) {
      this.error = this.isOnline ? this.getErrorMessage(error) : null;
      throw error;
    } finally {
      this.isDownloading = false;
    }
  }

  async installUpdate() {
    if (!this.isReadyToRestart) {
      throw new Error(
        "The update must finish downloading before it can be applied.",
      );
    }

    this.error = null;
    this.isApplying = true;
    try {
      await api.systemInstallUpdate();
    } catch (error) {
      this.error = this.isOnline ? this.getErrorMessage(error) : null;
      this.isApplying = false;
      throw error;
    }
  }

  async retry() {
    if (!this.isOnline || (typeof navigator !== "undefined" && !navigator.onLine)) {
      this.error = null;
      throw new Error("No internet connection");
    }
    if (this.isReadyToRestart) {
      await this.installUpdate();
      return;
    }
    if (this.updateResult?.has_update) {
      await this.downloadUpdate();
      return;
    }
    await this.checkForUpdates();
  }

  private async ensureProgressListener() {
    if (!isDesktopApp) return;
    if (!this.listenerPromise) {
      this.listenerPromise = listen<UpdateProgress>(
        "desktop-update-progress",
        ({ payload }) => {
          this.progress = payload;
          this.isDownloading = true;
        },
      );
    }
    try {
      await this.listenerPromise;
    } catch (error) {
      this.listenerPromise = null;
      throw error;
    }
  }

  private getErrorMessage(error: unknown) {
    return error instanceof Error ? error.message : String(error);
  }

  private bindNetworkEvents() {
    if (this.networkCleanup || typeof window === "undefined") return;

    const handleOnline = () => {
      this.isOnline = true;
      if (
        !this.hasChecked &&
        !this.isChecking &&
        !this.isDownloading &&
        !this.isApplying
      ) {
        void this.checkForUpdates().catch(() => {
          this.error = null;
        });
      }
    };
    const handleOffline = () => {
      this.isOnline = false;
      this.error = null;
      this.isChecking = false;
      this.isDownloading = false;
      this.isApplying = false;
      this.progress = null;
    };

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);
    this.networkCleanup = () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }
}

export const updateManager = new UpdateManager();
