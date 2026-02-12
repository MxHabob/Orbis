import { PutObjectCommand, S3Client } from "@aws-sdk/client-s3";

const endpoint = process.env.S3_ENDPOINT_URL ?? "http://localhost:9000";
const accessKey = process.env.MINIO_ROOT_USER || process.env.S3_ACCESS_KEY || "";
const secretKey = process.env.MINIO_ROOT_PASSWORD || process.env.S3_SECRET_KEY || "";
const region = process.env.S3_REGION ?? "us-east-1";

export const s3Client = new S3Client({
  endpoint,
  credentials: {
    accessKeyId: accessKey,
    secretAccessKey: secretKey,
  },
  region,
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
