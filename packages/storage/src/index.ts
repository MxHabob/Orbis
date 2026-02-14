import { PutObjectCommand, GetObjectCommand, DeleteObjectCommand, S3Client } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";

const endpoint = process.env.S3_ENDPOINT_URL ?? "http://localhost:9000";
const accessKeyId = process.env.S3_ACCESS_KEY_ID || process.env.MINIO_ROOT_USER || "minioadmin";
const secretAccessKey = process.env.S3_SECRET_ACCESS_KEY || process.env.MINIO_ROOT_PASSWORD || "zsftjlvfneswwb0h";
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

export async function uploadFile(params: {
  bucket: string;
  key: string;
  file: Buffer | Uint8Array;
  contentType?: string;
}): Promise<{ url: string; path: string }> {
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
    expiresIn: params.expiresIn ?? 3600 
  });
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
