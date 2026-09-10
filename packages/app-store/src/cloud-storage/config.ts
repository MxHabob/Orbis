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
    "Store documents, backups, and media on local filesystem or MinIO / private S3 storage.",
  short_description_ar:
    "تخزين المستندات والنسخ الاحتياطية على نظام الملفات المحلي أو وحدات MinIO الخاصة.",
  description: `Manage document storage and automated backups for your on-premise institution.

**Document Storage**
Store student photos, scanned documents, report cards, and certificates locally or on private network storage.

**Automated Backups**
Schedule automatic database and document backups to secure on-premise or remote backup targets.

**Multiple Providers**
Support for Local Filesystem (Default), MinIO (Private Server), Amazon S3, and Google Cloud Storage.`,
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
      description: "Select cloud storage provider.",
      options: [
        { label: "Local Filesystem", value: "local" },
        { label: "Amazon S3", value: "s3" },
        { label: "Google Cloud Storage", value: "gcs" },
        { label: "MinIO", value: "minio" },
      ],
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
      required: false,
      value: "",
      description: "Access key ID (not needed for local).",
    },
    {
      id: "secret_key",
      label: "Secret Key",
      type: "password",
      required: false,
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
  ],
  adapter_name: "local_storage",
  provider_trait: "StorageProvider",
};

export default config;
