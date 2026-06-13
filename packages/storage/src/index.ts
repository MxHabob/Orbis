import {
  CreateBucketCommand,
  DeleteObjectCommand,
  DeleteObjectsCommand,
  GetObjectCommand,
  HeadBucketCommand,
  PutObjectCommand,
  S3Client,
} from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";

const endpoint = process.env.S3_ENDPOINT_URL ?? "http://localhost:9000";
const accessKeyId =
  process.env.S3_ACCESS_KEY_ID ||
  process.env.STORAGE_ACCESS_KEY ||
  process.env.RUSTFS_ROOT_USER ||
  "rustfsadmin";
const secretAccessKey =
  process.env.S3_SECRET_ACCESS_KEY ||
  process.env.STORAGE_SECRET_KEY ||
  process.env.RUSTFS_ROOT_PASSWORD ||
  "rustfs_local_2026";
const region = process.env.S3_REGION ?? "us-east-1";

export const s3Client = new S3Client({
  endpoint,
  region,
  credentials: {
    accessKeyId,
    secretAccessKey,
  },
  forcePathStyle: true,
});

// ── Bucket auto-creation ────────────────────────────────────────────────────
const verifiedBuckets = new Set<string>();

async function ensureBucket(bucket: string): Promise<void> {
  if (verifiedBuckets.has(bucket)) return;

  try {
    await s3Client.send(new HeadBucketCommand({ Bucket: bucket }));
    verifiedBuckets.add(bucket);
  } catch (err: any) {
    const code = err?.name || err?.$metadata?.httpStatusCode;
    // Bucket does not exist — create it
    if (
      code === "NotFound" ||
      code === "NoSuchBucket" ||
      code === 404 ||
      code === 403
    ) {
      try {
        await s3Client.send(new CreateBucketCommand({ Bucket: bucket }));
        console.log(`[Storage] Created bucket: ${bucket}`);
        verifiedBuckets.add(bucket);
      } catch (createErr: any) {
        // BucketAlreadyOwnedByYou / BucketAlreadyExists — fine, another process created it
        if (
          createErr?.name === "BucketAlreadyOwnedByYou" ||
          createErr?.name === "BucketAlreadyExists"
        ) {
          verifiedBuckets.add(bucket);
        } else {
          console.error(
            `[Storage] Failed to create bucket '${bucket}':`,
            createErr,
          );
          throw createErr;
        }
      }
    } else {
      console.error(
        `[Storage] Failed to check bucket '${bucket}':`,
        err,
      );
      throw err;
    }
  }
}

export async function uploadFile(params: {
  bucket: string;
  key: string;
  file: Buffer | Uint8Array;
  contentType?: string;
}): Promise<{ url: string; path: string }> {
  // Auto-create bucket if it does not exist
  await ensureBucket(params.bucket);

  await s3Client.send(
    new PutObjectCommand({
      Bucket: params.bucket,
      Key: params.key,
      Body: params.file,
      ContentType: params.contentType ?? "application/octet-stream",
    }),
  );

  const baseUrl = endpoint.replace(/\/$/, "");
  const url = `${baseUrl}/${params.bucket}/${params.key}`;

  return { url, path: params.key };
}

export async function getDownloadUrl(params: {
  bucket: string;
  key: string;
  expiresIn?: number;
}): Promise<string> {
  const command = new GetObjectCommand({
    Bucket: params.bucket,
    Key: params.key,
  });

  return getSignedUrl(s3Client, command, {
    expiresIn: params.expiresIn ?? 3600,
  });
}

export async function getFile(params: {
  bucket: string;
  key: string;
  range?: string | null;
}) {
  const result = await s3Client.send(
    new GetObjectCommand({
      Bucket: params.bucket,
      Key: params.key,
      Range: params.range ?? undefined,
    }),
  );

  return result;
}

export async function deleteFile(params: {
  bucket: string;
  key: string;
}): Promise<void> {
  await s3Client.send(
    new DeleteObjectCommand({
      Bucket: params.bucket,
      Key: params.key,
    }),
  );
}

export async function deleteFiles(params: {
  bucket: string;
  keys: string[];
}): Promise<void> {
  if (params.keys.length === 0) return;

  await s3Client.send(
    new DeleteObjectsCommand({
      Bucket: params.bucket,
      Delete: {
        Objects: params.keys.map((key) => ({ Key: key })),
      },
    }),
  );
}
