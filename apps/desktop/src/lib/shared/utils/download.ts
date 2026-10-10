import { openPath } from "@tauri-apps/plugin-opener";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import * as m from "$lib/paraglide/messages";

export type OpenDownloadedFile = () => void | Promise<void>;

export function downloadBlob(
  blob: Blob,
  filename: string,
): OpenDownloadedFile {
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 60_000);

  return () => {
    window.open(url, "_blank", "noopener,noreferrer");
  };
}

export function openDownloadedPath(path: string): OpenDownloadedFile {
  return () => openPath(path);
}

export function createOpenDownloadToastAction(
  openFile: OpenDownloadedFile,
) {
  function reportOpenError(error: unknown) {
    console.error("Failed to open downloaded file:", error);
    toast.error(extractErrorMessage(error));
  }

  return {
    label: m.common_docs_action_open_file(),
    onClick: () => {
      try {
        const result = openFile();
        if (result) void result.catch(reportOpenError);
      } catch (error) {
        reportOpenError(error);
      }
    },
  };
}
