export type CertificateTemplate = {
  id: string;
  name: string;
  isDefault: boolean;
  orientation: string;
  size: string;
  primaryColor: string | null;
  backgroundImageUrl: string | null;
  content: any;
  createdAt: string | Date;
  updatedAt: string | Date;
};
