import { auth } from "@orbis/auth";
import { uploadFile } from "@orbis/storage";
import { stripSpecialCharacters } from "@orbis/utils";
import { nanoid } from "nanoid";
import type { Context } from "hono";

export async function handleUpload(c: Context) {
  const session = await auth.api.getSession({
    headers: c.req.raw.headers,
  });

  if (!session?.user) {
    return c.json({ error: "Unauthorized" }, 401);
  }

  const formData = await c.req.parseBody();
  const file = formData["file"];
  const bucket = (formData["bucket"] as string) ?? "avatars";
  const pathInput = formData["path"] as string | undefined;

  if (!file || !(file instanceof File)) {
    return c.json({ error: "No file provided" }, 400);
  }

  const filename = stripSpecialCharacters(file.name);
  const pathParts: string[] = pathInput
    ? pathInput.split("/").filter(Boolean)
    : [nanoid(), filename];
  
  const key = pathParts.join("/");
  const body = Buffer.from(await file.arrayBuffer());

  try {
    const result = await uploadFile({
      bucket,
      key,
      file: body,
      contentType: file.type,
    });

    return c.json(result, 200);
  } catch (err) {
    console.error("Upload error:", err);
    return c.json({ 
      error: "Upload failed", 
      details: err instanceof Error ? err.message : String(err) 
    }, 500);
  }
}
