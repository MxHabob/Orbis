export interface Document {
  id: string;
  org_id: string;
  uploaded_by?: string;
  entity_type?: string;
  entity_id?: string;
  name: string;
  description?: string;
  category: string;
  visibility: string;
  size?: number;
  mime_type?: string;
  extension?: string;
  storage_provider: string;
  path: string;
  url?: string;
  deleted_at?: string;
  created_at: string;
  updated_at: string;
}

export interface UploadDocumentPayload {
  entity_type: string;
  entity_id: string;
  category: string;
  visibility: string;
  file: File;
}
