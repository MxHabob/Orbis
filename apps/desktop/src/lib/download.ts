import { invoke } from "@orbis/desktop-client/core";
import { isDesktopApp } from "@orbis/desktop-client/platform";

export async function downloadFile(url: string, filename: string) {
  if (!isDesktopApp()) {
    // Web mode - check if URL is cross-origin
    try {
      const urlObj = new URL(url, window.location.origin);
      const isCrossOrigin = urlObj.origin !== window.location.origin;

      if (isCrossOrigin) {
        // Cross-origin: open in new tab (download attribute doesn't work cross-origin)
        window.open(url, "_blank", "noopener,noreferrer");
      } else {
        // Same-origin: use download attribute
        const link = document.createElement("a");
        link.href = url;
        link.download = filename;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
      }
    } catch {
      // Fallback: open in new tab
      window.open(url, "_blank", "noopener,noreferrer");
    }
    return;
  }

  try {
    // Desktop mode - open download URL in default browser
    // The browser will have access to user's authentication and handle the download
    console.log("Opening download in browser:", { url, filename });

    // Check if URL is already absolute (starts with http:// or https://)
    // If relative, prepend window.location.origin
    const downloadUrl =
      url.startsWith("http://") || url.startsWith("https://")
        ? url
        : `${window.location.origin}${url}`;

    // Use Tauri's opener plugin via invoke to open URL in default browser
    await invoke("plugin:opener|open_url", {
      url: downloadUrl,
    });

    console.log("Download opened in browser:", downloadUrl);
  } catch (error) {
    console.error("Failed to open download in browser:", error);
    throw error;
  }
}
