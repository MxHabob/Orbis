import type { Document, UploadDocumentPayload } from '../types';

export interface DocumentCommands {
  uploadDocument(payload: UploadDocumentPayload): Promise<Document>;
  getDocuments(ownerId: string): Promise<Document[]>;
  deleteDocument(id: string): Promise<void>;
}
