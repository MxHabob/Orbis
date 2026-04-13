export type CertificateTemplate = {
  id: string;
  name: string;
  description: string | null;
  category: string;
  isDefault: boolean;
  isArchived: boolean;
  thumbnailUrl: string | null;
  version: number;
  content: any;
  createdAt: string | Date;
  updatedAt: string | Date;
};
