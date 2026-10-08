import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "cloud-storage",
  name: "Storage & Backups",
  name_ar: "التخزين والنسخ الاحتياطي",
  category: "storage",
  active: true,
  icon: "HardDrive",
  short_description:
    "Store documents, backups, and media on a configured local filesystem or S3-compatible provider.",
  short_description_ar:
    "تخزين المستندات والنسخ الاحتياطية على نظام الملفات المحلي أو وحدات MinIO الخاصة.",
  description: `Manage document storage and automated backups for your on-premise institution.

**Document Storage**
Store student photos, scanned documents, report cards, and certificates locally or on private network storage.

**Automated Backups**
Schedule automatic database and document backups to secure on-premise or remote backup targets.

**Multiple Providers**
Support for an explicitly configured local filesystem, Amazon S3, and MinIO / private S3-compatible storage.`,
  features: [
    "Local filesystem storage",
    "Private MinIO integration",
    "Automated backups",
    "Document management",
    "Strict access control",
  ],
  settings: [
    {
      id: "provider",
      label: "Storage Provider",
      type: "select",
      required: true,
      value: "local",
      description: "Select a storage provider.",
      options: [
        { label: "Local Filesystem", value: "local" },
        { label: "Amazon S3", value: "s3" },
        { label: "MinIO", value: "minio" },
      ],
    },
    {
      id: "local_path",
      label: "Storage Folder",
      label_ar: "مجلد التخزين",
      type: "directory",
      required: true,
      value: "",
      description: "Choose a folder on the machine running the Orbis backend.",
      description_ar: "اختر مجلداً على الجهاز الذي يعمل عليه خادم Orbis.",
      placeholder: "Select a folder",
    },
    {
      id: "bucket",
      label: "Bucket Name",
      type: "text",
      required: true,
      value: "",
      description: "Storage bucket or container name.",
      placeholder: "orbis-school-docs",
    },
    {
      id: "access_key",
      label: "Access Key",
      type: "text",
      required: true,
      value: "",
      description: "Access key ID (not needed for local).",
    },
    {
      id: "secret_key",
      label: "Secret Key",
      type: "password",
      required: true,
      value: "",
      description: "Secret access key (not needed for local).",
    },
    {
      id: "region",
      label: "Region",
      type: "text",
      required: false,
      value: "",
      description: "Cloud region (e.g. us-east-1).",
      placeholder: "us-east-1",
    },
    {
      id: "endpoint",
      label: "S3-Compatible Endpoint",
      type: "url",
      required: false,
      value: "",
      description: "Required for MinIO or other private S3-compatible storage.",
      placeholder: "https://storage.example.com",
    },
  ],
  adapter_name: "storage",
  legacy_provider_name: "local_storage",
  provider_trait: "StorageProvider",
};

export default config;
