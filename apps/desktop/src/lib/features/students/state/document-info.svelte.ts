import {
  api,
  type StudentDocumentFulfillmentDto,
  type DocumentRequirementDto,
  type DocumentDto,
  type StudentDetailDto,
} from "$lib/api";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";

export interface StudentDocAttachment {
  requirementCode: string;
  documentId?: string;
  fileName?: string;
  fileSize?: number;
  mimeType?: string;
  url?: string;
  file?: File;
  isUploaded: boolean;
}

export class DocumentInfoState {
  orgId: string;
  studentId?: string;

  orgRequiredDocuments = $state<string[]>([]);
  requirements = $state<DocumentRequirementDto[]>([]);
  fulfillments = $state<StudentDocumentFulfillmentDto[]>([]);
  submittedDocs = $state<string[]>([]);
  attachments = $state<Record<string, StudentDocAttachment>>({});
  isLoading = $state(false);
  isUploading = $state<Record<string, boolean>>({});

  constructor(orgId: string, studentId?: string) {
    this.orgId = orgId;
    this.studentId = studentId;
  }

  toggleDoc(docId: string) {
    if (this.submittedDocs.includes(docId)) {
      this.submittedDocs = this.submittedDocs.filter((d) => d !== docId);
    } else {
      this.submittedDocs = [...this.submittedDocs, docId];
    }
  }

  setRequiredDocuments(docs: string[]) {
    this.orgRequiredDocuments = docs || [];
  }

  async loadRequirements(gradeLevelId?: string, isNational = true, isTransfer = false) {
    this.isLoading = true;
    try {
      if (this.studentId) {
        this.fulfillments = await api.evaluateStudentDocumentRequirements(this.studentId, {
          grade_level_id: gradeLevelId,
          is_national: isNational,
          is_transfer: isTransfer,
        });
      } else {
        this.requirements = await api.listDocumentRequirements().catch(() => []);
      }
    } catch (err) {
      console.error("Failed to load document requirements:", err);
    } finally {
      this.isLoading = false;
    }
  }

  attachDocumentFile(reqCode: string, file: File) {
    if (!file) return;

    this.attachments[reqCode] = {
      requirementCode: reqCode,
      fileName: file.name,
      fileSize: file.size,
      mimeType: file.type,
      file,
      isUploaded: false,
    };

    if (!this.submittedDocs.includes(reqCode)) {
      this.submittedDocs = [...this.submittedDocs, reqCode];
    }

    toast.success(m.operations_ui_student_document_attached({ fileName: file.name }));
  }

  async uploadPendingAttachments(
    targetStudentId: string,
    folderId: string,
  ): Promise<string[]> {
    if (!targetStudentId || !folderId) return [];

    const pendingKeys = Object.keys(this.attachments).filter((k) => {
      const att = this.attachments[k];
      return att && att.file && !att.documentId;
    });
    const errors: string[] = [];

    for (const key of pendingKeys) {
      const att = this.attachments[key];
      const file = att?.file;
      if (!att || !file) continue;

      this.isUploading[key] = true;
      try {
        const doc = await api.uploadFile({
          file,
          name: att.fileName || file.name,
          entity_type: "student",
          entity_id: targetStudentId,
          folder_id: folderId,
          category: att.requirementCode,
          asset_type: "document",
          visibility: "private",
        });

        this.attachments[key] = {
          ...att,
          documentId: doc.id,
          url: doc.url || undefined,
          isUploaded: true,
          file: undefined,
        };
      } catch (err) {
        console.error(`Failed to upload attached document ${key}:`, err);
        errors.push(
          m.students_archive_upload_failed({
            fileName: att.fileName || file.name,
            error: err instanceof Error ? err.message : String(err),
          }),
        );
      } finally {
        this.isUploading[key] = false;
      }
    }

    return errors;
  }

  removeAttachment(reqCode: string) {
    const existing = this.attachments[reqCode];
    if (existing?.documentId) {
      api.deleteDocument(existing.documentId).catch((err) => console.warn("Delete doc error:", err));
    }
    delete this.attachments[reqCode];
    this.submittedDocs = this.submittedDocs.filter((d) => d !== reqCode);
  }

  async populate(originalStudent: StudentDetailDto | null) {
    if (!originalStudent || !this.studentId) return;

    try {
      await this.loadRequirements();
      const studentDocs = await api.listDocumentsByEntity("student", this.studentId).catch(() => []);

      const submitted: string[] = [];
      const attachMap: Record<string, StudentDocAttachment> = {};

      for (const f of this.fulfillments) {
        if (f.is_fulfilled) {
          submitted.push(f.requirement.code);
          if (f.document_id) {
            attachMap[f.requirement.code] = {
              requirementCode: f.requirement.code,
              documentId: f.document_id,
              fileName: f.document_name || f.requirement.code,
              isUploaded: true,
            };
          }
        }
      }

      for (const d of studentDocs) {
        const cat = d.category || d.name || "";
        if (cat && !submitted.includes(cat)) {
          submitted.push(cat);
        }
        if (cat && !attachMap[cat]) {
          attachMap[cat] = {
            requirementCode: cat,
            documentId: d.id,
            fileName: d.name,
            fileSize: d.file_size_bytes ?? undefined,
            mimeType: d.mime_type ?? undefined,
            url: d.url ?? undefined,
            isUploaded: true,
          };
        }
      }

      this.submittedDocs = submitted;
      this.attachments = attachMap;
    } catch (docErr) {
      console.error("Failed to load student documents:", docErr);
    }
  }
}
