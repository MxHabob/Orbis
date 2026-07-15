import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "cloud-storage",
  name: "Cloud Storage",
  name_ar: "التخزين السحابي",
  category: "storage",
  active: true,
  icon: "Cloud",
  short_description: "Store documents, backups, and media in Amazon S3, Google Cloud Storage, or MinIO.",
  short_description_ar: "تخزين المستندات والنسخ الاحتياطية في Amazon S3 أو Google Cloud أو MinIO.",
  description: `Connect cloud storage for centralized document management and automated backups.

**Document Storage**
Store student photos, scanned documents, report cards, and certificates in the cloud.

**Automated Backups**
Schedule automatic database and document backups to cloud storage.

**Multiple Providers**
Support for Amazon S3, Google Cloud Storage, MinIO, and S3-compatible providers.`,
  features: ["S3-compatible storage", "Automated backups", "Document management", "Pre-signed URLs", "Lifecycle policies"],
  settings: [
    { id: "provider", label: "Storage Provider", type: "select", required: true, value: "local", description: "Select cloud storage provider.", options: [{ label: "Local Filesystem", value: "local" }, { label: "Amazon S3", value: "s3" }, { label: "Google Cloud Storage", value: "gcs" }, { label: "MinIO", value: "minio" }] },
    { id: "bucket", label: "Bucket Name", type: "text", required: true, value: "", description: "Storage bucket or container name.", placeholder: "orbis-school-docs" },
    { id: "access_key", label: "Access Key", type: "text", required: false, value: "", description: "Access key ID (not needed for local)." },
    { id: "secret_key", label: "Secret Key", type: "password", required: false, value: "", description: "Secret access key (not needed for local)." },
    { id: "region", label: "Region", type: "text", required: false, value: "", description: "Cloud region (e.g. us-east-1).", placeholder: "us-east-1" },
  ],
  adapter_name: "local_storage",
  provider_trait: "StorageProvider",
};

export default config;
