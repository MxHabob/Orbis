import { PutObjectCommand, S3Client } from "@aws-sdk/client-s3";

const endpoint = process.env.S3_ENDPOINT_URL ?? "http://172.16.21.51:9000";
const accessKey = process.env.MINIO_ROOT_USER ?? "minioadmin";
const secretKey = process.env.MINIO_ROOT_PASSWORD ?? "";
const region = process.env.S3_REGION ?? "us-east-1";

const s3Client = new S3Client({
  endpoint,
  credentials: {
    accessKeyId: accessKey,
    secretAccessKey: secretKey,
  },
  region,
  forcePathStyle: true,
});

/**
 * Upload file to MinIO (S3-compatible)
 * @returns public URL for the uploaded file
 */
export async function uploadToMinIO(params: {
  bucket: string;
  path: string[];
  file: File | Buffer;
  contentType?: string;
}): Promise<{ url: string; path: string }> {
  const key = params.path.join("/");
  const body = params.file instanceof Buffer ? params.file : Buffer.from(await params.file.arrayBuffer());
  const contentType = params.contentType ?? (params.file instanceof File ? params.file.type : "application/octet-stream");

  await s3Client.send(
    new PutObjectCommand({
      Bucket: params.bucket,
      Key: key,
      Body: body,
      ContentType: contentType,
    }),
  );

  // MinIO public URL: http://endpoint/bucket/key
  const baseUrl = endpoint.replace(/\/$/, "");
  const url = `${baseUrl}/${params.bucket}/${key}`;

  return { url, path: key };
}
