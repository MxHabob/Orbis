import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";

export class DocumentInfoState {
  orgId: string;
  studentId?: string;

  orgRequiredDocuments = $state<string[]>([]);
  submittedDocs = $state<string[]>([]);

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

  async populate(originalStudent: any) {
    if (!originalStudent || !this.studentId) return;

    try {
      const studentDocs = await api.listDocuments();

      const submitted: any[] = [];
      const requiredDocNames =
        this.orgRequiredDocuments.length > 0
          ? this.orgRequiredDocuments
          : ["birth_certificate", "national_id", "previous_record"];

      for (const rDoc of requiredDocNames) {
        const hasDoc = studentDocs.some((d: any) => {
          const title = String(d.title || d.name || "").toLowerCase();
          const rName = rDoc.toLowerCase();
          return (
            title === rName || title.includes(rName) || rName.includes(title)
          );
        });
        if (hasDoc) {
          submitted.push(rDoc);
        }
      }
      this.submittedDocs = submitted;
    } catch (docErr) {
      console.error("Failed to load student documents:", docErr);
    }
  }
}
