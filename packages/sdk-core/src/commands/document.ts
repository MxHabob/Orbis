import type { Document, UploadDocumentPayload, CursorPage, CursorRequest } from '../types';

export interface GetDocumentsParams extends CursorRequest {
  ownerId?: string;
  entityType?: string;
  category?: string;
}

export interface DocumentCommands {
  uploadDocument(payload: UploadDocumentPayload): Promise<Document>;
  getDocuments(params: GetDocumentsParams): Promise<CursorPage<Document>>;
  deleteDocument(id: string): Promise<void>;
}
