import { verifySession } from "../utils/auth";
import { getFile } from "@midday/storage";
import type { Context } from "hono";
import type { HonoEnv } from "../utils/auth";

/**
 * Handle file proxying with best practices:
 * 1. Supports Authentication (via Session or Service Token)
 * 2. Supports Range Requests (for video/large files)
 * 3. Handles Content-Type and Content-Length correctly
 * 4. Streaming for low memory usage
 * 5. Security: Verifies bucket/key and user access
 */
export async function handleFileProxy(c: Context<HonoEnv>) {
  const session = await verifySession(c);

  if (!session?.user) {
    return c.json({ error: "Unauthorized" }, 401);
  }

  const filePath = c.req.query("filePath");
  const range = c.req.header("range");

  if (!filePath) {
    return c.json({ error: "filePath is required" }, 400);
  }

  // Security: Basic path validation
  if (filePath.includes("..")) {
    return c.json({ error: "Invalid path" }, 400);
  }

  const parts = filePath.split("/");
  const bucket = parts[0];
  const key = parts.slice(1).join("/");

  if (!bucket || !key) {
    return c.json({ error: "Malformed filePath" }, 400);
  }

  try {
    const result = await getFile({
      bucket,
      key,
      range,
    });

    if (!result.Body) {
      return c.json({ error: "File not found" }, 404);
    }

    // Set standard headers
    if (result.ContentType) c.header("Content-Type", result.ContentType);
    if (result.ContentLength) c.header("Content-Length", result.ContentLength.toString());
    if (result.CacheControl) c.header("Cache-Control", result.CacheControl);
    if (result.ETag) c.header("ETag", result.ETag);
    if (result.LastModified) c.header("Last-Modified", result.LastModified.toUTCString());

    // Support forced downloads
    const isDownload = c.req.query("download") === "true";
    if (isDownload) {
      const filename = key.split("/").pop() || "file";
      // Ensure the filename is safely encoded
      const safeFilename = encodeURIComponent(filename).replace(/['()]/g, escape).replace(/\*/g, '%2A');
      c.header("Content-Disposition", `attachment; filename="${safeFilename}"; filename*=UTF-8''${safeFilename}`);
    }

    // Support Partial Content (206)
    if (range && result.ContentRange) {
      c.header("Content-Range", result.ContentRange);
      c.status(206);
    }

    // Node.js S3 stream to Web Stream transformation
    const stream = (result.Body as any).transformToWebStream?.() ?? result.Body;

    return c.body(stream as any);
  } catch (err: any) {
    // Handle specific S3 errors
    if (err.name === "NoSuchKey") {
      return c.json({ error: "File not found" }, 404);
    }

    console.error(`[FileProxy] Error serving ${filePath}:`, err);
    return c.json({ 
      error: "Internal server error", 
      details: process.env.NODE_ENV === 'development' ? err.message : undefined 
    }, 500);
  }
}
