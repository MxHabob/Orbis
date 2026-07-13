import { commands } from "$lib/core/ipc";

export interface GuardianLink {
  id?: string;
  user_id?: string;
  full_name: string;
  phone: string;
  email: string;
  national_id: string;
  relationship: string;
  relationship_ar: string;
  is_primary: boolean;
  is_emergency_contact?: boolean;
  is_payer?: boolean;
  payment_share_percent?: number;
  occupation?: string;
}

export class GuardianState {
  orgId: string;
  studentId?: string;

  linkedGuardians = $state<GuardianLink[]>([
    {
      full_name: "",
      phone: "",
      email: "",
      national_id: "",
      relationship: "father",
      relationship_ar: "الأب",
      is_primary: true,
      is_emergency_contact: true,
      is_payer: true,
      payment_share_percent: 100,
      occupation: "",
    },
  ]);

  parentSearchResults = $state<any[]>([]);
  activeSearchIndex = $state<number | null>(null);

  constructor(orgId: string, studentId?: string) {
    this.orgId = orgId;
    this.studentId = studentId;
  }

  async populate() {
    if (!this.studentId) return;
    try {
      const gList = await commands.academic.listGuardiansByStudent(
        this.orgId,
        this.studentId,
      );
      if (gList && gList.length > 0) {
        this.linkedGuardians = gList.map((g: any) => ({
          id: String(g.id),
          user_id: g.user_id ? String(g.user_id) : undefined,
          full_name: g.name || g.name_ar || "",
          phone: g.phone || "",
          email: g.email || "",
          national_id: g.national_id || "",
          relationship: g.relationship || "",
          relationship_ar: g.relationship_ar || "",
          is_primary: g.is_primary === 1,
          is_emergency_contact: g.is_emergency_contact === 1,
          is_payer: g.is_payer === 1,
          payment_share_percent: g.payment_share_percent,
          occupation: g.occupation,
        }));
      }
    } catch (e) {
      console.warn("Failed to load guardians:", e);
    }
  }

  addGuardian() {
    this.linkedGuardians = [
      ...this.linkedGuardians,
      {
        full_name: "",
        phone: "",
        email: "",
        national_id: "",
        relationship: "",
        relationship_ar: "",
        is_primary: false,
        is_emergency_contact: false,
        is_payer: false,
        payment_share_percent: 0,
        occupation: "",
      },
    ];
  }

  searchParent(query: string, index: number) {}
  selectExistingParent(parent: any, index: number) {}

  removeGuardian(index: number) {
    this.linkedGuardians = this.linkedGuardians.filter((_, i) => i !== index);
  }
}
