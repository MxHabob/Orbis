import { api, type StudentDocumentFulfillmentDto, type DocumentRequirementDto, type DocumentDto } from "$lib/api";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";

export interface StudentDocAttachment {
  requirementCode: string;
  documentId?: string;
  fileName?: string;
  fileSize?: number;
  mimeType?: string;
  url?: string;
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

  async uploadDocumentFile(reqCode: string, file: File) {
    if (!file) return;
    this.isUploading[reqCode] = true;

    try {
      const targetEntityId = this.studentId || `temp_student_${Date.now()}`;
      const doc = await api.uploadDocument({
        name: file.name,
        mime_type: file.type || "application/octet-stream",
        entity_type: "student",
        entity_id: targetEntityId,
        category: reqCode,
        visibility: "private",
        file_path: file.name,
        file_size_bytes: file.size,
      });

      this.attachments[reqCode] = {
        requirementCode: reqCode,
        documentId: doc.id,
        fileName: file.name,
        fileSize: file.size,
        mimeType: file.type,
        url: doc.url || undefined,
        isUploaded: true,
      };

      if (!this.submittedDocs.includes(reqCode)) {
        this.submittedDocs = [...this.submittedDocs, reqCode];
      }

      toast.success(l(`تم رفع المستند "${file.name}" وتوثيقه في الأرشيف`, `Document "${file.name}" uploaded to archive`));
    } catch (err: any) {
      console.error("Failed to upload document file:", err);
      toast.error(l("فشل رفع المستند إلى الأرشيف", "Failed to upload document to archive"));
    } finally {
      this.isUploading[reqCode] = false;
    }
  }

  removeAttachment(reqCode: string) {
    const existing = this.attachments[reqCode];
    if (existing?.documentId) {
      api.deleteDocument(existing.documentId).catch((err) => console.warn("Delete doc error:", err));
    }
    delete this.attachments[reqCode];
    this.submittedDocs = this.submittedDocs.filter((d) => d !== reqCode);
  }

  async populate(originalStudent: any) {
    if (!originalStudent || !this.studentId) return;

    try {
      await this.loadRequirements();
      const studentDocs = await api.listDocuments().catch(() => []);

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
