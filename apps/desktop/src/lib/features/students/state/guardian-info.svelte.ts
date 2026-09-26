import { api, type UserDto } from "$lib/api";

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

  parentSearchResults = $state<UserDto[]>([]);
  activeSearchIndex = $state<number | null>(null);
  isSearching = $state(false);

  private searchDebounceTimer?: ReturnType<typeof setTimeout>;

  constructor(orgId: string, studentId?: string) {
    this.orgId = orgId;
    this.studentId = studentId;
  }

  async populate() {
    if (!this.studentId) return;
    try {
      const gList = await api.listGuardians(this.studentId);
      if (gList && gList.length > 0) {
        this.linkedGuardians = gList.map((g: any) => ({
          id: String(g.id),
          user_id: g.guardian_id ? String(g.guardian_id) : undefined,
          full_name: g.name || g.name_ar || "",
          phone: g.phone || "",
          email: g.email || "",
          national_id: g.national_id || "",
          relationship: g.relationship || "",
          relationship_ar: g.relationship_ar || g.relationship || "",
          is_primary: g.is_primary === 1 || g.is_primary === true,
          is_emergency_contact:
            g.is_emergency_contact === 1 || g.is_emergency_contact === true,
          is_payer: g.is_payer === 1 || g.is_payer === true,
          payment_share_percent: g.payment_share_percent ?? 100,
          occupation: g.occupation || "",
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
        relationship: "other",
        relationship_ar: "آخر",
        is_primary: false,
        is_emergency_contact: false,
        is_payer: false,
        payment_share_percent: 0,
        occupation: "",
      },
    ];
  }

  removeGuardian(index: number) {
    if (this.linkedGuardians.length <= 1) {
      // Keep at least one guardian structure ready
      this.linkedGuardians = [
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
      ];
      return;
    }
    this.linkedGuardians = this.linkedGuardians.filter((_, i) => i !== index);
  }

  async searchParent(query: string, index: number) {
    this.activeSearchIndex = index;
    const trimmed = (query || "").trim().toLowerCase();

    if (this.searchDebounceTimer) {
      clearTimeout(this.searchDebounceTimer);
    }

    if (!trimmed || trimmed.length < 2) {
      this.parentSearchResults = [];
      return;
    }

    this.searchDebounceTimer = setTimeout(async () => {
      this.isSearching = true;
      try {
        const users = await api.listUsers().catch(() => []);
        const filtered = (users || []).filter((u: UserDto) => {
          const nameMatch = (u.full_name || "").toLowerCase().includes(trimmed) || 
                            (u.full_name_ar || "").toLowerCase().includes(trimmed);
          const emailMatch = (u.email || "").toLowerCase().includes(trimmed);
          const phoneMatch = (u.phone || "").includes(trimmed);
          return nameMatch || emailMatch || phoneMatch;
        });
        this.parentSearchResults = filtered.slice(0, 8);
      } catch (err) {
        console.error("Failed to search registered parents:", err);
        this.parentSearchResults = [];
      } finally {
        this.isSearching = false;
      }
    }, 250);
  }

  selectExistingParent(parent: UserDto, index: number) {
    if (!this.linkedGuardians[index]) return;

    this.linkedGuardians[index] = {
      ...this.linkedGuardians[index],
      user_id: parent.id,
      full_name: parent.full_name_ar || parent.full_name || parent.email || "",
      email: parent.email || "",
      phone: parent.phone || this.linkedGuardians[index].phone || "",
      national_id: parent.national_id || this.linkedGuardians[index].national_id || "",
      occupation: this.linkedGuardians[index].occupation || "",
    };

    this.parentSearchResults = [];
    this.activeSearchIndex = null;
  }
}
