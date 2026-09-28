import { api, type UserDto, type GuardianDto } from "$lib/api";

export interface GuardianLink {
  id?: string;
  guardian_id?: string;
  full_name: string;
  phone: string;
  email: string;
  national_id: string;
  relationship: string;
  relationship_ar: string;
  is_primary: boolean;
  is_emergency_contact?: boolean;
  is_authorized_pickup?: boolean;
  is_payer?: boolean;
  payment_share_percent?: number;
  occupation?: string;
  employer?: string;
  work_phone?: string;
  notes?: string;
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
      is_authorized_pickup: false,
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
      const gList: GuardianDto[] = await api.listGuardians(this.studentId);
      if (gList && gList.length > 0) {
        const guardiansWithUsers = await Promise.all(
          gList.map(async (g: GuardianDto) => {
            let u: UserDto | null = null;
            try {
              u = await api.getUser(g.guardian_id);
            } catch {}
            return {
              id: g.id,
              guardian_id: g.guardian_id,
              full_name: u?.full_name || u?.full_name_ar || "",
              phone: u?.phone || "",
              email: u?.email || "",
              national_id: u?.national_id || "",
              relationship: g.relationship || "other",
              relationship_ar: g.relationship_ar || g.relationship || "آخر",
              is_primary: g.is_primary,
              is_emergency_contact: g.is_emergency_contact,
              is_authorized_pickup: g.is_authorized_pickup,
              is_payer: g.is_payer,
              payment_share_percent: g.payment_share_percent ?? 100,
              occupation: g.occupation || "",
              employer: g.employer || "",
              work_phone: g.work_phone || "",
              notes: g.notes || "",
            };
          }),
        );
        this.linkedGuardians = guardiansWithUsers;
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
        is_authorized_pickup: false,
        is_payer: false,
        payment_share_percent: 0,
        occupation: "",
      },
    ];
  }

  removeGuardian(index: number) {
    if (this.linkedGuardians.length <= 1) {
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
          is_authorized_pickup: false,
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
          const nameMatch =
            (u.full_name || "").toLowerCase().includes(trimmed) ||
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
      guardian_id: parent.id,
      full_name: parent.full_name_ar || parent.full_name || parent.email || "",
      email: parent.email || "",
      phone: parent.phone || this.linkedGuardians[index].phone || "",
      national_id:
        parent.national_id || this.linkedGuardians[index].national_id || "",
      occupation: this.linkedGuardians[index].occupation || "",
    };

    this.parentSearchResults = [];
    this.activeSearchIndex = null;
  }
}
