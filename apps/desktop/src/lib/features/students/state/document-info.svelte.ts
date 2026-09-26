import { api, type StudentDocumentFulfillmentDto, type DocumentRequirementDto } from "$lib/api";

export class DocumentInfoState {
  orgId: string;
  studentId?: string;

  orgRequiredDocuments = $state<string[]>([]);
  requirements = $state<DocumentRequirementDto[]>([]);
  fulfillments = $state<StudentDocumentFulfillmentDto[]>([]);
  submittedDocs = $state<string[]>([]);
  isLoading = $state(false);

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

  async populate(originalStudent: any) {
    if (!originalStudent || !this.studentId) return;

    try {
      await this.loadRequirements();
      const studentDocs = await api.listDocuments().catch(() => []);

      const submitted: string[] = [];
      for (const f of this.fulfillments) {
        if (f.is_fulfilled) {
          submitted.push(f.requirement.code);
        }
      }

      for (const d of studentDocs) {
        const name = d.name || "";
        if (name && !submitted.includes(name)) {
          submitted.push(name);
        }
      }

      this.submittedDocs = submitted;
    } catch (docErr) {
      console.error("Failed to load student documents:", docErr);
    }
  }
}
