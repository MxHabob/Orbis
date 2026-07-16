export interface Document {
  id: string;
  org_id: string;
  owner_id: string;
  uploaded_by?: string;
  name: string;
  title?: string;
  description?: string;
  type: string;
  visibility: string;
  size?: number;
  mime_type?: string;
  extension?: string;
  storage_provider: string;
  bucket?: string;
  key: string;
  url?: string;
  thumbnail_url?: string;
  parent_id?: string;
  path?: string;
  depth?: number;
  shared_with?: Record<string, any>;
  share_link?: string;
  share_link_expires_at?: string;
  share_password?: string;
  version?: number;
  previous_version_id?: string;
  is_latest_version: boolean;
  processing_status?: string;
  ocr_text?: string;
  extracted_data?: Record<string, any>;
  is_pinned: boolean;
  is_starred: boolean;
  is_template: boolean;
  tags?: Record<string, any>;
  meta?: Record<string, any>;
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
