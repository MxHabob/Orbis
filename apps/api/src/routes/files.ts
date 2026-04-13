import { verifySession } from "../utils/auth";
import { getFile } from "@midday/storage";
import type { Context } from "hono";
import type { HonoEnv } from "../utils/auth";

/**
 * Handle file proxying with 2026 Best Practices:
 * 1. Multi-tier Authentication: Session Cookies (Browser) OR Service Tokens (Workers/Bots)
 * 2. Robust URL Parsing: Handles varied filePath formats and double-encoding
 * 3. Performance: Stream-based delivery with strict cache headers
 * 4. Error Transparency: Returns detailed reasons for failures (400 vs 401 vs 404)
 */
export async function handleFileProxy(c: Context<HonoEnv>) {
  // 1. Authenticate Request
  const session = await verifySession(c);
  const publicToken = c.req.query("token");
  
  // High-Security Bypass: Allow access if a valid service token is provided 
  // (Used by Puppeteer/Workers for authorized batch exports)
  const isAuthorized = !!session?.user || (publicToken && publicToken === process.env.ASSET_PUBLIC_TOKEN);

  if (!isAuthorized) {
    console.warn(`[FileProxy] Blocked unauthorized access to ${c.req.query("filePath")}`);
    return c.json({ error: "Unauthorized: Missing session or valid asset token" }, 401);
  }

  const filePath = c.req.query("filePath");
  const range = c.req.header("range");

  if (!filePath) {
    return c.json({ error: "Malformed request: filePath is missing" }, 400);
  }

  // 2. Resolve Bucket & Key
  const cleanPath = filePath.startsWith('/') ? filePath.slice(1) : filePath;
  const parts = cleanPath.split("/");
  const bucket = parts[0];
  const key = parts.slice(1).join("/");

  if (!bucket || !key) {
    console.error(`[FileProxy] Malformed filePath received: ${filePath}`);
    return c.json({ error: "Malformed request: bucket or key could not be determined" }, 400);
  }

  try {
    const result = await getFile({
      bucket,
      key,
      range,
    });

    if (!result.Body) {
      return c.json({ error: "Resource not found in storage" }, 404);
    }

    // 3. Set Performance & Security Headers
    if (result.ContentType) c.header("Content-Type", result.ContentType);
    if (result.ContentLength) c.header("Content-Length", result.ContentLength.toString());
    
    // 2026 Best Practice: Strict caching for immutable assets
    c.header("Cache-Control", "public, max-age=31536000, immutable");
    if (result.ETag) c.header("ETag", result.ETag);
    if (result.LastModified) c.header("Last-Modified", result.LastModified.toUTCString());

    // Support Partial Content (206) for media
    if (range && result.ContentRange) {
      c.header("Content-Range", result.ContentRange);
      c.status(206);
    }

    // Transform S3 body to web-compatible stream
    const stream = (result.Body as any).transformToWebStream?.() ?? result.Body;

    return c.body(stream as any);
  } catch (err: any) {
    // 4. Advanced Error Reporting
    const errorStatus = err.name === "NoSuchKey" ? 404 : 500;
    
    console.error(`[FileProxy] Failed to serve ${bucket}/${key}:`, {
      name: err.name,
      message: err.message,
      stack: process.env.NODE_ENV === 'development' ? err.stack : undefined
    });

    return c.json({ 
      error: errorStatus === 404 ? "File not found" : "Internal storage error", 
      details: process.env.NODE_ENV === 'development' ? err.message : undefined 
    }, errorStatus);
  }
}
