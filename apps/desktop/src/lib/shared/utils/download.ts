import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import * as m from "$lib/paraglide/messages";

export type RevealDownloadedLocation = () => void | Promise<void>;

export function downloadBlob(
  blob: Blob,
  filename: string,
): null {
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 60_000);

  return null;
}

export function revealDownloadedPath(path: string): RevealDownloadedLocation {
  return () => revealItemInDir(path);
}

export function createShowInFolderToastAction(
  revealLocation: RevealDownloadedLocation,
) {
  function reportRevealError(error: unknown) {
    console.error("Failed to reveal downloaded file location:", error);
    toast.error(extractErrorMessage(error));
  }

  return {
    label: m.common_docs_action_show_in_folder(),
    onClick: () => {
      try {
        const result = revealLocation();
        if (result) void result.catch(reportRevealError);
      } catch (error) {
        reportRevealError(error);
      }
    },
  };
}
